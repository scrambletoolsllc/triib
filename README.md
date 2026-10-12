# triib

A lightweight ATDECC (IEEE 1722.1) controller with Milan and AVB Lite
support, for Linux, Windows and macOS. On a computer with a hardware
timestamping Ethernet interface it will also run its own talkers and
listeners, routed to the computer's audio. Built in Rust with
[iced](https://iced.rs), sharing its look with
[prev](https://github.com/scrambletools/prev) through
[scramble-ui](https://github.com/scrambletools/scramble-ui).

Website: **[triib.run](https://triib.run)**, with
[a tour](https://triib.run/guide.html) and
[the specifications](https://triib.run/specs.html).

> **Status:** 1.0.0. On Linux, macOS and Windows, triib discovers ATDECC entities, reads their
> entity models (names, streams, clocks, Milan support) and keeps them for
> next time, shows and changes stream connections in a matrix, maps the
> network from each entity's gPTP path, identifies entities, renames
> them, changes their stream formats, sampling rates and clock sources
> (each entity's media clock right in the entity list), shows and
> changes how their channels map to streams, shows and sets their
> controls, such as gain and mute, saves and recalls presets of all of
> that and the connections, logs every ATDECC frame sent and heard,
> marking those that break the rules, and shows how entities run AVB
> Lite, what their streams take of each link and the alarms the AVB Lite
> profile calls for, and what Wi-Fi stations and access points report
> under [AVB Wireless](https://github.com/avbcommunity/profiles/blob/main/avb_wireless.md),
> each station under its access point in the network view. On a Mac it
> reaches the Mac's own AVB entity too. AI agents can use it through
> `triib --mcp`. On Linux it runs talkers, listeners and devices of its
> own, Milan entities streaming AAF or AM824 at 48, 96 or 192 kHz to and
> from the computer's audio devices, each of a device's streams with its
> own virtual audio device. It speaks 38 languages, following the
> system's or the one picked in Settings. See the [plan](docs/PLAN.md).

## Installing

[Releases](https://github.com/scrambletoolsllc/triib/releases) have
packages for Linux on x86_64, ARM64 and RISC-V, Windows on x64 and ARM64,
and macOS on Apple Silicon, each setting up what triib needs to send and
receive raw Ethernet:

| System | Package | What it sets up |
|---|---|---|
| Linux | .deb, .rpm, PKGBUILD | `CAP_NET_RAW` for `triib`, `triib-cli` and `triib-endpointd` |
| Linux | .tar.gz | nothing: run `sudo setcap cap_net_raw+ep` on the three programs |
| macOS (Apple Silicon) | .pkg | access to `/dev/bpf*` for the user installing it, at every start, as Wireshark's ChmodBPF does; `triib-cli` in /usr/local/bin |
| Windows | .msi, .zip | nothing itself: install [Npcap](https://npcap.com) first, which the installer and triib point to when it is missing |

On macOS the Mac's own AVB entity cannot be read from the same Mac, as
the system never hands it the commands triib writes; triib says so.
[docs/RELEASING.md](docs/RELEASING.md) has the details.

## This computer's talkers and listeners

On Linux, the Entities view adds talkers and listeners of the computer's
own, which `triib-endpointd` runs, on a wired interface with a PTP
hardware clock (`ethtool -T <interface>` shows one). They need
[linuxptp](https://linuxptp.nwtime.org)'s `ptp4l` running gPTP on that
interface, as with its `configs/gPTP.cfg`, so the clock keeps the
network's time; triib reads ptp4l's state through its read-only socket,
`/var/run/ptp4lro`, and says when ptp4l isn't there. Each endpoint is a
Milan entity with one stream of AAF or AM824 at 48, 96 or 192 kHz, 8
channels unless the inspector picks another count, which controllers,
triib among them, bind like any other; the inspector picks the audio
device a talker sends from or a listener plays to. Talkers take their stream addresses
with MAAP, and the stream threads ask RealtimeKit for real-time
scheduling, as PipeWire does.

The endpoints are listed in `endpoints.toml` in triib's data folder
(`~/.local/share/triib` on Linux), which the app writes and the daemon
reads again whenever it changes; the names, formats and bindings
controllers give are kept in it, so a listener binds again after a
restart, and presets keep it too:

```toml
interface = "enp2s0"
ptp4l_socket = "/var/run/ptp4lro"

[[endpoint]]
kind = "talker"
instance = 0
name = "Host talker 1"
source = "default"   # or "silence", "tone", or an input's name

[[endpoint]]
kind = "listener"
instance = 1
name = "Host listener 1"
sink = "default"     # or "discard", or an output's name
```

Where no AVB bridge answers on the interface, the endpoints fall back to
[AVB Lite](https://github.com/avbcommunity/profiles/blob/main/avb_lite.md),
a working if degraded path over switches that are not AVB bridges: CVU
SRP declarations in place of MSRP, and streams unicast to each listener.
At the top of the file:

- `avb_lite = "off"` keeps them on AVB, and `"on"` puts them on AVB Lite
  from the start.
- `unicast_fanout` sets how many listeners a talker serves unicast, 2 if
  not given. A listener beyond that is refused with Talker Failed,
  unless `multicast_escalation = true` lets the talker move the stream
  to multicast, which non-AVB switches flood to every port; allow it
  only where the media VLAN is confined. Controllers can set it too
  (`triib-cli lite-config`).
- `media_vlan` sets the VLAN of AVB Lite's streams, 2 if not given, or 0
  for a priority tag only on switches whose VLANs cannot be set up.

ptp4l cannot switch to AVB Lite's PTP profile by itself. The Linux
packages install `triib-ptp4l-gptp@<interface>` and
`triib-ptp4l-lite@<interface>` units and a polkit rule; with ptp4l run
by `systemctl enable --now triib-ptp4l-gptp@enp2s0`, the daemon moves it
to the AVB Lite profile when its endpoints fall back, and back when the
link comes up again.

AVB Lite also asks for Energy-Efficient Ethernet and PAUSE off the
endpoint's link. The `triib-link@<interface>` unit turns them off with
`ethtool`; the ptp4l units pull it in, or
`systemctl enable --now triib-link@enp2s0` runs it at every boot. Where
it changes them on an interface that is up, it takes the interface down
and up again, as some drivers keep PTP from working after the change
otherwise. Should they come back on, as after the driver starts again,
the daemon runs the unit once more. Some drivers, atlantic's among them
(Aquantia cards), also keep their time stamps on received PTP frames
after the link renegotiates, as when a cable goes back in, and ptp4l
drops every frame until the interface goes down and up; the daemon sees
them and restarts the interface through `triib-link-reset@<interface>`.

The daemon runs MSRP and MVRP once for the port, for all its endpoints,
as MRP runs per port: two participants on one point-to-point port do not
hear each other's Leave, so one withdrawing VLAN 2 would take it from the
bridge while the other still needs it. So one daemon runs on an interface
whichever user starts it, and triib says when another user's holds it, or
when another program on this computer declares MSRP or MVRP on the port.

## Using triib with AI agents

`triib --mcp` serves the Model Context Protocol on stdin and stdout, so
an agent such as Claude Code can see and change the network through the
triib on your desktop, which it starts when it is not running. To add it
to Claude Code:

```sh
claude mcp add triib -- triib --mcp
```

The tools read what triib knows (its status, the entities, an entity's
streams, formats and clocks, the connections) and change what a click
would: connecting and disconnecting streams, setting a stream's format, a
sampling rate or a clock source, renaming and identifying. A change
answers once the device has, with what it said. The first time an agent
connects, triib asks whether to let it in; Settings lists the agents
allowed and has a switch to turn outside control off.

## Building

```
git clone https://github.com/scrambletools/triib
cd triib
cargo run -p triib          # the app
cargo run -p triib-cli -- interfaces
cargo run -p triib-cli -- discover <interface>
cargo run -p triib-cli -- describe <interface> [entity-id]
cargo run -p triib-cli -- network <interface>
cargo run -p triib-cli -- name <interface> <entity-id> <entity|group|type:index> <name>
cargo run -p triib-cli -- format <interface> <entity-id> <stream-input:N|stream-output:N> <hex>
cargo run -p triib-cli -- rate <interface> <entity-id> <audio-unit> <hertz>
cargo run -p triib-cli -- clock <interface> <entity-id> <clock-domain> <clock-source>
cargo run -p triib-cli -- clocks <interface>
cargo run -p triib-cli -- maps <interface> [entity-id]
cargo run -p triib-cli -- map <interface> <entity-id> <add|remove> <stream-port-input:N|stream-port-output:N> <stream:channel=cluster:channel>...
cargo run -p triib-cli -- controls <interface> [entity-id]
cargo run -p triib-cli -- control <interface> <entity-id> <control> <value>...
cargo run -p triib-cli -- connect <interface> <talker-id>:<output> <listener-id>:<input>
cargo run -p triib-cli -- disconnect <interface> <listener-id>:<input>
cargo run -p triib-cli -- disconnect-talker <interface> <talker-id>:<output> <listener-id>:<input>
cargo run -p triib-cli -- identify <interface> <entity-id> [seconds]
cargo run -p triib-cli -- streams <interface> <entity-id>
cargo run -p triib-cli -- wireless <interface> <entity-id>
cargo run -p triib-cli -- transit <interface> <entity-id> <output> [nanoseconds]
cargo run -p triib-cli -- descriptor <interface> <entity-id> <type:index>
cargo run -p triib-cli -- harvest <interface> <entity-id> [repeat]
cargo run -p triib-endpointd -- [--config <endpoints.toml>] [--interface <interface>]
```

triib uses [scramble-ui](https://github.com/scrambletools/scramble-ui)
at the revision `Cargo.toml` pins, as prev does. To change both together,
clone scramble-ui beside triib and build with
`cargo --config .cargo/scramble-ui-local.toml build`, a git-ignored file
pointing at that checkout (for clippy and test, put `--config` after the
subcommand):

```toml
[patch."https://github.com/scrambletools/scramble-ui"]
scramble-ui = { path = "../scramble-ui" }

[patch.crates-io]
iced_graphics = { path = "../scramble-ui/vendor/iced_graphics" }
iced_widget = { path = "../scramble-ui/vendor/iced_widget" }
```

Once the scramble-ui change is pushed, move the pin to its revision and
build once without the file, so `Cargo.lock` names the revision again.

On Linux, sending and receiving ATDECC frames needs `CAP_NET_RAW`:

```
sudo setcap cap_net_raw+ep target/debug/triib
sudo setcap cap_net_raw+ep target/debug/triib-endpointd
```

Building on Linux needs ALSA's and PipeWire's headers for audio, and
clang for PipeWire's bindings (`libasound2-dev`, `libpipewire-0.3-dev`
and `libclang-dev` on Debian and Ubuntu; `alsa-lib-devel`,
`pipewire-devel` and `clang-devel` on Fedora; `alsa-lib`, `libpipewire`
and `clang` on Arch).

On macOS it needs access to `/dev/bpf*`, as Wireshark's ChmodBPF or
triib's package gives (or, until the next restart,
`sudo chown $USER /dev/bpf*`). It reaches the Mac's own AVB entity
through Apple's AudioVideoBridging framework, as the system never shows
that entity what a program writes to BPF. On Windows it needs
[Npcap](https://npcap.com). `cargo run -p triib-cli -- interfaces` says
whether raw Ethernet is ready.

## Workspace

| Crate | What it does |
|---|---|
| `triib` | The app |
| `triib-cli` | Headless controller |
| `triib-endpointd` | Runs this computer's talkers and listeners |
| `triib-stream` | AAF streams, the media clock and audio devices |
| `triib-pipewire` | Virtual audio devices as PipeWire nodes |
| `triib-store` | Settings and cache files |
| `atdecc` | IEEE 1722.1 ATDECC with Milan: frames and state machines, no I/O |
| `avb-mrp` | IEEE 802.1Q MRP, MSRP and MVRP: frames and state machines, no I/O |
| `avb-net` | Raw Ethernet, interfaces and hardware timestamps per operating system |

`atdecc`, `avb-mrp` and `avb-net` are meant for other applications too:
they depend on nothing but `avb-net` (and, for `atdecc`'s driver on
macOS, the AudioVideoBridging framework), do no I/O of their own outside
that optional platform support, and build without std for
microcontrollers.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option. Unless you explicitly state
otherwise, any contribution intentionally submitted for inclusion in triib
by you, as defined in the Apache-2.0 license, shall be dual licensed as
above, without any additional terms or conditions.

Milan and AVB are trademarks of Avnu Alliance. triib is not certified by
Avnu Alliance.
