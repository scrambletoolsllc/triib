//! Whole frames for media streams: sending frames a talker builds itself,
//! VLAN tag included, and receiving the frames of one AVTP stream whole,
//! tagged or with the tag already taken off by the interface. Linux only
//! so far; elsewhere opening fails as unsupported.

use std::io;
use std::time::Duration;

use crate::MacAddress;

/// The AVTP ethertype, and the VLAN tag's.
const ETHERTYPE_AVTP: u16 = 0x22f0;
const ETHERTYPE_VLAN: u16 = 0x8100;

/// Sends whole Ethernet frames on one interface.
pub struct FrameSender {
    inner: platform::FrameSender,
}

impl FrameSender {
    /// Opens `interface`. On Linux this needs `CAP_NET_RAW`.
    pub fn open(interface: &str) -> io::Result<Self> {
        Ok(Self {
            inner: platform::FrameSender::open(interface)?,
        })
    }

    /// Sends `frame`, from its destination address on.
    pub fn send(&self, frame: &[u8]) -> io::Result<()> {
        self.inner.send(frame)
    }
}

/// Receives the whole frames of one AVTP stream: frames of an AVTP subtype
/// sent to one destination, or either of two, as an AVB Lite listener
/// takes a stream unicast to it or at its multicast address.
pub struct FrameReceiver {
    inner: platform::FrameReceiver,
}

impl FrameReceiver {
    /// Opens `interface` for frames of AVTP `subtype` sent to any of
    /// `destinations` (one or two), and joins those that are group
    /// addresses.
    pub fn open(interface: &str, subtype: u8, destinations: &[MacAddress]) -> io::Result<Self> {
        if destinations.is_empty() || destinations.len() > 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "a stream comes to one or two addresses",
            ));
        }
        Ok(Self {
            inner: platform::FrameReceiver::open(interface, subtype, destinations)?,
        })
    }

    /// Waits up to `timeout` (forever when `None`) for a frame and writes
    /// it to `buffer` whole, returning its length.
    pub fn receive(
        &self,
        buffer: &mut [u8],
        timeout: Option<Duration>,
    ) -> io::Result<Option<usize>> {
        self.inner.receive(buffer, timeout)
    }

    /// Waits up to `timeout` (forever when `None`) for a frame at any of
    /// `receivers`, giving the places of those that have one: one thread
    /// taking several streams.
    pub fn ready(
        receivers: &[&FrameReceiver],
        timeout: Option<Duration>,
    ) -> io::Result<Vec<usize>> {
        let inner: Vec<&platform::FrameReceiver> =
            receivers.iter().map(|receiver| &receiver.inner).collect();
        platform::FrameReceiver::ready(&inner, timeout)
    }

    /// The frames the socket had no room for since last asked: its reader
    /// fell behind.
    pub fn dropped(&self) -> u64 {
        self.inner.dropped()
    }
}

/// Where an AVTP frame's payload starts: after the header, and the VLAN
/// tag when it still has one. `None` for another kind of frame.
pub fn avtp_payload(frame: &[u8]) -> Option<&[u8]> {
    let ethertype = |at: usize| Some(u16::from_be_bytes([*frame.get(at)?, *frame.get(at + 1)?]));
    match ethertype(12)? {
        ETHERTYPE_AVTP => frame.get(14..),
        ETHERTYPE_VLAN if ethertype(16)? == ETHERTYPE_AVTP => frame.get(18..),
        _ => None,
    }
}

#[cfg(target_os = "linux")]
#[allow(unsafe_code)]
mod platform {
    use std::io;
    use std::mem::{size_of, zeroed};
    use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
    use std::time::Duration;

    use super::{ETHERTYPE_AVTP, ETHERTYPE_VLAN};
    use crate::MacAddress;

    /// The receive buffer a stream's socket asks for: a quarter of a second
    /// of a class A stream of 1500-octet frames, as the kernel counts them.
    const RECEIVE_ROOM: libc::c_int = 1 << 20;

