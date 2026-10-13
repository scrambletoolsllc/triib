# triib's interface text in British English. The source is
# i18n/en/triib.ftl; only British spelling and quotation style differ.

## Language

language-name = English (UK)

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
settings-language-system = System default: { $language }
settings-language-note = Text fields type in the system's input language.
settings-appearance-system = System
settings-appearance-light = Light
settings-appearance-dark = Dark
settings-colors = Colours
settings-system-accent = Use the system accent colour
settings-accent-picked = The colour below seeds triib's colours.
settings-accent-omarchy = From the Omarchy theme, { $theme }.
settings-accent-desktop = From the desktop's accent colour.
settings-accent-none = The desktop has no accent colour, so the colour below is used.
settings-motion = Motion
settings-animations = Animations
settings-animations-note = Springs and slides as things change.
settings-animations-reduced = The desktop asks for reduced motion, so triib stays still.

common-cancel = Cancel
common-save = Save
common-not-set = Not set
common-unnamed = Unnamed
common-none = None
common-mac-address = MAC address
common-list-separator = {", "}

## Network interfaces

interface-up = up
interface-link-down = link down
interface-wireless = wireless
interface-hardware-clock = hardware clock
interface-hardware-clock-named = hardware clock { $clock }
interface-virtual = virtual

## Toolbar

toolbar-choose-interface = Choose an interface
toolbar-interface = Network interface
toolbar-show-virtual = Show virtual interfaces
toolbar-hide-virtual = Hide virtual interfaces
toolbar-connections = Connections
toolbar-network = Network
toolbar-entities = Entities
toolbar-rediscover = Ask every entity to announce itself
toolbar-search = Search entities and streams
toolbar-presets = Presets
toolbar-log = Log
toolbar-inspector = Inspector
toolbar-settings = Settings

## The network's state, in place of a view

state-no-interface = No interface
state-no-interface-note = Choose the interface on the AVB network to discover entities.
state-starting = Starting
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
inspector-offline = { $entity } is offline.
inspector-rename = Rename
inspector-name = Name
inspector-identify = Identify
inspector-model-not-read = Its entity model is not read.
inspector-no-streams = No streams.
inspector-no-controls = No controls to show.
inspector-no-diagnostics = No interfaces or counters reported.
inspector-reading = Reading descriptors, { $count } so far.
inspector-read-failed = Could not read the entity model: { $reason }.

entity-section = Entity
entity-name = Name
entity-group = Group
entity-product = Product
entity-firmware = Firmware
entity-serial-number = Serial number
entity-configuration = Configuration
entity-configuration-of = { $name } ({ $number } of { $count })
entity-milan = Milan
entity-media-clock = Media clock
entity-clock-domain = Clock domain
entity-sampling-rate = Sampling rate
clock-source-numbered = Source { $index }
rate-pull = pull { $pull }

stream-max-transit-time = Max transit time { $time }

avb-interfaces = AVB interfaces
avb-interface = Interface
avb-interface-clock-identity = Clock identity
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domain { $domain }
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

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } descriptor type
   *[other] { $count } descriptor types
}
tree-clock = Clock
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
advert-indexes = Entity model indexes
advert-identify-control = Identify control
advert-avb-interface = AVB interface
advert-advertising = Advertising
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
status-discovering-as = Discovering as { $controller }
status-stopped = Stopped by an error
status-alarm = Alarm
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } and { $count } more

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = controller
role-none = no roles
classes-a-and-b = A and B
clock-no-gptp = No gPTP

read-not-read = Not read
read-reading = Reading, { $count } so far
read-ready-unreadable = Ready, { $count } unreadable
read-ready-cached = Ready, from cache
read-ready = Ready
read-failed = Failed: { $reason }

milan-no = No
milan-before-1-3 = before 1.3
milan-certified = { $version }, certified { $certification }
milan-not-certified = { $version }, not certified

outcome-status = status { $status }
outcome-no-response = no response
outcome-not-possible = not possible
outcome-connect = Could not connect { $talker } to { $listener }: { $reason }.
outcome-disconnect = Could not disconnect { $listener }: { $reason }.
outcome-identify = Could not identify { $entity }: { $reason }.
outcome-rename = Could not rename { $what } to ‘{ $name }’: { $reason }.
outcome-rename-group = Could not rename { $entity }'s group to ‘{ $name }’: { $reason }.
outcome-format-streaming = Could not change the format of { $stream }: it is streaming. Disconnect it first.
outcome-format = Could not change the format of { $stream }: { $reason }.
outcome-sampling-rate = Could not change the sampling rate of { $entity }: { $reason }.
outcome-clock-source = Could not change the clock source of { $entity }: { $reason }.
outcome-map = Could not map the channel on { $entity }: { $reason }.
outcome-unmap = Could not unmap the channel on { $entity }: { $reason }.
outcome-control = Could not set ‘{ $control }’ on { $entity }: { $reason }.
outcome-control-numbered = Could not set control { $index } on { $entity }: { $reason }.

