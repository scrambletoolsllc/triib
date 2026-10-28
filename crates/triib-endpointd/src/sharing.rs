//! The port's MRP participant, shared with other programs on this
//! computer (`avb_mrp::sharing`), such as plaa sending what it plays: two
//! participants on one port would withdraw each other's declarations, so
//! the daemon declares their talkers in its own, and listeners for the
//! streams they listen to, tells them as the listeners of their streams
//! and the talkers of the others come and go, and withdraws a program's
//! declarations when it goes. Programs that run AVB Lite with it it
//! declares there too, by CVU SRP in their entities' names.

use avb_mrp::msrp::TalkerDeclaration;

use crate::status::SharerStatus;

#[cfg(unix)]
pub use unix::*;

#[cfg(not(unix))]
pub use elsewhere::*;

/// What a program that went had declared and listened to.
pub struct Closed {
    pub entity: Option<u64>,
    pub streams: Vec<TalkerDeclaration>,
    pub listens: Vec<u64>,
}

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

    use atdecc::MacAddress;
    use avb_mrp::msrp::{ListenerState, TalkerDeclaration};
    use avb_mrp::sharing::{self, Hello, Mode, Notice, Request, feature};

    use super::{Closed, SharerStatus};

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
            // Gone first, so that no program connects while it stops.
            let _ = std::fs::remove_file(&self.path);
            self.stop.store(true, Ordering::Relaxed);
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
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

    /// A program sharing the participant: where to tell it, what it said
    /// in its hello and the entity it named, the streams it declared and
    /// those it listens to, with the talker it was last told of for each,
    /// none before the first Talker notice, and the mode it was last told.
    struct Sharer {
        writer: UnixStream,
        hello: Option<Hello>,
        entity: Option<u64>,
        streams: HashMap<u64, Declared>,
        listens: HashMap<u64, Option<Option<TalkerDeclaration>>>,
        mode: Option<Mode>,
    }

    impl Sharer {
        /// Whether it runs AVB Lite with the participant: its hello says
        /// so and it named its entity, which AVB Lite's declarations name.
        fn lite(&self) -> bool {
            self.entity.is_some() && self.hello.is_some_and(|hello| hello.has(feature::AVB_LITE))
        }

        /// Whether it takes its declarations over as the participant
        /// stops, its hello saying HANDOFF.
        fn hands_over(&self) -> bool {
            self.hello.is_some_and(|hello| hello.has(feature::HANDOFF))
        }
    }

    /// A stream a program declared: its Talker Advertise, the listeners it
    /// was last told of, and in AVB Lite the copies.
    struct Declared {
        declaration: TalkerDeclaration,
        told: Option<ListenerState>,
        copies: Option<(u16, Vec<MacAddress>)>,
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
                    hello: None,
                    entity: None,
                    streams: HashMap::new(),
                    listens: HashMap::new(),
                    mode: None,
                },
            );
        }

        /// Notes `id`'s hello.
        pub fn said_hello(&mut self, id: u64, hello: Hello) {
            if let Some(sharer) = self.sharers.get_mut(&id) {
                sharer.hello = Some(hello);
            }
        }

        /// Notes the entity `id` declares and listens for.
        pub fn named(&mut self, id: u64, entity: u64) {
            if let Some(sharer) = self.sharers.get_mut(&id) {
                sharer.entity = Some(entity);
            }
        }

        /// Whether `id` runs AVB Lite with the participant.
        pub fn lite(&self, id: u64) -> bool {
            self.sharers.get(&id).is_some_and(Sharer::lite)
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
                        copies: None,
                    },
                );
            }
        }

        /// Notes that `id` withdrew `stream_id`: what it had declared.
        pub fn withdrawn(&mut self, id: u64, stream_id: u64) -> Option<TalkerDeclaration> {
            self.sharers
                .get_mut(&id)
                .and_then(|sharer| sharer.streams.remove(&stream_id))
                .map(|declared| declared.declaration)
        }

        /// Notes that `id` listens to `stream_id`: whether it did not
        /// already.
        pub fn listen(&mut self, id: u64, stream_id: u64) -> bool {
            let Some(sharer) = self.sharers.get_mut(&id) else {
                return false;
            };
            if sharer.listens.contains_key(&stream_id) {
                return false;
            }
            sharer.listens.insert(stream_id, None);
            true
        }

        /// Notes that `id` stopped listening to `stream_id`: whether it
        /// did.
        pub fn unlisten(&mut self, id: u64, stream_id: u64) -> bool {
            self.sharers
                .get_mut(&id)
                .is_some_and(|sharer| sharer.listens.remove(&stream_id).is_some())
        }

        /// `id` went: what it had declared and listened to.
        pub fn close(&mut self, id: u64) -> Closed {
            let Some(sharer) = self.sharers.remove(&id) else {
                return Closed {
                    entity: None,
                    streams: Vec::new(),
                    listens: Vec::new(),
                };
            };
            Closed {
                entity: sharer.entity,
                streams: sharer
                    .streams
                    .into_values()
                    .map(|declared| declared.declaration)
                    .collect(),
                listens: sharer.listens.into_keys().collect(),
            }
        }

        /// The streams that the programs not running AVB Lite with the
        /// participant declared and listened to, which no longer are, as
        /// the port moves to AVB Lite: they declare and listen again when
        /// told they can.
        pub fn forget_unable(&mut self) -> (Vec<u64>, Vec<u64>) {
            let mut streams = Vec::new();
            let mut listens = Vec::new();
            for sharer in self.sharers.values_mut().filter(|sharer| !sharer.lite()) {
                streams.extend(sharer.streams.drain().map(|(stream_id, _)| stream_id));
                listens.extend(sharer.listens.drain().map(|(stream_id, _)| stream_id));
            }
            (streams, listens)
        }

        /// The streams that programs which cannot take them over declared,
        /// which go as the participant stops; those of the others stay
        /// registered for them to declare again.
        pub fn stranded(&self) -> Vec<u64> {
            self.sharers
                .values()
                .filter(|sharer| !sharer.hands_over())
                .flat_map(|sharer| sharer.streams.keys().copied())
                .collect()
        }

        /// How many programs take their declarations over as the
        /// participant stops.
        pub fn handing_over(&self) -> usize {
            self.sharers
                .values()
                .filter(|sharer| sharer.hands_over())
                .count()
        }

        /// What each program declares and listens to, for the status.
        pub fn statuses(&self) -> Vec<SharerStatus> {
            let mut statuses: Vec<SharerStatus> = self
                .sharers
                .values()
                .map(|sharer| SharerStatus {
                    talkers: sharer.streams.len(),
                    listening: sharer.listens.len(),
                    hands_over: sharer.hands_over(),
                })
                .collect();
            statuses.sort_by_key(|status| (status.talkers, status.listening, status.hands_over));
            statuses
        }

        /// Every stream the programs declared.
        pub fn streams(&self) -> Vec<u64> {
            self.sharers
                .values()
                .flat_map(|sharer| sharer.streams.keys().copied())
                .collect()
        }

        /// Every stream the programs listen to, once each.
        pub fn listened(&self) -> Vec<u64> {
            let mut listened: Vec<u64> = self
                .sharers
                .values()
                .flat_map(|sharer| sharer.listens.keys().copied())
                .collect();
            listened.sort_unstable();
            listened.dedup();
            listened
        }

        /// How many programs listen to `stream_id`.
        pub fn listening(&self, stream_id: u64) -> usize {
            self.sharers
                .values()
                .filter(|sharer| sharer.listens.contains_key(&stream_id))
                .count()
        }

        /// The entity of a program running AVB Lite with the participant
        /// that listens to `stream_id`, in whose name to declare.
        pub fn listening_entity(&self, stream_id: u64) -> Option<u64> {
            let mut ids: Vec<&u64> = self.sharers.keys().collect();
            ids.sort_unstable();
            ids.into_iter()
                .filter_map(|id| self.sharers.get(id))
                .filter(|sharer| sharer.lite() && sharer.listens.contains_key(&stream_id))
                .find_map(|sharer| sharer.entity)
        }

        /// The program that declared `stream_id`.
        pub fn owner(&self, stream_id: u64) -> Option<u64> {
            self.sharers
                .iter()
                .find(|(_, sharer)| sharer.streams.contains_key(&stream_id))
                .map(|(id, _)| *id)
        }

        /// The entity of the program that declared `stream_id`.
        pub fn owner_entity(&self, stream_id: u64) -> Option<u64> {
            self.sharers
                .values()
                .find(|sharer| sharer.streams.contains_key(&stream_id))
                .and_then(|sharer| sharer.entity)
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

        /// Tells the program that declared `stream_id` where to send its
        /// copies in AVB Lite, and with which tag, unless that is what it
        /// was told last.
        pub fn tell_copies(&mut self, stream_id: u64, tag: u16, destinations: Vec<MacAddress>) {
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
            let copies = (tag, destinations);
            if declared.copies.as_ref() == Some(&copies) {
                return;
            }
            declared.copies = Some(copies.clone());
            let (tag, destinations) = copies;
            self.tell(
                owner,
                Notice::Copies {
                    stream_id,
                    tag,
                    destinations,
                },
            );
        }

        /// Tells each program listening to `stream_id` its `talker`, unless
        /// that is what it was told last; the first time, whatever it is.
        pub fn tell_talker(&mut self, stream_id: u64, talker: Option<TalkerDeclaration>) {
            let mut ids: Vec<u64> = Vec::new();
            for (id, sharer) in &mut self.sharers {
                if let Some(told) = sharer.listens.get_mut(&stream_id)
                    && *told != Some(talker)
                {
                    *told = Some(talker);
                    ids.push(*id);
                }
            }
            for id in ids {
                self.tell(id, Notice::Talker { stream_id, talker });
            }
        }

        /// Tells each program running AVB Lite with the participant the
        /// port's `mode`, unless that is what it was told last. Back to
        /// AVB, their copies end.
        pub fn tell_mode(&mut self, mode: Mode) {
            let mut ids: Vec<u64> = Vec::new();
            for (id, sharer) in &mut self.sharers {
                if !sharer.lite() || sharer.mode == Some(mode) {
                    continue;
                }
                sharer.mode = Some(mode);
                if mode.lite.is_none() {
                    for declared in sharer.streams.values_mut() {
                        declared.copies = None;
                    }
                }
                ids.push(*id);
            }
            for id in ids {
                self.tell(id, Notice::Mode(mode));
            }
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

        /// Tells the programs not running AVB Lite with the participant
        /// `notice`, as that their streams cannot be declared there.
        pub fn tell_unable(&mut self, notice: &Notice) {
            let ids: Vec<u64> = self
                .sharers
                .iter()
                .filter(|(_, sharer)| !sharer.lite())
                .map(|(id, _)| *id)
                .collect();
            for id in ids {
                self.tell(id, notice.clone());
            }
        }

        /// Tells every program `notice`.
        pub fn tell_all(&mut self, notice: &Notice) {
            let ids: Vec<u64> = self.sharers.keys().copied().collect();
            for id in ids {
                self.tell(id, notice.clone());
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

        /// Everything `theirs` was told until the participant's end
        /// closed.
        fn told(sharers: Sharers, theirs: &mut UnixStream) -> Vec<Notice> {
            drop(sharers);
            let mut told = Vec::new();
            while let Ok(notice) = sharing::read_notice(theirs) {
                told.push(notice);
            }
            told
        }

        /// As the participant stops, every program hears it leaves, and
        /// only the streams of those that cannot take them over go.
        #[test]
        fn a_program_saying_handoff_keeps_its_streams_as_the_participant_stops() {
            let (ours, mut new) = UnixStream::pair().unwrap();
            let (ours_old, mut old) = UnixStream::pair().unwrap();
            let mut sharers = Sharers::default();
            sharers.open(1, ours);
            sharers.open(2, ours_old);
            sharers.said_hello(
                1,
                Hello {
                    version: sharing::VERSION,
                    features: feature::LISTENERS | feature::HANDOFF,
                },
            );
            sharers.said_hello(
                2,
                Hello {
                    version: sharing::VERSION,
                    features: feature::LISTENERS,
                },
            );
            sharers.declared(1, talker(0x77));
            sharers.listen(1, 0x80);
            sharers.declared(2, talker(0x78));
            sharers.listen(2, 0x81);
            assert_eq!(sharers.handing_over(), 1);
            assert_eq!(sharers.stranded(), [0x78]);
            let statuses = sharers.statuses();
            assert_eq!(statuses.len(), 2);
            assert_eq!(statuses.iter().map(SharerStatus::cut_off).sum::<usize>(), 2);
            assert!(statuses.iter().any(|status| status.hands_over));
            sharers.tell_all(&Notice::Leaving);
            sharers.close_all();
            assert_eq!(told(sharers, &mut new), [Notice::Leaving]);
            // Kind 16 is new to it, but it reads it as any other.
            assert_eq!(sharing::read_notice(&mut old).unwrap(), Notice::Leaving);
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
            assert_eq!(sharers.withdrawn(1, 0x78), None);
            assert!(sharers.listen(1, 0x80));
            assert_eq!(sharers.forget_unable(), (vec![0x77], vec![0x80]));
            assert!(sharers.streams().is_empty());
            assert!(sharers.listened().is_empty());
            sharers.declared(1, talker(0x79));
            sharers.listen(1, 0x81);
            let closed = sharers.close(1);
            assert_eq!(closed.streams, [talker(0x79)]);
            assert_eq!(closed.listens, [0x81]);
        }

        /// A program listening hears of the talker at once, whatever it
        /// is, then once each change; asking twice is asking once.
        #[test]
        fn a_listener_hears_of_its_talker_once_each_change() {
            let (ours, mut theirs) = UnixStream::pair().unwrap();
            let mut sharers = Sharers::default();
            sharers.open(1, ours);
            assert!(sharers.listen(1, 0x77));
            assert!(!sharers.listen(1, 0x77));
            assert_eq!(sharers.listening(0x77), 1);
            assert_eq!(sharers.listened(), [0x77]);
            sharers.tell_talker(0x77, None);
            sharers.tell_talker(0x77, None);
            sharers.tell_talker(0x77, Some(talker(0x77)));
            sharers.tell_talker(0x77, Some(talker(0x77)));
            // Nobody listens to this one.
            sharers.tell_talker(0x78, Some(talker(0x78)));
            assert!(sharers.unlisten(1, 0x77));
            assert!(!sharers.unlisten(1, 0x77));
            sharers.tell_talker(0x77, None);
            assert_eq!(
                told(sharers, &mut theirs),
                [
                    Notice::Talker {
                        stream_id: 0x77,
                        talker: None,
                    },
                    Notice::Talker {
                        stream_id: 0x77,
                        talker: Some(talker(0x77)),
                    },
                ]
            );
        }

        /// A program runs AVB Lite with the participant once its hello
        /// says so and it named its entity: it keeps its streams as the
        /// port moves there, hears the mode, and where its copies go.
        #[test]
        fn a_program_naming_its_entity_runs_avb_lite() {
            let (ours, mut theirs) = UnixStream::pair().unwrap();
            let (other, _) = UnixStream::pair().unwrap();
            let mut sharers = Sharers::default();
            sharers.open(1, ours);
            sharers.open(2, other);
            sharers.said_hello(
                1,
                Hello {
                    version: sharing::VERSION,
                    features: feature::AVB_LITE,
                },
            );
            assert!(!sharers.lite(1));
            sharers.named(1, 0xa010_a3ff_fea5_1726);
            assert!(sharers.lite(1));
            sharers.declared(1, talker(0x77));
            sharers.listen(1, 0x80);
            sharers.declared(2, talker(0x78));
            assert_eq!(sharers.owner_entity(0x77), Some(0xa010_a3ff_fea5_1726));
            assert_eq!(sharers.listening_entity(0x80), Some(0xa010_a3ff_fea5_1726));
            assert_eq!(sharers.forget_unable(), (vec![0x78], vec![]));
            assert_eq!(sharers.streams(), [0x77]);
            let lite = Mode {
                lite: Some(2),
                media_vlan: 2,
                unicast_fanout: 2,
                escalation: false,
            };
            sharers.tell_mode(lite);
            sharers.tell_mode(lite);
            let peer = MacAddress([0xe8, 0xf6, 0x0a, 0xe0, 0x92, 0x20]);
            sharers.tell_copies(0x77, 0xa002, vec![peer]);
            sharers.tell_copies(0x77, 0xa002, vec![peer]);
            let avb = Mode { lite: None, ..lite };
            sharers.tell_mode(avb);
            // Back in AVB Lite, its copies are told afresh.
            sharers.tell_mode(lite);
            sharers.tell_copies(0x77, 0xa002, vec![peer]);
            let copies = Notice::Copies {
                stream_id: 0x77,
                tag: 0xa002,
                destinations: vec![peer],
            };
            assert_eq!(
                told(sharers, &mut theirs),
                [
                    Notice::Mode(lite),
                    copies.clone(),
                    Notice::Mode(avb),
                    Notice::Mode(lite),
                    copies,
                ]
            );
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
            assert_eq!(
                told(sharers, &mut theirs),
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

    use atdecc::MacAddress;
    use avb_mrp::msrp::{ListenerState, TalkerDeclaration};
    use avb_mrp::sharing::{Hello, Mode, Notice, Request};

    use super::{Closed, SharerStatus};

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
        pub fn said_hello(&mut self, _id: u64, _hello: Hello) {}
        pub fn named(&mut self, _id: u64, _entity: u64) {}
        pub fn lite(&self, _id: u64) -> bool {
            false
        }
        pub fn declared(&mut self, _id: u64, _declaration: TalkerDeclaration) {}
        pub fn withdrawn(&mut self, _id: u64, _stream_id: u64) -> Option<TalkerDeclaration> {
            None
        }
        pub fn listen(&mut self, _id: u64, _stream_id: u64) -> bool {
            false
        }
        pub fn unlisten(&mut self, _id: u64, _stream_id: u64) -> bool {
            false
        }
        pub fn close(&mut self, _id: u64) -> Closed {
            Closed {
                entity: None,
                streams: Vec::new(),
                listens: Vec::new(),
            }
        }
        pub fn forget_unable(&mut self) -> (Vec<u64>, Vec<u64>) {
            (Vec::new(), Vec::new())
        }
        pub fn stranded(&self) -> Vec<u64> {
            Vec::new()
        }
        pub fn handing_over(&self) -> usize {
            0
        }
        pub fn statuses(&self) -> Vec<SharerStatus> {
            Vec::new()
        }
        pub fn streams(&self) -> Vec<u64> {
            Vec::new()
        }
        pub fn listened(&self) -> Vec<u64> {
            Vec::new()
        }
        pub fn listening(&self, _stream_id: u64) -> usize {
            0
        }
        pub fn listening_entity(&self, _stream_id: u64) -> Option<u64> {
            None
        }
        pub fn owner(&self, _stream_id: u64) -> Option<u64> {
            None
        }
        pub fn owner_entity(&self, _stream_id: u64) -> Option<u64> {
            None
        }
        pub fn declaration(&self, _stream_id: u64) -> Option<TalkerDeclaration> {
            None
        }
        pub fn tell_listener(&mut self, _stream_id: u64, _state: Option<ListenerState>) {}
        pub fn tell_copies(&mut self, _stream_id: u64, _tag: u16, _destinations: Vec<MacAddress>) {}
        pub fn tell_talker(&mut self, _stream_id: u64, _talker: Option<TalkerDeclaration>) {}
        pub fn tell_mode(&mut self, _mode: Mode) {}
        pub fn tell(&mut self, _id: u64, _notice: Notice) {}
        pub fn tell_unable(&mut self, _notice: &Notice) {}
        pub fn tell_all(&mut self, _notice: &Notice) {}
        pub fn close_all(&mut self) {}
    }
}
