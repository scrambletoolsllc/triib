//! The tools triib offers agents through `triib --mcp`. Each has a name,
//! a kind, a description the model reads, an input schema made from its
//! input type, and a function that runs it as a click would. A tool that
//! reads answers at once; one that changes a device answers once the
//! device has, with what it said.

use std::sync::LazyLock;
use std::time::Duration;

use atdecc::aem::Counters;
use atdecc::controller::Outcome;
use atdecc::descriptor::SamplingRate;
use atdecc::model::{EntityModel, EnumerationFailure, EnumerationState};
use atdecc::stream_format::Fit;
use atdecc::{DescriptorType, EntityId, StreamFormat};
use iced::Task;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::app::{Message, NetworkState, Triib};
use crate::control::{Call, Error, code};
use crate::describe;
use crate::netmap::Status;
use crate::network::{Action, Name, NameTarget, Remedy};

/// What a tool does, which MCP's hints tell clients.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Reads what triib knows of the network.
    Read,
    /// Changes a device on the network.
    Change,
}

/// What a tool gives back.
#[derive(Debug, Clone)]
pub enum Output {
    /// An object, as structured content and the same as text for agents
    /// that read only text.
    Json(Value),
    Text(String),
}

impl Output {
    /// The output as an MCP tool result.
    fn into_result(self) -> Value {
        match self {
            Output::Json(value) => json!({
                "content": [{ "type": "text", "text": value.to_string() }],
                "structuredContent": value,
            }),
            Output::Text(text) => json!({ "content": [{ "type": "text", "text": text }] }),
        }
    }
}

/// Where a tool's answer goes. A tool that waits on a device keeps it and
/// answers once the device has; dropped unanswered, it tells the agent
/// triib did not answer.
#[derive(Debug, Clone)]
pub struct Answer(pub Call);

impl Answer {
    pub fn send(&self, result: Result<Output, Error>) {
        self.0.reply(result.map(Output::into_result));
    }
}

type Run = Box<dyn Fn(&mut Triib, Value, Answer) -> Task<Message> + Send + Sync>;

pub struct Tool {
    pub name: &'static str,
    pub title: &'static str,
    pub kind: Kind,
    description: &'static str,
    schema: fn() -> Value,
    run: Run,
}

/// A tool whose input is `I`, read from the call's arguments.
fn tool<I: DeserializeOwned + JsonSchema + 'static>(
    name: &'static str,
    title: &'static str,
    kind: Kind,
    description: &'static str,
    run: fn(&mut Triib, I, Answer) -> Task<Message>,
) -> Tool {
    Tool {
        name,
        title,
        kind,
        description,
        schema: || {
            let mut schema = serde_json::to_value(schemars::schema_for!(I)).unwrap_or_default();
            // The tool's own name and description say what these would.
            if let Some(schema) = schema.as_object_mut() {
                schema.remove("$schema");
                schema.remove("title");
                schema.remove("description");
                // Some clients need properties even when there are none.
                schema.entry("properties").or_insert_with(|| json!({}));
            }
            compact(&mut schema);
            schema
        },
        run: Box::new(
            move |app, arguments, answer| match serde_json::from_value::<I>(arguments) {
                Ok(input) => run(app, input, answer),
                Err(error) => reply(
                    &answer,
                    Err(Error::new(code::INVALID_PARAMS, format!("{name}: {error}"))),
                ),
            },
        ),
    }
}

/// Leaves out what a schema says only for a validator, as every agent
/// reads the schemas on every turn and triib checks the input itself: the
/// number formats and their bounds, `null` beside optional fields' types,
/// shared definitions (written where they are used instead) and the line
/// breaks of descriptions.
fn compact(schema: &mut Value) {
    let definitions = schema
        .as_object_mut()
        .and_then(|schema| schema.remove("$defs"))
        .unwrap_or_default();
    schema
        .as_object_mut()
        .map(|schema| schema.remove("additionalProperties"));
    tidy(schema, &definitions);
}

fn tidy(value: &mut Value, definitions: &Value) {
    match value {
        Value::Object(object) => {
            // An optional field is one of a type or null: the type alone.
            if let Some(Value::Array(any)) = object.get("anyOf")
                && any.len() == 2
                && let Some(other) = any.iter().find(|one| one["type"] != "null")
                && any.iter().any(|one| one["type"] == "null")
            {
                let other = other.clone();
                object.remove("anyOf");
                if let Value::Object(other) = other {
                    for (key, value) in other {
                        object.entry(key).or_insert(value);
                    }
                }
            }
            if let Some(name) = object
                .get("$ref")
                .and_then(Value::as_str)
                .and_then(|reference| reference.strip_prefix("#/$defs/"))
                && let Some(Value::Object(definition)) = definitions.get(name).cloned()
            {
                object.remove("$ref");
                for (key, value) in definition {
                    object.entry(key).or_insert(value);
                }
            }
            if let Some(Value::Array(types)) = object.get("type")
                && types.len() == 2
                && let Some(only) = types.iter().find(|one| *one != "null").cloned()
            {
                object.insert("type".into(), only);
            }
            if let Some(Value::String(format)) = object.remove("format")
                && format.starts_with("uint")
            {
                if object.get("minimum") == Some(&json!(0)) {
                    object.remove("minimum");
                }
                if object.get("maximum").is_some() {
                    object.remove("maximum");
                }
            }
            if let Some(Value::String(description)) = object.get_mut("description") {
                *description = description.replace('\n', " ");
            }
            for (key, value) in object.iter_mut() {
                match (key.as_str(), value) {
                    // Field names are not schemas, but their values are.
                    ("properties", Value::Object(fields)) => fields
                        .values_mut()
                        .for_each(|field| tidy(field, definitions)),
                    ("enum" | "required", _) => {}
                    (_, value) => tidy(value, definitions),
                }
            }
        }
        Value::Array(values) => values.iter_mut().for_each(|value| tidy(value, definitions)),
        _ => {}
    }
}

/// Answers `answer` with `result` at once.
fn reply(answer: &Answer, result: Result<Output, Error>) -> Task<Message> {
    answer.send(result);
    Task::none()
}

/// The input of a tool that takes none.
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Nothing {}