stream-not-connected = Not connected
stream-from = From { $stream }
stream-from-receiving = From { $stream }, receiving
stream-from-waiting = From { $stream }, waiting for the talker
stream-from-failed = From { $stream }, the talker's reservation failed: { $reason }
stream-sending-to = Sending to { $destination }

failure-no-response = it did not respond
failure-refused = it refused with { $status }
failure-malformed = its response did not decode
failure-on-this-computer = it runs on this computer; read it from another one

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
msrp-failure-at = { $reason }, at the bridge { $bridge }

## Entity list columns

column-vendor = Vendor
column-model = Model
column-state = State
column-entity-model-id = Entity model ID
column-talker-streams = Talker streams
column-listener-streams = Listener streams
column-avb-lite = AVB Lite
column-egress = Egress

## Settings file

settings-no-place = There is nowhere to keep the settings: the home folder is not known.
settings-unusable = Could not use { $path }: { $error }.
settings-unsaved = Could not save { $path }: { $error }.

column-remove = Remove column
column-move-left = Move left
column-move-right = Move right
column-add = Add a column
common-percent = { $value }%

## Network view

netmap-empty = No network to show yet
netmap-empty-note = Entities appear here once they have been read and have said where they sit in the gPTP tree.
netmap-focus-clock-path = { $name }’s clock path
netmap-focus-streams = { $name }’s streams
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
stream-numbered = Stream { $index }
netmap-bridge = Bridge
netmap-device = Device
netmap-this-computer = This computer
netmap-connected = Connected
netmap-advertised = Advertised, no listener ready
netmap-advertised-off-tree = Advertised, no listener ready ({ $listener } is not on the gPTP tree)
netmap-failed-at = Reservation failed at { $bridge }: { $reason }
netmap-failed = Reservation failed: { $reason }
netmap-no-bridge-on = No bridge heard on { $interface }
netmap-cannot-listen-on = Cannot listen for gPTP on { $interface }
netmap-on-this-computer = On this computer
netmap-path-not-reported = Path not reported
netmap-gptp-not-reported = gPTP not reported
netmap-off-tree = Not on the gPTP tree
netmap-off-ptp = Not on the PTP tree
netmap-not-lite = Not in AVB Lite
netmap-lite-not-reported = AVB Lite not reported
netmap-synced = Sync
netmap-not-synced = No sync
netmap-triib-on = triib on { $interface }
netmap-through-count = { $count } through
netmap-out = { $count } out
netmap-in = { $count } in
netmap-failed-count = { $count } failed
netmap-advertised-only = Advertised only
netmap-failed-state = Failed
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
netmap-link-drops = Link drops
netmap-synced-to-grandmaster = In sync with the grandmaster
netmap-host-no-gptp = No sync: this computer does not run gPTP
netmap-link-no-gptp = No sync: gPTP does not run on its link
netmap-ptp-offset-high = No sync: { $offset } from the grandmaster, past the 50 µs AVB Lite allows
netmap-ptp-no-offset = No sync: it has measured no offset from the grandmaster
netmap-audio = Audio
netmap-media-clock-streams = Media clock streams
netmap-audio-streams = Audio streams
netmap-bound = { $count } bound
netmap-flowing = Flowing
netmap-advertised-state = Advertised
netmap-media-clock-stream = Media clock stream
netmap-audio-stream = Audio stream
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
netmap-help-clock = The clock flows from the grandmaster through each bridge to every node on the tree. A broken grey line is a link gPTP does not run on. Click a device or its wire to inspect its clock path; click the background to clear.
netmap-help-media-clock = Media clock (CRF) streams only, drawn the same way as audio: one wire per stream, coloured by talker. Click a wire to inspect its stream, or a device to see its streams; click the background to clear.
netmap-help-audio = Each stream has its own wire, entering and leaving every bridge it crosses. Colour is by talker: each talker has a hue, and its streams are shades of it. Moving dots mean audio is flowing; a still red line is a failed reservation and a still grey line is advertised with no listener ready; both stop where the reservation stops. Devices in the middle column connect straight to the grandmaster's bridge. Click a wire to inspect its stream, or a device to see its streams; click the background to clear.

## Connections

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
matrix-bound-waiting = Bound, waiting for the talker's stream. Click to disconnect.
matrix-bound-failed = Bound, but the talker's reservation failed: { $reason }. Click to disconnect.
matrix-bound-formats-differ = Bound, but the formats differ: the talker sends { $sent }, the input is set to { $set }. Click to disconnect.
matrix-formats-match = Formats match ({ $format }). Click to connect.
matrix-format-must-change = The input takes { $sent } but is set to { $set }, so it may not play until its format changes. Click to choose a format.
matrix-incompatible = The input does not take { $sent }. It is set to { $set }.
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
matrix-stream-inspect = { $detail } Click to inspect { $entity }.
matrix-point = Point at a cell
matrix-point-note = to see its talker and listener and whether their formats meet.
matrix-legend-waiting = Bound, waiting for the stream
matrix-legend-trouble = Bound, something is wrong
matrix-legend-open = Can connect
matrix-legend-change = Input format must change first
matrix-legend-incompatible = Formats cannot meet
matrix-talker-outputs = Talker outputs
matrix-listener-inputs = Listener inputs

