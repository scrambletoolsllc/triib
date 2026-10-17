//! The Ethernet interfaces of this computer and what each can do.

use crate::MacAddress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interface {
    pub name: String,
    pub mac: MacAddress,
    /// The link is up.
    pub up: bool,
    /// Link speed in Mb/s, when the driver reports it.
    pub speed: Option<u32>,
    /// Backed by hardware, not a bridge, tunnel or other virtual device.
    pub physical: bool,
    pub wireless: bool,
    /// Index of the PTP hardware clock, on interfaces that timestamp in
    /// hardware.
    pub hardware_clock: Option<u32>,
}

/// The Ethernet interfaces, physical and up first, then by name.
pub fn interfaces() -> Vec<Interface> {
    let mut found = platform::interfaces();
    found.sort_by(|left, right| {
        (!left.physical, !left.up, &left.name).cmp(&(!right.physical, !right.up, &right.name))
    });
    found
}

/// The MAC address of the interface named `name`.
pub fn mac_of(name: &str) -> Option<MacAddress> {
    platform::interfaces()
        .into_iter()
        .find(|interface| interface.name == name)
        .map(|interface| interface.mac)
}

/// The kernel's index for the interface named `name`.
#[cfg(target_os = "linux")]
pub(crate) fn index_of(name: &str) -> Option<i32> {
    if name.contains('/') {
        return None;
    }
    std::fs::read_to_string(format!("/sys/class/net/{name}/ifindex"))
        .ok()?
        .trim()
        .parse()
        .ok()
}

/// What an interface does about Energy-Efficient Ethernet and PAUSE, as
/// its driver reports it. The AVB Lite profile asks endpoints to keep both
/// off their own link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LinkPower {
    /// EEE is enabled, so the interface advertises it and its link may
    /// sleep between frames.
    pub eee: bool,
    /// The interface sends PAUSE or acts on it, or offers to when it
    /// negotiates.
    pub pause: bool,
}

/// What the interface named `name` does about EEE and PAUSE, read without
/// privileges. What the driver does not support reads as off.
#[cfg(target_os = "linux")]
pub fn link_power(name: &str) -> std::io::Result<LinkPower> {
    ethtool::link_power(name)
}

/// Whether the interface named `name` drops VLAN-tagged frames in hardware
/// unless their VLAN is registered with it, as Intel's igb and igc do with
/// `rx-vlan-filter` on, read without privileges. A packet socket hears a
/// stream's tagged frames there only with the interface promiscuous, which
/// turns the filter off.
#[cfg(target_os = "linux")]
pub fn filters_vlans(name: &str) -> std::io::Result<bool> {
    ethtool::filters_vlans(name)
}

#[cfg(target_os = "linux")]
#[allow(unsafe_code)]
mod ethtool {
    use std::io;
    use std::mem::zeroed;
    use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

    use super::LinkPower;

    /// The ethtool commands read here (linux/ethtool.h).
    const GET_PAUSE: u32 = 0x12;
    const GET_STRINGS: u32 = 0x1b;
    const GET_SSET_INFO: u32 = 0x37;
    const GET_FEATURES: u32 = 0x3a;
    const GET_EEE: u32 = 0x44;
    /// `ETH_SS_FEATURES`, the string set naming the netdev features.
    const FEATURE_NAMES: u32 = 4;
    /// `ETH_GSTRING_LEN`, each name's room in a string set.
    const NAME_LENGTH: usize = 32;
    /// The feature that drops tagged frames of VLANs not registered.
    const VLAN_FILTER: &[u8] = b"rx-vlan-filter";

    /// `struct ethtool_pauseparam`.
    #[repr(C)]
    #[derive(Default)]
    struct Pause {
        cmd: u32,
        autoneg: u32,
        rx_pause: u32,
        tx_pause: u32,
    }

    /// `struct ethtool_eee`.
    #[repr(C)]
    #[derive(Default)]
    struct Eee {
        cmd: u32,
        supported: u32,
        advertised: u32,
        lp_advertised: u32,
        eee_active: u32,
        eee_enabled: u32,
        tx_lpi_enabled: u32,
        tx_lpi_timer: u32,
        reserved: [u32; 2],
    }

