# triib's interface text in English, the source for every other language.
#
# Each line is `key = text`. Keys stay the same in every language; only the
# text after `=` is translated. `{ $name }` is a value triib fills in, such
# as an entity's name or a count; keep it, and move it where the language
# needs it. Plural and other variants use Fluent's selectors:
# https://projectfluent.org/fluent/guide/selectors.html
#
# triib controls audio networks built on AVB, Milan and IEEE 1722.1
# (ATDECC). Their terms follow the glossary in docs/GLOSSARY.md: names the
# standards give, such as AVB, Milan, gPTP, CRF, AECP or a descriptor
# type, stay as written in every language, and so do units such as kHz
# and Mb/s.
#
# Sections follow the parts of the interface. Keys shared by several parts
# are under "Common".

## Language

# This language's name in itself, as the Settings language list shows it,
# such as English, Deutsch or עברית.
language-name = English (US)

## Common

common-close = Close
common-more = More
common-keep-toolbar-shown = Keep toolbar shown
common-auto-hide-toolbar = Hide toolbar automatically

## Settings

settings-title = Settings
settings-general = General
settings-appearance = Appearance
settings-language = Language
# The first choice in the language list: follow the system's language,
# named in itself, such as "System default: Deutsch".
settings-language-system = System default: { $language }
settings-language-note = Text fields type in the system's input language.
settings-appearance-system = System
settings-appearance-light = Light
settings-appearance-dark = Dark
settings-colors = Colors
settings-system-accent = Use the system accent color
settings-accent-picked = The color below seeds triib's colors.
# $theme is the name of the Omarchy desktop theme, such as Tokyo Night.
settings-accent-omarchy = From the Omarchy theme, { $theme }.
settings-accent-desktop = From the desktop's accent color.
settings-accent-none = The desktop has no accent color, so the color below is used.
settings-motion = Motion
settings-animations = Animations
settings-animations-note = Springs and slides as things change.
settings-animations-reduced = The desktop asks for reduced motion, so triib stays still.
settings-outside-control = Outside control
settings-allow-outside-control = Allow outside control
settings-allow-outside-control-note = Agents such as Claude Code can see and change your AVB network through triib --mcp. triib asks before each new agent.
# $agents is a list of agent names, such as claude-code.
settings-allowed-agents = Allowed: { $agents }
settings-forget-agents = Forget

## An agent asking to control triib

# $agent is the agent's name, such as Claude Code.
agent-prompt-title = Allow { $agent } to control triib?
agent-prompt-body = { $agent } asks to use triib's outside control: to see the entities on your network and change them, connecting streams and setting formats, clocks and names. You can turn outside control off in Settings.
agent-prompt-allow = Allow
agent-prompt-deny = Don't allow

common-cancel = Cancel
common-save = Save
# Shown for a name or text field an entity leaves empty.
common-not-set = Not set
common-unnamed = Unnamed
common-none = None
common-mac-address = MAC address
# Joins parts of a description, as in "enp6s0, up, hardware clock ptp0".
# Keep the quotes and braces; change only what is between the quotes, such
# as {"、"} in Japanese.
common-list-separator = {", "}

## Network interfaces
# A network interface of this computer, as the picker and status bar
# describe it: "enp6s0, 1 Gb/s, hardware clock".

interface-up = up
interface-link-down = link down
interface-wireless = wireless
# The interface's network adapter keeps time in hardware, which precise
# PTP timing needs.
interface-hardware-clock = hardware clock
# $clock is the clock's system name, such as ptp0.
interface-hardware-clock-named = hardware clock { $clock }
# A software interface, such as a bridge or a tunnel, not an adapter.
interface-virtual = virtual

## Toolbar

toolbar-choose-interface = Choose an interface
toolbar-interface = Network interface
toolbar-show-virtual = Show virtual interfaces
toolbar-hide-virtual = Hide virtual interfaces
# The view of streams that can connect: talker outputs against listener
# inputs.
toolbar-connections = Connections
toolbar-network = Network
toolbar-entities = Entities
toolbar-rediscover = Ask every entity to announce itself
# The same button with Shift held: forgets every entity and what was kept
# of each, and reads them all again.
toolbar-rescan = Clear and rescan all entities
toolbar-search = Search entities and streams
toolbar-presets = Presets
toolbar-log = Log
toolbar-inspector = Inspector
toolbar-settings = Settings

## The network's state, in place of a view

state-no-interface = No interface
state-no-interface-note = Choose the interface on the AVB network to discover entities.
state-starting = Starting
# $interface is the network interface's system name, such as enp6s0.
state-starting-note = Opening { $interface }.
state-listening = Listening
state-listening-note = Entities on { $interface } appear here as they announce themselves.
state-permission-needed = Permission needed
state-npcap-needed = Npcap needed
state-get-npcap = Get Npcap
state-copy-command = Copy the command
state-cannot-use = Cannot use { $interface }
state-try-again = Try again

## Entity list

