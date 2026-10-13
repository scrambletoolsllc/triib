# Changelog

All notable changes to triib. Versions follow
[Semantic Versioning](https://semver.org).

## [1.0.0] - 2026-10-12

AVB Wireless, devices of the computer's own with several streams each
way, AI agents through MCP, and on a Mac the Mac's own AVB entity.

### Controller

- AVB Wireless: triib asks each entity's AVB interfaces for their
  wireless status and shows it in the inspector, the entity list and the
  network map: the link, the time mode, and on an access point its
  stations and the frames a second it can deliver, which decide whether
  it admits Class A (12000 or more). Wi-Fi hops are marked on the map.
- A matrix cell whose input must change its format first opens a menu of
  the formats both ends take; picking one changes them and connects.
- The entity list keeps its headings and the names in view while the
  rest of it scrolls, and lines up plain media clock values with the
  menus.
- On a Mac, triib reads and controls the Mac's own AVB entity, through
  Apple's AudioVideoBridging framework, as macOS never hands that entity
  what a program writes to BPF.
- AI agents such as Claude Code can see and change the network through
  `triib --mcp`, a Model Context Protocol server: read the entities,
  their streams, formats, clocks and connections, connect and
  disconnect, change formats, sampling rates, clock sources and names,
  and identify. triib asks before each new agent, and Settings turns
  outside control off.
- Shift turns rediscover into clear and rescan.
- `triib-cli wireless` and `wireless-config` read AVB Wireless status
  and set whether an access point admits Class A; `triib-cli network`
  marks Wi-Fi hops.

### This computer's talkers and listeners (Linux)

- Devices: one entity with any number of streams each way, each set up
  on its own, with its own virtual audio device in PipeWire unless it
  uses a sound card. Its outputs send from one thread and its inputs
  take their streams on another.
- Talkers can send in SR class B, with class B's own formats.
- Talkers, listeners and devices each have their own icon.
- MRP runs once for the port, for all the endpoints, and the port
  declares both SR classes' domains, so a bridge passes class B
  streams as well.

### Packages

- The Linux packages are built on Ubuntu 24.04, as PipeWire's Rust
  bindings need its headers, so they need glibc 2.39 or newer: Ubuntu
  24.04, Debian 13, Fedora 40 and later. Ubuntu 22.04 and Debian 12 are
  no longer covered.

### Fixed

- Connections to a class B talker say so, and a class B talker
  advertises class B support.
- An output says it is sending only while a listener takes its stream.
- Entities sharing a clock each show in the map, and a Wi-Fi station
  without a gPTP path shows under its access point.
- Wireless status queries left unanswered are asked again.
- This computer's entities offer only their internal clock.
- `triib-cli` says when a listener's binding is not known.

## [0.9.0] - 2026-10-10

The first release: an ATDECC (IEEE 1722.1) controller with Milan and AVB
Lite support for Linux, Windows and macOS, and on Linux talkers and
listeners of the computer's own.

### Controller

- Discovers the ATDECC entities on the interface you pick, reads each
  entity model, its names, streams, clocks and Milan support, and keeps
  it, so a device seen before shows in full at once.
- A connection matrix, talker outputs across and listener inputs down,
  each cell showing whether the stream flows, waits or has failed, and
  whether the two formats can meet.
- A map of the network from each entity's gPTP path: bridges, devices,
  every stream as its own wire, failed reservations where they stop,
  and the clock tree from the grandmaster with each device in sync or
  not.
- Renames entities, changes stream formats, sampling rates and clock
  sources, maps channels to streams, sets controls such as gain and
  mute, and makes a device identify itself.
- Presets of each entity's clock sources, sampling rates, stream
  formats, controls and connections, recalled by changing only what
  differs.
- A log of every ATDECC frame sent and heard, decoded in plain words,
  with the frames that break the rules marked and the reason given.
- AVB Lite: each device's mode and why, its PTP offset, what its streams
  take of each link, and the alarms the profile calls for.
- `triib-cli` does the same from scripts.

### This computer's talkers and listeners (Linux)

- `triib-endpointd` runs Milan talkers and listeners streaming AAF or
  AM824 at 48, 96 or 192 kHz to and from the computer's audio devices,
  on a wired interface with a PTP hardware clock and linuxptp's ptp4l.
- Where no AVB bridge answers, they fall back to AVB Lite over switches
  that are not AVB bridges, and the daemon moves ptp4l between gPTP and
  the AVB Lite PTP profile through the `triib-ptp4l-gptp@` and
  `triib-ptp4l-lite@` units.
- `triib-link@` keeps Energy-Efficient Ethernet and PAUSE off the link,
  as AVB Lite asks, and `triib-link-reset@` restarts an interface whose
  driver keeps its time stamps on received PTP frames.

### Everywhere

- 38 languages, following the system's or the one picked in Settings,
  right-to-left languages mirrored.
- Material 3 design, light and dark, shared with prev.
- Packages for Linux on x86_64, ARM64 and RISC-V (.deb, .rpm, tarball,
  and a PKGBUILD for Arch), Windows on x64 and ARM64 (.msi and .zip,
  signed by Scramble Tools LLC), and macOS on Apple Silicon (.pkg), each
  setting up what triib needs to send and receive raw Ethernet.
- The protocols are Rust crates of their own, `atdecc`, `avb-mrp` and
  `avb-net`, which build without std for microcontrollers.

### Known limits

- The macOS package is not signed with a Developer ID or notarized, so
  macOS blocks it until you choose Open Anyway in System Settings,
  Privacy & Security.
- Talkers and listeners of the computer's own run on Linux only.
- The translations await review by native speakers.
- The AVB Lite Endpoint Declaration TLV goes out only while ptp4l does
  not run gPTP on the interface, until linuxptp can carry it.
- On Aquantia cards (Linux `atlantic`), the driver cuts unicast PTP
  frames short, so AVB Lite's delay requests stay multicast, and keeps
  time stamps on received PTP frames after the link renegotiates, which
  the daemon works around by restarting the interface.

[0.9.0]: https://github.com/scrambletoolsllc/triib/releases/tag/v0.9.0