    /// `struct ethtool_sset_info`, with room for the one count asked.
    #[repr(C)]
    #[derive(Default)]
    struct SetInfo {
        cmd: u32,
        reserved: u32,
        mask: u64,
        count: u32,
    }

    pub fn link_power(name: &str) -> io::Result<LinkPower> {
        let socket = control(name)?;
        let bytes = name.as_bytes();
        let mut pause = Pause {
            cmd: GET_PAUSE,
            ..Pause::default()
        };
        let mut eee = Eee {
            cmd: GET_EEE,
            ..Eee::default()
        };
        let pause = match ask(&socket, bytes, (&raw mut pause).cast()) {
            // Negotiating PAUSE while sending and taking none advertises
            // none, so it stays off.
            Ok(()) => pause.rx_pause != 0 || pause.tx_pause != 0,
            Err(error) if unsupported(&error) => false,
            Err(error) => return Err(error),
        };
        let eee = match ask(&socket, bytes, (&raw mut eee).cast()) {
            Ok(()) => eee.eee_enabled != 0,
            Err(error) if unsupported(&error) => false,
            Err(error) => return Err(error),
        };
        Ok(LinkPower { eee, pause })
    }

    pub fn filters_vlans(name: &str) -> io::Result<bool> {
        let socket = control(name)?;
        let bytes = name.as_bytes();
        let mut info = SetInfo {
            cmd: GET_SSET_INFO,
            mask: 1 << FEATURE_NAMES,
            ..SetInfo::default()
        };
        match ask(&socket, bytes, (&raw mut info).cast()) {
            Ok(()) => {}
            Err(error) if unsupported(&error) => return Ok(false),
            Err(error) => return Err(error),
        }
        let count = info.count as usize;
        // `struct ethtool_gstrings`: the command, the set and the count,
        // then each name in its room.
        let mut names = vec![0u32; 3 + count * NAME_LENGTH / 4];
        names[..3].copy_from_slice(&[GET_STRINGS, FEATURE_NAMES, info.count]);
        ask(&socket, bytes, names.as_mut_ptr().cast())?;
        let text: Vec<u8> = names[3..]
            .iter()
            .flat_map(|word| word.to_ne_bytes())
            .collect();
        let Some(index) = text
            .chunks(NAME_LENGTH)
            .position(|room| room.split(|&byte| byte == 0).next() == Some(VLAN_FILTER))
        else {
            return Ok(false);
        };
        // `struct ethtool_gfeatures`: the command and the block count, then
        // for each 32 features what is available, requested, active and
        // never changed.
        let blocks = count.div_ceil(32);
        let mut features = vec![0u32; 2 + blocks * 4];
        features[..2].copy_from_slice(&[GET_FEATURES, blocks as u32]);
        ask(&socket, bytes, features.as_mut_ptr().cast())?;
        let active = features[2 + index / 32 * 4 + 2];
        Ok(active & (1 << (index % 32)) != 0)
    }