entities-none-yet = No entities yet
entities-none-yet-note = Every entity on the network, with its roles, SR classes and clock.

## Inspector

inspector-title = Inspector
inspector-entity = Entity
inspector-streams = Streams
inspector-controls = Controls
inspector-diagnostics = Diagnostics
inspector-descriptors = Descriptors
inspector-select = Select an entity to see its details.
# $entity is an entity ID, such as 0xe8f60ae092200000.
inspector-offline = { $entity } is offline.
inspector-rename = Rename
inspector-name = Name
# Makes the entity show itself, such as by blinking a light.
inspector-identify = Identify
inspector-model-not-read = Its entity model is not read.
inspector-no-streams = No streams.
inspector-no-controls = No controls to show.
inspector-no-diagnostics = No interfaces or counters reported.
inspector-reading = Reading descriptors, { $count } so far.
# $reason says what went wrong.
inspector-read-failed = Could not read the entity model: { $reason }.

entity-section = Entity
entity-name = Name
entity-group = Group
entity-product = Product
entity-firmware = Firmware
entity-serial-number = Serial number
entity-configuration = Configuration
# The configuration in use: its name, its number and how many there are.
entity-configuration-of = { $name } ({ $number } of { $count })
entity-milan = Milan
# The clock the entity's audio runs on.
entity-media-clock = Media clock
entity-clock-domain = Clock domain
entity-sampling-rate = Sampling rate
# A clock source with no name, by its number.
clock-source-numbered = Source { $index }
# A sampling rate's pull, a factor the standard numbers, when it has no
# usual name.
rate-pull = pull { $pull }

# The most time a stream output's frames take through the network.
stream-format = Format
# A stream's ID, as in 0xd111e597f5440000.
stream-id = Stream ID
# The MAC address a stream is sent to.
stream-destination = Destination
stream-max-transit = Max transit time

avb-interfaces = AVB interfaces
avb-interface = Interface
avb-interface-clock-identity = Clock identity
# The clock all others on the network follow.
avb-interface-grandmaster = Grandmaster
# $grandmaster is a clock identity, $domain the gPTP domain's number.
avb-interface-grandmaster-domain = { $grandmaster }, domain { $domain }
# The time a frame takes over the link to the next device.
avb-interface-peer-delay = Peer delay
avb-interface-running = Running
avb-interface-none-reported = None reported
avb-interface-path = Path
avb-interface-own-grandmaster = Its own grandmaster
avb-interface-hops = { $count ->
    [one] { $count } hop from the grandmaster
   *[other] { $count } hops from the grandmaster
}
avb-interface-link-up = Link up
avb-interface-link-down = Link down
avb-interface-grandmaster-changes = Grandmaster changes
avb-interface-frames-sent = Frames sent
avb-interface-frames-received = Frames received
avb-interface-crc-errors = CRC errors

# The Descriptors tab: each descriptor with what matters most about it.
tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } descriptor type
   *[other] { $count } descriptor types
}
tree-clock = Clock
# $kind is the kind of clock source and $location where it comes from,
# both as the standard names them, such as "Input stream, from Stream
# input 0".
tree-clock-source-from = { $kind }, from { $location } { $index }
tree-clock-domain-using = Using { $source }
tree-clusters = { $count ->
    [one] { $count } cluster
   *[other] { $count } clusters
}
tree-maps = { $count ->
    [one] { $count } map
   *[other] { $count } maps
}

# What an entity advertises of itself (ADP).
advert-not-advertised = Not advertised
advert-identity = Identity
advert-entity-id = Entity ID
advert-entity-model = Entity model
advert-roles = Roles
advert-talker = Talker
advert-listener = Listener
advert-clock = Clock
advert-btc = BTC
advert-gptp-domain = gPTP domain
advert-sr-classes = SR classes
# The indexes, in its entity model, of what the entity advertises.
advert-indexes = Entity model indexes
advert-identify-control = Identify control
advert-avb-interface = AVB interface
advert-advertising = Advertising
# How long the advertisement holds before the entity counts as gone.
advert-valid-time = Valid time
advert-available-index = Available index
advert-association = Association
advert-capabilities = Capabilities

## Status bar

status-entities = { $count ->
    [one] { $count } entity
   *[other] { $count } entities
}
status-not-discovering = Not discovering
status-discovering = Discovering
# $controller is triib's own entity ID on the network.
status-discovering-as = Discovering as { $controller }
status-stopped = Stopped by an error
status-alarm = Alarm
# $entity is an entity's name and $alarm what is wrong.
status-alarm-of = { $entity }: { $alarm }
# $alarm is the first alarm, $count how many more there are.
status-alarm-more = { $alarm } and { $count } more

## Descriptions of entities, shared by the views

# An entity's roles, as in "Talker 2, listener 2". Each is written as in
# the middle of a list; triib capitalizes the first. A talker sends
# streams and a listener receives them; $count is how many streams.
role-talker = talker { $count }
role-listener = listener { $count }
# An entity that controls others, as triib does.
role-controller = controller
role-none = no roles
# SR classes are traffic classes A and B; keep the letters.
classes-a-and-b = A and B
clock-no-gptp = No gPTP

