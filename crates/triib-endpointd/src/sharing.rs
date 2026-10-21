//! The port's MRP participant, shared with other programs on this
//! computer (`avb_mrp::sharing`), such as plaa sending what it plays: two
//! participants on one port would withdraw each other's declarations, so
//! the daemon declares their talkers in its own, tells them as listeners
//! for their streams come and go, and withdraws a program's talkers when
//! it goes.

#[cfg(unix)]
pub use unix::*;

#[cfg(not(unix))]
pub use elsewhere::*;

#[cfg(unix)]
mod unix {
    use std::collections::HashMap;
    use std::io::ErrorKind;
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc::Sender;
    use std::thread::JoinHandle;
    use std::time::Duration;

    use avb_mrp::msrp::{ListenerState, TalkerDeclaration};
    use avb_mrp::sharing::{self, Notice, Request};

    /// Where a program sharing the participant is told.
    pub type Writer = UnixStream;

    /// How often the server looks whether to stop, between connections.
    const LOOK: Duration = Duration::from_millis(200);

    /// The thread taking connections. Dropping it stops it and removes the
    /// socket.
    pub struct Server {
        stop: Arc<AtomicBool>,
        thread: Option<JoinHandle<()>>,
        path: PathBuf,
    }

    impl Server {
        /// Serves `interface`'s participant, each connection, request and
        /// close going to `sender` as `opened`, `asked` and `closed` make
        /// them. None where another program serves it already, or where there
        /// is no runtime folder.
        pub fn start<T: Send + 'static>(
            interface: &str,
            sender: Sender<T>,
            opened: fn(u64, Writer) -> T,
            asked: fn(u64, Request) -> T,
            closed: fn(u64) -> T,
        ) -> Option<Self> {
            let path = sharing::socket_path(interface)?;
            if path.exists() {
                // Left by a daemon that ended without removing it, unless one
                // still answers.
                if UnixStream::connect(&path).is_ok() {
                    return None;
                }
                let _ = std::fs::remove_file(&path);
            }
            let listener = UnixListener::bind(&path).ok()?;
            listener.set_nonblocking(true).ok()?;
            let stop = Arc::new(AtomicBool::new(false));
            let thread = {
                let stop = stop.clone();
                std::thread::Builder::new()
                    .name("sharing".into())
                    .spawn(move || {
                        let mut next = 0u64;
                        while !stop.load(Ordering::Relaxed) {
                            match listener.accept() {
                                Ok((stream, _)) => {
                                    next += 1;
                                    serve(next, stream, &sender, opened, asked, closed);
                                }
                                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                                    std::thread::sleep(LOOK);
                                }
                                Err(_) => std::thread::sleep(LOOK),
                            }
                        }
                    })
                    .ok()?
            };
            Some(Self {
                stop,
                thread: Some(thread),
                path,
            })
        }
    }

    impl Drop for Server {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::Relaxed);
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
            let _ = std::fs::remove_file(&self.path);
        }
    }

    /// Hands connection `id`'s writing end over, and reads its requests on a
    /// thread of its own until it closes.
    fn serve<T: Send + 'static>(
        id: u64,
        stream: UnixStream,
        sender: &Sender<T>,
        opened: fn(u64, Writer) -> T,
        asked: fn(u64, Request) -> T,
        closed: fn(u64) -> T,
    ) {
        if stream.set_nonblocking(false).is_err() {
            return;
        }
        let Ok(writer) = stream.try_clone() else {
            return;
        };
        if sender.send(opened(id, writer)).is_err() {
            return;
        }
        let sender = sender.clone();
        let _ = std::thread::Builder::new()
            .name("sharer".into())
            .spawn(move || {
                let mut reader = stream;
                while let Ok(request) = sharing::read_request(&mut reader) {
                    if sender.send(asked(id, request)).is_err() {
                        return;
                    }
                }
                let _ = sender.send(closed(id));
            });
    }

    /// A program sharing the participant: where to tell it, and the streams
    /// it declared.
    struct Sharer {
        writer: UnixStream,
        streams: HashMap<u64, Declared>,
    }

    /// A stream a program declared: its Talker Advertise, and the listeners
    /// it was last told of.
    struct Declared {
        declaration: TalkerDeclaration,
        told: Option<ListenerState>,
    }

    /// The programs sharing the participant, by connection.
    #[derive(Default)]
    pub struct Sharers {
        sharers: HashMap<u64, Sharer>,
    }

    impl Sharers {
        pub fn open(&mut self, id: u64, writer: UnixStream) {
            self.sharers.insert(
                id,
                Sharer {
                    writer,
                    streams: HashMap::new(),
                },
            );
        }

        /// Notes that `id` declared `declaration`, of which it knows no
        /// listener yet.
        pub fn declared(&mut self, id: u64, declaration: TalkerDeclaration) {
            if let Some(sharer) = self.sharers.get_mut(&id) {
                sharer.streams.insert(
                    declaration.stream_id,
                    Declared {
                        declaration,
                        told: None,
                    },
                );
            }
        }

        /// Notes that `id` withdrew `stream_id`: whether it had declared it.
        pub fn withdrawn(&mut self, id: u64, stream_id: u64) -> bool {
            self.sharers
                .get_mut(&id)
                .is_some_and(|sharer| sharer.streams.remove(&stream_id).is_some())
        }

        /// `id` went: the streams it had declared.
        pub fn close(&mut self, id: u64) -> Vec<u64> {
            self.sharers
                .remove(&id)
                .map(|sharer| sharer.streams.into_keys().collect())
                .unwrap_or_default()
        }

        /// Every stream the programs declared, which no longer are, as when
        /// the port moves to AVB Lite: they declare them again when told they
        /// can.
        pub fn forget_streams(&mut self) -> Vec<u64> {
            self.sharers
                .values_mut()
                .flat_map(|sharer| sharer.streams.drain().map(|(stream_id, _)| stream_id))
                .collect()
        }

        /// Every stream the programs declared.
        pub fn streams(&self) -> Vec<u64> {
            self.sharers
                .values()
                .flat_map(|sharer| sharer.streams.keys().copied())
                .collect()
        }

        /// The program that declared `stream_id`.
        pub fn owner(&self, stream_id: u64) -> Option<u64> {
            self.sharers
                .iter()
                .find(|(_, sharer)| sharer.streams.contains_key(&stream_id))
                .map(|(id, _)| *id)
        }

        /// The Talker Advertise a program declared for `stream_id`.
        pub fn declaration(&self, stream_id: u64) -> Option<TalkerDeclaration> {
            self.sharers
                .values()
                .find_map(|sharer| sharer.streams.get(&stream_id))
                .map(|declared| declared.declaration)
        }

        /// Tells the program that declared `stream_id` its listeners'
        /// `state`, unless that is what it was told last.
        pub fn tell_listener(&mut self, stream_id: u64, state: Option<ListenerState>) {
            let Some(owner) = self.owner(stream_id) else {
                return;
            };
            let Some(declared) = self
                .sharers
                .get_mut(&owner)
                .and_then(|sharer| sharer.streams.get_mut(&stream_id))
            else {
                return;
            };
            if declared.told == state {
                return;
            }
            declared.told = state;
            self.tell(owner, Notice::Listener { stream_id, state });
        }

        /// Tells `id` `notice`, dropping it if it can no longer be told.
        pub fn tell(&mut self, id: u64, notice: Notice) {
            let gone = self
                .sharers
                .get_mut(&id)
                .is_some_and(|sharer| sharing::write_notice(&mut sharer.writer, &notice).is_err());
            if gone {
                self.sharers.remove(&id);
            }
        }

        pub fn tell_all(&mut self, notice: Notice) {
            let ids: Vec<u64> = self.sharers.keys().copied().collect();
            for id in ids {
                self.tell(id, notice);
            }
        }

        /// Every program goes: their connections close.
        pub fn close_all(&mut self) {
            for sharer in self.sharers.values() {
                let _ = sharer.writer.shutdown(std::net::Shutdown::Both);
            }
            self.sharers.clear();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use atdecc::MacAddress;

        fn talker(stream_id: u64) -> TalkerDeclaration {
            TalkerDeclaration {
                stream_id,
                destination: MacAddress([0x91, 0xe0, 0xf0, 0x00, 0x63, 0x26]),
                vlan_id: 2,
                max_frame_size: 224,
                max_interval_frames: 1,
                priority: 3,
                rank: true,
                accumulated_latency: 500_000,
                failure: None,
                event: avb_mrp::Event::New,
            }
        }

        #[test]
        fn sharers_keep_their_streams() {
            let (ours, mut theirs) = UnixStream::pair().unwrap();
            let mut sharers = Sharers::default();
            sharers.open(1, ours);
            sharers.declared(1, talker(0x77));
            assert_eq!(sharers.owner(0x77), Some(1));
            assert_eq!(sharers.owner(0x78), None);
            assert_eq!(sharers.declaration(0x77), Some(talker(0x77)));
            assert_eq!(sharers.declaration(0x78), None);
            sharers.tell(
                1,
                Notice::Listener {
                    stream_id: 0x77,
                    state: Some(ListenerState::Ready),
                },
            );
            assert_eq!(
                sharing::read_notice(&mut theirs).unwrap(),
                Notice::Listener {
                    stream_id: 0x77,
                    state: Some(ListenerState::Ready),
                }
            );
            assert!(!sharers.withdrawn(1, 0x78));
            assert_eq!(sharers.forget_streams(), [0x77]);
            assert!(sharers.streams().is_empty());
            sharers.declared(1, talker(0x79));
            assert_eq!(sharers.close(1), [0x79]);
        }

        #[test]
        fn a_sharer_hears_of_its_listeners_once_each_change() {
            let (ours, mut theirs) = UnixStream::pair().unwrap();
            let mut sharers = Sharers::default();
            sharers.open(1, ours);
            sharers.declared(1, talker(0x77));
            // None is what a new stream starts from, so nothing goes.
            sharers.tell_listener(0x77, None);
            sharers.tell_listener(0x77, Some(ListenerState::Ready));
            sharers.tell_listener(0x77, Some(ListenerState::Ready));
            sharers.tell_listener(0x77, None);
            // Nobody declared this one.
            sharers.tell_listener(0x78, Some(ListenerState::Ready));
            drop(sharers);
            let mut told = Vec::new();
            while let Ok(notice) = sharing::read_notice(&mut theirs) {
                told.push(notice);
            }
            assert_eq!(
                told,
                [
                    Notice::Listener {
                        stream_id: 0x77,
                        state: Some(ListenerState::Ready),
                    },
                    Notice::Listener {
                        stream_id: 0x77,
                        state: None,
                    },
                ]
            );
        }
    }
}

