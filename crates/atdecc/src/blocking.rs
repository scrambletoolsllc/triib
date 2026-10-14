//! A blocking driver for apps without a loop of their own: one raw socket
//! on one interface, feeding a [`Controller`] on the calling thread.

use std::collections::VecDeque;
use std::io;
use std::time::{Duration, SystemTime};

use avb_net::{MacAddress, Socket};

use crate::controller::{Config, Controller};
use crate::id::EntityId;
use crate::time::Instant;

#[cfg(target_os = "macos")]
mod apple;

/// Room for any Ethernet payload.
const FRAME: usize = 1536;

/// The longest wait for a frame while this Mac's own entities owe
/// answers, which come through the framework, not the socket.
#[cfg(target_os = "macos")]
const LOCAL_WAIT: Duration = Duration::from_millis(2);

/// Whether macOS runs AVB on `interface`, giving the Mac an AVB entity of
/// its own; `None` off macOS, or where its AudioVideoBridging framework
/// is missing.
pub fn avb_enabled(interface: &str) -> Option<bool> {
    #[cfg(target_os = "macos")]
    return apple::avb_enabled(interface);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = interface;
        None
    }
}

pub struct Driver {
    socket: Socket,
    controller: Controller,
    epoch: std::time::Instant,
    received: [u8; FRAME],
    sending: [u8; FRAME],
    /// The ATDECC frames sent and received, when kept, and how many at
    /// most.
    frames: VecDeque<Frame>,
    keep: usize,
    /// This Mac's own entities, reached through the AudioVideoBridging
    /// framework.
    #[cfg(target_os = "macos")]
    local: Option<apple::Local>,
}

/// An ATDECC frame the driver sent or received, kept when asked to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub at: SystemTime,
    /// The driver sent it; else it was received.
    pub sent: bool,
    /// Where it went, or where it came from.
    pub peer: MacAddress,
    /// The octets after the Ethernet header.
    pub bytes: Vec<u8>,
}

impl Driver {
    /// Opens `interface` for ATDECC, with a controller whose entity ID is
    /// made from the interface's MAC address and the default
    /// [`Config`].
    pub fn open(interface: &str) -> io::Result<Self> {
        Self::open_with(interface, |_| {})
    }