# How far reading an entity's model got.
read-not-read = Not read
read-reading = Reading, { $count } so far
read-ready-unreadable = Ready, { $count } unreadable
read-ready-cached = Ready, from cache
read-ready = Ready
read-failed = Failed: { $reason }

# Milan support: "1.3, certified 1.3", "1.3, not certified" or "No".
milan-no = No
milan-before-1-3 = before 1.3
milan-certified = { $version }, certified { $certification }
milan-not-certified = { $version }, not certified

# Why an action did not work. $reason is a status the standard names, kept
# as it writes it, or one of the reasons below.
outcome-status = status { $status }
outcome-no-response = no response
outcome-not-possible = not possible
# $talker and $listener are streams, as in "Mac mini: Audio Output Stream 1".
outcome-connect = Could not connect { $talker } to { $listener }: { $reason }.
outcome-disconnect = Could not disconnect { $listener }: { $reason }.
outcome-identify = Could not identify { $entity }: { $reason }.
# $what is an entity or a stream, $name the name it was to get.
outcome-rename = Could not rename { $what } to "{ $name }": { $reason }.
outcome-rename-group = Could not rename { $entity }'s group to "{ $name }": { $reason }.
outcome-format-streaming = Could not change the format of { $stream }: it is streaming. Disconnect it first.
outcome-format = Could not change the format of { $stream }: { $reason }.
outcome-max-transit-time = Could not change the max transit time of { $stream }: { $reason }.
outcome-max-transit-time-streaming = Could not change the max transit time of { $stream }: it is streaming. Disconnect it first.
outcome-sampling-rate = Could not change the sampling rate of { $entity }: { $reason }.
outcome-clock-source = Could not change the clock source of { $entity }: { $reason }.
outcome-map = Could not map the channel on { $entity }: { $reason }.
outcome-unmap = Could not unmap the channel on { $entity }: { $reason }.
# $control is a control's name, such as Speaker Volume.
outcome-control = Could not set "{ $control }" on { $entity }: { $reason }.
outcome-control-numbered = Could not set control { $index } on { $entity }: { $reason }.

# A stream input's state: the talker's stream it is bound to, and how that
# goes. A reservation is the bandwidth the network sets aside for a stream.
stream-not-connected = Not connected
stream-from = From { $stream }
stream-from-receiving = From { $stream }, receiving
stream-from-waiting = From { $stream }, waiting for the talker
stream-from-failed = From { $stream }, the talker's reservation failed: { $reason }
# A stream output's state; $destination is a MAC address.
stream-sending-to = Sending to { $destination }

# Why reading an entity's model failed. $status is a status the standard
# names, kept as it writes it.
failure-no-response = it did not respond
failure-refused = it refused with { $status }
failure-malformed = its response did not decode
failure-on-this-computer = it runs on this computer; read it from another one

# Why a stream's reservation failed, by the MSRP failure codes of IEEE
# 802.1Q (35.2.2.8.7). A bridge is an AVB switch; the egress port is the
# port a stream leaves a bridge by.
msrp-failure-1 = insufficient bandwidth
msrp-failure-2 = insufficient bridge resources
msrp-failure-3 = insufficient bandwidth for the traffic class
msrp-failure-4 = stream ID in use by another talker
msrp-failure-5 = destination address already in use
msrp-failure-6 = pre-empted by a stream of higher rank
msrp-failure-7 = reported latency has changed
msrp-failure-8 = egress port is not AVB capable
msrp-failure-9 = use a different destination address
msrp-failure-10 = out of MSRP resources
msrp-failure-11 = out of MMRP resources
msrp-failure-12 = cannot store the destination address
msrp-failure-13 = priority is not an SR class priority
msrp-failure-14 = frames too large for the medium
msrp-failure-15 = fan-in port limit reached
msrp-failure-16 = first value changed for a registered stream
msrp-failure-17 = VLAN blocked on the egress port
msrp-failure-18 = VLAN tagging disabled on the egress port
msrp-failure-19 = SR class priority mismatch
msrp-failure-unknown = unknown reason
# $bridge is the MAC address of the bridge reporting the failure.
msrp-failure-at = { $reason }, at the bridge { $bridge }

## Entity list columns
# Headings of the entity list's columns; the others share the inspector's
# words.

column-vendor = Vendor
column-model = Model
column-state = State
column-entity-model-id = Entity model ID
column-talker-streams = Talker streams
column-listener-streams = Listener streams
# Whether an entity runs AVB Lite, a profile of AVB for networks without
# AVB switches. Keep "AVB Lite" as written.
column-avb-lite = AVB Lite
# What an entity's talker streams take of its link.
column-egress = Egress
# Whether an entity's interface is a Wi-Fi station or access point, as the
# AVB Wireless profile has it.
column-wireless = Wireless

## Settings file

