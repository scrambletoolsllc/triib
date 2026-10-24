# Changelog

All notable changes to triib. Versions follow
[Semantic Versioning](https://semver.org).

## [Unreleased]

### Linux

- An entity added to this computer is listed at once, greyed, its State
  saying what it waits on: triib-endpointd starting, the interface being
  set up for AVB, its link. The bar above the list says what the daemon
  is doing until it is done, such as setting up the interface or starting
  ptp4l, or that linuxptp is not installed.
- This computer's entities advertise again as soon as their link comes
  back up, as after triib-endpointd turns EEE and PAUSE off, rather than
  10 s later: a new entity reached triib's list only after that wait.

## [1.2.0] - 2026-10-23

The network map draws the traffic on each link and places every device
in its clock domain, those running AVB Lite behind the non-AVB switch in
front of them, and the connection matrix keeps talkers and listeners in
different domains from connecting. triib follows entities that send no
notifications, shows entities' logos and their vendors', and on a Mac
has a menu bar. On Linux only a small helper holds `CAP_NET_RAW`, and
triib-endpointd shares its MSRP participant with other programs such as
plaa, hears other computers' AVB Lite declarations on the Intel I210 and
offers the same stream formats as plaa. The app ID is now `run.triib`.

### Controller

- On a Mac, the menu bar has triib's menus: Settings and Quit in the
  triib menu, Presets and Close Window in File, Cut, Copy, Paste and
  Select All in Edit, the three views, the log and the inspector in
  View, and Window and Help menus, in the interface's language.
- The network map draws the traffic on each link: a multicast stream to
  several listeners takes one wire as far as their paths run together
  and branches where they part, all in its color, while a unicast stream,
  as AVB Lite can send, has a wire to each listener from its talker. A
  device counts each stream it sends once however many listeners it
  has, and a bridge each stream passing through. Clicking a shared wire
  brings the whole stream forward with all its listeners.
- A stream its talker says it sends with no known listener bound to it
  still shows, on a wire from the talker to its bridge.
- The network map shows every entity and bridge it hears in its clock
  domain, each labelled with its number: a gPTP domain from its
  grandmaster down through the AVB bridges, or a PTP domain whose devices
  run AVB Lite, those that found no AVB bridge and fell back to it hanging
  under a dashed card for the non-AVB switch in front of them, with their
  streams crossing it; their details say why they run AVB Lite. A device
  that shares its domain with no other, as an AVB device alone on a
  non-AVB switch, has an area of its own, and only those whose domain is
  not known sit apart. The map is the same whether this computer's
  entities run gPTP, AVB Lite or nothing, its choices always Sync, Audio
  and CRF.
- The connection matrix keeps talkers and listeners in different clock
  domains, as the network map shows them, from connecting: their cell
  says so and does nothing, and hiding what cannot connect hides them.
- Where this computer runs none of its own entities, the network map and
  `triib-cli network` tell what it is plugged into from the gPTP
  messages on its link: an AVB bridge's port, or a switch taking no part
  in AVB, which lets the endpoints' own peer delay requests through, AVB
  Lite's declaring themselves and others known by their clocks. Behind
  one it hangs under the card for the devices running AVB Lite there,
  rather than taking an endpoint for its bridge.
- Entities that send no notifications, as some ESP32 endpoints do not,
  stay current as other controllers change them: triib reads each
  entity's names, stream formats, sampling rates and clock sources again
  in turn, asks every second how the stream the inspector's Streams tab
  shows stands, and learns at once from the responses entities send
  another controller on this computer. An entity that sends triib a
  notification is asked nothing again until it goes or restarts.
- triib reads entities' logos, their own and their vendors', through
  Address Access: the entity list can show each in a column of its own,
  and the inspector's Entity tab shows both.

### Linux

- triib, triib-cli and triib-endpointd no longer hold `CAP_NET_RAW`:
  `/usr/lib/triib/triib-netraw`, the one program the packages grant it,
  opens their raw sockets and passes them over. The desktop's portals
  answer triib directly again, and debuggers and crash reports see it as
  any other program.
- triib-endpointd shares its MSRP participant with other programs on
  this computer, such as plaa sending what it plays, which ask it on a
  socket in the user's runtime folder to declare their talkers and to
  listen to streams for them, and hear from it as their listeners and
  the talkers of the streams they listen to come and go, this computer's
  entities among them, which take those streams as they take their own;
  it withdraws a program's declarations when the program goes. It says
  hello to each program with its version and features, and answers each
  request to declare a talker, so a program can tell what it can do and
  why it refused. In AVB Lite it declares for the programs that run it
  too, by CVU SRP in their entities' names, admitting their listeners
  with its own entities' and telling them where to send each copy.
- Two of triib-endpointd's inputs on one stream no longer withdraw each
  other's listener declaration as one of them stops: a stream has one,
  for all its listeners on this computer, in AVB Lite too.
- This computer's entities carry triib's app icon as their logo and
  Scramble Tools' as their vendor's, PNG memory objects that controllers
  such as Hive read through Address Access.
- triib-endpointd rides out a fault of ptp4l's own, as a transmit time
  stamp coming late while the computer is busy, rather than falling
  back to AVB Lite as though no gPTP peer answered.
- triib-endpointd keeps its entities off AVB Lite on an interface
  without a PTP hardware clock, which AVB Lite needs: it no longer falls
  back to it there, says it can run it, or takes the link down to turn
  EEE and PAUSE off for it.
- triib starts triib-endpointd again whenever it stops while triib runs
  and this computer's entities are listed, as after a crash, not only
  as triib opens, waiting longer after each start that does not last;
  the log of the run before is kept as `endpointd.log.1`. A daemon triib
  started no longer lingers as a zombie once it stops, which triib took
  for one still running.
- An input of triib-endpointd's that stops listening while locked to its
  stream counts an unlock, so a controller reading its counters no
  longer takes it for still receiving.
- This computer's entities answer for their AVB interface's counters:
  how often its link came up and went down, and how often its gPTP
  grandmaster changed, as Milan has a controller ask.
- In AVB Lite, triib-endpointd hears other computers' CVU SRP on an
  interface that drops frames of unregistered VLANs in hardware, as the
  Intel I210 does: it holds the interface promiscuous while AVB Lite
  runs, where before it heard them only while one of its inputs
  listened.
- In AVB Lite, a talker of triib-endpointd's serves up to 5 listeners
  unicast unless `unicast_fanout` says otherwise, as the profile now
  recommends, where before it served 2.
- triib-endpointd's streams offer the same formats as plaa's, from avb's
  shared set: each of a stream's sizes at every rate where it fits a
  frame, as the Extended Audio profile asks, where before a size went
  only at the rates its stream's own channels fit. A stream of many
  channels now offers 96 and 192 kHz at the smaller sizes, and a new
  rate moves a stream too large for it to the most channels of its
  packing that fit. AM824 is declared as MOTU's 8D declares it, of its
  channels or fewer; a stream saved in the old AM824 comes back in the
  new one.

### Packages

- triib's app ID is `run.triib`, triib.run reversed, as registered with
  Apple, in place of `io.github.scrambletools.triib`: the Mac bundle and
  its BPF launch daemon, the desktop file, AppStream ID, icons and
  Wayland app ID. The Mac package removes the launch daemon and receipt
  earlier releases installed under the old ID, and the AppStream data
  says it replaces it. macOS asks again once for the microphone and
  audio capture, which it grants by bundle ID.

### Development

- The AVB crates moved, with their history, to
  [scrambletools/avb](https://github.com/scrambletools/avb), to be shared
  with other apps: `atdecc`, `avb-mrp`, `avb-net`, and `triib-stream` as
  `avb-stream`. They are published on crates.io, where triib takes them
  from.

## [1.1.1] - 2026-10-17

A close button in the toolbar under compositors that draw no title bar,
and fixes for computers that 1.1.0 did not start on or listed too many
interfaces on: a Windows without the Visual C++ runtime, Linux without a
Vulkan driver, and a Mac's Thunderbolt Bridge and USB-C interfaces. Every
package is now tested in clean systems before each release.

### Controller

- On Linux, under a compositor that draws no title bar, Hyprland, niri,
  river or dwl, the toolbar ends with a button that closes the window.
  `SCRAMBLE_UI_TITLE_BARS=0` shows it elsewhere, such as on sway set up
  without title bars, and `SCRAMBLE_UI_TITLE_BARS=1` hides it.

### Fixed

- On Windows, triib and triib-cli carry the C runtime they were built
  with, so they start on a Windows without the Visual C++ runtime, such
  as a fresh install, where 1.1.0 could not.
- On Linux without a Vulkan driver, triib draws with its software
  renderer rather than OpenGL, which made it quit as it opened under
  some compositors, such as one without a GPU on Debian 13 or Ubuntu
  26.04. `WGPU_BACKEND=gl` still asks for OpenGL.
- On a Mac, the interfaces without an Ethernet port of their own,
  Thunderbolt Bridge's and those of USB-C ports being USB network
  devices, count as virtual, so the interface picker leaves them out
  with the others; an M4 Mac mini listed six of them.

### Packages

- Every package is installed, upgraded from the last release, run and
  removed in clean systems before each release: fresh Debian, Ubuntu,
  Fedora and Arch containers, a Windows container and GitHub's Windows
  and macOS runners.

## [1.1.0] - 2026-10-16

The Streams tab with peak meters, max transit times, media clock streams
on host entities with one chosen as their clock source, and host talkers
and listeners folded into host entities. On a Mac, the Mac's own AVB
entity as a host entity, with its configurations, and Listen controls
for what it receives and sends. The macOS app and installer are signed
with Scramble Tools' Developer ID and notarized.

### Controller

- The Streams tab lists an entity's streams, outputs then inputs, five
  before it scrolls, and shows the picked one's details under the list:
  its name, format, state, stream ID, destination and VLAN, and for this
  computer's entities its audio and channels. The Entity tab no longer
  shows streams.
- Each stream in the Streams tab's list has a peak meter of its first two
  channels before its format, green, yellow from -12 dBFS and red from
  -6 dBFS, wherever this computer carries its audio: its own entities'
  streams, and other entities' streams connected to them.
  triib-endpointd measures its entities' streams and writes them 20
  times a second while the app shows them. On a Mac, a helper,
  `triib --meters`, measures the Mac's own entity's from its Core Audio
  device: what it receives from the device's input, what apps send it
  through a Core Audio tap. The list's rows line up with the inspector's
  headings and fields, and their formats with each other.
- An output's max transit time is picked in its details, from 0.25 ms to
  50 ms, on any entity that answers GET_MAX_TRANSIT_TIME, Milan or not,
  the Mac's AVB entity among them. A running stream's refusal says to
  disconnect it first.
- While a media clock stream is picked, the Streams tab leaves out the
  entity's channel mappings, which are its audio streams'.
- The matrix remembers which entities are folded, across restarts.
- An entity with several configurations has a configuration picker,
  each by its own name, read from its own strings, or by what it holds
  where names are missing or repeat: "16 in, 16 out, 8 ch streams".
  `triib-cli configuration` changes it, and `triib-cli describe` lists
  them.
- A stream output's details list its listeners where its talker keeps
  them, as talkers outside Milan do, those not online marked, each with
  a button that disconnects it; `triib-cli streams` lists them too.
- On a Mac, the Mac's own entity has Listen controls: a channel or a
  pair of what it receives, or of what apps send to one of its output
  streams, heard through a Core Audio tap (macOS 14.2 and later), played
  through an output picked, with a gain.
  The audio runs in a helper, `triib --monitor`, started again when it
  ends or hangs, which waits for a device that is not there, as the
  Mac's is not while none of its streams is connected; the choice is
  kept. Agents set it through the `set_audio_monitor`, `audio_monitor`
  and `list_audio_devices` tools. macOS asks once for the microphone,
  to hear what the Mac receives, and for audio recording, to hear what
  it sends. Before macOS 14.2, which has no taps, the controls offer
  what the Mac receives alone and say why.
- On a Mac, the Mac's own AVB entity shows as a host entity, with its
  Core Audio device, which macOS makes only while one of its streams is
  connected, and a button that opens Audio MIDI Setup; where AVB is
  off, the Entities view says where to turn it on.
- Controllers that do not advertise, as the Mac's own does not, are
  listed from the commands triib hears them send: their connection
  commands, which reach every station, and their commands to this
  computer's entities. Each is named for the computer it runs on, as
  "Controller on Mac mini", and stays listed for 5 minutes after its
  last command.
- The network view leaves out controllers, which take no part in the
  clock or the streams, and shows the local host as its host entities
  where there is one, rather than as This computer beside them.
- Controllers have an icon of their own, a traffic light, rather than
  the host's. triib's controller, which has no model to read, is
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
- The entity list's column and the inspector's field for the gPTP
  grandmaster an entity advertises say Grandmaster, rather than BTC, as
  does `triib-cli discover`; a column kept as `btc` in the settings
  loads as it.
- The inspector is titled by the picked entity's name, cut short to
  leave the close button its room.
- In the inspector, a value short enough to sit beside its label does:
  a rate, a channel count, a format, or a picker whose widest choice
  fits. Longer ones stay under their labels.
- The entity list's rows are 40 pixels tall rather than 56, its Name
  heading lines up with the names rather than their icons, and the line
  after the names ends with the last row.
- Each view has the same 16 pixel margin at the window's sides, below
  the toolbar and above the status bar; the matrix's sides were 24.
- On a narrow window, the matrix's Hide what cannot connect shrinks to
  its icon, its label a tooltip, and the network view's "Showing …
  streams" goes below the map, so neither moves the view down.
- Every scroll bar is as thin as the matrix's; the stream list's was
  thicker.
- The Streams tab marks an output with an arrow out of a circle,
  pointing right, and an input with one into a circle, pointing left.
- Settings and Presets open at once, rather than growing in.
- On macOS, the toolbar's color runs up behind the title bar, which is
  transparent, rather than the system's tinted bar with a seam under
  it; a double-click there zooms or minimizes as the system's setting
  says.

### Host entities (Linux)

- Installing triib is all a host entity takes on an interface with a
  PTP hardware clock: where no ptp4l answers for the interface, the
  daemon starts triib's `triib-ptp4l-gptp@<interface>` unit itself,
  which the polkit rule lets it, rather than waiting for one to be
  enabled by hand. A ptp4l run another way on the interface stays as it
  is, and where linuxptp is missing the daemon says to install it.
- triib opens with the host entities listed for its interface back on
  the network, starting the daemon, as after a restart; it started only
  as one was added or a preset recalled.
- A host entity whose ptp4l clock is more than a millisecond off the
  grandmaster's reports itself not asCapable, so controllers show it out
  of sync, and the daemon says how far off it is; it reported asCapable
  however far off. Out of sync for 10 s, as when a driver reset the
  hardware clock, triib's own ptp4l unit starts again, ptp4l stepping
  the clock as it starts.
- A stream's report says how many frames its socket dropped, its thread
  behind, telling frames lost here from frames lost on the way, and the
  socket holds about a tenth of a second of a stream rather than a few
  hundredths, where the system allows.
- Every audio stream takes 2 and 8 channels as well as its own count, as
  the AVB community's Basic Audio profile has every talker and listener
  do, and the profile's 16 and 32 below a larger stream's own, at its
  own rates, so a stereo talker and an 8-channel input meet once a
  controller picks the format both take. A talker sends its audio's
  channels as the stream's first, with silence in the rest; a listener
  plays the stream's channels from the first, with silence where the
  stream has fewer. A format chosen so is kept. The entities' model IDs
  change, so controllers read them again once.
- An output's SR class follows its format: class B's own formats, a frame
  each 250 µs, send it in class B, and the others in class A, so a
  controller picks the class by picking the format, and a class B stream
  is never Milan's frames sent two to an interval. Its max transit time
  moves from one class's default to the other's, where a controller set
  none. `class = "B"` in `entities.toml` starts an output in class B's
  formats. Every stream's descriptor says which classes its formats go
  in, and an entity with audio streams advertises class B support.
- A change of sampling rate keeps a stream in its SR class: class B's own
  format moves to class B's own at the new rate, not to Milan's.
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
  host entities.
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

### Packages

- The Arch package depends on linuxptp and ethtool, as the Debian and
  Fedora packages recommend them, which those systems install by
  default; installing has systemd read triib's units again.
- The macOS app says why it asks for the microphone and for audio
  recording, which Listen needs. `packaging/macos/bundle.sh` signs with
  the identity named in `TRIIB_SIGN_IDENTITY`, from the keychain in
  `TRIIB_SIGN_KEYCHAIN`, rather than ad hoc where it is set: an ad hoc
  signature changes with every build, and macOS then asks for those
  permissions again.
- The macOS app and package are signed with Scramble Tools' Developer ID
  certificates and the package is notarized, so Gatekeeper opens them
  without Open Anyway and macOS keeps Listen's permissions across
  updates. Both programs run under the hardened runtime, the app with
  the microphone entitlement. The release workflow signs, notarizes and
  staples, then fails unless the app and package are signed by the
  identities its variables name.
- The installers say when this computer cannot be an AVB endpoint. The
  Linux packages check for a wired interface with a PTP hardware clock
  as they install and, finding none, say so with the Ethernet cards that
  have one: Intel's I210, I225 and I226 and Marvell's AQtion AQC107, and
  no Wi-Fi card yet. The macOS installer warns as it opens when the Mac
  has no AVB Ethernet port, and the Windows installer's last page says
  that this computer cannot be an AVB endpoint on Windows yet.
- The packages, the installer and the AppStream metadata link to the
  repository where it lives now, github.com/scrambletoolsllc/triib, and
  the metadata to the website, triib.run, with its screenshots and the
  host entities, AVB Wireless and AI agents among the features.

### Fixed

- Host entities hear their streams on network cards that filter VLANs in
  hardware, as Intel's I210 does: the card dropped every stream frame,
  tagged with its VLAN, unless that VLAN was registered with it, which
  takes an administrator. A stream's socket takes the interface
  promiscuous there, which the packet socket's own privilege allows.
- The packaged ptp4l configurations step the clock when it is a second
  or more off. igb resets an I210's hardware clock when its link goes
  down and up, as triib-link does to turn EEE and PAUSE off, and ptp4l
  then slewed for good, decades off, still reporting itself in sync.
- An entity advertising a new gPTP grandmaster or domain has its gPTP
  state and path read again, so one read while it was its own
  grandmaster joins the network view's tree.
- The AVB Lite offset alarm shows only where an interface runs AVB Lite,
  not where it is only capable of it, as an AVB Wireless station is.
- An entity running AVB that could run AVB Lite shows "AVB (AVB Lite
  capable)".
- Adding or removing a host entity leaves the others
  running, their stream addresses as they were.
- Host entities show as the host's from the moment they advertise.
- A refused SET_MAX_TRANSIT_TIME no longer shows the time asked for.
- A stream's own SR class sets its priority and its reservation, so an
  entity's class B stream is reserved as class B.
- A preset moved to another computer renames the talkers its entities'
  inputs are bound to.
- An entity lists each sampling rate once.
- Host entities tell a talker when an input lets go of its
  stream: unbound, bound elsewhere, removed or leaving with its entity.
  A talker outside Milan, such as the Mac, kept the connection a probe
  made and streamed on to a listener long gone.
- On Linux, the links triib opens open: the desktop portal refused
  them, as it does any app holding a capability, as triib holds
  `cap_net_raw`. triib now asks it from a helper without one.
- A Mac's streams no longer read as failing their reservation, "for an
  unknown reason". macOS marks a stream's MSRP failure fields valid
  while they hold no failure, with failure code 0, and triib took the
  mark alone for a failure; a failure now needs a failure code.

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
