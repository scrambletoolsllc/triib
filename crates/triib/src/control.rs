//! The control channel: how other programs, such as `triib --mcp`, call
//! into the running triib: a socket in triib's private runtime folder on
//! Linux and macOS, a named pipe for the user on Windows. The triib
//! started last serves it, taking it over from one before.
//!
//! A connection stays open and carries JSON-RPC 2.0, one request and its
//! reply per line, answered in the order they came. The app gets each
//! request as a [`Call`] and replies through it, from any thread.

use std::io::{self, BufRead, BufReader, Read, Write};
use std::sync::{Arc, Mutex, mpsc};

use interprocess::local_socket::{ListenerOptions, Name, Stream, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The control channel's version, which `ping` reports; it goes up when
/// requests change in ways a client must know about.
pub const PROTOCOL: u32 = 1;

/// Longest request line taken, in bytes; longer ones end the connection.
const MAX_LINE: usize = 64 << 20;

/// JSON-RPC's error codes, and triib's own.
pub mod code {
    pub const PARSE_ERROR: i64 = -32700;
    pub const INVALID_REQUEST: i64 = -32600;
    pub const METHOD_NOT_FOUND: i64 = -32601;
    pub const INVALID_PARAMS: i64 = -32602;
    pub const INTERNAL_ERROR: i64 = -32603;
    /// The app went away, or dropped the call without answering.
    pub const NO_ANSWER: i64 = -32000;
    /// Outside control is off in Settings.
    pub const OFF: i64 = -32001;
    /// The user did not allow the agent to control triib.
    pub const DECLINED: i64 = -32002;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

impl Error {
    pub fn new(code: i64, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}

impl std::error::Error for Error {}

/// A request from a client, for the app to carry out and answer.
#[derive(Clone)]
pub struct Call {
    pub method: String,
    pub params: Value,
    reply: Reply,
}

impl std::fmt::Debug for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Call")
            .field("method", &self.method)
            .field("params", &self.params)
            .finish_non_exhaustive()
    }
}

impl Call {
    /// A call made inside triib, with no connection: its answer goes to
    /// `on_done`, on a thread of its own, and its notes are dropped.
    #[cfg(test)]
    pub fn local(
        method: &str,
        params: Value,
        on_done: impl FnOnce(Result<Value, Error>) + Send + 'static,
    ) -> Call {
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            let outcome = loop {
                match receiver.recv() {
                    Ok(Event::Note(_)) => {}
                    Ok(Event::Done(outcome)) => break outcome,
                    Err(_) => break Err(Error::new(code::NO_ANSWER, "triib did not answer")),
                }
            };
            on_done(outcome);
        });
        Call {
            method: method.to_owned(),
            params,
            reply: Reply(Arc::new(Mutex::new(Some(sender)))),
        }
    }

    /// Answers the call; only the first answer counts.
    pub fn reply(&self, result: Result<Value, Error>) {
        self.reply.send(result);
    }

    /// Tells the client something about the call before its answer, as a
    /// JSON-RPC notification on the call's connection.
    pub fn note(&self, method: &str, params: Value) {
        self.reply.note(serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        }));
    }
}

/// Where a call's answer goes. Clonable, as the app's messages are; the
/// answer is sent once, and a call dropped unanswered tells the client.
#[derive(Clone)]
struct Reply(Arc<Mutex<Option<Answer>>>);

/// The connection's end of a call, waiting for its answer.
type Answer = mpsc::Sender<Event>;

/// What the connection hears about a call.
enum Event {
    /// A notification to pass on before the answer.
    Note(Value),
    Done(Result<Value, Error>),
}

impl Reply {
    fn send(&self, result: Result<Value, Error>) {
        if let Some(sender) = self
            .0
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .take()
        {
            let _ = sender.send(Event::Done(result));
        }
    }

    fn note(&self, note: Value) {
        if let Some(sender) = self
            .0
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .as_ref()
        {
            let _ = sender.send(Event::Note(note));
        }
    }
}

#[derive(Deserialize)]
struct Request {
    jsonrpc: Option<String>,
    id: Option<Value>,
    method: Option<String>,
    #[serde(default)]
    params: Value,
}