settings-no-place = There is nowhere to keep the settings: the home folder is not known.
# $path is the settings file and $error what went wrong, from the system.
settings-unusable = Could not use { $path }: { $error }.
settings-unsaved = Could not save { $path }: { $error }.

column-remove = Remove column
column-move-left = Move left
column-move-right = Move right
column-add = Add a column
# A share, as in "80.0%"; write the space or sign as the language does,
# such as "{ $value } %".
common-percent = { $value }%

## Network view
# The network map: bridges (AVB switches) and devices on the gPTP tree,
# with the clock or the streams flowing along their wires. "gPTP" and
# "CRF" (media clock streams) stay as written.

netmap-empty = No network to show yet
netmap-empty-note = Entities appear here once they have been read and have said where they sit in the gPTP tree.
# What the map brings forward; $name is a device or bridge.
netmap-focus-clock-path = { $name }’s clock path
netmap-focus-streams = { $name }’s streams
# A button that stops bringing $what forward, such as "Showing Mac mini's
# streams".
netmap-showing = Showing { $what }
netmap-devices = { $count ->
    [one] { $count } device
   *[other] { $count } devices
}
netmap-bridges = { $count ->
    [one] { $count } bridge
   *[other] { $count } bridges
}
netmap-show-map = Show the map
netmap-show-details = Show the details
# A stream with no name, by its number.
stream-numbered = Stream { $index }
# An AVB switch with no vendor known.
netmap-bridge = Bridge
# A bridge whose port is a Wi-Fi access point.
netmap-access-point = Access point
netmap-device = Device
netmap-this-computer = This computer
netmap-connected = Connected
# A talker offers the stream, but no listener has asked for it.
netmap-advertised = Advertised, no listener ready
netmap-advertised-off-tree = Advertised, no listener ready ({ $listener } is not on the gPTP tree)
netmap-failed-at = Reservation failed at { $bridge }: { $reason }
netmap-failed = Reservation failed: { $reason }
netmap-no-bridge-on = No bridge heard on { $interface }
netmap-cannot-listen-on = Cannot listen for gPTP on { $interface }
netmap-on-this-computer = On this computer
netmap-path-not-reported = Path not reported
netmap-gptp-not-reported = gPTP not reported
# Also a label over the devices off the tree, in capitals where the script
# has them.
netmap-off-tree = Not on the gPTP tree
# The same in AVB Lite, over the devices not following a grandmaster.
netmap-off-ptp = Not on the PTP tree
# A device on a network running AVB Lite that does not run it.
netmap-not-lite = Not in AVB Lite
netmap-lite-not-reported = AVB Lite not reported
# In step with the grandmaster's clock: short for in sync.
netmap-synced = Sync
netmap-not-synced = No sync
netmap-triib-on = triib on { $interface }
# Streams passing through a bridge.
netmap-through-count = { $count } through
# Streams a device sends, receives, and that failed.
netmap-out = { $count } out
netmap-in = { $count } in
netmap-failed-count = { $count } failed
netmap-advertised-only = Advertised only
netmap-failed-state = Failed
# A stream in the details: its talker, its listener and its state.
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Not on the gPTP tree: it is its own grandmaster
netmap-apart-no-path = Its path was not reported; it follows grandmaster { $grandmaster }
netmap-apart-unreported = It has not reported its gPTP state
netmap-apart-no-neighbor = No bridge heard on this computer's interface
netmap-apart-cannot-listen = This computer cannot listen for gPTP on its interface
netmap-apart-on-this-computer = It runs on this computer; read it from another computer to see its gPTP state
netmap-apart-not-lite = It does not run AVB Lite, so it does not follow the grandmaster
netmap-apart-lite-unreported = It reports nothing of AVB Lite, so whom it follows is not known
netmap-clock-tree = Clock tree
netmap-no-grandmaster = No grandmaster heard
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Needs attention
netmap-nodes-below = Nodes below
netmap-bridges-below = Bridges below
netmap-clock-path = Clock path
netmap-hops = Hops from grandmaster
netmap-link-delay = Link delay
netmap-bridge-port = Bridge port
# How often the link went down.
netmap-link-drops = Link drops
netmap-synced-to-grandmaster = In sync with the grandmaster
netmap-host-no-gptp = No sync: this computer does not run gPTP
netmap-link-no-gptp = No sync: gPTP does not run on its link
# In AVB Lite. $offset is a time such as "-72 µs".
netmap-ptp-offset-high = No sync: { $offset } from the grandmaster, past the 50 µs AVB Lite allows
netmap-ptp-no-offset = No sync: it has measured no offset from the grandmaster
netmap-audio = Audio
netmap-media-clock-streams = Media clock streams
netmap-audio-streams = Audio streams
# Streams bound to a talker.
netmap-bound = { $count } bound
netmap-flowing = Flowing
netmap-advertised-state = Advertised
netmap-media-clock-stream = Media clock stream
netmap-audio-stream = Audio stream
# As far as a stream that is not flowing gets.
netmap-reaches = Reaches
netmap-passing-count = { $count ->
    [one] { $count } stream passing through
   *[other] { $count } streams passing through
}
netmap-through = Through
netmap-passing-through = Passing through
netmap-sending = Sending
netmap-receiving = Receiving
netmap-problems = Problems
netmap-help-back = Click the background to go back to the overview.
netmap-help-stream = Click a stream to inspect it, or the background to go back to the overview.
netmap-help-ptp = In AVB Lite the clock flows end to end from the grandmaster to every device, through switches that take no part, so none are shown. A device is in sync while it follows the grandmaster within 50 µs. Click a device or its wire to inspect its clock; click the background to clear.
netmap-help-clock = The clock flows from the grandmaster through each bridge to every node on the tree. A broken gray line is a link gPTP does not run on. Click a device or its wire to inspect its clock path; click the background to clear.
netmap-help-media-clock = Media clock (CRF) streams only, drawn the same way as audio: one wire per stream, colored by talker. Click a wire to inspect its stream, or a device to see its streams; click the background to clear.
netmap-help-audio = Each stream has its own wire, entering and leaving every bridge it crosses. Color is by talker: each talker has a hue, and its streams are shades of it. Moving dots mean audio is flowing; a still red line is a failed reservation and a still gray line is advertised with no listener ready; both stop where the reservation stops. Devices in the middle column connect straight to the grandmaster's bridge. Click a wire to inspect its stream, or a device to see its streams; click the background to clear.

