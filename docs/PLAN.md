# triib: design and plan

triib is a lightweight ATDECC (IEEE 1722.1-2021) controller with first class
Milan 1.3 and AVB Lite support, for Linux, Windows and macOS, in the spirit
of Hive. It discovers entities, shows and edits their entity model, connects
streams, manages media clocks and diagnoses problems. On a Linux computer
with a hardware timestamping Ethernet interface it also runs talkers and
listeners of its own, routed to the computer's audio, which appear in the
connection matrix like any other entity. Written in Rust with iced, sharing
prev's look, widgets and window layout through scramble-ui.

## Status

- Done: the controller on Linux, macOS and Windows (P0, P1, P2), in 38
  languages (P1.1), and this computer's own talkers and listeners on
  Linux, on AVB and AVB Lite networks (P3). Released as 0.9.0 on
  2026-10-10, with packages for Linux on x86_64, ARM64 and RISC-V,
  Windows on x64 and ARM64, the programs and installer signed, and macOS
  on Apple Silicon; the website is triib.run.
- Next: P3.1, the AVB Wireless profile and triib's view of wireless
  endpoints and bridges; then P4, whether Windows and then macOS can run
  talkers and listeners of their own, and this computer's entities with
  several streams.
- Waiting on others: the linuxptp organization TLV tables, the atlantic
  driver's time stamps, and native speakers' review of the translations.

## Guiding priorities

1. Correct on the wire: every frame we send is checked against 1722.1-2021,
   Milan 1.3 and the AVB Lite profile, and every frame we parse is tested
   against captures from real devices.
2. Small footprint: few dependencies, no async runtime, idle CPU near zero.
3. One portable core: protocol logic is plain Rust shared by every platform;
   platform code is confined to a thin layer.
4. Features follow capabilities, not operating systems: a feature is
   available wherever a backend can provide what it needs, and the
   architecture must never rule a platform out of one.

Hive is the reference for controller features; prev for look, layout and
code style; L-Acoustics avdecc is reference and test oracle, not a
dependency.

## Decisions

