# Changelog

All notable changes to triib. Versions follow
[Semantic Versioning](https://semver.org).

## [Unreleased]

The Streams tab, max transit times, media clock streams on this
computer's entities with one chosen as their clock source, and this
computer's talkers and listeners folded into its entities.

### Controller

- The Streams tab lists an entity's streams, outputs then inputs, five
  before it scrolls, and shows the picked one's details under the list:
  its name, format, state, stream ID, destination and VLAN, and for this
  computer's entities its audio and channels. The Entity tab no longer
  shows streams.
- An output's max transit time is picked in its details, from 0.25 ms to
  50 ms, on any entity that answers GET_MAX_TRANSIT_TIME, Milan or not,
  the Mac's AVB entity among them. A running stream's refusal says to
  disconnect it first.
- While a media clock stream is picked, the Streams tab leaves out the
  entity's channel mappings, which are its audio streams'.
- The matrix remembers which entities are folded, across restarts.
- An entity with several configurations has a configuration picker,
  each by its own name, or where its names repeat, as the Mac's ten do,
  by what it holds: "16 in, 16 out, 8 ch streams". `triib-cli
  configuration` changes it, and `triib-cli describe` lists them.
- On a Mac, the Mac's own AVB entity shows as this computer's, with its
  Core Audio device and a button that opens Audio MIDI Setup; where AVB
  is off, the Entities view says where to turn it on.
- Controllers that do not advertise, as the Mac's own does not, are
  listed from the commands triib hears them send: their connection
  commands, which reach every station, and their commands to this
  computer's entities. Each is named for the computer it runs on, as
  "Controller on Mac mini", and stays listed for 5 minutes after its
  last command.
- The network view leaves out controllers, which take no part in the
  clock or the streams, and shows this computer as its own entities
  where one is there, rather than as This computer beside them.
- Controllers have an icon of their own, a traffic light, rather than
  this computer's. triib's controller, which has no model to read, is
  named for the computer it runs on: "triib on Mac mini" after an entity
  from the same address, "triib on this computer", else "triib
  controller".
- Settings has a Storage tab, as prev has: the presets folder and the
  entities file, each typed or chosen with Choose…, and checked before
  it is taken.
- The interface picker's filter opens a menu that hides virtual
  interfaces, as before by default, wireless interfaces, and interfaces
  without a PTP hardware clock, each checked on its own.
- An entity advertising another model is read again in place: the list
  and the inspector keep showing what was read before until the new
  read is done.
- A stream that offers CRF formats counts as a media clock stream
  whatever it sends, as the Mac's media clock outputs sending one
  channel of AAF do.
- `triib-cli streams` prints each input's counters, and its probing
  state and last ACMP answer where Milan's stream info gives them.
- The new app icon.

### This computer's entities (Linux)

- Talkers and listeners are folded into entities of any number of
  streams, as one of one stream does what they did. The Entities view's
  one button, Add entity, adds an entity with a stream each way, and its
  Streams tab adds and removes streams. triib 1.0's talker and listener
  entries, in `entities.toml` and in presets, read as entities of their
  one stream, named as the entity is, with their audio, format, binding,
  transit time and class. Their entity IDs stay; their model IDs change,
  so controllers read them again once.
- One Add stream button, whose menu adds an audio input or output, or a
  media clock input or output.
- Media clock streams: CRF in Milan's format, 48 kHz with a timestamp
  every 96 samples, sent on the same timeline as the entity's audio,
  without channels, audio or mappings.
- An entity advertises the kinds of stream it has: a media clock source
  or sink for its media clock streams, an audio source or sink for its
  audio streams, rather than audio whatever it had.
- Any input can be the entity's clock source: its talkers then follow
  the clock recovered from a media clock's timestamps or an audio
  stream's presentation times, and gPTP time while it is not recovered.
  The choice is kept, and the clock domain's counters count each lock
  and loss.
- Controllers map an entity's inputs: their ports have dynamic
  mappings, as Milan has them, kept in `entities.toml` and played as
  mapped.
- An entity's input binds to its own output, which the matrix offers on
  this computer's entities.
- The max transit times controllers set on an entity's outputs are kept
  in `entities.toml`.
- Adding, removing or changing an entity's stream keeps the entity on
  the network, where controllers read it again in place, and leaves its
  other streams running as they were, their stream addresses, bindings
  and reservations with them. A stream added runs at the entity's rate.
- The entities' model name is triib entity, and their model IDs change
  once for it.
- The entities are listed in `entities.toml`, to which 1.0's
  `endpoints.toml` moves, or in the file Settings, Storage name; the
  daemon starts again on a file named there, one not there yet starting
  as a copy of the one in use.

### Fixed

- An entity advertising a new gPTP grandmaster or domain has its gPTP
  state and path read again, so one read while it was its own
  grandmaster joins the network view's tree.
- The AVB Lite offset alarm shows only where an interface runs AVB Lite,
  not where it is only capable of it, as an AVB Wireless station is.
- An entity running AVB that could run AVB Lite shows "AVB (AVB Lite
  capable)".
- Adding or removing one of this computer's entities leaves the others
  running, their stream addresses as they were.
- This computer's entities show as its own from the moment they
  advertise.
- A refused SET_MAX_TRANSIT_TIME no longer shows the time asked for.
- A stream's own SR class sets its priority and its reservation, so an
  entity's class B stream is reserved as class B.
- A preset moved to another computer renames the talkers its entities'
  inputs are bound to.
- An entity lists each sampling rate once.
- This computer's entities tell a talker when an input lets go of its
  stream: unbound, bound elsewhere, removed or leaving with its entity.
  A talker outside Milan, such as the Mac, kept the connection a probe
  made and streamed on to a listener long gone.
- On Linux, the links triib opens open: the desktop portal refused
  them, as it does any app holding a capability, as triib holds
  `cap_net_raw`. triib now asks it from a helper without one.

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