## Connections
# The matrix of talker stream outputs against listener stream inputs, where
# a click on a cell binds an input to an output. Stream formats such as
# "48k 8ch" or "48k CRF" stay as written.

matrix-nothing-shown = No streams to show
matrix-nothing-shown-note = Change the search or the filters to see more streams.
matrix-empty = No streams to connect
matrix-empty-note = Talker streams and listener streams meet here once entities with them have been read.
matrix-all-streams = All streams
matrix-connectable-only = Hide what cannot connect
matrix-none-hidden = Every stream shown can connect
matrix-hidden = { $count ->
    [one] { $count } stream hidden
   *[other] { $count } streams hidden
}
matrix-own = An entity's outputs do not connect to its own inputs.
matrix-working = Working on it.
matrix-waiting-change = Waiting for the last change to this input.
matrix-connected = Connected and receiving. Click to disconnect.
# Bound: the input is set to take the talker's stream.
matrix-bound-waiting = Bound, waiting for the talker's stream. Click to disconnect.
matrix-bound-failed = Bound, but the talker's reservation failed: { $reason }. Click to disconnect.
# $sent is the format the talker sends and $set the input's, such as
# "48k 8ch".
matrix-bound-formats-differ = Bound, but the formats differ: the talker sends { $sent }, the input is set to { $set }. Click to disconnect.
matrix-formats-match = Formats match ({ $format }). Click to connect.
matrix-format-must-change = The input takes { $sent } but is set to { $set }, so it may not play until its format changes. Click to choose a format.
# The menu a click on such a cell opens: the talker's format, which only
# the input changes to, then other formats both ends take, which both
# change to, each connecting once set; then connecting as they are.
matrix-format-input = Change the input to
matrix-format-both = Change both to
matrix-format-talker-busy = The talker streams to other listeners, so its format stays.
matrix-format-anyway = Connect without changing formats
matrix-incompatible = The input does not take { $sent }. It is set to { $set }.
# An entity's row or column folded into one cell.
matrix-group-none = Not connected. Expand to connect streams one by one.
matrix-group-connected = { $count } connected. Expand to see each one.
matrix-outputs-expand = { $count ->
    [one] { $count } stream output. Click the arrow to expand, the name to inspect it.
   *[other] { $count } stream outputs. Click the arrow to expand, the name to inspect it.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } stream output. Click the arrow to collapse, the name to inspect it.
   *[other] { $count } stream outputs. Click the arrow to collapse, the name to inspect it.
}
matrix-inputs-expand = { $count ->
    [one] { $count } stream input. Click the arrow to expand, the name to inspect it.
   *[other] { $count } stream inputs. Click the arrow to expand, the name to inspect it.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } stream input. Click the arrow to collapse, the name to inspect it.
   *[other] { $count } stream inputs. Click the arrow to collapse, the name to inspect it.
}
# $detail is the stream's format and state, $entity its entity's name.
matrix-stream-inspect = { $detail } Click to inspect { $entity }.
matrix-point = Point at a cell
# Follows "Point at a cell" on the same line.
matrix-point-note = to see its talker and listener and whether their formats meet.
matrix-legend-waiting = Bound, waiting for the stream
matrix-legend-trouble = Bound, something is wrong
matrix-legend-open = Can connect
matrix-legend-change = Input format must change first
matrix-legend-incompatible = Formats cannot meet
# Over the matrix's corner: which way each side runs.
matrix-talker-outputs = Talker outputs
matrix-listener-inputs = Listener inputs