    /// Opens `interface`, letting `configure` adjust the controller's
    /// configuration first.
    pub fn open_with(interface: &str, configure: impl FnOnce(&mut Config)) -> io::Result<Self> {
        let socket = Socket::open(interface, crate::ETHERTYPE_AVTP)?;
        socket.join_multicast(crate::ADP_ACMP_MULTICAST)?;
        // Streams share the ethertype; only ATDECC's subtypes are wanted.
        let _ =
            socket.keep_payloads_starting(crate::avtp::subtype::ADP..=crate::avtp::subtype::ACMP);
        let mut config = Config::new(EntityId::from_mac(socket.mac()));
        // A new start for sequence IDs each run, so a restarted controller
        // is not taken for a retry of its earlier commands.
        let clock = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |since| since.as_nanos() as u64);
        config.first_sequence_id = clock as u16;
        config.random_seed = config.entity_id.0 ^ clock;
        configure(&mut config);
        // The system's own AVB entity never sees what is written to BPF,
        // only what comes through the framework; without it, the entity
        // is out of reach.
        #[cfg(target_os = "macos")]
        let local = apple::Local::open(interface, config.entity_id);
        #[cfg(target_os = "macos")]
        if local.is_none() {
            config.own_mac = Some(socket.mac());
        }
        let controller = Controller::new(config);
        Ok(Self {
            socket,
            controller,
            epoch: std::time::Instant::now(),
            received: [0; FRAME],
            sending: [0; FRAME],
            frames: VecDeque::new(),
            keep: 0,
            #[cfg(target_os = "macos")]
            local,
        })
    }

    pub fn controller(&self) -> &Controller {
        &self.controller
    }

    pub fn controller_mut(&mut self) -> &mut Controller {
        &mut self.controller
    }

    /// Keeps the ATDECC frames sent and received from here on, up to
    /// `limit` of them waiting at once, the oldest dropped; zero keeps none.
    pub fn keep_frames(&mut self, limit: usize) {
        self.keep = limit;
        while self.frames.len() > limit {
            self.frames.pop_front();
        }
    }

    /// The oldest frame kept and not yet taken.
    pub fn poll_frame(&mut self) -> Option<Frame> {
        self.frames.pop_front()
    }

    /// Whether frames like `bytes` are kept: ATDECC ones, while asked to.
    fn keeps(&self, bytes: &[u8]) -> bool {
        self.keep > 0 && crate::pdu::is_atdecc(bytes)
    }

    fn kept(&mut self, sent: bool, peer: MacAddress, bytes: Vec<u8>) {
        if self.frames.len() == self.keep {
            self.frames.pop_front();
        }
        self.frames.push_back(Frame {
            at: SystemTime::now(),
            sent,
            peer,
            bytes,
        });
    }

    /// The controller's clock: the time since the driver opened.
    pub fn now(&self) -> Instant {
        Instant::from_nanos(self.epoch.elapsed().as_nanos() as u64)
    }

    /// Sends what the controller queued, waits up to `max_wait` (less if a
    /// deadline comes first) for a frame, hands it over, runs due
    /// timeouts, and sends again. Events are then ready on the controller.
    pub fn turn(&mut self, max_wait: Duration) -> io::Result<()> {
        self.flush()?;
        self.take_local();
        let now = self.now();
        #[cfg_attr(not(target_os = "macos"), allow(unused_mut))]
        let mut wait = match self.controller.poll_timeout() {
            Some(deadline) => deadline.saturating_duration_since(now).min(max_wait),
            None => max_wait,
        };
        #[cfg(target_os = "macos")]
        if self.local.as_ref().is_some_and(apple::Local::waiting) {
            wait = wait.min(LOCAL_WAIT);
        }
        if let Some(received) = self.socket.receive(&mut self.received, Some(wait))? {
            let now = self.now();
            let bytes = &self.received[..received.length];
            // A frame that does not decode is counted by the controller.
            let _ = self.controller.handle_frame(now, received.source, bytes);
            if self.keeps(&self.received[..received.length]) {
                let bytes = self.received[..received.length].to_vec();
                self.kept(false, received.source, bytes);
            }
        }
        self.take_local();
        let now = self.now();
        self.controller.handle_timeout(now);
        self.flush()
    }

    /// Hands the controller what this Mac's own entities answered through
    /// the framework, as frames from them.
    fn take_local(&mut self) {
        #[cfg(target_os = "macos")]
        while let Some(bytes) = self.local.as_ref().and_then(apple::Local::poll) {
            let now = self.now();
            let source = self.socket.mac();
            let _ = self.controller.handle_frame(now, source, &bytes);
            if self.keeps(&bytes) {
                self.kept(false, source, bytes);
            }
        }
    }

    /// Sends a command to one of this Mac's own entities through the
    /// framework, returning whether it did.
    #[cfg(target_os = "macos")]
    fn send_local(&self, bytes: &[u8]) -> bool {
        let Some(local) = &self.local else {
            return false;
        };
        let own = self.socket.mac();
        local.send(bytes, |entity_id| {
            self.controller
                .entity(entity_id)
                .is_some_and(|entity| entity.mac == own)
        })
    }

    /// Advertises that the controller is leaving, if it advertised, and
    /// sends what is still queued.
    pub fn close(mut self) -> io::Result<()> {
        self.controller.depart();
        self.flush()
    }

    fn flush(&mut self) -> io::Result<()> {
        while let Some(transmit) = self
            .controller
            .poll_transmit(&mut self.sending)
            .map_err(io::Error::other)?
        {
            let bytes = &self.sending[..transmit.length];
            #[cfg(target_os = "macos")]
            let local = self.send_local(bytes);
            #[cfg(not(target_os = "macos"))]
            let local = false;
            if !local {
                self.socket.send(transmit.destination, bytes)?;
            }
            if self.keeps(&self.sending[..transmit.length]) {
                let bytes = self.sending[..transmit.length].to_vec();
                self.kept(true, transmit.destination, bytes);
            }
        }
        Ok(())
    }
}
