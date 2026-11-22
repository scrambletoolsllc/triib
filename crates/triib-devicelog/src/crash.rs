//! A device's crash dump, kept in a CRASH_DUMP memory object: read one
//! Address Access READ at a time, as much as one response carries, each
//! part tried three times; reading never clears it. Erasing it is a
//! START_OPERATION (ERASE) on the memory object, after which the object's
//! length reads 0.
//!
//! An ESP32 board keeps ESP-IDF's core dump image as stored, which
//! `esp-coredump` reads with the crashed firmware's ELF.

use std::path::Path;

use atdecc::EntityId;
use atdecc::aecp::AaStatus;
use atdecc::controller::{MemoryRead, Outcome};
use atdecc::descriptor::MemoryObjectType;
use atdecc::entity::MEMORY_SEGMENT;
use atdecc::model::EntityModel;

/// Tries for each part of a dump.
pub const TRIES: u32 = 3;

/// What a device's CRASH_DUMP descriptor says: its index, where its
/// octets start and how many there are, 0 with no crash kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrashDump {
    pub index: u16,
    pub start: u64,
    pub length: u64,
}

/// The crash dump an entity keeps, as its model last read it.
pub fn crash_dump(model: &EntityModel) -> Option<CrashDump> {
    model
        .memory_objects()
        .find(|object| object.memory_object_type == MemoryObjectType::CRASH_DUMP)
        .map(|object| CrashDump {
            index: object.index,
            start: object.start_address,
            length: object.length,
        })
}

/// The file a dump is saved to unless another is given.
pub fn file_name(entity: EntityId) -> String {
    format!("crash-{:016x}.core", entity.0)
}

/// How to read a dump saved at `path`, in English.
pub fn decode_hint(path: &Path) -> String {
    format!(
        "esp-coredump info_corefile --core {} --core-format raw <the crashed firmware's .elf>",
        path.display()
    )
}

/// Why a dump could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Failure {
    /// The device refused a READ at this address.
    Refused { address: u64, status: AaStatus },
    /// A part went unread after every try: what the last try met.
    Unread { address: u64, last: Missed },
    /// The dump is too long to hold.
    TooLong,
}

/// What a try at a part met instead of its octets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Missed {
    /// An answer of another part, or of the wrong length.
    WrongPart,
    /// The command finished otherwise, unanswered or refused.
    Outcome(Outcome),
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Failure::Refused { address, status } => {
                write!(
                    f,
                    "the device refused to give its dump at 0x{address:x}: {status:?}"
                )
            }
            Failure::Unread {
                address,
                last: Missed::WrongPart,
            } => write!(f, "reading at 0x{address:x}: an answer of the wrong part"),
            Failure::Unread {
                address,
                last: Missed::Outcome(outcome),
            } => write!(f, "reading at 0x{address:x}: {outcome:?}"),
            Failure::TooLong => write!(f, "a dump too long to hold"),
        }
    }
}

/// A dump being read, a part at a time: [`next`](Self::next) gives the
/// READ to send, [`handle`](Self::handle) takes how it finished.
#[derive(Debug, Clone)]
pub struct DumpRead {
    object: CrashDump,
    total: usize,
    octets: Vec<u8>,
    tries: u32,
}

impl DumpRead {
    /// Reads the dump `object` describes, its length read afresh.
    pub fn new(object: CrashDump) -> Result<Self, Failure> {
        let total = usize::try_from(object.length).map_err(|_| Failure::TooLong)?;
        Ok(DumpRead {
            object,
            total,
            octets: Vec::with_capacity(total),
            tries: 0,
        })
    }

    /// The next part to read, as a READ TLV's address and length, none
    /// once it is all read.
    pub fn next(&self) -> Option<(u64, u16)> {
        let left = self.total - self.octets.len();
        (left > 0).then(|| {
            let length = left.min(usize::from(MEMORY_SEGMENT)) as u16;
            (self.object.start + self.octets.len() as u64, length)
        })
    }

