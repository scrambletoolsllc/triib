//! `triib --mcp`: an MCP server on stdin and stdout for agents such as
//! Claude Code. It holds no tools of its own: it relays the agent's calls
//! to the running triib over the control channel, starting triib first if
//! it is not running.

use std::io;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation,
    InitializeRequestParams, InitializeResult, ListToolsResult, PaginatedRequestParams,
    ProgressNotificationParam, ServerCapabilities, ServerConfig,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer, ServerHandler, ServiceExt};
use serde_json::{Value, json};
use tokio::sync::mpsc::UnboundedSender;

use crate::control::{self, Client};

/// What agents read first about triib's tools.
const INSTRUCTIONS: &str = "Controls triib, an ATDECC (IEEE 1722.1) and Milan controller for \
AVB networks, running on the user's desktop, where the user may be watching. It acts on the \
network interface chosen in triib. Entities are AVB devices, known by their entity ID, \
written as 0x and 16 hex digits; their stream outputs and inputs are numbered from 0. \
list_entities gives the entities, describe_entity one's streams, formats, clocks and \
controls, list_connections what each listener input is bound to. Changes go to the devices \
on the network and take effect there at once, for everyone; there is no undo.";

/// How long to wait for a triib this command started to open the
/// control channel.
const START_TIME: Duration = Duration::from_secs(20);

/// Serves MCP until the agent closes stdin.
pub fn run() -> io::Result<()> {
    let client = connect()?;
    let relay = Relay {
        client: Arc::new(Mutex::new(client)),
        agent: Arc::new(Mutex::new(Value::Null)),
    };
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let service = relay
                .serve(rmcp::transport::stdio())
                .await
                .map_err(io::Error::other)?;
            service.waiting().await.map_err(io::Error::other)?;
            Ok(())
        })
}

/// Connects to the running triib, starting one if none answers.
fn connect() -> io::Result<Client> {
    let address = control::address()?;
    if let Ok(client) = Client::connect(&address) {
        return Ok(client);
    }
    start_triib()?;
    let started = Instant::now();
    loop {
        match Client::connect(&address) {
            Ok(client) => return Ok(client),
            Err(error) if started.elapsed() > START_TIME => return Err(error),
            Err(_) => std::thread::sleep(Duration::from_millis(100)),
        }
    }
}

/// Starts triib on its own, away from this command's stdin and stdout and
/// its process group, so it outlives the agent.
fn start_triib() -> io::Result<()> {
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(&mut command, 0);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x8;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x200;
        command.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
        keep_std_handles();
    }
    command.spawn().map(drop)
}

/// Stops triib from inheriting this command's stdin and stdout pipes:
/// Windows hands a new process every inheritable handle, and an agent
/// waits for the pipes to close, which a running triib would hold open.
#[cfg(windows)]
#[allow(unsafe_code)]
fn keep_std_handles() {
    use windows_sys::Win32::Foundation::{
        HANDLE_FLAG_INHERIT, INVALID_HANDLE_VALUE, SetHandleInformation,
    };
    use windows_sys::Win32::System::Console::{
        GetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
    };
    for which in [STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE] {
        // SAFETY: takes no pointers; a handle that is missing or not
        // inheritable is left as it is.
        unsafe {
            let handle = GetStdHandle(which);
            if !handle.is_null() && handle != INVALID_HANDLE_VALUE {
                SetHandleInformation(handle, HANDLE_FLAG_INHERIT, 0);
            }
        }
    }
}

#[derive(Clone)]
struct Relay {
    client: Arc<Mutex<Client>>,
    /// The agent's name, title and version, which triib asks the user
    /// about.
    agent: Arc<Mutex<Value>>,
}

impl Relay {
    /// Calls triib off the async thread, since the control client blocks.
    /// The outer error is a lost connection, the inner one triib's answer.
    async fn call(
        &self,
        method: &'static str,
        params: Value,
    ) -> Result<Result<Value, control::Error>, ErrorData> {
        self.call_noting(method, params, None).await
    }

    /// Calls as [`Relay::call`] does, passing what triib says while the
    /// call waits, such as for the user's answer, to `notes`.
    async fn call_noting(
        &self,
        method: &'static str,
        params: Value,
        notes: Option<UnboundedSender<String>>,
    ) -> Result<Result<Value, control::Error>, ErrorData> {
        let client = self.client.clone();
        let answer = tokio::task::spawn_blocking(move || {
            client
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .call_noting(method, params, &mut |_, note| {
                    if let (Some(notes), Some(message)) = (&notes, note["message"].as_str()) {
                        let _ = notes.send(message.to_owned());
                    }
                })
        })
        .await
        .map_err(|error| ErrorData::internal_error(error.to_string(), None))?;
        answer.map_err(|error| {
            ErrorData::internal_error(format!("lost the connection to triib: {error}"), None)
        })
    }

    /// Calls triib, where an answer that is an error fails the request.
    async fn call_or_fail(&self, method: &'static str, params: Value) -> Result<Value, ErrorData> {
        self.call(method, params)
            .await?
            .map_err(|error| ErrorData::internal_error(error.message, None))
    }
}

impl ServerHandler for Relay {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("triib", env!("CARGO_PKG_VERSION")))
            .with_instructions(INSTRUCTIONS)
    }

    async fn initialize(
        &self,
        request: InitializeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, ErrorData> {
        context.peer.set_peer_info(request.clone());
        // triib asks the user before the first call of an agent it has not
        // seen, so it needs the agent's name.
        let agent = json!({
            "name": request.client_info.name,
            "title": request.client_info.title,
            "version": request.client_info.version,
        });
        *self
            .agent
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = agent.clone();
        // With outside control off, the agent still connects, and hears
        // why at its first call.
        self.call("agent/hello", agent).await?.ok();
        self.negotiate_initialize(&request)
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let tools = self.call_or_fail("tools/list", json!({})).await?;
        serde_json::from_value(tools)
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        // What triib says while the call waits, such as that the user must
        // allow it, goes to the agent as progress, if it asked for that.
        let (notes, mut noted) = tokio::sync::mpsc::unbounded_channel::<String>();
        let peer = context.peer.clone();
        let token = context.meta.get_progress_token();
        let forward = tokio::spawn(async move {
            let mut progress = 0.0;
            while let Some(message) = noted.recv().await {
                progress += 1.0;
                if let Some(token) = &token {
                    let _ = peer
                        .notify_progress(
                            ProgressNotificationParam::new(token.clone(), progress)
                                .with_message(message),
                        )
                        .await;
                }
            }
        });
        let agent = self
            .agent
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        let answer = self
            .call_noting(
                "tools/call",
                json!({ "agent": agent, "name": request.name, "arguments": request.arguments }),
                Some(notes),
            )
            .await;
        // The notes end with the call; send the last of them before the
        // answer.
        let _ = forward.await;
        let answer = answer?;
        // triib's refusals go to the agent as the tool's failure, which it
        // reads, rather than as a broken request.
        let result = match answer {
            Ok(result) => serde_json::from_value::<CallToolResult>(result)
                .map_err(|error| ErrorData::internal_error(error.to_string(), None))?,
            Err(error) => CallToolResult::error(vec![ContentBlock::text(error.message)]),
        };
        Ok(result.into())
    }
}