#[derive(Serialize)]
struct Response<'a> {
    jsonrpc: &'static str,
    id: &'a Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Error>,
}

/// Where the channel is: its name, and on Linux and macOS the socket
/// file, which is made private to the user once it exists.
#[derive(Debug, Clone)]
pub struct Address {
    name: Name<'static>,
    file: Option<std::path::PathBuf>,
}

impl Address {
    /// A channel by name alone, without a socket file.
    #[cfg(any(windows, test))]
    pub fn named(name: Name<'static>) -> Self {
        Self { name, file: None }
    }
}

/// The channel's address for this user and build: development builds have
/// their own, as they have their own runtime folder.
pub fn address() -> io::Result<Address> {
    #[cfg(unix)]
    {
        use interprocess::local_socket::{GenericFilePath, ToFsName};
        let dir = triib_store::paths::runtime_dir()
            .ok_or_else(|| io::Error::other("no runtime folder for the control socket"))?;
        let file = dir.join(SOCKET_NAME);
        Ok(Address {
            name: file.clone().to_fs_name::<GenericFilePath>()?,
            file: Some(file),
        })
    }
    #[cfg(windows)]
    {
        use interprocess::local_socket::{GenericNamespaced, ToNsName};
        let app = if triib_store::paths::PRODUCTION {
            "triib"
        } else {
            "triib-dev"
        };
        let user = std::env::var("USERNAME").unwrap_or_default();
        Ok(Address::named(
            format!("{app}-{user}-control").to_ns_name::<GenericNamespaced>()?,
        ))
    }
}

/// The socket's file name on Linux and macOS, in the runtime folder.
#[cfg(unix)]
pub const SOCKET_NAME: &str = "control.sock";

/// Starts serving the channel at `address` on background threads, handing
/// each request to `on_call`. It takes the name over from a triib before,
/// one that quit without cleaning up or that still runs.
pub fn serve(address: Address, on_call: impl Fn(Call) + Send + Sync + 'static) -> io::Result<()> {
    if let Some(dir) = address.file.as_ref().and_then(|file| file.parent()) {
        std::fs::create_dir_all(dir)?;
    }
    let listener = ListenerOptions::new()
        .name(address.name)
        .try_overwrite(true)
        .create_sync()?;
    #[cfg(unix)]
    if let Some(file) = &address.file {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(file, std::fs::Permissions::from_mode(0o600))?;
    }
    let on_call = Arc::new(on_call);
    std::thread::Builder::new()
        .name("triib-control".into())
        .spawn(move || {
            for stream in listener.incoming() {
                let Ok(stream) = stream else { continue };
                let on_call = Arc::clone(&on_call);
                let _ = std::thread::Builder::new()
                    .name("triib-control-client".into())
                    .spawn(move || {
                        let _ = serve_connection(stream, &*on_call);
                    });
            }
        })?;
    Ok(())
}

/// Reads requests from one client until it hangs up, answering each.
fn serve_connection(stream: Stream, on_call: &dyn Fn(Call)) -> io::Result<()> {
    let (receive, mut send) = stream.split();
    let mut lines = BufReader::new(receive);
    let mut line = Vec::new();
    loop {
        line.clear();
        let read = (&mut lines)
            .take(MAX_LINE as u64 + 1)
            .read_until(b'\n', &mut line)?;
        if read == 0 {
            return Ok(());
        }
        if line.len() > MAX_LINE {
            return Err(io::Error::other("request too long"));
        }
        if line.iter().all(u8::is_ascii_whitespace) {
            continue;
        }
        let mut note = |mut bytes: Vec<u8>| {
            bytes.push(b'\n');
            let _ = send.write_all(&bytes);
            let _ = send.flush();
        };
        let Some(reply) = answer(&line, on_call, &mut note) else {
            continue;
        };
        send.write_all(&reply)?;
        send.flush()?;
    }
}