    /// Takes how the READ of [`next`](Self::next) finished, and its answer
    /// where it was done; a part is tried again until it has been tried
    /// [`TRIES`] times.
    pub fn handle(&mut self, outcome: Outcome, answer: Option<&MemoryRead>) -> Result<(), Failure> {
        let Some((address, length)) = self.next() else {
            return Ok(());
        };
        let missed = match (outcome, answer) {
            (Outcome::Done, Some(answer)) if answer.status != AaStatus::SUCCESS => {
                return Err(Failure::Refused {
                    address,
                    status: answer.status,
                });
            }
            (Outcome::Done, Some(answer)) => match answer.tlvs.first() {
                Some((at, octets)) if *at == address && octets.len() == usize::from(length) => {
                    self.octets.extend_from_slice(octets);
                    self.tries = 0;
                    return Ok(());
                }
                _ => Missed::WrongPart,
            },
            (outcome, _) => Missed::Outcome(outcome),
        };
        self.tries += 1;
        if self.tries >= TRIES {
            return Err(Failure::Unread {
                address,
                last: missed,
            });
        }
        Ok(())
    }

    /// Octets read so far, and of how many.
    pub fn progress(&self) -> (usize, usize) {
        (self.octets.len(), self.total)
    }

    pub fn done(&self) -> bool {
        self.octets.len() == self.total
    }

    /// The memory object being read.
    pub fn object(&self) -> CrashDump {
        self.object
    }

    /// The dump, once [`done`](Self::done).
    pub fn into_octets(self) -> Vec<u8> {
        self.octets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_of(address: u64, octets: Vec<u8>) -> MemoryRead {
        MemoryRead {
            status: AaStatus::SUCCESS,
            tlvs: vec![(address, octets)],
        }
    }

    /// A dump longer than one response reads in parts, each from where the
    /// last ended, a part missed once tried again.
    #[test]
    fn a_dump_reads_a_part_at_a_time() {
        let segment = usize::from(MEMORY_SEGMENT);
        let total = 2 * segment + 10;
        let dump: Vec<u8> = (0..total).map(|place| place as u8).collect();
        let object = CrashDump {
            index: 2,
            start: 0x2000_0000,
            length: total as u64,
        };
        let mut read = DumpRead::new(object).unwrap();
        let mut answered_wrong = false;
        while let Some((address, length)) = read.next() {
            let from = (address - object.start) as usize;
            if from == segment && !answered_wrong {
                answered_wrong = true;
                read.handle(Outcome::NoResponse, None).unwrap();
                continue;
            }
            let part = dump[from..from + usize::from(length)].to_vec();
            read.handle(Outcome::Done, Some(&read_of(address, part)))
                .unwrap();
        }
        assert!(read.done());
        assert_eq!(read.progress(), (total, total));
        assert_eq!(read.into_octets(), dump);
    }

    #[test]
    fn a_part_unread_after_every_try_fails() {
        let object = CrashDump {
            index: 0,
            start: 0x100,
            length: 8,
        };
        let mut read = DumpRead::new(object).unwrap();
        read.handle(Outcome::NoResponse, None).unwrap();
        read.handle(Outcome::Done, Some(&read_of(0x200, vec![0; 8])))
            .unwrap();
        assert_eq!(
            read.handle(Outcome::NoResponse, None),
            Err(Failure::Unread {
                address: 0x100,
                last: Missed::Outcome(Outcome::NoResponse)
            })
        );
    }

    #[test]
    fn a_refused_read_fails_at_once() {
        let object = CrashDump {
            index: 0,
            start: 0x100,
            length: 8,
        };
        let mut read = DumpRead::new(object).unwrap();
        let refused = MemoryRead {
            status: AaStatus::ADDRESS_INVALID,
            tlvs: Vec::new(),
        };
        assert!(matches!(
            read.handle(Outcome::Done, Some(&refused)),
            Err(Failure::Refused { address: 0x100, .. })
        ));
    }

    #[test]
    fn an_empty_dump_has_nothing_to_read() {
        let object = CrashDump {
            index: 0,
            start: 0x100,
            length: 0,
        };
        let read = DumpRead::new(object).unwrap();
        assert_eq!(read.next(), None);
        assert!(read.done());
    }
}