/// Where Unix sockets are not: no program shares the participant.
#[cfg(not(unix))]
mod elsewhere {
    use std::sync::mpsc::Sender;

    use avb_mrp::msrp::{ListenerState, TalkerDeclaration};
    use avb_mrp::sharing::{Notice, Request};

    /// No program to tell.
    pub enum Writer {}

    pub struct Server;

    impl Server {
        pub fn start<T: Send + 'static>(
            _interface: &str,
            _sender: Sender<T>,
            _opened: fn(u64, Writer) -> T,
            _asked: fn(u64, Request) -> T,
            _closed: fn(u64) -> T,
        ) -> Option<Self> {
            None
        }
    }

    #[derive(Default)]
    pub struct Sharers {
        _none: (),
    }

    impl Sharers {
        pub fn open(&mut self, _id: u64, writer: Writer) {
            match writer {}
        }
        pub fn declared(&mut self, _id: u64, _declaration: TalkerDeclaration) {}
        pub fn withdrawn(&mut self, _id: u64, _stream_id: u64) -> bool {
            false
        }
        pub fn close(&mut self, _id: u64) -> Vec<u64> {
            Vec::new()
        }
        pub fn forget_streams(&mut self) -> Vec<u64> {
            Vec::new()
        }
        pub fn streams(&self) -> Vec<u64> {
            Vec::new()
        }
        pub fn owner(&self, _stream_id: u64) -> Option<u64> {
            None
        }
        pub fn declaration(&self, _stream_id: u64) -> Option<TalkerDeclaration> {
            None
        }
        pub fn tell_listener(&mut self, _stream_id: u64, _state: Option<ListenerState>) {}
        pub fn tell(&mut self, _id: u64, _notice: Notice) {}
        pub fn tell_all(&mut self, _notice: Notice) {}
        pub fn close_all(&mut self) {}
    }
}