| Area | Decision |
|---|---|
| Language | Rust stable, edition 2024, same toolchain floor as prev (1.95) |
| GUI | iced 0.14, with scramble-ui's vendored patches, as prev |
| Design | Material 3 Expressive, with Omarchy's or the system's accent, from `scramble-ui`, the crate prev and triib share |
| License | MIT OR Apache-2.0 (see Licensing) |
| Devices | Milan entities first class; plain 1722.1 entities get what they offer |
| Virtual endpoints | One ATDECC entity per talker or listener; from P4 also entities with several stream inputs and outputs, beside them |
| Controller entity | triib advertises itself as a controller (valid time 62 s, entity model ID `0x8c1f6436c0000001` under the Scramble Tools MA-S `8C-1F-64-36-C`), answers CONTROLLER_AVAILABLE, and registers for unsolicited notifications from each entity it reads |
| ATDECC | Our own Rust stack, controller and entity roles, in the reusable `atdecc` crate, see below |
| PTP | linuxptp's ptp4l on Linux, in gPTP or the AVB Lite PTP profile, run by triib's systemd units; the daemon reads it through its read-only socket. Our own engine (`avb-ptp`) only where P4 shows a platform needs one |
| SRP | Our own MRP, MSRP and MVRP in the reusable `avb-mrp` crate; MAAP and AVB Lite's CVU SRP framing in `atdecc`, its attribute lists in `avb-mrp` |
| Concurrency | No tokio. Network thread per interface with a poll loop and timers; real time threads for streaming and audio; `std::sync::mpsc` into an iced subscription (prev's `External` + `post()` pattern) |
| Processes | GUI process (controller) and `triib-endpointd` for this computer's talkers and listeners, so audio survives GUI restarts |
| Platform layer | `avb-net`: raw Ethernet frames, interfaces and hardware timestamps per operating system, the only platform code the protocol crates use. Audio (cpal) and the media clock live in `triib-stream` |
| Settings, cache, presets | TOML settings through `triib-store`; the entity model cache keeps raw descriptor bytes, decoded by `atdecc` on load, so the protocol crates need no serde; TOML presets in the data folder |
| Interface text | Fluent files under `i18n/`, 38 languages, as prev |
| Packages | .deb, .rpm, AUR, tarball, macOS .pkg, Windows .msi and .zip. No Flatpak or AppImage: raw sockets cannot be granted to either |

## Our own ATDECC stack

L-Acoustics avdecc, which Hive uses, would bring years of field quirks, but
it would gate triib's core: this computer's entities, AVB Lite's CVU SRP
and status query, and vendor extensions would mean patching or forking a
C++17 library behind FFI, under the LGPL. Its focus is the controller, so a
Milan talker or listener (AEM responder, listener binding, MVU) would be
our code anyway. Quirks are met instead with captures from every device on
the bench, Hive beside triib as an oracle, and fuzzing the codecs.

## Reusable protocol crates

The protocol implementations are crates of their own, meant for other
applications with very different constraints (a blocking CLI, an async
daemon, a GUI, firmware on a microcontroller), not only for triib:

- **`atdecc`**: IEEE 1722.1-2021 with Milan 1.3, MAAP and the AVB Lite
  vendor unique messages: frame codecs, controller state machines
  (discovery, AECP command tracking, enumeration, ACMP, unsolicited
  notifications) and entity state machines (AEM responder, Milan listener
  binding, ACMP talker), and the entity model as data.
- **`avb-mrp`**: IEEE 802.1Q MRP (applicant, registrar, LeaveAll, periodic
  timers), MSRP and MVRP. MSRP attribute lists are also usable outside
  MRPDUs, for AVB Lite's CVU SRP.
- **`avb-net`**: raw Ethernet per operating system: interfaces, frames,
  multicast membership, hardware timestamps, link settings. The only crate
  with unsafe code, and the only dependency of the two above.
- Perhaps later, on the same rules: `avb-ptp` (gPTP, the AVB Lite PTP
  profile and its fallback detection), and an AVTP streaming crate.

Rules for these crates:

1. **Sans-I/O core.** No sockets, threads, clock or runtime in the protocol
   code. The caller feeds received frames and the current time
   (`handle_frame(now, bytes)`, `handle_timeout(now)`) and drains frames
   to send, events and the next deadline (`poll_transmit`, `poll_event`,
   `poll_timeout`). Commands return an ID and complete as an event. This
   fits a blocking loop, tokio, embassy or a firmware main loop alike.
2. **`no_std`.** Codecs need neither std nor alloc: fixed-size PDUs decode
   into plain values, variable ones as views over the received bytes, and
   both encode into the caller's buffer. Reserved values are kept, so a
   frame decodes and encodes back unchanged. State machines need `alloc`
   only. The `std` feature (default) adds the `avb-net` transports and a
   small blocking driver for simple apps.
3. **No dependencies beyond `avb-net`**, and `avb-net` only depends on
   what its platform needs (`libc` on Linux and macOS, `pcap` on Windows).
   No serde, no logging framework, no async runtime.
4. **Never panic on input.** Malformed frames are errors, fuzzed in CI.
5. **No knowledge of each other.** Glue between protocols, such as CVU SRP
   (`atdecc` framing around `avb-mrp` attribute lists) or MSRP state in a
   Milan stream input's flags, lives in the application.
6. **Enforced in CI**: each crate tested on its own, and built with
   `--no-default-features` for `riscv32imac-unknown-none-elf` (the ESP32-C6).
7. **Published** to crates.io under these names once the API settles, with
   their own semver; they stay in this workspace until outside users need
   them elsewhere.

## Workspace layout

```
crates/
  atdecc          reusable: ATDECC frames, controller and entity machines
  avb-mrp         reusable: MRP, MSRP, MVRP
  avb-net         reusable: raw Ethernet, interfaces, timestamps per OS
  triib-store     settings and cache files
  triib-stream    AAF and AM824 talkers and listeners, the media clock,
                  audio through cpal, drift and resampling
  triib-endpointd daemon running this computer's talkers and listeners
  triib-cli       headless controller
  triib           the iced app, including the entity store the GUI draws
```

## Platform layer

| Capability | Linux | macOS | Windows |
|---|---|---|---|
| Controller frames (`0x22F0`) and listening to gPTP | `AF_PACKET`, bound to every protocol with a BPF filter, so it hears other programs' frames | BPF, one device per socket | Npcap |
| Hardware timestamps, PTP | ptp4l on the PHC (`/dev/ptpN`) | The system's own gPTP; open (P4) | Not available to us yet; open (P4) |
| Paced transmit | User space pacing on the PHC's time | Open (P4) | Open (P4) |
| Audio | cpal (PipeWire, ALSA, JACK) | cpal (Core Audio) | cpal (WASAPI) |
| What an interface can do | `ETHTOOL_GET_TS_INFO`, the PHC index, link speed | `getifaddrs`, `SIOCGIFMEDIA` | `GetAdaptersAddresses` |

`avb_net::check_access` says whether raw Ethernet is ready and, when not,
why: no `CAP_NET_RAW`, no access to `/dev/bpf*`, or Npcap missing or for
administrators only. The app shows the fix. This computer's talkers and
listeners are offered per interface where it reports a PTP hardware clock
and a wired link.

On Linux the app, `triib-cli` and `triib-endpointd` need `CAP_NET_RAW`,
which the packages set; the PHC is readable by everyone under systemd's
rules. ptp4l runs as root under triib's units, which a polkit rule lets the
person at the computer start and stop, as the daemon does.

## PTP

On Linux, ptp4l disciplines the interface's PHC and `triib-endpointd` asks
its read-only socket (`/var/run/ptp4lro`) every 2 s for each entity's
GET_AVB_INFO, GET_AS_PATH and counters. triib's systemd units run it:

- `triib-ptp4l-gptp@<interface>` and `triib-ptp4l-lite@<interface>`, each
  stopping the other, for gPTP and the AVB Lite PTP profile (end to end,
  L2, domain 0, delay requests multicast). The daemon moves ptp4l between
  them as its endpoints fall back to AVB Lite and as the link comes up
  again; a ptp4l run any other way it leaves alone.
- `triib-link@<interface>` keeps Energy-Efficient Ethernet and PAUSE off
  the link at boot, as AVB Lite asks; the daemon runs it again should they
  come back.
- `triib-link-reset@<interface>` takes the interface down and up when the
  daemon sees received PTP frames keep the driver's time stamp, as the
  atlantic driver leaves them after the link renegotiates.

The fallback follows the profile's 2.2 to 2.5: weighed from each link-up,
paused while the link is down, from the Endpoint Declaration TLV, nine
unanswered requests or two responders. Where ptp4l already runs the AVB
Lite profile it listens instead: one gPTP peer asking without the TLV, as a
bridge asks every second, moves ptp4l to gPTP at once while it keeps
listening, and a beacon then brings it back without waiting. The media
clock corrects for the grandmaster's link speed from its Grandmaster Link
TLV.

This computer's card is a TP-Link TX401 (Marvell AQtion AQC107, Linux
`atlantic`, firmware 3.1.100): hardware time stamps, the PTP v2 L2 event
filter, two-step, PHC `ptp0`. Its driver mishandles received time stamps:
it leaves them on multicast PTP frames after a renegotiation, which the
daemon works around, and cuts 12 octets from unicast PTP frames, so delay
requests stay multicast. Both are reported to the driver's maintainer. Its
PHC now and then reads 2^32 ns off, a torn read, or about 167 us off for a
tenth of a second, so the media clock takes the median of a burst of
readings and follows a jump only once three measurements in a row show it.
Intel's i210, i225 and i226 are the known choices with launch time.

