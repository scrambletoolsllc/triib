//! The tools triib offers agents through `triib --mcp`. Each has a name,
//! a kind, a description the model reads, an input schema made from its
//! input type, and a function that runs it as a click would. A tool that
//! reads answers at once; one that changes a device answers once the
//! device has, with what it said.

use std::sync::LazyLock;

use atdecc::controller::Outcome;
use atdecc::descriptor::SamplingRate;
use atdecc::model::{EntityModel, EnumerationState};
use atdecc::{DescriptorType, EntityId, StreamFormat};
use iced::Task;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::app::{Message, NetworkState, Triib};
use crate::control::{Call, Error, code};
use crate::describe;
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
             as describe_entity lists them. An entity may refuse while the stream runs, and \
             one may move its sampling rate and its other streams with it. Answers once the \
             entity has.",
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
                    format!("failed: {}; the user can run {command}", failure.message)
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

    #[test]
    fn outputs_become_mcp_results() {
        let json = Output::Json(json!({ "a": 1 })).into_result();
        assert_eq!(json["structuredContent"]["a"], 1);
        assert_eq!(json["content"][0]["text"], r#"{"a":1}"#);
        let text = Output::Text("done".into()).into_result();
        assert_eq!(text["content"][0]["text"], "done");
    }
}