common-thousands-separator = {","}
common-decimal-separator = {"."}

## Diagnostics

diag-since-start = Counted since the entity started.
diag-stream-input = Stream input
diag-stream-output = Stream output
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
diag-latency = { $microseconds } µs accumulated latency

## AVB Lite

lite-active = Active
lite-active-untagged = Active, untagged
lite-active-vlan = Active, VLAN { $vlan }
lite-capable = Capable
lite-mode = Mode
lite-mode-capable = AVB (AVB Lite capable)
lite-because = Because
lite-fallback-none = no reason given
lite-fallback-endpoint = another endpoint's declaration came through, so no AVB bridge is between them
lite-fallback-unanswered = nine peer delay requests went unanswered
lite-fallback-responders = two or more answered one peer delay request, so the switch is not an AVB bridge
lite-fallback-configured = the operator or a controller set it
lite-fallback-other = a reason the profile does not name
lite-other-profile = Another profile
lite-ptp-domain = { $profile }, domain { $domain }
lite-offset = Offset
lite-offset-from = { $offset } from { $grandmaster }
lite-media-vlan = Media VLAN
lite-untagged = Untagged
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Up to { $count } listener a stream, then multicast
   *[other] Up to { $count } listeners a stream, then multicast
}
lite-link = Link
lite-bandwidth = Bandwidth
lite-egress-of = { $used } of { $link }, { $share }
lite-egress-of-assumed = { $used } of { $link }, { $share }, a gigabit link assumed
lite-egress-reported = As the entity counts its admitted streams.
lite-egress-worked-out = From the formats of its connected stream outputs.
lite-alarm-offset = PTP offset { $offset }, past the 50 µs AVB Lite allows
lite-alarm-egress = Egress at { $share } of the link, past the { $limit } streams may take

## Log

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
log-shown-of = { $shown } of { $all } frames
log-sent = Sent
log-heard = Heard
log-not-decoded = Not decoded
log-warning-short = Its control_data_length claims { $missing } octets past the end of the frame.
log-warning-undecodable = It does not decode: { $error }.
log-warning-long-acmp = It is in the long ACMP form, which a Milan entity may not send (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Channel mappings
mapping-inputs = Inputs
mapping-outputs = Outputs
mapping-port = port { $number }
mapping-fixed = fixed
mapping-not-read = Not read yet.
mapping-no-clusters = No clusters.
mapping-no-streams = No audio streams.
mapping-none = No mappings.
mapping-not-mapped = Not mapped
mapping-cluster-numbered = Cluster { $index }

## Presets

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
presets-saved = { $count ->
    [one] Saved ‘{ $name }’ with { $count } entity.
   *[other] Saved ‘{ $name }’ with { $count } entities.
}
presets-nothing-differs = Nothing differs from ‘{ $name }’.
presets-recalling = { $count ->
    [one] Recalling ‘{ $name }’: { $count } change.
   *[other] Recalling ‘{ $name }’: { $count } changes.
}
presets-missing = { $report } Not here or not read: { $missing }.
presets-deleted = Deleted ‘{ $name }’.
presets-host-note = It also keeps this computer's own talkers and listeners, and starts them again on recall.
presets-host-endpoints = { $count } on this computer
presets-starting-host = Starting this computer's talkers and listeners for ‘{ $name }’; the rest follows once they are back.

## Controls

control-numbered = Control { $index }
control-not-shown = Not shown here
control-option = Option { $number }

## Network errors

network-permission = triib needs permission to send and receive raw Ethernet frames.
network-needs-npcap = triib needs Npcap to send and receive raw Ethernet frames.
network-npcap-administrators = Npcap lets only administrators send and receive raw Ethernet frames. Run triib as administrator, or install Npcap again without its administrators only option.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own talkers and listeners

host-add-talker = Add talker
host-add-listener = Add listener
host-show-mine = Show only this computer's own talkers and listeners
host-show-all = Show every entity
host-new-talker = Host talker { $number }
host-new-listener = Host listener { $number }
host-failed = Could not add it to this computer: { $reason }
host-needs-clock = This computer's own talkers and listeners need a wired interface with a PTP hardware clock
host-no-ptp4l = ptp4l does not answer, so this computer's streams cannot keep gPTP time
host-state = State
host-streaming = Streaming
host-waiting = Waiting for a listener
host-listening = Listening
host-bound = Bound, waiting for the talker
host-unbound = Not bound
host-audio-from = Audio from
host-audio-to = Audio to
host-channels = Channels
host-silence = Silence
host-tone = Test tone
host-nowhere = Nowhere
host-default-device = Default device
host-remove = Remove from this computer
