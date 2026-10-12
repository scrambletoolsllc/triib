//! Outside control: the calls programs make over the control channel,
//! mostly AI agents through `triib --mcp`. triib asks the user before an
//! agent it has not seen gets to act.

use std::collections::BTreeSet;
use std::time::{Duration, Instant};

use iced::{Element, Task, window};
use scramble_ui::button::{self, Kind};
use scramble_ui::component;
use scramble_ui::icon::Icon;
use serde_json::{Value, json};

use crate::app::{Message, Triib};
use crate::control::{Call, Error, code};
use crate::fl;
use crate::network::Action;
use crate::tools::{self, Answer};

/// The notification a call sends while it waits for the user.
const WAITING: &str = "triib/waiting";

/// What agents hear while outside control is off.
const OFF_MESSAGE: &str = "Outside control is off in triib's Settings.";

/// How long a new prompt's buttons wait before taking clicks, so a click
/// meant for the window under it does not answer it.
const ANSWER_DELAY: Duration = Duration::from_millis(700);

/// Agents waiting for the user to allow them, the ones turned away, and
/// the changes agents wait on.
#[derive(Default)]
pub struct Agents {
    /// The agents to ask about, the first shown first.
    prompts: Vec<Prompt>,
    /// Agents the user did not allow, until triib quits.
    declined: BTreeSet<String>,
    /// When the first prompt came up.
    shown_at: Option<Instant>,
    /// Actions agents asked for, answered once the network has done them.
    pub waits: Vec<(Action, Answer)>,
}

/// An agent waiting for the user's answer, with the calls it made
/// meanwhile.
struct Prompt {
    name: String,
    /// The name shown to the user: the agent's title when it gives one.
    shown: String,
    waiting: Vec<Call>,
}

/// The agent a call came from, as `triib --mcp` passes it on.
fn agent(params: &Value) -> (String, String) {
    let name = params["name"].as_str().unwrap_or_default().trim();
    let name = if name.is_empty() { "agent" } else { name };
    let shown = params["title"]
        .as_str()
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .unwrap_or(name);
    (name.to_owned(), shown.to_owned())
}

impl Triib {
    pub fn control(&mut self, call: Call) -> Task<Message> {
        let answer = match call.method.as_str() {
            "ping" => Ok(json!({
                "protocol": crate::control::PROTOCOL,
                "version": env!("CARGO_PKG_VERSION"),
                "development": !triib_store::paths::PRODUCTION,
            })),
            // Listing does nothing, so agents see the tools while outside
            // control is off, and hear why at their first call.
            "tools/list" => Ok(json!({ "tools": tools::list() })),
            _ if !self.settings.outside_control => Err(Error::new(code::OFF, OFF_MESSAGE)),
            // An agent connected: ask the user now, not at its first call.
            "agent/hello" => {
                let (name, shown) = agent(&call.params);
                call.reply(Ok(Value::Null));
                return self.ask_about(name, shown, None);
            }
            "tools/call" => {
                let (name, shown) = agent(&call.params["agent"]);
                if self.settings.allowed_agents.contains(&name) {
                    return self.run_tool(call);
                } else if self.agents.declined.contains(&name) {
                    Err(declined(&shown))
                } else {
                    return self.ask_about(name, shown, Some(call));
                }
            }
            method => Err(Error::new(
                code::METHOD_NOT_FOUND,
                format!("triib has no method {method}"),
            )),
        };
        call.reply(answer);
        Task::none()
    }

    /// Runs the tool a call names.
    fn run_tool(&mut self, call: Call) -> Task<Message> {
        let name = call.params["name"].as_str().unwrap_or_default().to_owned();
        let arguments = match &call.params["arguments"] {
            Value::Null => json!({}),
            arguments => arguments.clone(),
        };
        match tools::find(&name) {
            Some(tool) => tool.start(self, arguments, Answer(call)),
            None => {
                call.reply(Err(Error::new(
                    code::INVALID_PARAMS,
                    format!("triib has no tool {name}"),
                )));
                Task::none()
            }
        }
    }