/// The reply line to one request line, or `None` for a notification,
/// which JSON-RPC answers with nothing. Notes the call sends before its
/// answer go to `note` as they come.
fn answer(line: &[u8], on_call: &dyn Fn(Call), note: &mut dyn FnMut(Vec<u8>)) -> Option<Vec<u8>> {
    let null = Value::Null;
    let respond = |id: &Value, outcome: Result<Value, Error>| {
        let (result, error) = match outcome {
            Ok(result) => (Some(result), None),
            Err(error) => (None, Some(error)),
        };
        let response = Response {
            jsonrpc: "2.0",
            id,
            result,
            error,
        };
        let mut bytes = serde_json::to_vec(&response).unwrap_or_default();
        bytes.push(b'\n');
        bytes
    };
    let request: Request = match serde_json::from_slice(line) {
        Ok(request) => request,
        Err(error) => {
            return Some(respond(
                &null,
                Err(Error::new(code::PARSE_ERROR, error.to_string())),
            ));
        }
    };
    let id = request.id.clone();
    let invalid = |message: &str| {
        Some(respond(
            id.as_ref().unwrap_or(&null),
            Err(Error::new(code::INVALID_REQUEST, message)),
        ))
    };
    if request.jsonrpc.as_deref() != Some("2.0") {
        return invalid("jsonrpc must be \"2.0\"");
    }
    let Some(method) = request.method else {
        return invalid("no method");
    };
    let (sender, receiver) = mpsc::channel();
    on_call(Call {
        method,
        params: request.params,
        reply: Reply(Arc::new(Mutex::new(Some(sender)))),
    });
    let outcome = loop {
        match receiver.recv() {
            Ok(Event::Note(params)) => note(serde_json::to_vec(&params).unwrap_or_default()),
            Ok(Event::Done(outcome)) => break outcome,
            Err(_) => break Err(Error::new(code::NO_ANSWER, "triib did not answer")),
        }
    };
    let id = request.id?;
    Some(respond(&id, outcome))
}

/// A connection to the running triib's control channel.
pub struct Client {
    send: interprocess::local_socket::SendHalf,
    receive: BufReader<interprocess::local_socket::RecvHalf>,
    next_id: u64,
}

impl Client {
    pub fn connect(address: &Address) -> io::Result<Self> {
        let (receive, send) = Stream::connect(address.name.clone())?.split();
        Ok(Self {
            send,
            receive: BufReader::new(receive),
            next_id: 1,
        })
    }

    /// Calls `method` with `params` and waits for the answer. The outer
    /// error is the connection's, the inner one triib's answer.
    #[cfg(test)]
    pub fn call(&mut self, method: &str, params: Value) -> io::Result<Result<Value, Error>> {
        self.call_noting(method, params, &mut |_, _| {})
    }