# Sets thousands apart in counts, as in 1,204,331. Keep the quotes and
# braces; change only what is between the quotes, such as {"."} or {" "}.
common-thousands-separator = {","}

## Diagnostics
# What each clock domain and stream counted since the entity started. Each
# fact is written as in the middle of a list; triib capitalizes the first.
# $count picks the words; $number is the same count with thousands set
# apart, to show. Exact matches such as [2] may be dropped where the
# language has no word like "twice".

diag-since-start = Counted since the entity started.
diag-stream-input = Stream input
diag-stream-output = Stream output
# A clock domain locked to its clock source.
diag-locked = { $count ->
    [0] not locked
    [1] locked once
    [2] locked twice
   *[other] locked { $number } times
}
diag-lost-lock = { $count ->
    [0] not lost lock
    [1] lost lock once
    [2] lost lock twice
   *[other] lost lock { $number } times
}
diag-frames-in = { $count ->
    [one] { $number } frame in
   *[other] { $number } frames in
}
diag-frames-out = { $count ->
    [one] { $number } frame out
   *[other] { $number } frames out
}
# The stream input locked to the media clock its stream carries.
diag-media-locked = { $count ->
    [0] not media locked
    [1] media locked once
    [2] media locked twice
   *[other] media locked { $number } times
}
diag-lost-media-lock = { $count ->
    [0] not lost media lock
    [1] lost media lock once
    [2] lost media lock twice
   *[other] lost media lock { $number } times
}
diag-interrupted = { $count ->
    [0] not interrupted
    [1] interrupted once
    [2] interrupted twice
   *[other] interrupted { $number } times
}
diag-out-of-sequence = { $count ->
    [one] { $number } frame out of sequence
   *[other] { $number } frames out of sequence
}
diag-media-resets = { $count ->
    [one] { $number } media reset
   *[other] { $number } media resets
}
diag-timestamps-uncertain = { $count ->
    [0] timestamps not uncertain
    [1] timestamps uncertain once
    [2] timestamps uncertain twice
   *[other] timestamps uncertain { $number } times
}
diag-no-timestamp = { $count ->
    [one] { $number } frame without a timestamp
   *[other] { $number } frames without a timestamp
}
diag-unsupported-format = { $count ->
    [one] { $number } frame in an unsupported format
   *[other] { $number } frames in an unsupported format
}
# Frames that came after, or before, the time they were to play.
diag-late = { $count ->
    [one] { $number } frame late
   *[other] { $number } frames late
}
diag-early = { $count ->
    [one] { $number } frame early
   *[other] { $number } frames early
}
diag-started = { $count ->
    [0] not started
    [1] started once
    [2] started twice
   *[other] started { $number } times
}
diag-stopped = { $count ->
    [0] not stopped
    [1] stopped once
    [2] stopped twice
   *[other] stopped { $number } times
}
diag-reservation-failed = the talker's reservation failed: { $reason }
# The latency the stream gathered on its way through the network.
diag-latency = { $microseconds } µs accumulated latency

## AVB Lite
# AVB Lite is a profile of AVB for networks without AVB switches; keep
# "AVB Lite", "AVB", "PTP", "gPTP" and "VLAN" as written. An entity falls
# back to AVB Lite when it finds no AVB bridge (AVB switch) on its link.

# How an entity runs, in the entity list.
lite-active = Active
lite-active-untagged = Active, untagged
# $vlan is the number of the VLAN its streams use.
lite-active-vlan = Active, VLAN { $vlan }
lite-capable = Capable
lite-mode = Mode
lite-mode-capable = AVB (AVB Lite capable)
# Why the entity fell back to AVB Lite, after "Because".
lite-because = Because
lite-fallback-none = no reason given
lite-fallback-endpoint = another endpoint's declaration came through, so no AVB bridge is between them
lite-fallback-unanswered = nine peer delay requests went unanswered
lite-fallback-responders = two or more answered one peer delay request, so the switch is not an AVB bridge
lite-fallback-configured = the operator or a controller set it
lite-fallback-other = a reason the profile does not name
lite-other-profile = Another profile
# $profile is a PTP profile, such as "gPTP (802.1AS)", and $domain its
# domain number.
lite-ptp-domain = { $profile }, domain { $domain }
# How far the entity's clock is from the grandmaster's.
lite-offset = Offset
lite-offset-from = { $offset } from { $grandmaster }
lite-media-vlan = Media VLAN
# Streams sent without a VLAN tag.
lite-untagged = Untagged
lite-unicast = Unicast
# A talker sends each stream to up to $count listeners one by one, then to
# all at once.
lite-fanout = { $count ->
    [one] Up to { $count } listener a stream, then multicast
   *[other] Up to { $count } listeners a stream, then multicast
}
lite-link = Link
lite-bandwidth = Bandwidth
# $used and $link are rates such as "16.5 Mb/s", $share a share such as
# "1.7%".
lite-egress-of = { $used } of { $link }, { $share }
lite-egress-of-assumed = { $used } of { $link }, { $share }, a gigabit link assumed
lite-egress-reported = As the entity counts its admitted streams.
lite-egress-worked-out = From the formats of its connected stream outputs.
# Alarms. $offset is a time such as "-72 µs".
lite-alarm-offset = PTP offset { $offset }, past the 50 µs AVB Lite allows
# $share is how much of the link streams take, $limit the most they may.
lite-alarm-egress = Egress at { $share } of the link, past the { $limit } streams may take