    /// Asks the user whether agent `name` may control triib, unless they
    /// already answered; `call` waits for the answer.
    fn ask_about(&mut self, name: String, shown: String, call: Option<Call>) -> Task<Message> {
        if self.settings.allowed_agents.contains(&name) || self.agents.declined.contains(&name) {
            if let Some(call) = call {
                return self.control(call);
            }
            return Task::none();
        }
        if let Some(call) = &call {
            call.note(
                WAITING,
                json!({
                    "message": format!(
                        "Waiting for the user to allow {shown} to control triib; triib asks in \
                         its window."
                    ),
                }),
            );
        }
        if let Some(prompt) = self
            .agents
            .prompts
            .iter_mut()
            .find(|prompt| prompt.name == name)
        {
            prompt.waiting.extend(call);
            return Task::none();
        }
        self.agents.prompts.push(Prompt {
            name,
            shown,
            waiting: call.into_iter().collect(),
        });
        if self.agents.prompts.len() == 1 {
            self.prompt_shown()
        } else {
            Task::none()
        }
    }

    /// A prompt came up first: its buttons take clicks after a moment, and
    /// the window asks for attention until the user comes to it, as triib
    /// may be behind other windows.
    fn prompt_shown(&mut self) -> Task<Message> {
        self.agents.shown_at = Some(Instant::now());
        let (ready, waited) = tokio::sync::oneshot::channel();
        std::thread::spawn(move || {
            std::thread::sleep(ANSWER_DELAY);
            let _ = ready.send(());
        });
        Task::batch([
            Task::perform(waited, |_| Message::AgentPromptReady),
            window::latest().and_then(|id| {
                window::request_user_attention(id, Some(window::UserAttention::Critical))
            }),
        ])
    }

    /// Whether the first prompt has been up long enough to take answers.
    fn prompt_ready(&self) -> bool {
        self.agents
            .shown_at
            .is_some_and(|shown| shown.elapsed() >= ANSWER_DELAY)
    }

    /// The user answered the first prompt.
    pub fn agent_answered(&mut self, allow: bool) -> Task<Message> {
        if self.agents.prompts.is_empty() || !self.prompt_ready() {
            return Task::none();
        }
        let prompt = self.agents.prompts.remove(0);
        if allow {
            self.settings.allowed_agents.push(prompt.name);
            self.save_settings();
        } else {
            self.agents.declined.insert(prompt.name);
        }
        let mut tasks = Vec::new();
        for call in prompt.waiting {
            if allow {
                tasks.push(self.run_tool(call));
            } else {
                call.reply(Err(declined(&prompt.shown)));
            }
        }
        if self.agents.prompts.is_empty() {
            self.agents.shown_at = None;
        } else {
            tasks.push(self.prompt_shown());
        }
        Task::batch(tasks)
    }

    /// Outside control was turned off: waiting agents hear so.
    pub fn outside_control_off(&mut self) {
        for prompt in self.agents.prompts.drain(..) {
            for call in prompt.waiting {
                call.reply(Err(Error::new(code::OFF, OFF_MESSAGE)));
            }
        }
        self.agents.shown_at = None;
    }

    /// The network finished `action`: the agent that asked for it hears
    /// how it went.
    pub fn agent_action_finished(&mut self, action: Action, outcome: atdecc::controller::Outcome) {
        if let Some(position) = self
            .agents
            .waits
            .iter()
            .position(|(waiting, _)| *waiting == action)
        {
            let (_, answer) = self.agents.waits.remove(position);
            answer.send(tools::finished(self, action, outcome));
        }
    }

    /// The network stopped: the changes agents wait on will not finish.
    pub fn agent_actions_dropped(&mut self) {
        for (_, answer) in self.agents.waits.drain(..) {
            answer.send(Err(Error::new(
                code::INTERNAL_ERROR,
                "triib's network stopped before the device answered",
            )));
        }
    }

    /// The prompt asking whether the first agent may control triib, over
    /// `base`.
    pub fn agent_prompt<'a>(&'a self, base: Element<'a, Message>) -> Element<'a, Message> {
        let Some(prompt) = self.agents.prompts.first() else {
            return base;
        };
        let agent = prompt.shown.as_str();
        let ready = self.prompt_ready();
        component::dialog(
            base,
            Some(Icon::Lock),
            fl!("agent-prompt-title", agent = agent),
            fl!("agent-prompt-body", agent = agent),
            vec![
                button::button(Kind::Text, fl!("agent-prompt-deny"))
                    .on_press_maybe(ready.then_some(Message::AgentAnswered(false)))
                    .into(),
                button::button(Kind::Filled, fl!("agent-prompt-allow"))
                    .on_press_maybe(ready.then_some(Message::AgentAnswered(true)))
                    .into(),
            ],
        )
    }
}

fn declined(agent: &str) -> Error {
    Error::new(
        code::DECLINED,
        format!("The user did not allow {agent} to control triib."),
    )
}
