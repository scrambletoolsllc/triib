//! The inspector's diagnostics: how each clock domain and stream has
//! fared since the entity started, from the counters it keeps, with what
//! went wrong marked, and how each bound stream input's reservation
//! stands.

use atdecc::descriptor::DescriptorType;
use atdecc::model::EntityModel;
use iced::Element;
use scramble_ui::component;
use scramble_ui::font::{Type, styled};
use scramble_ui::style;

use crate::app::Message;
use crate::describe;
use crate::fl;

/// What a line says: what went fine, and what went wrong.
#[derive(Debug, Default, PartialEq)]
struct Facts {
    fine: Vec<String>,
    wrong: Vec<String>,
}

impl Facts {
    fn fine(&mut self, text: String) {
        self.fine.push(text);
    }

    /// A count of something that should not happen, in words, when it
    /// did.
    fn wrong_count(&mut self, count: Option<u32>, words: impl Fn(u32) -> String) {
        if let Some(count) = count.filter(|&count| count > 0) {
            self.wrong.push(words(count));
        }
    }

    fn is_empty(&self) -> bool {
        self.fine.is_empty() && self.wrong.is_empty()
    }
}

/// A count with thousands set apart, as 1,204,331.
fn number(count: u32) -> String {
    let separator = crate::i18n::thousands_separator();
    let digits = count.to_string();
    let mut text = String::new();
    for (position, digit) in digits.chars().enumerate() {
        if position > 0 && (digits.len() - position).is_multiple_of(3) {
            text.push_str(&separator);
        }
        text.push(digit);
    }
    text
}

/// Text for a count: `$count` to pick the words by, `$number` to show,
/// as in "stopped 3 times" or "12 frames late".
macro_rules! counted {
    ($key:literal, $count:expr) => {{
        let count: u32 = $count;
        fl!($key, count = count, number = number(count))
    }};
}

/// What the diagnostics say of one of an entity's clock domains or
/// streams: what went fine, and what went wrong.
pub struct Finding {
    pub descriptor_type: DescriptorType,
    pub index: u16,
    pub name: String,
    pub fine: Vec<String>,
    pub wrong: Vec<String>,
}

/// The diagnostics section, or nothing when the entity reports no
/// counters and no reservations.
pub fn diagnostics<'a>(model: &EntityModel) -> Vec<Element<'a, Message>> {
    let items: Vec<Element<'a, Message>> = findings(model)
        .into_iter()
        .filter_map(|finding| {
            item(
                &finding.name,
                Facts {
                    fine: finding.fine,
                    wrong: finding.wrong,
                },
            )
        })
        .collect();
    if items.is_empty() {
        return items;
    }
    let mut section = vec![
        component::section(fl!("inspector-diagnostics")),
        styled(fl!("diag-since-start"), Type::BodySmall)
            .style(style::on_surface_variant)
            .into(),
    ];
    section.extend(items);
    section
}