## AVB Wireless
# AVB Wireless is a profile of AVB for one Wi-Fi hop: an access point that
# is an AVB bridge on its wired port, and stations that are talkers and
# listeners. Keep "AVB", "FTM", "TM", "BSSID", "asCapable", "Wi-Fi",
# "802.1AS", "Mode A", "Mode B" and "Class A" as written.

# A Wi-Fi interface's role.
wireless-station = Station
wireless-access-point = Access point
wireless-role = Role
# How a station gets the grandmaster's time: 802.1AS over Fine Timing
# Measurement or Timing Measurement frames, or the time the access point
# puts in its beacons.
wireless-mode = Mode
wireless-time = Time
wireless-mode-a-ftm = Mode A, 802.1AS over FTM
wireless-mode-a-tm = Mode A, 802.1AS over TM
wireless-mode-b = Mode B, from beacons
wireless-no-time = No time
wireless-other-mode = A mode the profile does not name
# How a station's time holds: locked to the access point, keeping time on
# its own after losing it, or not locked.
wireless-locked = Locked
wireless-holdover = Holding over
wireless-not-locked = Not locked
# A Wi-Fi interface in the entity list: a station and how its time holds,
# or an access point and how many stations it has.
wireless-row-locked = Station, locked
wireless-row-holdover = Station, holding over
wireless-row-not-locked = Station, not locked
wireless-row-access-point = { $count ->
    [one] Access point, { $count } station
   *[other] Access point, { $count } stations
}
wireless-link = Link
wireless-channel = channel { $channel }
wireless-not-known = Not known
# The strength of the access point's signal at the station.
wireless-signal = Signal
# The rate the station sends at.
wireless-rate = Rate
# $share is a share such as "98%".
wireless-ftm-valid = { $share } valid
# The round trip time of the last measurement; $rtt is a time such as
# "42 ns".
wireless-rtt = round trip { $rtt }
wireless-bursts = { $count ->
    [one] bursts of { $count } frame
   *[other] bursts of { $count } frames
}
# Why the station's link is not an 802.1AS link; $reason is one of the
# reasons below.
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = the access point grants FTM bursts of other than three or two frames
wireless-reason-measurement = neither FTM nor TM with the access point
wireless-reason-signaling = no gPTP-capable Signaling from the access point
wireless-reason-other = a reason the profile does not name
# The station's own estimate of how far its clock is off.
wireless-servo = Servo error
wireless-stations = Stations
wireless-station-count = { $count ->
    [one] { $count } station
   *[other] { $count } stations
}
# A station that cannot measure with FTM.
wireless-no-ftm = without FTM
# Listeners the access point does not serve, past its unicast limit.
wireless-unserved = Listeners not served
# The frames a second the access point can send on Wi-Fi, and how many its
# streams take, each listener's own copy counted.
wireless-frame-budget = Frame budget
wireless-frame-use = { $used } of { $budget } frames a second
wireless-stream-frames = Stream frames
# What the access point did with stream frames: sent to each station on
# its own, found no listener for, dropped, and gave back their address.
wireless-frames-of = { $readdressed } to stations, { $unmapped } without a listener, { $dropped } dropped, { $restored } from stations
wireless-class-a-allowed = Allowed
wireless-class-a-not-allowed = Not allowed
# Alarms.
wireless-alarm-not-locked = Wi-Fi time not locked to the access point
wireless-alarm-holdover = Wi-Fi time holding over, lost from the access point
wireless-alarm-unserved = { $count ->
    [one] { $count } listener on the Wi-Fi port not served, past the unicast limit
   *[other] { $count } listeners on the Wi-Fi port not served, past the unicast limit
}

## Log
# The ATDECC frames triib sends and hears. Each frame's summary stays in the
# standards' words, as packet analyzers show them; ADP, AECP and ACMP are
# protocol names.

log-all = All
log-warnings = Warnings
log-pause = Pause
log-resume = Resume
log-clear = Clear
log-empty = Every ATDECC frame triib sends and hears appears here, newest first.
log-none-match = No frame kept matches the filter.
log-frames = { $count ->
    [one] { $count } frame
   *[other] { $count } frames
}
# Frames the filter shows, of all kept.
log-shown-of = { $shown } of { $all } frames
# Which way a frame went: sent by triib, or heard from the network.
log-sent = Sent
log-heard = Heard
log-not-decoded = Not decoded
# control_data_length is a field of the frame; keep it as written.
log-warning-short = Its control_data_length claims { $missing } octets past the end of the frame.
# $error says what does not decode, in the standard's words.
log-warning-undecodable = It does not decode: { $error }.
log-warning-long-acmp = It is in the long ACMP form, which a Milan entity may not send (Milan 1.3, 5.5.2.2).