## Virtual endpoints: this computer's talkers and listeners

- Added and removed in the Entities view, on interfaces with a PTP
  hardware clock and a wired link; the inspector picks each one's audio
  device, channels and format. They show as this computer's in the list,
  the matrix and the network view, and presets keep them.
- Each is a Milan entity with an entity ID from the interface's address
  and an instance number, advertised on the wire, so Hive and other
  controllers see and bind them too: one stream of AAF or AM824 at 48, 96
  or 192 kHz, 8 channels unless another count is picked.
- `triib-endpointd` runs them from `endpoints.toml`, which the app writes
  and the daemon reads again when it changes, keeping the names, formats
  and bindings controllers give.
- Audio to and from any cpal device, a test tone or nothing. The device's
  clock is not the network's, so the resampler follows its drift, and the
  listener plays each sample at its presentation time, or a fixed delay
  after it where the device cannot keep up.
- Streams are paced in user space on the PHC's time, the threads under
  RealtimeKit's real-time scheduling.
- On an AVB network they reserve through MSRP and MVRP; on AVB Lite they
  declare with CVU SRP and send unicast to each listener, to a fan-out
  that is configurable, multicast beyond it only where allowed.

## AVB Lite

Per the AVB Lite profile in avbcommunity/profiles:

- As a controller, triib asks each entity GET_LITE_STATUS and hears CVU SRP
  declarations, and shows how each interface runs: mode, why it fell back,
  PTP profile and domain, offset, media VLAN, unicast fan-out, link, and
  what its streams take of the link. Alarms for an offset past 50 us and
  egress past 75% show in Diagnostics and on the status bar. Where entities
  run AVB Lite and no AVB bridge is heard, the network view draws the PTP
  tree, each device in sync or not.