    /// A socket to hand the driver of the interface named `name` ethtool
    /// commands through.
    fn control(name: &str) -> io::Result<OwnedFd> {
        let bytes = name.as_bytes();
        if bytes.is_empty() || bytes.len() >= libc::IFNAMSIZ || bytes.contains(&0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "no such interface name",
            ));
        }
        // SAFETY: socket takes plain integers; the result is checked.
        let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM | libc::SOCK_CLOEXEC, 0) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: fd is a new socket nothing else owns.
        Ok(unsafe { OwnedFd::from_raw_fd(fd) })
    }

    fn unsupported(error: &io::Error) -> bool {
        matches!(error.raw_os_error(), Some(libc::EOPNOTSUPP | libc::EINVAL))
    }

    /// Hands the interface's driver the ethtool command `data` points to.
    fn ask(socket: &OwnedFd, name: &[u8], data: *mut libc::c_char) -> io::Result<()> {
        // SAFETY: ifreq is plain data, valid when zeroed.
        let mut request: libc::ifreq = unsafe { zeroed() };
        for (place, &byte) in request.ifr_name.iter_mut().zip(name) {
            *place = byte as libc::c_char;
        }
        request.ifr_ifru.ifru_data = data;
        // SAFETY: `request` is a valid ifreq whose data points to an ethtool
        // command struct of the size its command reads, both living through
        // the call.
        if unsafe { libc::ioctl(socket.as_raw_fd(), libc::SIOCETHTOOL, &raw mut request) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use std::fs;
    use std::path::Path;

    use super::Interface;

    /// `ARPHRD_ETHER`, the link type of Ethernet and Wi-Fi interfaces.
    const LINK_TYPE_ETHERNET: u32 = 1;

    pub fn interfaces() -> Vec<Interface> {
        let Ok(entries) = fs::read_dir("/sys/class/net") else {
            return Vec::new();
        };
        entries
            .flatten()
            .filter_map(|entry| read(&entry.path(), entry.file_name().to_str()?))
            .collect()
    }

    fn read(dir: &Path, name: &str) -> Option<Interface> {
        let value = |file: &str| fs::read_to_string(dir.join(file)).ok();
        let link_type: u32 = value("type")?.trim().parse().ok()?;
        if link_type != LINK_TYPE_ETHERNET {
            return None;
        }
        let mac = value("address")?.trim().parse().ok()?;
        Some(Interface {
            name: name.to_owned(),
            mac,
            up: value("operstate").is_some_and(|state| state.trim() == "up"),
            // Negative or unreadable while the link is down.
            speed: value("speed").and_then(|speed| speed.trim().parse().ok()),
            physical: dir.join("device").exists(),
            wireless: dir.join("wireless").exists() || dir.join("phy80211").exists(),
            hardware_clock: hardware_clock(dir),
        })
    }

    /// The `ptpN` the driver registered for the interface, as `ethtool -T`
    /// reports: under the PCI device for most drivers (igb, igc, mlx5),
    /// under the interface itself for some (atlantic).
    pub(super) fn hardware_clock(dir: &Path) -> Option<u32> {
        let clocks = |folder: std::path::PathBuf| {
            fs::read_dir(folder)
                .into_iter()
                .flatten()
                .flatten()
                .filter_map(|entry| {
                    entry
                        .file_name()
                        .to_str()?
                        .strip_prefix("ptp")?
                        .parse::<u32>()
                        .ok()
                })
        };
        clocks(dir.join("device/ptp"))
            .chain(clocks(dir.to_path_buf()))
            .min()
    }
}

#[cfg(target_os = "macos")]
#[allow(unsafe_code)]
mod platform {
    use std::ffi::CStr;
    use std::mem::{offset_of, size_of, zeroed};
    use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

    use super::Interface;
    use crate::MacAddress;

    /// `IFT_ETHER`, the link type of Ethernet and Wi-Fi interfaces.
    const LINK_TYPE_ETHERNET: u8 = 6;
    /// The network type in a media word (`IFM_NMASK`), and its values for
    /// Ethernet and Wi-Fi.
    const MEDIA_NETWORK: i32 = 0xe0;
    const MEDIA_WIFI: i32 = 0x80;
    /// The media status word says whether the link is up (`IFM_AVALID`),
    /// and that it is (`IFM_ACTIVE`).
    const MEDIA_STATUS_VALID: i32 = 0x01;
    const MEDIA_ACTIVE: i32 = 0x02;

    /// `struct ifmediareq`, for `SIOCGIFMEDIA`, packed to four octets as
    /// macOS declares it.
    #[derive(Clone, Copy)]
    #[repr(C, packed(4))]
    struct MediaRequest {
        name: [libc::c_char; libc::IFNAMSIZ],
        current: i32,
        mask: i32,
        status: i32,
        active: i32,
        count: i32,
        list: *mut i32,
    }

    pub fn interfaces() -> Vec<Interface> {
        let mut head: *mut libc::ifaddrs = std::ptr::null_mut();
        // SAFETY: getifaddrs fills `head` with a list freed below.
        if unsafe { libc::getifaddrs(&raw mut head) } != 0 {
            return Vec::new();
        }
        // A socket to ask each interface about its media on.
        // SAFETY: plain system call with integer arguments.
        let raw = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0) };
        // SAFETY: `raw`, when valid, is a socket this function owns.
        let query = (raw >= 0).then(|| unsafe { OwnedFd::from_raw_fd(raw) });
        let mut found = Vec::new();
        let mut entry = head;
        while !entry.is_null() {
            // SAFETY: each entry of the list getifaddrs made is valid until
            // it is freed.
            let address = unsafe { &*entry };
            entry = address.ifa_next;
            if let Some(interface) = read(address, query.as_ref()) {
                found.push(interface);
            }
        }
        // SAFETY: `head` came from getifaddrs and is freed once.
        unsafe { libc::freeifaddrs(head) };
        found
    }

    /// The interface an `AF_LINK` entry describes, when it is Ethernet.
    fn read(address: &libc::ifaddrs, query: Option<&OwnedFd>) -> Option<Interface> {
        if address.ifa_addr.is_null() {
            return None;
        }
        // SAFETY: a non-null address points at a sockaddr at least as long
        // as its header.
        if i32::from(unsafe { (*address.ifa_addr).sa_family }) != libc::AF_LINK {
            return None;
        }
        // SAFETY: an AF_LINK address is a sockaddr_dl.
        let link = unsafe { &*address.ifa_addr.cast::<libc::sockaddr_dl>() };
        if link.sdl_type != LINK_TYPE_ETHERNET || link.sdl_alen != 6 {
            return None;
        }
        // The address follows the name in `sdl_data`, within `sdl_len`.
        let start = offset_of!(libc::sockaddr_dl, sdl_data) + usize::from(link.sdl_nlen);
        if start + 6 > usize::from(link.sdl_len) {
            return None;
        }
        let mut mac = [0; 6];
        // SAFETY: the six octets lie within the address, as checked.
        unsafe {
            std::ptr::copy_nonoverlapping(
                (link as *const libc::sockaddr_dl).cast::<u8>().add(start),
                mac.as_mut_ptr(),
                6,
            );
        }
        // SAFETY: the name is a NUL-terminated string in the entry.
        let bsd_name = unsafe { CStr::from_ptr(address.ifa_name) };
        let name = bsd_name.to_string_lossy().into_owned();
        let speed = (!address.ifa_data.is_null())
            .then(|| {
                // SAFETY: an AF_LINK entry's data is its if_data.
                unsafe { (*address.ifa_data.cast::<libc::if_data>()).ifi_baudrate }
            })
            .filter(|&bits| bits > 0)
            .map(|bits| bits / 1_000_000);
        let media = query.and_then(|query| media(query, &name));
        let flags = address.ifa_flags as i32;
        let up = flags & libc::IFF_UP != 0
            && match media {
                Some(media) if media.status & MEDIA_STATUS_VALID != 0 => {
                    media.status & MEDIA_ACTIVE != 0
                }
                _ => flags & libc::IFF_RUNNING != 0,
            };
        Some(Interface {
            // macOS names its Ethernet and Wi-Fi ports enN; anpi, ap, awdl,
            // llw and the like are its own, and so are some enN.
            physical: name.starts_with("en") && !without_port(bsd_name),
            wireless: media.is_some_and(|media| media.current & MEDIA_NETWORK == MEDIA_WIFI),
            name,
            mac: MacAddress(mac),
            up,
            speed: speed.filter(|_| up),
            hardware_clock: None,
        })
    }

    /// Whether the interface named `name` has no Ethernet port of its
    /// own, as its controller shows: Thunderbolt Bridge's IP over
    /// Thunderbolt, or a USB-C port being a USB network device for another
    /// computer.
    fn without_port(name: &CStr) -> bool {
        const CONTROLLERS: [&CStr; 2] = [c"AppleThunderboltIPPort", c"AppleUSBDeviceNCMData"];
        // SAFETY: IOServiceGetMatchingService takes the matching dictionary
        // IOBSDNameMatching made, and each object found is released once.
        unsafe {
            let matching = iokit::IOBSDNameMatching(iokit::MAIN_PORT, 0, name.as_ptr());
            if matching.is_null() {
                return false;
            }
            let interface = iokit::IOServiceGetMatchingService(iokit::MAIN_PORT, matching);
            if interface == 0 {
                return false;
            }
            let mut controller = 0;
            let found = iokit::IORegistryEntryGetParentEntry(
                interface,
                c"IOService".as_ptr(),
                &raw mut controller,
            ) == 0;
            iokit::IOObjectRelease(interface);
            if !found {
                return false;
            }
            let without = CONTROLLERS
                .iter()
                .any(|class| iokit::IOObjectConformsTo(controller, class.as_ptr()) != 0);
            iokit::IOObjectRelease(controller);
            without
        }
    }

    /// The few IOKit calls that look up an interface's controller.
    mod iokit {
        use std::ffi::{c_char, c_void};

        /// `kIOMainPortDefault`.
        pub const MAIN_PORT: u32 = 0;

        #[link(name = "IOKit", kind = "framework")]
        unsafe extern "C" {
            pub fn IOBSDNameMatching(port: u32, options: u32, name: *const c_char) -> *mut c_void;
            pub fn IOServiceGetMatchingService(port: u32, matching: *mut c_void) -> u32;
            pub fn IORegistryEntryGetParentEntry(
                entry: u32,
                plane: *const c_char,
                parent: *mut u32,
            ) -> i32;
            pub fn IOObjectConformsTo(object: u32, class: *const c_char) -> u32;
            pub fn IOObjectRelease(object: u32) -> i32;
        }
    }

    /// What `SIOCGIFMEDIA` says of the interface named `name`.
    fn media(query: &OwnedFd, name: &str) -> Option<MediaRequest> {
        // SAFETY: MediaRequest is plain data, valid when zeroed.
        let mut request: MediaRequest = unsafe { zeroed() };
        let bytes = name.as_bytes();
        if bytes.len() >= request.name.len() {
            return None;
        }
        for (place, &byte) in request.name.iter_mut().zip(bytes) {
            *place = byte as libc::c_char;
        }
        // _IOWR('i', 56, struct ifmediareq)
        let code = 0xc000_0000
            | ((size_of::<MediaRequest>() as libc::c_ulong & 0x1fff) << 16)
            | (libc::c_ulong::from(b'i') << 8)
            | 56;
        // SAFETY: `request` is a valid ifmediareq, with no list to fill,
        // that lives through the call.
        let result = unsafe { libc::ioctl(query.as_raw_fd(), code, &raw mut request) };
        (result == 0).then_some(request)
    }
}