    /// Calls as [`Client::call`] does, handing `on_note` each notification
    /// triib sends about the call before its answer.
    pub fn call_noting(
        &mut self,
        method: &str,
        params: Value,
        on_note: &mut dyn FnMut(&str, Value),
    ) -> io::Result<Result<Value, Error>> {
        let id = self.next_id;
        self.next_id += 1;
        let mut line = serde_json::to_vec(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        }))?;
        line.push(b'\n');
        self.send.write_all(&line)?;
        self.send.flush()?;
        #[derive(Deserialize)]
        struct Reply {
            id: Value,
            result: Option<Value>,
            error: Option<Error>,
        }
        let reply = loop {
            let mut line = String::new();
            if self.receive.read_line(&mut line)? == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "triib closed the control channel",
                ));
            }
            let message: Value = serde_json::from_str(&line)?;
            match (
                message.get("id"),
                message.get("method").and_then(Value::as_str),
            ) {
                (None, Some(method)) => on_note(method, message["params"].clone()),
                _ => break serde_json::from_value::<Reply>(message)?,
            }
        };
        if reply.id != id {
            return Err(io::Error::other("the answer is for another request"));
        }
        Ok(match (reply.result, reply.error) {
            (_, Some(error)) => Err(error),
            (Some(result), None) => Ok(result),
            (None, None) => Ok(Value::Null),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use interprocess::local_socket::{GenericNamespaced, ToNsName};
    use serde_json::json;

    /// A name of its own for each test, as tests run side by side.
    fn test_name(test: &str) -> Address {
        Address::named(
            format!("triib-test-{}-{test}", std::process::id())
                .to_ns_name::<GenericNamespaced>()
                .unwrap(),
        )
    }

    fn echo(call: Call) {
        match call.method.as_str() {
            "echo" => call.reply(Ok(call.params.clone())),
            "slow" => {
                call.note("waiting", json!({ "message": "one" }));
                call.note("waiting", json!({ "message": "two" }));
                call.reply(Ok(json!("done")));
            }
            "fail" => call.reply(Err(Error::new(code::INVALID_PARAMS, "no"))),
            // Dropped without an answer.
            "drop" => {}
            _ => call.reply(Err(Error::new(code::METHOD_NOT_FOUND, "unknown"))),
        }
    }

    #[test]
    fn calls_are_answered_in_order_on_one_connection() {
        serve(test_name("order"), echo).unwrap();
        let mut client = Client::connect(&test_name("order")).unwrap();
        for n in 0..5 {
            let answer = client.call("echo", json!({ "n": n })).unwrap();
            assert_eq!(answer, Ok(json!({ "n": n })));
        }
        assert_eq!(
            client.call("fail", Value::Null).unwrap(),
            Err(Error::new(code::INVALID_PARAMS, "no"))
        );
        assert_eq!(
            client.call("nope", Value::Null).unwrap().unwrap_err().code,
            code::METHOD_NOT_FOUND
        );
        assert_eq!(
            client.call("drop", Value::Null).unwrap().unwrap_err().code,
            code::NO_ANSWER
        );
    }

    #[test]
    fn calls_from_other_threads_are_answered() {
        // The app answers later, from another thread, as iced does.
        serve(test_name("thread"), |call| {
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(20));
                call.reply(Ok(json!("later")));
            });
        })
        .unwrap();
        let mut first = Client::connect(&test_name("thread")).unwrap();
        let mut second = Client::connect(&test_name("thread")).unwrap();
        assert_eq!(first.call("a", Value::Null).unwrap(), Ok(json!("later")));
        assert_eq!(second.call("b", Value::Null).unwrap(), Ok(json!("later")));
    }

    #[test]
    fn local_calls_answer_their_callback() {
        let (sender, receiver) = mpsc::channel();
        let answered = sender.clone();
        let call = Call::local("slow", Value::Null, move |outcome| {
            answered.send(outcome).unwrap();
        });
        echo(call);
        assert_eq!(receiver.recv().unwrap(), Ok(json!("done")));
        drop(Call::local("drop", Value::Null, move |outcome| {
            sender.send(outcome).unwrap();
        }));
        assert_eq!(receiver.recv().unwrap().unwrap_err().code, code::NO_ANSWER);
    }

    #[test]
    fn bad_requests_get_errors_and_notifications_nothing() {
        let on_call = |call: Call| call.reply(Ok(json!(true)));
        let parse = |line: &str| -> Value {
            serde_json::from_slice(&answer(line.as_bytes(), &on_call, &mut |_| {}).unwrap())
                .unwrap()
        };
        assert_eq!(parse("{not json")["error"]["code"], code::PARSE_ERROR);
        assert_eq!(
            parse(r#"{"id":1,"method":"x"}"#)["error"]["code"],
            code::INVALID_REQUEST
        );
        assert_eq!(
            parse(r#"{"jsonrpc":"2.0","id":2}"#)["error"]["code"],
            code::INVALID_REQUEST
        );
        let reply = parse(r#"{"jsonrpc":"2.0","id":"a","method":"x"}"#);
        assert_eq!(reply["id"], "a");
        assert_eq!(reply["result"], true);
        assert!(answer(br#"{"jsonrpc":"2.0","method":"x"}"#, &on_call, &mut |_| {}).is_none());
    }

    #[test]
    fn notes_come_before_the_answer() {
        serve(test_name("notes"), echo).unwrap();
        let mut client = Client::connect(&test_name("notes")).unwrap();
        let mut notes = Vec::new();
        let answer = client
            .call_noting("slow", Value::Null, &mut |method, params| {
                notes.push(format!("{method}: {}", params["message"]))
            })
            .unwrap();
        assert_eq!(answer, Ok(json!("done")));
        assert_eq!(notes, ["waiting: \"one\"", "waiting: \"two\""]);
        // The connection goes on as before.
        assert_eq!(client.call("echo", json!(1)).unwrap(), Ok(json!(1)));
    }
}