- Its own endpoints do all of the profile's endpoint side: the fallback and
  beacon, the Endpoint Declaration TLV, CVU SRP, admission against their
  link, escalation to multicast only where allowed (SET_LITE_CONFIG, or
  `endpoints.toml`), the media VLAN, the link-speed correction, and
  GET_LITE_STATUS answers.
- The profile's informative 2.5 gives the start-up sequences that triib's
  fallback follows.

## Licensing

triib is MIT OR Apache-2.0. Things to keep that true:

- **scramble-ui**, shared with prev, is MIT OR Apache-2.0; prev stays AGPL
  and uses it.
- **Dependencies** are permissive (MIT, Apache, BSD, 0BSD); MPL-2.0 is fine
  as file level copyleft used unmodified. triib's `deny.toml` allows only
  those.
- **System libraries** loaded dynamically: libasound and JACK (LGPL) through
  cpal stay dynamically linked; libpcap is BSD.
- **Npcap** (Windows) is proprietary and may not be redistributed without an
  OEM license: users install it themselves, as with Hive and Wireshark.
- **Avoid**: GPL or AGPL crates, and statically linked LGPL.
- **Reference code**: la_avdecc (LGPL-3.0) and Hive (GPL) are for
  behavior, never copied; the IEEE and Avnu specifications are the source
  for implementation. Do not paste standard text into code or docs beyond
  field names and short citations.
- **Fonts**: Roboto Flex (OFL-1.1) and Material Symbols (Apache-2.0) ship
  with their license files; the vendored iced crates carry iced's MIT
  notice.
- **Trademarks**: "Milan" and "AVB" belong to Avnu Alliance. Say "Milan
  compatible", never "certified", unless certified.
- **Why dual**: the Rust convention, and Apache adds an explicit patent
  grant from contributors, which matters for a protocol implementation.

## Window layout

- **Toolbar**: interface picker with link and timestamping capability,
  the views (Connections, Network, Entities), asking every entity to
  announce itself, search, presets, log, inspector and settings. Tools
  that do not fit move into a "More" menu, as in prev.