/// The capture device Npcap knows the interface named `name` by.
#[cfg(windows)]
pub(crate) fn device_of(name: &str) -> Option<String> {
    platform::adapters()
        .into_iter()
        .find(|(interface, _)| interface.name == name)
        .map(|(_, guid)| format!("\\Device\\NPF_{guid}"))
}

#[cfg(windows)]
#[allow(unsafe_code)]
mod platform {
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        GAA_FLAG_SKIP_ANYCAST, GAA_FLAG_SKIP_DNS_SERVER, GAA_FLAG_SKIP_MULTICAST,
        GAA_FLAG_SKIP_UNICAST, GetAdaptersAddresses, GetIfEntry2, IF_TYPE_ETHERNET_CSMACD,
        IF_TYPE_IEEE80211, IP_ADAPTER_ADDRESSES_LH, MIB_IF_ROW2,
    };
    use windows_sys::Win32::NetworkManagement::Ndis::IfOperStatusUp;

    use super::Interface;
    use crate::MacAddress;

    /// `ERROR_BUFFER_OVERFLOW`, when the list outgrew the room given.
    const BUFFER_OVERFLOW: u32 = 111;
    /// The interface flag set when a connector is present, which is to
    /// say a network adapter is (`ConnectorPresent`).
    const CONNECTOR_PRESENT: u8 = 1 << 2;

    pub fn interfaces() -> Vec<Interface> {
        adapters()
            .into_iter()
            .map(|(interface, _)| interface)
            .collect()
    }

    /// The Ethernet and Wi-Fi adapters, each with the GUID that names its
    /// device.
    pub fn adapters() -> Vec<(Interface, String)> {
        let flags = GAA_FLAG_SKIP_UNICAST
            | GAA_FLAG_SKIP_ANYCAST
            | GAA_FLAG_SKIP_MULTICAST
            | GAA_FLAG_SKIP_DNS_SERVER;
        // Room for a few adapters first, then as much as Windows asks for.
        let mut size: u32 = 16 * 1024;
        let mut buffer: Vec<u64> = Vec::new();
        for _ in 0..4 {
            buffer = vec![0; (size as usize).div_ceil(8)];
            // SAFETY: the buffer, eight-octet aligned, holds `size` octets
            // and lives through the call.
            let result = unsafe {
                GetAdaptersAddresses(
                    0,
                    flags,
                    std::ptr::null(),
                    buffer.as_mut_ptr().cast(),
                    &raw mut size,
                )
            };
            match result {
                0 => break,
                BUFFER_OVERFLOW => buffer.clear(),
                _ => return Vec::new(),
            }
        }
        if buffer.is_empty() {
            return Vec::new();
        }
        let mut found = Vec::new();
        let mut entry = buffer.as_ptr().cast::<IP_ADAPTER_ADDRESSES_LH>();
        while !entry.is_null() {
            // SAFETY: each entry of the list lies within the buffer.
            let adapter = unsafe { &*entry };
            entry = adapter.Next;
            if let Some(read) = read(adapter) {
                found.push(read);
            }
        }
        found
    }

    fn read(adapter: &IP_ADAPTER_ADDRESSES_LH) -> Option<(Interface, String)> {
        let wireless = match adapter.IfType {
            IF_TYPE_ETHERNET_CSMACD => false,
            IF_TYPE_IEEE80211 => true,
            _ => return None,
        };
        if adapter.PhysicalAddressLength != 6 || adapter.FriendlyName.is_null() {
            return None;
        }
        let mut mac = [0; 6];
        mac.copy_from_slice(&adapter.PhysicalAddress[..6]);
        // SAFETY: the friendly name is a NUL-terminated wide string and the
        // adapter name a NUL-terminated string, both within the list.
        let (name, guid) = unsafe {
            (
                String::from_utf16_lossy(wide(adapter.FriendlyName)),
                std::ffi::CStr::from_ptr(adapter.AdapterName.cast())
                    .to_string_lossy()
                    .into_owned(),
            )
        };
        let up = adapter.OperStatus == IfOperStatusUp;
        // SAFETY: MIB_IF_ROW2 is plain data, valid when zeroed; the call
        // reads the LUID set in it and fills in the rest.
        let physical = unsafe {
            let mut row: MIB_IF_ROW2 = std::mem::zeroed();
            row.InterfaceLuid = adapter.Luid;
            GetIfEntry2(&raw mut row) == 0
                && row.InterfaceAndOperStatusFlags._bitfield & CONNECTOR_PRESENT != 0
        };
        // Unknown speeds read as all ones.
        let speed = (up && adapter.TransmitLinkSpeed != u64::MAX)
            .then(|| u32::try_from(adapter.TransmitLinkSpeed / 1_000_000).ok())
            .flatten()
            .filter(|&speed| speed > 0);
        Some((
            Interface {
                name,
                mac: MacAddress(mac),
                up,
                speed,
                physical,
                wireless,
                hardware_clock: None,
            },
            guid,
        ))
    }

    /// The characters of a NUL-terminated wide string.
    ///
    /// # Safety
    ///
    /// `string` points at a NUL-terminated wide string that outlives the
    /// slice.
    unsafe fn wide<'a>(string: *const u16) -> &'a [u16] {
        let mut length = 0;
        // SAFETY: the string ends at a NUL, as the caller promises.
        unsafe {
            while *string.add(length) != 0 {
                length += 1;
            }
            std::slice::from_raw_parts(string, length)
        }
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
mod platform {
    use super::Interface;

    /// Not listed on this system yet.
    pub fn interfaces() -> Vec<Interface> {
        Vec::new()
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn lists_without_loopback() {
        assert!(interfaces().iter().all(|found| found.name != "lo"));
    }

    #[test]
    fn finds_clocks_under_the_device_or_the_interface() {
        let root = std::env::temp_dir().join(format!("avb-net-clocks-{}", std::process::id()));
        let device = root.join("igc0");
        let own = root.join("atlantic0");
        let none = root.join("r8169");
        for folder in [
            device.join("device/ptp/ptp2"),
            own.join("ptp0"),
            none.join("device"),
        ] {
            std::fs::create_dir_all(folder).unwrap();
        }
        // An entry that only starts like a clock is not one.
        std::fs::create_dir_all(own.join("ptpx")).unwrap();
        assert_eq!(platform::hardware_clock(&device), Some(2));
        assert_eq!(platform::hardware_clock(&own), Some(0));
        assert_eq!(platform::hardware_clock(&none), None);
        std::fs::remove_dir_all(root).unwrap();
    }
}