/// The input of a tool that acts on one entity.
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct On {
    /// The entity's ID from list_entities, as 0x and 16 hex digits.
    entity_id: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Connect {
    /// The talker's entity ID.
    talker_entity_id: String,
    /// The talker's stream output, from 0.
    output: u16,
    /// The listener's entity ID.
    listener_entity_id: String,
    /// The listener's stream input, from 0.
    input: u16,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Disconnect {
    /// The listener's entity ID.
    listener_entity_id: String,
    /// The listener's stream input, from 0.
    input: u16,
}

/// Which way a stream goes.
#[derive(Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum Direction {
    Input,
    Output,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SetFormat {
    /// The entity's ID.
    entity_id: String,
    /// Whether the stream is an input or an output.
    direction: Direction,
    /// The stream's index, from 0.
    index: u16,
    /// The format, as 0x and 16 hex digits, one describe_entity lists for
    /// the stream.
    format: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct StreamOf {
    /// The entity's ID.
    entity_id: String,
    /// Whether the stream is an input or an output.
    direction: Direction,
    /// The stream's index, from 0.
    index: u16,
    /// Seconds to watch the stream's counters over, up to 60, to tell what
    /// goes wrong now from what went wrong since the entity started; no
    /// sample when left out.
    sample_seconds: Option<u16>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SetRate {
    /// The entity's ID.
    entity_id: String,
    /// The audio unit's index, 0 for most entities.
    audio_unit: Option<u16>,
    /// The sampling rate in hertz, one describe_entity lists for the
    /// audio unit.
    hertz: u32,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SetClock {
    /// The entity's ID.
    entity_id: String,
    /// The clock domain's index, 0 for most entities.
    clock_domain: Option<u16>,
    /// The clock source's index, one describe_entity lists for the
    /// domain.
    clock_source: u16,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Rename {
    /// The entity's ID.
    entity_id: String,
    /// What to rename: "entity" (its name, when left out), "group" (its
    /// group name), or a stream as "input:N" or "output:N".
    what: Option<String>,
    /// The new name, up to 64 bytes of UTF-8; empty clears a group.
    name: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SetMonitor {
    /// Listening, or not.
    enabled: Option<bool>,
    /// The audio device, by name, as list_audio_devices lists them.
    input: Option<String>,
    /// "received" for what the device receives, its inputs; "sent" for
    /// what apps send to one of its output streams (on a Mac, macOS 14.2
    /// and later).
    side: Option<crate::monitor::Side>,
    /// For "sent", the device's output stream, from 0.
    stream: Option<u16>,
    /// The first channel listened to, from 0, of the device's inputs or of
    /// the output stream.
    first_channel: Option<u16>,
    /// Channels listened to: 1, played on both sides, or 2.
    channels: Option<u16>,
    /// The audio output, by name, or "default" for the default output.
    output: Option<String>,
    /// Gain in decibels, from -60 to 24.
    gain_db: Option<f32>,
}

/// Every tool, in the order agents see them.
static TOOLS: LazyLock<Vec<Tool>> = LazyLock::new(|| {
    vec![
        tool(
            "status",
            "Status",
            Kind::Read,
            "Says what triib runs on: the network interface it controls, whether its \
             network is running, its own controller entity ID, how many entities it has \
             found, and this computer's own entities.",
            |app, _: Nothing, answer| reply(&answer, Ok(Output::Json(status(app)))),
        ),
        tool(
            "list_entities",
            "List entities",
            Kind::Read,
            "Lists the AVB entities triib has found on the interface: each one's entity ID, \
             name, group, MAC address, roles with their stream counts, entity model ID, \
             Milan support, media clock source, gPTP grandmaster, whether its model is read, \
             and whether it is one of this computer's own.",
            |app, _: Nothing, answer| {
                reply(
                    &answer,
                    Ok(Output::Json(json!({ "entities": entities(app) }))),
                )
            },
        ),
        tool(
            "describe_entity",
            "Describe an entity",
            Kind::Read,
            "Describes one entity from its model: its names, firmware and serial number, its \
             audio units with their sampling rates, its clock domains with their sources, and \
             its stream inputs and outputs, each with its current format and the formats it \
             takes, in hex and in words, and what it is bound to or sending.",
            |app, on: On, answer| {
                let result = entity_id(&on.entity_id)
                    .and_then(|entity| describe(app, entity))
                    .map(Output::Json);
                reply(&answer, result)
            },
        ),
        tool(
            "list_connections",
            "List connections",
            Kind::Read,
            "Lists every stream connection triib knows of: each listener input bound to a \
             talker output, with both ends' entity IDs, names and stream indexes, and how it \
             is going: receiving, waiting for the stream, or failed and why.",
            |app, _: Nothing, answer| {
                reply(
                    &answer,
                    Ok(Output::Json(json!({ "connections": connections(app) }))),
                )
            },
        ),
        tool(
            "list_problems",
            "List problems",
            Kind::Read,
            "Lists what triib finds wrong on the network, errors first, as its views mark \
             it: the network not running, entities it could not read, bound streams whose \
             talker's reservation failed and where, whose formats differ, whose ends keep \
             time in different clock domains, that wait for their talker or whose talker is \
             gone, devices out of gPTP or PTP sync and why, a Mac receiving on its own \
             clock, which drops audio until set_clock_source has it follow the stream, \
             what this computer's own entities wait on, and what entities' counters say \
             went wrong since they started. Start here, then look closer with \
             stream_diagnostics and clock_tree.",
            |app, _: Nothing, answer| {
                reply(
                    &answer,
                    Ok(Output::Json(json!({ "problems": problems(app) }))),
                )
            },
        ),
        tool(
            "stream_diagnostics",
            "Stream diagnostics",
            Kind::Read,
            "Asks an entity afresh how one of its streams stands. For an input: what it is \
             bound to, whether the talker's reservation got through and if not why and at \
             which bridge, the accumulated latency, and how far the network map traces it. \
             For both: the stream ID, destination MAC and VLAN, the counters (media locks \
             and unlocks, interruptions, sequence mismatches, late and early timestamps, \
             frames) and what they say went wrong, and the entity's clock domains' locks. \
             The counters run from when the entity started: with sample_seconds, triib \
             reads them again after that long and gives what changed, to tell a problem now \
             from an old one. Answers after a second, or once the sample is done.",
            |app, input: StreamOf, answer| start_sampling(app, input, answer),
        ),
        tool(
            "clock_tree",
            "Clock tree",
            Kind::Read,
            "Gives the network's time as the network map shows it: each domain's \
             grandmaster, and every node (bridges, entities, this computer, and the plain \
             switches and PTP grandmasters of devices running AVB Lite) with its parent, \
             its gPTP or PTP domain, whether it keeps time and how or why not, its link up \
             (gPTP on it, peer delay, bridge port, offset from the grandmaster in AVB Lite, \
             a Wi-Fi hop's time lock and signal), its link drops and grandmaster changes; \
             and this computer's ptp4l grandmaster and offset, where it runs one.",
            |app, _: Nothing, answer| {
                reply(&answer, Ok(Output::Json(crate::netmap::clock_tree(app).1)))
            },
        ),
        tool(
            "connect",
            "Connect a stream",
            Kind::Change,
            "Binds a listener's stream input to a talker's stream output (ACMP CONNECT_RX, \
             Milan's BIND_RX), so the listener receives the talker's stream. Answers once \
             the listener has. Their formats should match: describe_entity lists them.",
            |app, input: Connect, answer| {
                let (talker, listener) = match (
                    entity_id(&input.talker_entity_id),
                    entity_id(&input.listener_entity_id),
                ) {
                    (Ok(talker), Ok(listener)) => (talker, listener),
                    (Err(error), _) | (_, Err(error)) => return reply(&answer, Err(error)),
                };
                act(
                    app,
                    answer,
                    Action::Connect {
                        talker: (talker, input.output),
                        listener: (listener, input.input),
                    },
                )
            },
        ),
        tool(
            "disconnect",
            "Disconnect a stream",
            Kind::Change,
            "Unbinds a listener's stream input from the talker output it is bound to (ACMP \
             DISCONNECT_RX, Milan's UNBIND_RX). Answers once the listener has.",
            |app, input: Disconnect, answer| match entity_id(&input.listener_entity_id) {
                Ok(listener) => act(
                    app,
                    answer,
                    Action::Disconnect {
                        listener: (listener, input.input),
                    },
                ),
                Err(error) => reply(&answer, Err(error)),
            },
        ),
        tool(
            "set_stream_format",
            "Set a stream's format",
            Kind::Change,
            "Sets a stream input's or output's format (SET_STREAM_FORMAT) to one it takes, \
             as describe_entity lists them. A stream that refuses while it runs is stopped, \
             set and started again. An entity may move its sampling rate and its other \
             streams with it. Answers once the entity has.",
            |app, input: SetFormat, answer| {
                let entity = match entity_id(&input.entity_id) {
                    Ok(entity) => entity,
                    Err(error) => return reply(&answer, Err(error)),
                };
                let Some(format) = u64::from_str_radix(input.format.trim_start_matches("0x"), 16)
                    .ok()
                    .map(StreamFormat)
                else {
                    return reply(
                        &answer,
                        Err(Error::new(
                            code::INVALID_PARAMS,
                            "a format is 0x and 16 hex digits, as describe_entity lists them",
                        )),
                    );
                };
                act(
                    app,
                    answer,
                    Action::SetStreamFormat {
                        entity,
                        descriptor_type: match input.direction {
                            Direction::Input => DescriptorType::STREAM_INPUT,
                            Direction::Output => DescriptorType::STREAM_OUTPUT,
                        },
                        index: input.index,
                        format,
                    },
                )
            },
        ),
        tool(
            "set_sampling_rate",
            "Set a sampling rate",
            Kind::Change,
            "Sets an audio unit's sampling rate (SET_SAMPLING_RATE) to one it lists in \
             describe_entity; its streams' formats may follow. Answers once the entity has.",
            |app, input: SetRate, answer| match entity_id(&input.entity_id) {
                Ok(entity) => act(
                    app,
                    answer,
                    Action::SetSamplingRate {
                        entity,
                        unit: input.audio_unit.unwrap_or(0),
                        rate: SamplingRate(input.hertz),
                    },
                ),
                Err(error) => reply(&answer, Err(error)),
            },
        ),
        tool(
            "set_clock_source",
            "Set a clock source",
            Kind::Change,
            "Picks a clock domain's clock source (SET_CLOCK_SOURCE) from those \
             describe_entity lists, such as the internal clock or a stream input. Answers \
             once the entity has.",
            |app, input: SetClock, answer| match entity_id(&input.entity_id) {
                Ok(entity) => act(
                    app,
                    answer,
                    Action::SetClockSource {
                        entity,
                        domain: input.clock_domain.unwrap_or(0),
                        source: input.clock_source,
                    },
                ),
                Err(error) => reply(&answer, Err(error)),
            },
        ),
        tool(
            "rename",
            "Rename",
            Kind::Change,
            "Renames an entity, its group, or one of its streams (SET_NAME). Answers once \
             the entity has.",
            |app, input: Rename, answer| {
                let entity = match entity_id(&input.entity_id) {
                    Ok(entity) => entity,
                    Err(error) => return reply(&answer, Err(error)),
                };
                let Some(name) = Name::new(&input.name) else {
                    return reply(
                        &answer,
                        Err(Error::new(
                            code::INVALID_PARAMS,
                            "a name holds 64 bytes at most",
                        )),
                    );
                };
                let what = input.what.as_deref().unwrap_or("entity");
                let target = |descriptor_type, index, name_index| NameTarget {
                    entity,
                    descriptor_type,
                    index,
                    name_index,
                };
                let target = match what.split_once(':') {
                    None if what == "entity" => target(DescriptorType::ENTITY, 0, 0),
                    None if what == "group" => target(DescriptorType::ENTITY, 0, 1),
                    Some(("input", index)) if index.parse::<u16>().is_ok() => {
                        target(DescriptorType::STREAM_INPUT, index.parse().unwrap_or(0), 0)
                    }
                    Some(("output", index)) if index.parse::<u16>().is_ok() => {
                        target(DescriptorType::STREAM_OUTPUT, index.parse().unwrap_or(0), 0)
                    }
                    _ => {
                        return reply(
                            &answer,
                            Err(Error::new(
                                code::INVALID_PARAMS,
                                "what is entity, group, input:N or output:N",
                            )),
                        );
                    }
                };
                act(app, answer, Action::Rename { target, name })
            },
        ),
        tool(
            "identify",
            "Identify an entity",
            Kind::Change,
            "Makes an entity identify itself for a few seconds, as by blinking a light, so \
             the user can find it.",
            |app, on: On, answer| match entity_id(&on.entity_id) {
                Ok(entity) => act(app, answer, Action::Identify(entity)),
                Err(error) => reply(&answer, Err(error)),
            },
        ),
        tool(
            "list_audio_devices",
            "List audio devices",
            Kind::Read,
            "Lists this computer's audio inputs and outputs by name, as the audio monitor \
             takes them, and on a Mac the Core Audio device macOS makes of its own AVB \
             entity, which is there only while one of that entity's streams is connected, \
             with the channels of each of its output streams, whose audio the monitor can \
             hear as sent.",
            |app, _: Nothing, answer| {
                reply(
                    &answer,
                    Ok(Output::Json(json!({
                        "inputs": app.audio_devices.0,
                        "outputs": app.audio_devices.1,
                        "this_mac_avb_device": app.mac_audio_device,
                        "this_mac_avb_output_streams": app.mac_output_streams,
                    }))),
                )
            },
        ),
        tool(
            "audio_monitor",
            "Audio monitor",
            Kind::Read,
            "Says what this computer listens to through the audio monitor: whether it is \
             on, the input and its channels, the output and the gain, and what the monitor \
             is doing: starting, waiting for the input or the output to be there, or \
             playing, with the loudest level in the last second.",
            |app, _: Nothing, answer| reply(&answer, Ok(Output::Json(monitor(app)))),
        ),
        tool(
            "set_audio_monitor",
            "Set the audio monitor",
            Kind::Change,
            "Routes an audio device on this computer to an audio output, as the Listen \
             controls do: what it receives, or what apps send to one of its output \
             streams, one channel played on both sides or a pair, with a gain. What is \
             left out stays as it was; on a Mac the input defaults to its own AVB entity's \
             device. The monitor waits for a device that is not there and plays once it \
             is, and starts again by itself when it stops; the setting is kept across \
             restarts. Answers with the new settings and state.",
            |app, input: SetMonitor, answer| {
                let mut listen = app.settings.listen.clone();
                if let Some(enabled) = input.enabled {
                    listen.enabled = enabled;
                }
                if let Some(name) = input.input {
                    listen.input = Some(name);
                }
                if listen.input.is_none() {
                    listen.input = app.mac_audio_device.clone();
                }
                if let Some(side) = input.side {
                    listen.side = side;
                }
                if let Some(stream) = input.stream {
                    listen.stream = stream;
                }
                if let Some(first) = input.first_channel {
                    listen.first_channel = first;
                }
                if let Some(channels) = input.channels {
                    listen.channels = channels;
                }
                if let Some(output) = input.output {
                    listen.output = (!output.is_empty() && output != "default").then_some(output);
                }
                if let Some(gain) = input.gain_db {
                    listen.gain_db = gain;
                }
                if listen.side == crate::monitor::Side::Sent && !crate::monitor::hears_sent() {
                    return reply(
                        &answer,
                        Err(Error::new(
                            code::INVALID_PARAMS,
                            "hearing what a device sends takes a Mac with macOS 14.2 or later",
                        )),
                    );
                }
                if listen.enabled && listen.input.is_none() {
                    return reply(
                        &answer,
                        Err(Error::new(
                            code::INVALID_PARAMS,
                            "name the input, as list_audio_devices lists them",
                        )),
                    );
                }
                app.set_listen(listen);
                reply(&answer, Ok(Output::Json(monitor(app))))
            },
        ),
    ]
});

/// The audio monitor's settings and what it is doing.
fn monitor(app: &Triib) -> Value {
    json!({
        "listen": app.settings.listen,
        "status": app.monitor_status,
    })
}

/// The tools as MCP lists them.
pub fn list() -> Value {
    Value::Array(
        TOOLS
            .iter()
            .map(|tool| {
                json!({
                    "name": tool.name,
                    "title": tool.title,
                    "description": tool.description,
                    "inputSchema": (tool.schema)(),
                    "annotations": annotations(tool.kind),
                })
            })
            .collect(),
    )
}

/// MCP's hints for clients: what only reads, and what changes devices.
fn annotations(kind: Kind) -> Value {
    match kind {
        Kind::Read => json!({ "readOnlyHint": true }),
        Kind::Change => json!({ "readOnlyHint": false, "destructiveHint": false }),
    }
}

/// The tool named `name`.
pub fn find(name: &str) -> Option<&'static Tool> {
    TOOLS.iter().find(|tool| tool.name == name)
}

impl Tool {
    /// Runs the tool on `arguments`; it answers through `answer`.
    pub fn start(&self, app: &mut Triib, arguments: Value, answer: Answer) -> Task<Message> {
        (self.run)(app, arguments, answer)
    }
}

/// An entity ID from text: 0x and 16 hex digits, or the digits alone.
fn entity_id(text: &str) -> Result<EntityId, Error> {
    let digits = text.trim().trim_start_matches("0x");
    u64::from_str_radix(digits, 16)
        .ok()
        .filter(|_| digits.len() == 16)
        .map(EntityId)
        .ok_or_else(|| {
            Error::new(
                code::INVALID_PARAMS,
                format!("{text} is not an entity ID: one is 0x and 16 hex digits"),
            )
        })
}

/// Sends `action` to the network, answering once it is done, unless
/// triib has no network running or does not know the entity.
fn act(app: &mut Triib, answer: Answer, action: Action) -> Task<Message> {
    if !matches!(app.network_state, NetworkState::Running { .. }) {
        return reply(
            &answer,
            Err(Error::new(
                code::INTERNAL_ERROR,
                "triib's network is not running; status says why",
            )),
        );
    }
    let entity = match action {
        Action::Connect { listener, .. } | Action::Disconnect { listener } => listener.0,
        Action::Identify(entity)
        | Action::SetStreamFormat { entity, .. }
        | Action::SetSamplingRate { entity, .. }
        | Action::SetClockSource { entity, .. } => entity,
        Action::Rename { target, .. } => target.entity,
        _ => EntityId(0),
    };
    if !app.entities.contains_key(&entity) {
        return reply(
            &answer,
            Err(Error::new(
                code::INVALID_PARAMS,
                format!("triib knows no entity {entity}; list_entities gives them"),
            )),
        );
    }
    app.agents.waits.push((action, answer));
    app.update(Message::Act(action))
}

/// The answer to an action an agent asked for, once the network has
/// done it: what changed, or why it did not, as the log says it.
pub fn finished(app: &Triib, action: Action, outcome: Outcome) -> Result<Output, Error> {
    match describe::outcome(app, action, outcome) {
        None => Ok(Output::Text(done(app, action))),
        Some(failure) => Err(Error::new(code::INTERNAL_ERROR, failure)),
    }
}

/// What a done action changed.
fn done(app: &Triib, action: Action) -> String {
    let input = |(entity, index)| app.stream_name(entity, DescriptorType::STREAM_INPUT, index);
    let output = |(entity, index)| app.stream_name(entity, DescriptorType::STREAM_OUTPUT, index);
    match action {
        Action::Connect { talker, listener } => {
            format!("Connected {} to {}.", output(talker), input(listener))
        }
        Action::Disconnect { listener } => format!("Disconnected {}.", input(listener)),
        Action::Identify(entity) => {
            format!("{} is identifying itself.", app.entity_name_of(entity))
        }
        Action::Rename { name, .. } => format!(
            "Renamed to {}.",
            if name.as_str().is_empty() {
                "nothing"
            } else {
                name.as_str()
            }
        ),
        Action::SetStreamFormat {
            entity,
            descriptor_type,
            index,
            format,
        } => format!(
            "{} now uses {format} ({:#018x}).",
            app.stream_name(entity, descriptor_type, index),
            format.0
        ),
        Action::SetSamplingRate { entity, rate, .. } => format!(
            "{} now runs at {} Hz.",
            app.entity_name_of(entity),
            rate.base_frequency()
        ),
        Action::SetClockSource {
            entity,
            domain,
            source,
        } => format!(
            "{} now follows {}.",
            app.entity_name_of(entity),
            app.models
                .get(&entity)
                .and_then(|model| model.name_of(DescriptorType::CLOCK_SOURCE, source))
                .map_or_else(
                    || format!("clock source {source} in domain {domain}"),
                    str::to_owned
                )
        ),
        _ => "Done.".to_owned(),
    }
}

fn status(app: &Triib) -> Value {
    let interface = app.interface().map(|interface| {
        json!({
            "name": interface.name,
            "mac": interface.mac.to_string(),
            "up": interface.up,
            "speed_mbps": interface.speed,
            "hardware_clock": interface.hardware_clock.is_some(),
        })
    });
    let (network, controller) = match &app.network_state {
        NetworkState::Idle => ("no interface chosen".to_owned(), None),
        NetworkState::Starting => ("starting".to_owned(), None),
        NetworkState::Running { controller } => {
            ("running".to_owned(), Some(controller.to_string()))
        }
        NetworkState::Failed(failure) => (
            match &failure.remedy {
                Some(Remedy::Command(command)) => {
                    format!(
                        "failed: {}; the user can run {command}, and triib carries on by \
                         itself once it has",
                        failure.message
                    )
                }
                _ => format!("failed: {}", failure.message),
            },
            None,
        ),
    };
    let host = app.endpoints.as_ref().map(|status| &status.endpoints);
    json!({
        "triib": env!("CARGO_PKG_VERSION"),
        "interface": interface,
        "network": network,
        "controller_entity_id": controller,
        "entities": app.entities.len(),
        "this_computer": host,
    })
}

fn entities(app: &Triib) -> Vec<Value> {
    let mut all: Vec<_> = app.entities.values().collect();
    all.sort_by_key(|entity| (app.entity_name(entity).to_lowercase(), entity.entity_id()));
    all.into_iter()
        .map(|entity| {
            let id = entity.entity_id();
            let model = app.models.get(&id);
            let descriptor = model.and_then(EntityModel::entity);
            json!({
                "entity_id": id.to_string(),
                "name": app.entity_name(entity),
                "group": descriptor.map(|entity| entity.group_name.to_owned()),
                "mac": entity.mac.to_string(),
                "roles": describe::roles(&entity.adp),
                "entity_model_id": format!("{:#018x}", entity.adp.entity_model_id.0),
                "milan": model
                    .filter(|model| model.milan.is_some())
                    .map(describe::milan),
                "media_clock": model.map(describe::media_clock),
                "grandmaster": describe::clock(&entity.adp),
                "model_read": model.is_some_and(|model| model.state == EnumerationState::Complete),
                "this_computer": app.is_host(id),
            })
        })
        .collect()
}

/// A format as describe_entity gives it: its code and its words.
fn format(format: StreamFormat) -> Value {
    json!({ "code": format!("{:#018x}", format.0), "words": format.to_string() })
}

fn describe(app: &Triib, entity: EntityId) -> Result<Value, Error> {
    let Some(model) = app.models.get(&entity) else {
        return Err(Error::new(
            code::INVALID_PARAMS,
            if app.entities.contains_key(&entity) {
                format!("triib has not read {entity}'s model yet")
            } else {
                format!("triib knows no entity {entity}; list_entities gives them")
            },
        ));
    };
    let descriptor = model.entity();
    let localized = |reference| model.localized(reference).map(str::to_owned);
    let audio_units: Vec<Value> = model
        .audio_units()
        .map(|unit| {
            json!({
                "index": unit.index,
                "name": model.name_of(DescriptorType::AUDIO_UNIT, unit.index),
                "sampling_rate": unit.current_sampling_rate.base_frequency(),
                "sampling_rates": unit
                    .sampling_rates()
                    .map(|rate| rate.base_frequency())
                    .collect::<Vec<_>>(),
            })
        })
        .collect();
    let clock_domains: Vec<Value> = model
        .clock_domains()
        .map(|domain| {
            let sources: Vec<Value> = domain
                .clock_sources()
                .map(|source| {
                    json!({
                        "index": source,
                        "name": model.name_of(DescriptorType::CLOCK_SOURCE, source),
                    })
                })
                .collect();
            json!({
                "index": domain.index,
                "name": model.name_of(DescriptorType::CLOCK_DOMAIN, domain.index),
                "clock_source": domain.clock_source_index,
                "clock_sources": sources,
            })
        })
        .collect();
    let streams = |input: bool| -> Vec<Value> {
        model
            .streams(input)
            .map(|stream| {
                let bound = input
                    .then(|| model.binding(stream.index))
                    .flatten()
                    .and_then(|binding| binding.talker_stream())
                    .map(|(talker, output)| {
                        json!({ "talker_entity_id": talker.to_string(), "output": output })
                    });
                json!({
                    "index": stream.index,
                    "name": model.name_of(stream.descriptor_type, stream.index),
                    "format": format(stream.current_format),
                    "formats": stream.formats().map(format).collect::<Vec<_>>(),
                    "bound_to": bound,
                    "state": describe::stream_state(app, entity, model, &stream),
                })
            })
            .collect()
    };
    Ok(json!({
        "entity_id": entity.to_string(),
        "name": model.entity_name(),
        "group": descriptor.map(|entity| entity.group_name.to_owned()),
        "vendor": descriptor.and_then(|entity| localized(entity.vendor_name)),
        "model": descriptor.and_then(|entity| localized(entity.model_name)),
        "firmware": descriptor.map(|entity| entity.firmware_version.to_owned()),
        "serial_number": descriptor.map(|entity| entity.serial_number.to_owned()),
        "audio_units": audio_units,
        "clock_domains": clock_domains,
        "stream_inputs": streams(true),
        "stream_outputs": streams(false),
    }))
}

fn connections(app: &Triib) -> Vec<Value> {
    let mut found = Vec::new();
    for (listener, model) in &app.models {
        for input in model.streams(true) {
            let Some((talker, output)) = model
                .binding(input.index)
                .and_then(|binding| binding.talker_stream())
            else {
                continue;
            };
            found.push(json!({
                "listener_entity_id": listener.to_string(),
                "listener": app.entity_name_of(*listener),
                "input": input.index,
                "input_name": model.name_of(DescriptorType::STREAM_INPUT, input.index),
                "talker_entity_id": talker.to_string(),
                "talker": app.entity_name_of(talker),
                "output": output,
                "output_name": app
                    .models
                    .get(&talker)
                    .and_then(|talker| talker.name_of(DescriptorType::STREAM_OUTPUT, output)),
                "state": describe::stream_state(app, *listener, model, &input),
            }));
        }
    }
    found
}

/// Each problem triib finds, as list_problems gives them: errors, then
/// warnings.
fn problems(app: &Triib) -> Vec<Value> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    if !matches!(app.network_state, NetworkState::Running { .. }) {
        errors.push(json!({
            "kind": "network",
            "says": status(app)["network"],
        }));
        return problem_list(errors, warnings);
    }
    if let Some(interface) = app.interface().filter(|interface| !interface.up) {
        errors.push(json!({
            "kind": "link_down",
            "says": format!("{} is down", interface.name),
        }));
    }
    for (&entity, model) in &app.models {
        if let EnumerationState::Failed(failure) = model.state
            && failure != EnumerationFailure::OnThisComputer
        {
            warnings.push(json!({
                "kind": "unread",
                "entity_id": entity.to_string(),
                "entity": app.entity_name_of(entity),
                "says": describe::failure(failure),
            }));
        }
    }
    let domains = crate::netmap::Domains::of(app);
    for traced in crate::netmap::traced(app) {
        let (listener, input) = traced.listener;
        let (talker, output) = traced.talker;
        let stream = json!({
            "listener_entity_id": listener.to_string(),
            "input": input,
            "talker_entity_id": talker.to_string(),
            "output": output,
            "stream": format!(
                "{} {} {}",
                app.stream_name(talker, DescriptorType::STREAM_OUTPUT, output),
                crate::i18n::arrow(""),
                app.stream_name(listener, DescriptorType::STREAM_INPUT, input)
            ),
        });
        let with = |kind: &str, says: String| {
            let mut problem = stream.clone();
            problem["kind"] = json!(kind);
            problem["says"] = json!(says);
            problem
        };
        if !app.entities.contains_key(&talker) {
            errors.push(with(
                "talker_gone",
                format!("{talker} is not on the network"),
            ));
            continue;
        }
        if domains.apart(talker, listener) {
            errors.push(with("other_domain", crate::fl!("matrix-other-domain")));
        }
        let formats = app.models.get(&talker).zip(app.models.get(&listener));
        if let Some((sent, set)) = formats.and_then(|(talker_model, listener_model)| {
            let sent = talker_model
                .streams(false)
                .find(|stream| stream.index == output)?;
            let set = listener_model
                .streams(true)
                .find(|stream| stream.index == input)?;
            let fit =
                atdecc::stream_format::fit(sent.current_format, set.current_format, set.formats());
            (fit != Fit::Matches).then_some((sent.current_format, set.current_format))
        }) {
            errors.push(with(
                "format_mismatch",
                crate::fl!(
                    "matrix-bound-formats-differ",
                    sent = sent.to_string(),
                    set = set.to_string()
                ),
            ));
        }
        match traced.status {
            Status::Failed => {
                let mut problem = with("reservation_failed", traced.reason);
                // The code and the bridge that refused it, as the listener
                // tells them, whether or not the bridge is on the map.
                if let Some(info) = app
                    .models
                    .get(&listener)
                    .and_then(|model| model.stream_info(DescriptorType::STREAM_INPUT, input))
                {
                    let [_, _, mac @ ..] = info.msrp_failure_bridge_id.to_be_bytes();
                    problem["msrp_failure_code"] = json!(info.msrp_failure_code);
                    problem["failing_bridge_mac"] =
                        json!((mac != [0; 6]).then(|| avb_net::MacAddress(mac).to_string()));
                }
                errors.push(problem);
            }
            Status::Advertised => warnings.push(with("waiting", traced.reason)),
            Status::Flowing | Status::Unsynced => {}
        }
    }
    let (nodes, tree) = crate::netmap::clock_tree(app);
    let grandmasters: Vec<String> = tree["grandmasters"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect();
    if grandmasters.len() > 1 {
        warnings.push(json!({
            "kind": "several_domains",
            "says": format!(
                "{} clock domains, each with its own grandmaster: {}; streams cannot pass \
                 between them",
                grandmasters.len(),
                crate::i18n::list(grandmasters)
            ),
        }));
    }
    for node in nodes.into_iter().filter(|node| !node.synced) {
        let problem = json!({
            "kind": "not_synced",
            "entity_id": node.entity_id.map(|entity| entity.to_string()),
            "node": node.name,
            "says": node.state,
        });
        // An entity in the tree whose time does not reach it; one that does
        // not say where it is, or this computer, may yet be fine.
        if node.placed && node.entity_id.is_some() {
            errors.push(problem);
        } else {
            warnings.push(problem);
        }
    }
    for own in crate::diagnostics_view::own_clocks(app) {
        let Some(alarm) = own.alarm(app) else {
            continue;
        };
        errors.push(json!({
            "kind": "mac_own_clock",
            "entity_id": own.entity.to_string(),
            "entity": app.entity_name_of(own.entity),
            "clock_domain": own.domain,
            "clock_source": own.source,
            "follow_clock_source": own.follow,
            "says": alarm.text,
        }));
    }
    if let Some(note) = crate::host_view::note(app) {
        warnings.push(json!({ "kind": "this_computer", "says": note }));
    }
    for (&entity, model) in &app.models {
        for finding in crate::diagnostics_view::findings(model) {
            if finding.wrong.is_empty() {
                continue;
            }
            warnings.push(json!({
                "kind": "counters",
                "entity_id": entity.to_string(),
                "entity": app.entity_name_of(entity),
                "of": finding.name,
                "descriptor": descriptor_name(finding.descriptor_type),
                "index": finding.index,
                "says": crate::i18n::list(finding.wrong),
                "since": crate::fl!("diag-since-start"),
            }));
        }
    }
    problem_list(errors, warnings)
}

fn problem_list(errors: Vec<Value>, warnings: Vec<Value>) -> Vec<Value> {
    let marked = |severity: &str, problems: Vec<Value>| {
        problems.into_iter().map({
            let severity = severity.to_owned();
            move |mut problem| {
                problem["severity"] = json!(severity);
                problem
            }
        })
    };
    marked("error", errors)
        .chain(marked("warning", warnings))
        .collect()
}

/// A descriptor type as the tools name it.
fn descriptor_name(descriptor_type: DescriptorType) -> &'static str {
    match descriptor_type {
        DescriptorType::STREAM_INPUT => "stream_input",
        DescriptorType::STREAM_OUTPUT => "stream_output",
        DescriptorType::CLOCK_DOMAIN => "clock_domain",
        _ => "descriptor",
    }
}

/// How long an entity has to answer a fresh question before its stream is
/// read from what it said.
const ANSWER_TIME: Duration = Duration::from_millis(800);

/// The longest sample, in seconds.
const LONGEST_SAMPLE: u16 = 60;

/// An agent's look at how a stream fares, between its steps.
#[derive(Debug, Clone)]
pub struct Sampling {
    answer: Answer,
    entity: EntityId,
    descriptor_type: DescriptorType,
    index: u16,
    seconds: u16,
    step: Step,
}

#[derive(Debug, Clone, Copy)]
enum Step {
    /// The entity was asked how the stream stands.
    Asked,
    /// The sample began with these counters; it is time to ask again.
    Began(Option<Counters>),
    /// Asked again at the end of the sample that began with these.
    AskedAgain(Option<Counters>),
}

/// `message`, after `wait`.
fn after(wait: Duration, message: Message) -> Task<Message> {
    let (ready, waited) = tokio::sync::oneshot::channel();
    std::thread::spawn(move || {
        std::thread::sleep(wait);
        let _ = ready.send(());
    });
    Task::perform(waited, move |_| message)
}

/// Starts stream_diagnostics: asks the entity how the stream stands, and
/// reads what it said a moment later.
fn start_sampling(app: &mut Triib, input: StreamOf, answer: Answer) -> Task<Message> {
    let entity = match entity_id(&input.entity_id) {
        Ok(entity) => entity,
        Err(error) => return reply(&answer, Err(error)),
    };
    let descriptor_type = match input.direction {
        Direction::Input => DescriptorType::STREAM_INPUT,
        Direction::Output => DescriptorType::STREAM_OUTPUT,
    };
    if !matches!(app.network_state, NetworkState::Running { .. }) {
        return reply(
            &answer,
            Err(Error::new(
                code::INTERNAL_ERROR,
                "triib's network is not running; status says why",
            )),
        );
    }
    if let Err(error) = stream_diagnostics(app, entity, descriptor_type, input.index, None) {
        return reply(&answer, Err(error));
    }
    app.refresh_stream(entity, descriptor_type, input.index);
    after(
        ANSWER_TIME,
        Message::StreamSampled(Sampling {
            answer,
            entity,
            descriptor_type,
            index: input.index,
            seconds: input.sample_seconds.unwrap_or(0).min(LONGEST_SAMPLE),
            step: Step::Asked,
        }),
    )
}

/// The next step of stream_diagnostics, once the entity had time to answer
/// or the sample is over.
pub fn sampled(app: &mut Triib, sampling: Sampling) -> Task<Message> {
    let counters = || {
        app.models
            .get(&sampling.entity)
            .and_then(|model| model.counters(sampling.descriptor_type, sampling.index))
            .copied()
    };
    let next = |step| {
        Message::StreamSampled(Sampling {
            step,
            ..sampling.clone()
        })
    };
    let sample = match sampling.step {
        Step::Asked if sampling.seconds > 0 => {
            return after(
                Duration::from_secs(u64::from(sampling.seconds)),
                next(Step::Began(counters())),
            );
        }
        Step::Began(first) => {
            app.refresh_stream(sampling.entity, sampling.descriptor_type, sampling.index);
            return after(ANSWER_TIME, next(Step::AskedAgain(first)));
        }
        Step::Asked => None,
        Step::AskedAgain(first) => Some((first, sampling.seconds)),
    };
    let result = stream_diagnostics(
        app,
        sampling.entity,
        sampling.descriptor_type,
        sampling.index,
        sample,
    );
    reply(&sampling.answer, result.map(Output::Json))
}

/// A stream's counters by name, as its kind of stream numbers them.
fn named_counters(
    model: &EntityModel,
    descriptor_type: DescriptorType,
    counters: &Counters,
) -> Vec<(&'static str, Option<u32>)> {
    if descriptor_type == DescriptorType::STREAM_INPUT {
        let Some(counters) = counters.stream_input() else {
            return Vec::new();
        };
        vec![
            ("media_locked", counters.media_locked),
            ("media_unlocked", counters.media_unlocked),
            ("stream_interrupted", counters.stream_interrupted),
            ("seq_num_mismatch", counters.seq_num_mismatch),
            ("media_reset", counters.media_reset),
            ("timestamp_uncertain", counters.timestamp_uncertain),
            ("timestamp_valid", counters.timestamp_valid),
            ("timestamp_not_valid", counters.timestamp_not_valid),
            ("unsupported_format", counters.unsupported_format),
            ("late_timestamp", counters.late_timestamp),
            ("early_timestamp", counters.early_timestamp),
            ("frames_rx", counters.frames_rx),
        ]
    } else {
        let before_milan_1_3 = model
            .milan
            .is_some_and(|milan| milan.specification_version.is_none());
        let Some(counters) = counters.stream_output(before_milan_1_3) else {
            return Vec::new();
        };
        vec![
            ("stream_start", counters.stream_start),
            ("stream_stop", counters.stream_stop),
            ("stream_interrupted", counters.stream_interrupted),
            ("media_reset", counters.media_reset),
            ("timestamp_uncertain", counters.timestamp_uncertain),
            ("timestamp_valid", counters.timestamp_valid),
            ("timestamp_not_valid", counters.timestamp_not_valid),
            ("frames_tx", counters.frames_tx),
        ]
    }
}

/// The counters whose rise means something went wrong.
const BAD_COUNTERS: [&str; 9] = [
    "media_unlocked",
    "stream_interrupted",
    "seq_num_mismatch",
    "media_reset",
    "timestamp_uncertain",
    "timestamp_not_valid",
    "unsupported_format",
    "late_timestamp",
    "early_timestamp",
];

/// What stream_diagnostics says of a stream, from what the entity said
/// last, with what changed over a sample that began with `sample`'s
/// counters and lasted its seconds.
fn stream_diagnostics(
    app: &Triib,
    entity: EntityId,
    descriptor_type: DescriptorType,
    index: u16,
    sample: Option<(Option<Counters>, u16)>,
) -> Result<Value, Error> {
    let input = descriptor_type == DescriptorType::STREAM_INPUT;
    let Some(model) = app.models.get(&entity) else {
        return Err(Error::new(
            code::INVALID_PARAMS,
            if app.entities.contains_key(&entity) {
                format!("triib has not read {entity}'s model yet")
            } else {
                format!("triib knows no entity {entity}; list_entities gives them")
            },
        ));
    };
    let Some(stream) = model.streams(input).find(|stream| stream.index == index) else {
        return Err(Error::new(
            code::INVALID_PARAMS,
            format!(
                "{} has no stream {} {index}; describe_entity lists its streams",
                app.entity_name_of(entity),
                if input { "input" } else { "output" }
            ),
        ));
    };
    let info = model.stream_info(descriptor_type, index);
    let stream_info = info.map(|info| {
        use atdecc::aem::StreamInfoFlags as Flags;
        let has = |flag| info.flags.contains(flag);
        json!({
            "flags": info.flags.names().collect::<Vec<_>>(),
            "bound": info.bound(),
            "talker_failed": info.talker_failed(),
            "reservation_failure": info.talker_failed().then(|| json!({
                "code": info.msrp_failure_code,
                "reason": describe::msrp_failure(info.msrp_failure_code),
                "says": describe::reservation_failure(info),
            })),
            "accumulated_latency_ns": info.registering().then_some(info.msrp_accumulated_latency),
            "stream_id": has(Flags::STREAM_ID_VALID).then(|| format!("{:#018x}", info.stream_id.0)),
            "destination_mac": has(Flags::STREAM_DEST_MAC_VALID)
                .then(|| info.stream_dest_mac.to_string()),
            "vlan_id": has(Flags::STREAM_VLAN_ID_VALID).then_some(info.stream_vlan_id),
            "format": has(Flags::STREAM_FORMAT_VALID).then(|| format(info.stream_format)),
        })
    });
    let counters = model.counters(descriptor_type, index);
    let named = |counters: &Counters| -> serde_json::Map<String, Value> {
        named_counters(model, descriptor_type, counters)
            .into_iter()
            .filter_map(|(name, value)| Some((name.to_owned(), json!(value?))))
            .collect()
    };
    let went_wrong = crate::diagnostics_view::findings(model)
        .into_iter()
        .find(|finding| finding.descriptor_type == descriptor_type && finding.index == index)
        .map(|finding| finding.wrong)
        .unwrap_or_default();
    let sample = sample.map(|(first, seconds)| {
        let (Some(first), Some(last)) = (first, counters) else {
            return json!({
                "seconds": seconds,
                "says": "the entity reported no counters for this stream",
            });
        };
        let before = named_counters(model, descriptor_type, &first);
        let changed: serde_json::Map<String, Value> = named_counters(model, descriptor_type, last)
            .into_iter()
            .zip(before)
            .filter_map(|((name, now), (_, then))| {
                Some((name.to_owned(), json!(now?.wrapping_sub(then?))))
            })
            .collect();
        let worse: serde_json::Map<String, Value> = changed
            .iter()
            .filter(|(name, change)| {
                BAD_COUNTERS.contains(&name.as_str()) && change.as_u64().is_some_and(|n| n > 0)
            })
            .map(|(name, change)| (name.clone(), change.clone()))
            .collect();
        let frames = changed
            .get(if input { "frames_rx" } else { "frames_tx" })
            .and_then(Value::as_u64);
        json!({
            "seconds": seconds,
            "changed": changed,
            "went_wrong": worse,
            "frames_per_second": frames.map(|frames| frames / u64::from(seconds.max(1))),
        })
    });
    let traced = input
        .then(|| {
            crate::netmap::traced(app)
                .into_iter()
                .find(|traced| traced.listener == (entity, index))
        })
        .flatten()
        .map(|traced| {
            json!({
                "status": match traced.status {
                    Status::Flowing => "flowing",
                    Status::Advertised => "waiting",
                    Status::Failed => "failed",
                    Status::Unsynced => "unsynced",
                },
                "says": traced.reason,
                "via": traced.via,
                "stops_at": traced.stops_at,
            })
        });
    let bound_to = input
        .then(|| model.binding(index))
        .flatten()
        .and_then(|binding| binding.talker_stream())
        .map(|(talker, output)| {
            json!({
                "talker_entity_id": talker.to_string(),
                "talker": app.entity_name_of(talker),
                "output": output,
                "output_name": app.stream_name(talker, DescriptorType::STREAM_OUTPUT, output),
            })
        });
    let clock_domains: Vec<Value> = model
        .clock_domains()
        .map(|domain| {
            let counters = model.clock_domain_counters(domain.index);
            json!({
                "index": domain.index,
                "name": model.name_of(DescriptorType::CLOCK_DOMAIN, domain.index),
                "clock_source": model.name_of(DescriptorType::CLOCK_SOURCE, domain.clock_source_index),
                "locked": counters.and_then(|counters| counters.locked),
                "unlocked": counters.and_then(|counters| counters.unlocked),
            })
        })
        .collect();
    Ok(json!({
        "entity_id": entity.to_string(),
        "entity": app.entity_name_of(entity),
        "direction": if input { "input" } else { "output" },
        "index": index,
        "name": model.name_of(descriptor_type, index),
        "format": format(stream.current_format),
        "state": describe::stream_state(app, entity, model, &stream),
        "bound_to": bound_to,
        "network": traced,
        "stream_info": stream_info,
        "counters": counters.map(named),
        "counters_since": crate::fl!("diag-since-start"),
        "went_wrong": went_wrong,
        "sample": sample,
        "clock_domains": clock_domains,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The audio monitor is set as the Listen controls would, what is left
    /// out kept, on a Mac its input the Mac's own AVB device, and read back.
    #[test]
    fn the_audio_monitor_is_set_and_read() {
        use crate::monitor::State;

        let (entities, models) = crate::view::tests::bench();
        let interface = avb_net::Interface {
            name: "en0".to_owned(),
            mac: avb_net::MacAddress([2, 0, 0, 0, 0, 1]),
            up: true,
            speed: None,
            physical: true,
            wireless: false,
            hardware_clock: None,
        };
        let mut app = Triib::sample(
            crate::settings::Settings::default(),
            interface,
            entities,
            models,
        );
        app.mac_audio_device = Some("Mac mini:Ethernet".to_owned());
        let run = |app: &mut Triib, name: &str, arguments: Value| -> Value {
            let (sender, answered) = std::sync::mpsc::channel();
            let call = Call::local(name, Value::Null, move |result| {
                let _ = sender.send(result);
            });
            let _ = find(name).unwrap().start(app, arguments, Answer(call));
            answered
                .recv_timeout(std::time::Duration::from_secs(5))
                .unwrap()
                .unwrap()
        };
        let set = run(
            &mut app,
            "set_audio_monitor",
            json!({ "enabled": true, "first_channel": 2, "channels": 1, "output": "KM-HIFI" }),
        );
        let listen = &app.settings.listen;
        assert!(listen.enabled);
        assert_eq!(listen.input.as_deref(), Some("Mac mini:Ethernet"));
        assert_eq!((listen.first_channel, listen.channels), (2, 1));
        assert_eq!(listen.output.as_deref(), Some("KM-HIFI"));
        assert_eq!(app.monitor_status.state, State::Starting);
        assert_eq!(set["structuredContent"]["listen"]["output"], "KM-HIFI");
        // Left out, kept; "default" is the default output.
        run(
            &mut app,
            "set_audio_monitor",
            json!({ "output": "default", "gain_db": -6 }),
        );
        assert_eq!(app.settings.listen.output, None);
        assert_eq!(app.settings.listen.first_channel, 2);
        assert_eq!(app.settings.listen.gain_db, -6.0);
        // What a device sends, on one of its output streams, where Core
        // Audio's taps are (a Mac on macOS 14.2 and later); refused, and
        // the setting kept, elsewhere.
        let (sender, answered) = std::sync::mpsc::channel();
        let call = Call::local("set_audio_monitor", Value::Null, move |result| {
            let _ = sender.send(result);
        });
        let _ = find("set_audio_monitor").unwrap().start(
            &mut app,
            json!({ "side": "sent", "stream": 1 }),
            Answer(call),
        );
        let sent = answered
            .recv_timeout(std::time::Duration::from_secs(5))
            .unwrap();
        if crate::monitor::hears_sent() {
            assert!(sent.is_ok());
            assert_eq!(app.settings.listen.side, crate::monitor::Side::Sent);
            assert_eq!(app.settings.listen.stream, 1);
        } else {
            assert!(sent.is_err());
            assert_eq!(app.settings.listen.side, crate::monitor::Side::Received);
        }
        run(&mut app, "set_audio_monitor", json!({ "enabled": false }));
        let read = run(&mut app, "audio_monitor", json!({}));
        assert_eq!(read["structuredContent"]["status"]["state"], "off");
    }

    #[test]
    fn names_are_unique_and_snake_case() {
        let mut names: Vec<_> = TOOLS.iter().map(|tool| tool.name).collect();
        assert!(names.iter().all(|name| {
            !name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase() || c == '_')
        }));
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), TOOLS.len());
    }

    #[test]
    fn every_schema_is_a_compact_object() {
        for tool in TOOLS.iter() {
            let schema = (tool.schema)();
            assert_eq!(schema["type"], "object", "{}: {schema}", tool.name);
            assert!(schema["properties"].is_object(), "{}: {schema}", tool.name);
            let text = schema.to_string();
            for left_out in ["\"$defs\"", "\"$ref\"", "\"anyOf\"", "\"null\"", "uint"] {
                assert!(
                    !text.contains(left_out),
                    "{}: {left_out} in {text}",
                    tool.name
                );
            }
            // Every field the tool requires is described.
            for field in schema["required"].as_array().into_iter().flatten() {
                let field = field.as_str().expect("field names");
                assert!(
                    schema["properties"][field]["description"].is_string(),
                    "{}: {field} in {text}",
                    tool.name
                );
            }
        }
        // A field named as a schema keyword is a field all the same.
        let format = (find("set_stream_format").expect("set_stream_format").schema)();
        assert_eq!(format["properties"]["format"]["type"], "string");
        assert_eq!(format["properties"]["direction"]["enum"][0], "input");
    }

    #[test]
    fn list_has_what_mcp_needs() {
        let list = list();
        for tool in list.as_array().expect("tools") {
            for field in ["name", "title", "description", "inputSchema", "annotations"] {
                assert!(tool.get(field).is_some(), "{field} in {tool}");
            }
        }
        assert_eq!(list[0]["annotations"]["readOnlyHint"], true);
    }

    #[test]
    fn entity_ids_read_with_or_without_0x() {
        assert_eq!(
            entity_id("0xd111e597f5448000").ok(),
            Some(EntityId(0xd111_e597_f544_8000))
        );
        assert_eq!(
            entity_id("d111e597f5448000").ok(),
            Some(EntityId(0xd111_e597_f544_8000))
        );
        assert!(entity_id("0x1234").is_err());
        assert!(entity_id("Mac mini").is_err());
    }

    const MAC_MINI: EntityId = EntityId(0xd111_e597_f544_8000);
    const CONTROLLER: EntityId = EntityId(0x9c6b_00ff_fe30_9a2b);

    fn sample_app(entities_and_models: BenchModels) -> Triib {
        let (entities, models) = entities_and_models;
        let interface = avb_net::Interface {
            name: "en0".to_owned(),
            mac: avb_net::MacAddress([2, 0, 0, 0, 0, 1]),
            up: true,
            speed: None,
            physical: true,
            wireless: false,
            hardware_clock: None,
        };
        Triib::sample(
            crate::settings::Settings::default(),
            interface,
            entities,
            models,
        )
    }

    type BenchModels = (
        std::collections::BTreeMap<EntityId, atdecc::DiscoveredEntity>,
        std::collections::BTreeMap<EntityId, EntityModel>,
    );

    /// The bench with the wired ESP's first input bound to the Mac mini's
    /// first output by another controller, the talker's reservation failed
    /// at a bridge for want of bandwidth, and the input's counters as a
    /// while into a show: locks, lost locks, interruptions, late frames.
    fn troubled() -> Triib {
        use atdecc::aem::StreamInfoFlags;
        use atdecc::{
            AcmpFlags, AcmpMessageType, AcmpStatus, Acmpdu, AecpHeader, AecpMessageType,
            AemCommandType, AemPdu, MacAddress, StreamId,
        };

        let esp = crate::view::tests::WIRED_ESP;
        let connect = Acmpdu {
            message_type: AcmpMessageType::CONNECT_RX_RESPONSE,
            status: AcmpStatus::SUCCESS,
            stream_id: StreamId(MAC_MINI.0 & !0xffff),
            controller_entity_id: EntityId(0x0011_2233_4455_6677),
            talker_entity_id: MAC_MINI,
            listener_entity_id: esp,
            stream_dest_mac: MacAddress([0x91, 0xe0, 0xf0, 0x00, 0x6a, 0x20]),
            connection_count: 1,
            flags: AcmpFlags::empty(),
            stream_vlan_id: 2,
            ..Acmpdu::default()
        };
        let mut out = [0; 200];
        let length = connect.encode(&mut out).unwrap();
        let mut frames = vec![out[..length].to_vec()];
        let mut notify = |command_type, payload: &[u8]| {
            let notification = AemPdu {
                header: AecpHeader {
                    message_type: AecpMessageType::AEM_RESPONSE,
                    status: 0,
                    target_entity_id: esp,
                    controller_entity_id: CONTROLLER,
                    sequence_id: 0,
                },
                unsolicited: true,
                controller_request: false,
                command_type,
                payload,
            };
            let length = notification.encode(&mut out).unwrap();
            frames.push(out[..length].to_vec());
        };
        let input = DescriptorType::STREAM_INPUT.0.to_be_bytes();
        let mut info = vec![0; 48];
        info[..2].copy_from_slice(&input);
        let flags = StreamInfoFlags::CONNECTED
            | StreamInfoFlags::SRP_REGISTRATION_FAILED
            | StreamInfoFlags::MSRP_FAILURE_VALID;
        info[4..8].copy_from_slice(&flags.0.to_be_bytes());
        // Insufficient bandwidth, at the bench's bridge.
        info[34] = 1;
        info[36..44].copy_from_slice(&[0x80, 0x00, 0x00, 0x01, 0xf2, 0xff, 0x3b, 0x14]);
        notify(AemCommandType::GET_STREAM_INFO, &info);
        let mut counters = vec![0; 136];
        counters[..2].copy_from_slice(&input);
        counters[4..8].copy_from_slice(&0xa07u32.to_be_bytes());
        for (position, count) in [(0usize, 3u32), (1, 2), (2, 2), (9, 12), (11, 1000)] {
            counters[8 + 4 * position..12 + 4 * position].copy_from_slice(&count.to_be_bytes());
        }
        notify(AemCommandType::GET_COUNTERS, &counters);
        sample_app(crate::view::tests::bench_then(&frames))
    }

    /// Runs the tool `name` on `arguments`, for a tool that answers at once.
    fn call(app: &mut Triib, name: &str, arguments: Value) -> Result<Value, Error> {
        let (sender, answered) = std::sync::mpsc::channel();
        let call = Call::local(name, Value::Null, move |result| {
            let _ = sender.send(result);
        });
        let _ = find(name).unwrap().start(app, arguments, Answer(call));
        answered.recv_timeout(Duration::from_secs(5)).unwrap()
    }

    /// The problems list a failed reservation and what the counters say,
    /// errors first, and what triib cannot tell as warnings.
    /// A Mac receiving on its own clock is an error, with the clock
    /// source that ends it.
    #[test]
    fn a_mac_on_its_own_clock_is_a_problem() {
        let mut app = crate::diagnostics_view::tests::mac_listening();
        let listed = call(&mut app, "list_problems", json!({})).unwrap();
        let problems = listed["structuredContent"]["problems"].as_array().unwrap();
        let own = problems
            .iter()
            .find(|problem| problem["kind"] == "mac_own_clock")
            .unwrap();
        assert_eq!(own["severity"], "error");
        assert_eq!(
            own["entity_id"],
            crate::diagnostics_view::tests::MAC_MINI.to_string()
        );
        assert_eq!(own["clock_source"], 2);
        assert_eq!(own["follow_clock_source"], 0);
    }

    #[test]
    fn problems_name_what_is_wrong_errors_first() {
        let mut app = troubled();
        let listed = call(&mut app, "list_problems", json!({})).unwrap();
        let problems = listed["structuredContent"]["problems"]
            .as_array()
            .unwrap()
            .clone();
        let kinds: Vec<(&str, &str)> = problems
            .iter()
            .map(|problem| {
                (
                    problem["severity"].as_str().unwrap(),
                    problem["kind"].as_str().unwrap(),
                )
            })
            .collect();
        assert!(
            kinds.contains(&("error", "reservation_failed")),
            "{kinds:?}"
        );
        assert!(kinds.contains(&("warning", "counters")), "{kinds:?}");
        let first_warning = kinds
            .iter()
            .position(|(severity, _)| *severity == "warning");
        let last_error = kinds.iter().rposition(|(severity, _)| *severity == "error");
        assert!(last_error < first_warning, "{kinds:?}");
        let failed = problems
            .iter()
            .find(|problem| problem["kind"] == "reservation_failed")
            .unwrap();
        assert_eq!(failed["listener_entity_id"], "0xe8f60ae092200000");
        assert_eq!(failed["talker_entity_id"], MAC_MINI.to_string());
        assert!(
            failed["says"]
                .as_str()
                .unwrap()
                .contains("insufficient bandwidth"),
            "{failed}"
        );
        assert_eq!(failed["msrp_failure_code"], 1);
        assert_eq!(failed["failing_bridge_mac"], "00:01:f2:ff:3b:14");
        let counted = problems
            .iter()
            .find(|problem| problem["kind"] == "counters")
            .unwrap();
        assert_eq!(counted["descriptor"], "stream_input");
        assert!(
            counted["says"].as_str().unwrap().contains("late"),
            "{counted}"
        );
        // The bench reports no gPTP state: not known to be wrong.
        assert!(
            problems
                .iter()
                .filter(|problem| problem["kind"] == "not_synced")
                .all(|problem| problem["severity"] == "warning")
        );
        // Off the network, only that.
        app.network_state = NetworkState::Starting;
        let listed = call(&mut app, "list_problems", json!({})).unwrap();
        assert_eq!(
            listed["structuredContent"]["problems"],
            json!([{ "kind": "network", "says": "starting", "severity": "error" }])
        );
    }

    /// A stream's diagnostics give its reservation and counters as the
    /// entity said them, and over a sample what changed.
    #[test]
    fn stream_diagnostics_read_and_sample_the_counters() {
        let mut app = troubled();
        let esp = crate::view::tests::WIRED_ESP;
        let unknown = call(
            &mut app,
            "stream_diagnostics",
            json!({ "entity_id": "0x0000000000000001", "direction": "input", "index": 0 }),
        );
        assert!(unknown.is_err());
        let missing = call(
            &mut app,
            "stream_diagnostics",
            json!({ "entity_id": esp.to_string(), "direction": "input", "index": 9 }),
        );
        assert!(missing.is_err());
        let run = |app: &mut Triib, seconds: u16, step: Step| -> Value {
            let (sender, answered) = std::sync::mpsc::channel();
            let call = Call::local("stream_diagnostics", Value::Null, move |result| {
                let _ = sender.send(result);
            });
            let _ = sampled(
                app,
                Sampling {
                    answer: Answer(call),
                    entity: esp,
                    descriptor_type: DescriptorType::STREAM_INPUT,
                    index: 0,
                    seconds,
                    step,
                },
            );
            answered
                .recv_timeout(Duration::from_secs(5))
                .unwrap()
                .unwrap()["structuredContent"]
                .clone()
        };
        let read = run(&mut app, 0, Step::Asked);
        assert_eq!(read["bound_to"]["talker_entity_id"], MAC_MINI.to_string());
        assert_eq!(read["stream_info"]["talker_failed"], true);
        assert_eq!(read["stream_info"]["reservation_failure"]["code"], 1);
        assert_eq!(read["network"]["status"], "failed");
        assert_eq!(read["counters"]["late_timestamp"], 12);
        assert_eq!(read["counters"]["frames_rx"], 1000);
        assert!(read["counters"].get("early_timestamp").is_none());
        assert!(!read["went_wrong"].as_array().unwrap().is_empty());
        assert!(read["sample"].is_null());
        // Ten seconds before, 400 frames and 2 late frames fewer.
        let mut first = *app
            .models
            .get(&esp)
            .unwrap()
            .counters(DescriptorType::STREAM_INPUT, 0)
            .unwrap();
        first.block[9] -= 2;
        first.block[11] -= 400;
        let sampled = run(&mut app, 10, Step::AskedAgain(Some(first)));
        assert_eq!(sampled["sample"]["seconds"], 10);
        assert_eq!(sampled["sample"]["changed"]["frames_rx"], 400);
        assert_eq!(sampled["sample"]["changed"]["media_unlocked"], 0);
        assert_eq!(
            sampled["sample"]["went_wrong"],
            json!({ "late_timestamp": 2 })
        );
        assert_eq!(sampled["sample"]["frames_per_second"], 40);
    }

    /// The clock tree gives each node with its parent and whether it keeps
    /// time, and the domain's grandmaster.
    #[test]
    fn the_clock_tree_follows_the_map() {
        let mut app = sample_app(crate::view::tests::bench_then(
            &crate::view::tests::lite_frames(-180, 6_336),
        ));
        let tree = call(&mut app, "clock_tree", json!({})).unwrap()["structuredContent"].clone();
        let nodes = tree["nodes"].as_array().unwrap();
        let esp = nodes
            .iter()
            .find(|node| node["entity_id"] == crate::view::tests::WIRED_ESP.to_string())
            .unwrap();
        assert_eq!(esp["synced"], true, "{esp}");
        assert_eq!(esp["link_up"]["offset_from_grandmaster_ns"], -180);
        let parent = esp["parent"].as_u64().unwrap() as usize;
        assert_eq!(nodes[parent]["kind"], "switch taking no part in AVB");
        assert!(!tree["grandmasters"].as_array().unwrap().is_empty());
        for node in nodes {
            assert!(node["state"].is_string(), "{node}");
        }
    }

    #[test]
    fn outputs_become_mcp_results() {
        let json = Output::Json(json!({ "a": 1 })).into_result();
        assert_eq!(json["structuredContent"]["a"], 1);
        assert_eq!(json["content"][0]["text"], r#"{"a":1}"#);
        let text = Output::Text("done".into()).into_result();
        assert_eq!(text["content"][0]["text"], "done");
    }
}