/// What the entity's counters and reservations say of each of its clock
/// domains and streams, since it started, leaving out those it says
/// nothing of.
pub fn findings(model: &EntityModel) -> Vec<Finding> {
    let mut items = Vec::new();
    let mut found = |descriptor_type, index, name: String, facts: Facts| {
        if !facts.is_empty() {
            items.push(Finding {
                descriptor_type,
                index,
                name,
                fine: facts.fine,
                wrong: facts.wrong,
            });
        }
    };
    for domain in model.clock_domains() {
        let Some(counters) = model.clock_domain_counters(domain.index) else {
            continue;
        };
        let mut facts = Facts::default();
        if let Some(locked) = counters.locked {
            facts.fine(counted!("diag-locked", locked));
        }
        facts.wrong_count(counters.unlocked, |count| counted!("diag-lost-lock", count));
        let name = model
            .name_of(DescriptorType::CLOCK_DOMAIN, domain.index)
            .map_or_else(|| fl!("entity-clock-domain"), str::to_owned);
        found(DescriptorType::CLOCK_DOMAIN, domain.index, name, facts);
    }
    for stream in model.streams(true) {
        let mut facts = Facts::default();
        if let Some(counters) = model.stream_input_counters(stream.index) {
            if let Some(frames) = counters.frames_rx {
                facts.fine(counted!("diag-frames-in", frames));
            }
            if let Some(locked) = counters.media_locked {
                facts.fine(counted!("diag-media-locked", locked));
            }
            facts.wrong_count(counters.media_unlocked, |count| {
                counted!("diag-lost-media-lock", count)
            });
            facts.wrong_count(counters.stream_interrupted, |count| {
                counted!("diag-interrupted", count)
            });
            facts.wrong_count(counters.seq_num_mismatch, |count| {
                counted!("diag-out-of-sequence", count)
            });
            facts.wrong_count(counters.media_reset, |count| {
                counted!("diag-media-resets", count)
            });
            facts.wrong_count(counters.timestamp_uncertain, |count| {
                counted!("diag-timestamps-uncertain", count)
            });
            facts.wrong_count(counters.timestamp_not_valid, |count| {
                counted!("diag-no-timestamp", count)
            });
            facts.wrong_count(counters.unsupported_format, |count| {
                counted!("diag-unsupported-format", count)
            });
            facts.wrong_count(counters.late_timestamp, |count| {
                counted!("diag-late", count)
            });
            facts.wrong_count(counters.early_timestamp, |count| {
                counted!("diag-early", count)
            });
        }
        if let Some(info) = model.stream_info(DescriptorType::STREAM_INPUT, stream.index)
            && info.bound()
        {
            if info.talker_failed() {
                facts.wrong.push(fl!(
                    "diag-reservation-failed",
                    reason = describe::reservation_failure(info)
                ));
            } else if info.registering() {
                facts.fine(fl!(
                    "diag-latency",
                    microseconds = number(info.msrp_accumulated_latency / 1000)
                ));
            }
        }
        let name = model
            .name_of(DescriptorType::STREAM_INPUT, stream.index)
            .map_or_else(|| fl!("diag-stream-input"), str::to_owned);
        found(DescriptorType::STREAM_INPUT, stream.index, name, facts);
    }
    for stream in model.streams(false) {
        let Some(counters) = model.stream_output_counters(stream.index) else {
            continue;
        };
        let mut facts = Facts::default();
        if let Some(frames) = counters.frames_tx {
            facts.fine(counted!("diag-frames-out", frames));
        }
        if let Some(started) = counters.stream_start {
            facts.fine(counted!("diag-started", started));
        }
        if let Some(stopped) = counters.stream_stop {
            facts.fine(counted!("diag-stopped", stopped));
        }
        facts.wrong_count(counters.stream_interrupted, |count| {
            counted!("diag-interrupted", count)
        });
        facts.wrong_count(counters.media_reset, |count| {
            counted!("diag-media-resets", count)
        });
        facts.wrong_count(counters.timestamp_uncertain, |count| {
            counted!("diag-timestamps-uncertain", count)
        });
        facts.wrong_count(counters.timestamp_not_valid, |count| {
            counted!("diag-no-timestamp", count)
        });
        let name = model
            .name_of(DescriptorType::STREAM_OUTPUT, stream.index)
            .map_or_else(|| fl!("diag-stream-output"), str::to_owned);
        found(DescriptorType::STREAM_OUTPUT, stream.index, name, facts);
    }
    items
}

/// A name with what went fine under it, then what went wrong, marked.
fn item<'a>(name: &str, facts: Facts) -> Option<Element<'a, Message>> {
    if facts.is_empty() {
        return None;
    }
    let mut lines = scramble_ui::column![styled(name.to_owned(), Type::BodyMedium)].spacing(2);
    // The facts as a sentence, or in right to left languages each on a
    // line of its own: cosmic-text wraps a right to left line holding left
    // to right words, such as "0 µs", wider than the room it has.
    let paragraphs = |facts: Vec<String>| {
        if scramble_ui::dir::mirrored() {
            facts.iter().map(|fact| capitalized(fact)).collect()
        } else {
            vec![capitalized(&crate::i18n::list(facts))]
        }
    };
    let line = |text: String| {
        styled(text, Type::BodySmall)
            .width(iced::Fill)
            .align_x(scramble_ui::dir::text_start())
    };
    for text in paragraphs(facts.fine) {
        lines = lines.push(line(text).style(style::on_surface_variant));
    }
    for text in paragraphs(facts.wrong) {
        lines = lines.push(line(text).style(style::error_text));
    }
    Some(lines.into())
}

fn capitalized(text: &str) -> String {
    crate::i18n::capitalized(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_read_naturally() {
        assert_eq!(number(7), "7");
        assert_eq!(number(1204), "1,204");
        assert_eq!(number(1_204_331), "1,204,331");
        assert_eq!(counted!("diag-stopped", 0), "not stopped");
        assert_eq!(counted!("diag-locked", 1), "locked once");
        assert_eq!(counted!("diag-locked", 2), "locked twice");
        assert_eq!(
            counted!("diag-interrupted", 1000),
            "interrupted 1,000 times"
        );
        assert_eq!(counted!("diag-frames-in", 1), "1 frame in");
        assert_eq!(counted!("diag-late", 12), "12 frames late");
        assert_eq!(capitalized("lost lock 2 times"), "Lost lock 2 times");
    }

    #[test]
    fn only_what_happened_is_wrong() {
        let mut facts = Facts::default();
        facts.wrong_count(Some(0), |count| format!("{count} late"));
        facts.wrong_count(None, |count| format!("{count} early"));
        assert!(facts.is_empty());
        facts.wrong_count(Some(3), |count| format!("{count} late"));
        assert_eq!(facts.wrong, ["3 late"]);
    }
}