- **Content**: the active view, Connections by default. Each view selects
  entities its own way (the matrix's headings, the entity table, the
  network's cards); there is no entity sidebar.
- **Right inspector**: the selected entity in tabs (Entity, Streams,
  Controls, Diagnostics, Descriptors); for this computer's endpoints also
  their audio and stream state.
- **Log**: a panel under the view, resized from its top edge.
- **Status bar**: the interface and its state, AVB Lite alarms, the entity
  count and the controller's entity ID.
- **Narrow windows**, down to a phone's width: the toolbar keeps the
  interface picker and the inspector button with the rest under "More",
  the inspector takes the view's place until closed, tables scroll
  sideways, and the network view shows its map or its panel.

## Use cases

### P0: controller daily driver (done)

Interfaces and live discovery; the entity list with columns picked,
moved, resized and kept; the connection matrix; identify; the inspector's
tabs; names, stream formats, sampling rates and clock sources edited;
unsolicited notifications; the entity model cache, so a known model is
read again only where entities differ.

### P1: Hive parity and AVB Lite control (done)

The media clock in the entity list; channel mappings and controls in the
inspector; diagnostics from each entity's counters and each bound input's
reservation; the network view with the gPTP tree, audio and CRF streams on
wires of their own; the log of every ATDECC frame with the rule breakers
marked; AVB Lite status and alarms; presets; and `triib-cli` doing the
same from scripts.

### P1.1: every language prev speaks (done)

triib's text in prev's 38 languages, as Fluent files, following the system
or picked in Settings, right to left languages mirrored; the terms follow
`docs/GLOSSARY.md`, with each language's choices in `docs/glossary/`.

- To do: review by native speakers.

### P2: macOS and Windows, everything but virtual endpoints (done)

The controller through BPF on macOS and Npcap on Windows, everything of P0
and P1 on both, and packages with what each system needs set up. The Mac's
own AVB entity cannot be read from the same Mac, which triib says.

- To do: try the macOS package's install on a Mac (it needs an
  administrator's password); its contents, scripts and app are checked.

### P3: virtual endpoints on Linux (done)

`triib-endpointd` with MSRP, MVRP, MAAP and CVU SRP, ptp4l's two profiles
switched by the daemon, AAF and AM824 talkers and listeners routed to the
computer's audio, shown in the matrix and inspector, kept in presets.
Checked against Milan endpoints, macOS's AVB listener, and an AVB Lite ESP
endpoint through a non-AVB switch.

- To do:
  - The linuxptp organization TLV tables, as Erez Geva prefers a table or
    a built-in option, then the Endpoint Declaration TLV from ptp4l in
    gPTP mode: today it goes out only while ptp4l does not run gPTP on
    the interface, as the daemon's own requests and beacons.
  - The atlantic driver's time stamps, reported to its maintainer and
    netdev: answer the thread, and drop the workaround once a fix is in
    the kernels triib's users run.
  - Sinc resampling (`rubato`) if cubic ever falls short.

### P3.1: AVB Wireless (next)

The AVB Wireless profile (avbcommunity/profiles, `avb_wireless.md`)
extends AVB and AVB Lite over one 802.11 hop: an access point that is a
bridge on its wired port, and Wi-Fi stations as talkers and listeners,
timed by 802.1AS over FTM or, in the interim, by the grandmaster's time
in the access point's beacons, and streamed unicast on the air.

- **The profile** (1.1-draft): done. Two time modes that never combine,
  Mode A, 802.1AS over FTM or TM, preferred, and Mode B, the beacon
  carrier with FTM for link delay only, where a platform cannot run
  Mode A, with what the ESP's work measured; the beacon time element identified
  by the full MA-S and sub-ID `0x006`; and a first-pass discovery and
  status query, GET_WIRELESS_STATUS under sub-protocol `0x005`, answered
  per wireless interface by stations and by access points that run an
  ATDECC entity. It reports the role, PHY, link, time mode and how well
  it holds, the access point a station follows (BSSID and gPTP port
  identity), and on an access point its stream translation counters,
  the listeners it cannot serve and its stations, with an unsolicited
  response on change. SET_WIRELESS_CONFIG only sets the Class A bench
  opt-in.
- **The ESP:** the beacon element's new identifier, and answering the
  status query.
- **triib, as a controller:** done. atdecc asks each AVB interface once
  an entity is read and the wireless ones every 5 s, and follows
  unsolicited responses. The inspector's Wireless section, an entity
  list column, each station under its access point in the network view
  with its hop dotted and in sync while locked, alarms for a station not
  locked and listeners an access point cannot serve, the log, and
  `triib-cli wireless` and `wireless-config`.
- To do: check it against the ESP wireless station and access point once
  they answer the query.
- Not in P3.1: this computer as a wireless station, which needs its Wi-Fi
  card's own PTP clock and timing support; a later phase.

### P4: virtual endpoints on Windows, then macOS, and entities with several streams

- **Windows.** Windows reports no time stamps on the Intel I226-V and the
  Realtek RTL8111 here (`GetInterfaceSupportedTimestampCapabilities`
  answers ERROR_NOT_SUPPORTED), and its API names PTP over UDP only, so
  L2 gPTP would rely on its all-receive and tagged-transmit flags. Npcap
  does not pass on NDIS hardware time stamps yet (its issue 581). Intel's
  I210 driver (e1r 14.1.24.0) keeps the time sync interface Avnu's gPTP
  daemon used: the hidden `TimeSync` setting beside
  `*PtpHardwareTimestamp`, and the private OIDs for the transmit and
  receive stamps and the clock. An M.2 I210 card, ordered for the
  Windows PC, is the next step; gPTP, SRP and paced transmit on it decide
  whether Windows gets `triib-endpointd`, and whether it needs our own
  PTP engine.
- **macOS.** Whether user space can read the system's gPTP time and its
  relation to the card, or get hardware time stamps at all; if not, the
  system's own AVB audio device, controlled by triib like any entity.

#### This computer's entities with several streams

Beside today's endpoints, one stream each, this computer runs entities
with several stream inputs and outputs on one audio unit and clock
domain, as hardware presents itself: this computer as one device for a
DAW, say 8 in and 8 out at 48 kHz, with a single talker at 96 kHz or a
test tone beside it. Both kinds share the interface, the daemon and its
budgets: 240 instances, 75% of the link, and the real-time threads.

- **The entity.** One Milan entity: its streams at the audio unit's one
  sampling rate, each with its own format and channels; the clock
  sources its internal clock and each stream input. The entity role
  models this already: `EndpointModel` takes any number of inputs and
  outputs, with a cluster for each channel and a fixed audio map for
  each stream. Its stream IDs come from its instance and each output's
  index, as today's do.
- **endpoints.toml.** An entry gains `inputs` and `outputs`, 0 and 1 for
  a talker and 1 and 0 for a listener when not given, so today's files
  read as before; `kind = "device"` takes both, with one `source` for
  all its outputs and one `sink` for all its inputs. Stream k carries
  the device's channels from k × its channels on, in order.
- **Entity model IDs.** One for each shape, as the descriptors change
  with the counts: `0x8C1F6436C001` and then the inputs and the outputs,
  each an octet, such as `0x8C1F6436C0010808` for 8 in and 8 out, so
  entities of one shape share one cached model. Channel counts that
  differ change the configuration's descriptor counts, which the cache
  compares, so it reads such a model again rather than mixing them up.
- **The daemon.** One pacing thread sends every output's frame in turn
  each class interval, and one takes every input's, so an entity takes
  two real-time threads whatever its streams, against RealtimeKit's 25
  for a user; today's endpoints take one each. One audio stream each
  way: the talker side reads the device's channels and splits them into
  streams, the listener side merges its streams into the device's
  channels, each way with one drift estimate against the device's clock.
  Stream inputs from talkers on other media clocks each keep their own
  presentation times and drift correction; the clock source says which
  the domain follows.
- **Reservations.** As now, stream by stream: a Talker Advertise for each
  output and a Listener declaration for each input, through the port's
  one MSRP participant; addresses from the daemon's one MAAP range; in
  AVB Lite, CVU SRP and the unicast fan-out for each stream.
- **The app.** A third button in the Entities view's bar adds one,
  asking for its inputs and outputs. The inspector gives its audio in
  and out, the shared sampling rate and each stream's channels; the
  matrix shows its streams as it shows a hardware entity's; presets keep
  it. Changing its counts gives it another model, so it departs and
  advertises again, which controllers read as a new model.
- **Steps.** The daemon's endpoint from one talker or listener to any
  number of each, with today's endpoints as the case of one; then the
  file and the model IDs; then the app. Checked by binding several of
  its streams from Hive and from the Mac mini, with today's endpoints
  running beside it.
- **Open.** Controllers changing its channel mapping (Milan's dynamic
  mappings, ADD_AUDIO_MAPPINGS and REMOVE_AUDIO_MAPPINGS); a CRF stream
  input as the domain's clock source; streams at other sampling rates,
  which stay entities of their own.

### Later

- Pacing streams in hardware: launch time (SO_TXTIME with the etf qdisc)
  or CBS where the card has them.
- Unicast delay requests (ptp4l's hybrid_e2e) as an option, on cards that
  receive unicast PTP whole.
- The matrix showing AVB Lite transport: unicast, fan-out, escalated.
- Electing a media clock reference by priority and connecting the CRF
  streams to it.
- Switch ports on the network view, from LLDP tables over SNMP.
- Saving the log, and warnings for timing rules such as an
  ENTITY_DISCOVER answered late.
- Editing array and text controls, Bode plots, and meters drawn as bars;
  a grid editor for channel mappings with many channels.
- Reading the Mac's own AVB entity through the AudioVideoBridging
  framework.
- Firmware update over MVU, several interfaces at once, an MCP server, a
  CRF talker, PipeWire native nodes per endpoint.

### Not in scope

Dante, AES67, video streams, acting as an AVB bridge or AVB/Lite gateway.

## Footprint targets

- GUI binary under 20 MB; daemon under 10 MB (its release build is 1.3 MB,
  about 10 MB resident).
- Cold start to window under 300 ms.
- Idle CPU under 1% with 200 entities; memory under 100 MB.
- Virtual endpoint: under 5% of one core per 8 channel 48 kHz stream (4%
  to send, 3% to receive).