    fn open_raw(interface: &str) -> io::Result<(OwnedFd, i32)> {
        let index = crate::interfaces::index_of(interface).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("no interface named {interface}"),
            )
        })?;
        // SAFETY: plain system call with integer arguments.
        let raw = unsafe { libc::socket(libc::AF_PACKET, libc::SOCK_RAW | libc::SOCK_CLOEXEC, 0) };
        if raw < 0 {
            let error = io::Error::last_os_error();
            return Err(match error.raw_os_error() {
                Some(libc::EPERM | libc::EACCES) => crate::socket::raw_capability(),
                _ => error,
            });
        }
        // SAFETY: `raw` is a socket this function just opened and owns.
        let fd = unsafe { OwnedFd::from_raw_fd(raw) };
        Ok((fd, index))
    }

    fn bind(fd: &OwnedFd, index: i32, protocol: u16) -> io::Result<()> {
        // SAFETY: sockaddr_ll is plain data, valid when zeroed.
        let mut address: libc::sockaddr_ll = unsafe { zeroed() };
        address.sll_family = libc::AF_PACKET as u16;
        address.sll_protocol = protocol.to_be();
        address.sll_ifindex = index;
        // SAFETY: `address` is a valid sockaddr_ll that lives through the
        // call, and the length passed is its size.
        let bound = unsafe {
            libc::bind(
                fd.as_raw_fd(),
                (&raw const address).cast(),
                size_of::<libc::sockaddr_ll>() as libc::socklen_t,
            )
        };
        if bound < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    pub struct FrameSender {
        fd: OwnedFd,
    }

    impl FrameSender {
        pub fn open(interface: &str) -> io::Result<Self> {
            let (fd, index) = open_raw(interface)?;
            // Bound for no protocol: it sends, and hears nothing.
            bind(&fd, index, 0)?;
            Ok(Self { fd })
        }

        pub fn send(&self, frame: &[u8]) -> io::Result<()> {
            // SAFETY: `frame` is valid for its length through the call.
            let sent =
                unsafe { libc::send(self.fd.as_raw_fd(), frame.as_ptr().cast(), frame.len(), 0) };
            if sent < 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        }
    }

    pub struct FrameReceiver {
        fd: OwnedFd,
    }

    impl FrameReceiver {
        pub fn open(interface: &str, subtype: u8, destinations: &[MacAddress]) -> io::Result<Self> {
            let (fd, index) = open_raw(interface)?;
            // The filter goes on before the socket hears anything: the
            // destination, either of two, then the AVTP subtype, after a
            // VLAN tag or not.
            let step = |code: u16, jt: u8, jf: u8, k: u32| libc::sock_filter { code, jt, jf, k };
            let halves = |mac: MacAddress| {
                let mac = mac.0;
                (
                    u32::from_be_bytes([mac[0], mac[1], mac[2], mac[3]]),
                    u32::from(u16::from_be_bytes([mac[4], mac[5]])),
                )
            };
            let (high, low) = halves(destinations[0]);
            // With one destination the second test repeats the first.
            let (other_high, other_low) = halves(*destinations.get(1).unwrap_or(&destinations[0]));
            let program = [
                step(0x20, 0, 0, 0),                         // 0: ld [0]
                step(0x15, 0, 2, high),                      // 1: jeq #first[0..4], else 4
                step(0x28, 0, 0, 4),                         // 2: ldh [4]
                step(0x15, 4, 0, low),                       // 3: jeq #first[4..6], 8
                step(0x20, 0, 0, 0),                         // 4: ld [0]
                step(0x15, 0, 12, other_high),               // 5: jeq #second[0..4], else 18
                step(0x28, 0, 0, 4),                         // 6: ldh [4]
                step(0x15, 0, 10, other_low),                // 7: jeq #second[4..6], else 18
                step(0x28, 0, 0, 12),                        // 8: ldh [12]
                step(0x15, 0, 2, u32::from(ETHERTYPE_AVTP)), // 9: jeq #avtp
                step(0x30, 0, 0, 14),                        // 10: ldb [14]
                step(0x05, 0, 0, 4),                         // 11: ja 16
                step(0x15, 0, 5, u32::from(ETHERTYPE_VLAN)), // 12: jeq #vlan
                step(0x28, 0, 0, 16),                        // 13: ldh [16]
                step(0x15, 0, 3, u32::from(ETHERTYPE_AVTP)), // 14: jeq #avtp
                step(0x30, 0, 0, 18),                        // 15: ldb [18]
                step(0x15, 0, 1, u32::from(subtype)),        // 16: jeq #subtype
                step(0x06, 0, 0, u32::MAX),                  // 17: ret #-1
                step(0x06, 0, 0, 0),                         // 18: ret #0
            ];
            let filter = libc::sock_fprog {
                len: program.len() as u16,
                filter: program.as_ptr().cast_mut(),
            };
            // SAFETY: `filter` points at `program`, which both live through
            // the call; the kernel copies the program and does not write it.
            let attached = unsafe {
                libc::setsockopt(
                    fd.as_raw_fd(),
                    libc::SOL_SOCKET,
                    libc::SO_ATTACH_FILTER,
                    (&raw const filter).cast(),
                    size_of::<libc::sock_fprog>() as libc::socklen_t,
                )
            };
            if attached < 0 {
                return Err(io::Error::last_os_error());
            }
            bind(&fd, index, libc::ETH_P_ALL as u16)?;
            // A membership of the socket's, which ends with it: a multicast
            // address the interface takes, or the interface promiscuous.
            let join = |kind: i32, mac: Option<[u8; 6]>| {
                // SAFETY: packet_mreq is plain data, valid when zeroed.
                let mut request: libc::packet_mreq = unsafe { zeroed() };
                request.mr_ifindex = index;
                request.mr_type = kind as u16;
                if let Some(mac) = mac {
                    request.mr_alen = 6;
                    request.mr_address[..6].copy_from_slice(&mac);
                }
                // SAFETY: `request` is a valid packet_mreq that lives
                // through the call, and the length passed is its size.
                let joined = unsafe {
                    libc::setsockopt(
                        fd.as_raw_fd(),
                        libc::SOL_PACKET,
                        libc::PACKET_ADD_MEMBERSHIP,
                        (&raw const request).cast(),
                        size_of::<libc::packet_mreq>() as libc::socklen_t,
                    )
                };
                if joined < 0 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            };
            for destination in destinations {
                let mac = destination.0;
                if mac[0] & 1 == 0 {
                    continue;
                }
                join(libc::PACKET_MR_MULTICAST, Some(mac))?;
            }
            // Streams come tagged with their VLAN, which an interface that
            // filters VLANs in hardware, as the I210 does, drops unless the
            // VLAN is registered with it, as only an administrator can.
            // Promiscuous, it takes them all.
            if crate::interfaces::filters_vlans(interface).unwrap_or(false) {
                join(libc::PACKET_MR_PROMISC, None)?;
            }
            // Room for a reader held up a while: as much as the system lets
            // a socket ask for, up to RECEIVE_ROOM, rather than the default
            // of a few tens of milliseconds of a stream.
            let room = RECEIVE_ROOM;
            // SAFETY: `room` is a c_int that lives through the call, and the
            // length passed is its size.
            unsafe {
                libc::setsockopt(
                    fd.as_raw_fd(),
                    libc::SOL_SOCKET,
                    libc::SO_RCVBUF,
                    (&raw const room).cast(),
                    size_of::<libc::c_int>() as libc::socklen_t,
                );
            }
            Ok(Self { fd })
        }

        pub fn dropped(&self) -> u64 {
            // SAFETY: tpacket_stats is plain data, valid when zeroed.
            let mut stats: libc::tpacket_stats = unsafe { zeroed() };
            let mut length = size_of::<libc::tpacket_stats>() as libc::socklen_t;
            // SAFETY: `stats` and `length` live through the call, and the
            // length says how much of `stats` the kernel may write; reading
            // the counts resets them.
            let read = unsafe {
                libc::getsockopt(
                    self.fd.as_raw_fd(),
                    libc::SOL_PACKET,
                    libc::PACKET_STATISTICS,
                    (&raw mut stats).cast(),
                    &raw mut length,
                )
            };
            if read < 0 {
                0
            } else {
                u64::from(stats.tp_drops)
            }
        }

        pub fn receive(
            &self,
            buffer: &mut [u8],
            timeout: Option<Duration>,
        ) -> io::Result<Option<usize>> {
            let milliseconds = match timeout {
                Some(timeout) => {
                    timeout.as_nanos().div_ceil(1_000_000).min(i32::MAX as u128) as i32
                }
                None => -1,
            };
            let mut poll = libc::pollfd {
                fd: self.fd.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            };
            // SAFETY: one valid pollfd, which lives through the call.
            let ready = unsafe { libc::poll(&raw mut poll, 1, milliseconds) };
            if ready <= 0 {
                let error = io::Error::last_os_error();
                return match error.kind() {
                    _ if ready == 0 => Ok(None),
                    io::ErrorKind::Interrupted => Ok(None),
                    _ => Err(error),
                };
            }
            // SAFETY: `buffer` is valid for its length through the call.
            let received = unsafe {
                libc::recv(
                    self.fd.as_raw_fd(),
                    buffer.as_mut_ptr().cast(),
                    buffer.len(),
                    libc::MSG_DONTWAIT,
                )
            };
            if received < 0 {
                let error = io::Error::last_os_error();
                return match error.kind() {
                    io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted => Ok(None),
                    _ => Err(error),
                };
            }
            Ok(Some(received as usize))
        }

        pub fn ready(receivers: &[&Self], timeout: Option<Duration>) -> io::Result<Vec<usize>> {
            let milliseconds = match timeout {
                Some(timeout) => {
                    timeout.as_nanos().div_ceil(1_000_000).min(i32::MAX as u128) as i32
                }
                None => -1,
            };
            let mut polls: Vec<libc::pollfd> = receivers
                .iter()
                .map(|receiver| libc::pollfd {
                    fd: receiver.fd.as_raw_fd(),
                    events: libc::POLLIN,
                    revents: 0,
                })
                .collect();
            // SAFETY: `polls` holds valid pollfds and lives through the call.
            let ready = unsafe {
                libc::poll(
                    polls.as_mut_ptr(),
                    polls.len() as libc::nfds_t,
                    milliseconds,
                )
            };
            if ready < 0 {
                let error = io::Error::last_os_error();
                return match error.kind() {
                    io::ErrorKind::Interrupted => Ok(Vec::new()),
                    _ => Err(error),
                };
            }
            Ok(polls
                .iter()
                .enumerate()
                .filter(|(_, poll)| poll.revents & libc::POLLIN != 0)
                .map(|(place, _)| place)
                .collect())
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod platform {
    use std::io;
    use std::time::Duration;

    use crate::MacAddress;

    fn unsupported() -> io::Error {
        io::Error::new(
            io::ErrorKind::Unsupported,
            "media streams are not supported on this system yet",
        )
    }

    pub struct FrameSender;

    impl FrameSender {
        pub fn open(_interface: &str) -> io::Result<Self> {
            Err(unsupported())
        }

        pub fn send(&self, _frame: &[u8]) -> io::Result<()> {
            Err(unsupported())
        }
    }

    pub struct FrameReceiver;

    impl FrameReceiver {
        pub fn open(
            _interface: &str,
            _subtype: u8,
            _destinations: &[MacAddress],
        ) -> io::Result<Self> {
            Err(unsupported())
        }

        pub fn receive(
            &self,
            _buffer: &mut [u8],
            _timeout: Option<Duration>,
        ) -> io::Result<Option<usize>> {
            Err(unsupported())
        }

        pub fn ready(_receivers: &[&Self], _timeout: Option<Duration>) -> io::Result<Vec<usize>> {
            Err(unsupported())
        }

        pub fn dropped(&self) -> u64 {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payloads_start_after_the_tag_if_there_is_one() {
        let mut frame = vec![0u8; 12];
        frame.extend_from_slice(&[0x22, 0xf0, 0x02, 0x81]);
        assert_eq!(avtp_payload(&frame), Some(&[0x02, 0x81][..]));
        let mut tagged = vec![0u8; 12];
        tagged.extend_from_slice(&[0x81, 0x00, 0x60, 0x02, 0x22, 0xf0, 0x02]);
        assert_eq!(avtp_payload(&tagged), Some(&[0x02][..]));
        let mut other = vec![0u8; 12];
        other.extend_from_slice(&[0x08, 0x00, 0x45]);
        assert_eq!(avtp_payload(&other), None);
        assert_eq!(avtp_payload(&[0; 5]), None);
    }
}