## Channel mappings
# How an entity's audio channels map to the channels of its streams. A
# cluster is a group of the entity's own audio channels, as IEEE 1722.1
# names it; a stream port joins streams to clusters.

mapping-section = Channel mappings
mapping-inputs = Inputs
mapping-outputs = Outputs
mapping-port = port { $number }
# Mappings the entity sets itself, which a controller cannot change.
mapping-fixed = fixed
mapping-not-read = Not read yet.
mapping-no-clusters = No clusters.
mapping-no-streams = No audio streams.
mapping-none = No mappings.
mapping-not-mapped = Not mapped
mapping-cluster-numbered = Cluster { $index }

## Presets
# A preset keeps the network's settings and connections, to bring back
# later.

presets-note = A preset keeps each entity's clock sources, sampling rates, stream formats, controls and connections. Recalling it changes what differs.
presets-none = No presets saved yet.
presets-connections = { $count ->
    [one] { $count } connection
   *[other] { $count } connections
}
presets-recall = Recall
presets-delete = Delete
presets-no-place = There is nowhere to keep presets: the home folder is not known.
presets-undeletable = Could not delete { $path }: { $error }.
# $name is the preset's name.
presets-saved = { $count ->
    [one] Saved "{ $name }" with { $count } entity.
   *[other] Saved "{ $name }" with { $count } entities.
}
presets-nothing-differs = Nothing differs from "{ $name }".
presets-recalling = { $count ->
    [one] Recalling "{ $name }": { $count } change.
   *[other] Recalling "{ $name }": { $count } changes.
}
# $report is one of the two above; $missing lists the entities the preset
# holds that are not on the network or not read yet.
presets-missing = { $report } Not here or not read: { $missing }.
presets-deleted = Deleted "{ $name }".
# Follows presets-note where this computer can run talkers and listeners
# of its own.
presets-host-note = It also keeps this computer's own talkers and listeners, and starts them again on recall.
# Beside the counts of entities and connections in a preset's row.
presets-host-endpoints = { $count } on this computer
# While a recalled preset waits for this computer's endpoints to start.
presets-starting-host = Starting this computer's talkers and listeners for "{ $name }"; the rest follows once they are back.

## Controls
# An entity's CONTROL descriptors, such as gain or mute.

control-numbered = Control { $index }
control-not-shown = Not shown here
# A choice of a selector control that has no name.
control-option = Option { $number }

## Network errors

network-permission = triib needs permission to send and receive raw Ethernet frames.
network-needs-npcap = triib needs Npcap to send and receive raw Ethernet frames.
network-npcap-administrators = Npcap lets only administrators send and receive raw Ethernet frames. Run triib as administrator, or install Npcap again without its administrators only option.

# A stream's format, such as "48k 8ch", and a stream input's state.
matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

# The mark between a number's whole part and its fraction, as in 44.1 kHz.
# Keep the quotes and braces; change only what is between them, such as
# {","}.
common-decimal-separator = {"."}

## This computer's own talkers and listeners

host-add-talker = Add talker
host-add-listener = Add listener
# This computer as one device with stream outputs and inputs.
host-add-device = Add device
host-show-mine = Show only this computer's own talkers and listeners
host-show-all = Show every entity
host-new-talker = Host talker { $number }
host-new-listener = Host listener { $number }
host-new-device = Host device { $number }
host-failed = Could not add it to this computer: { $reason }
host-needs-clock = This computer's own talkers and listeners need a wired interface with a PTP hardware clock
host-no-ptp4l = ptp4l does not answer, so this computer's streams cannot keep gPTP time
# Another user's triib-endpointd, or root's, runs this computer's talkers
# and listeners on the interface; this user's cannot run beside it.
host-elsewhere = triib-endpointd runs on { $interface } for another user or as root, so this computer's talkers and listeners run there, not here
# MSRP and MVRP are protocols that reserve bandwidth and VLANs; keep them as
# written. Another program on this computer runs them on the same port.
host-foreign-mrp = Another program declares MSRP or MVRP on { $interface } from this computer's address, which can withdraw what this computer's streams need
host-alarm-foreign-mrp = Another program on this computer declares MSRP or MVRP on { $interface }
host-state = State
host-streaming = Streaming
host-waiting = Waiting for a listener
host-listening = Listening
host-bound = Bound, waiting for the talker
host-unbound = Not bound
host-audio-from = Audio from
host-audio-to = Audio to
host-channels = Channels
# One of a device's streams with an audio device of its own on this
# computer, which apps play into or record from.
host-virtual-device = Virtual device
host-add-output-stream = Add output stream
host-add-input-stream = Add input stream
host-remove-stream = Remove stream
host-silence = Silence
host-tone = Test tone
host-nowhere = Nowhere
host-default-device = Default device
host-remove = Remove from this computer
