## Language

language-name = বাংলা

## Common

common-close = বন্ধ করুন
common-more = আরও
common-keep-toolbar-shown = টুলবার সবসময় দেখান
common-auto-hide-toolbar = টুলবার স্বয়ংক্রিয়ভাবে লুকান

## Settings

settings-title = সেটিংস
settings-general = সাধারণ
settings-appearance = চেহারা
settings-language = ভাষা
settings-language-system = সিস্টেম ডিফল্ট: { $language }
settings-language-note = টেক্সট ফিল্ডে সিস্টেমের ইনপুট ভাষায় টাইপ হয়।
settings-appearance-system = সিস্টেম
settings-appearance-light = হালকা
settings-appearance-dark = গাঢ়
settings-colors = রং
settings-system-accent = সিস্টেমের অ্যাকসেন্ট রং ব্যবহার করুন
settings-accent-picked = নিচের রং থেকে triib-এর রংগুলো তৈরি হয়।
settings-accent-omarchy = Omarchy থিম { $theme } থেকে।
settings-accent-desktop = ডেস্কটপের অ্যাকসেন্ট রং থেকে।
settings-accent-none = ডেস্কটপে কোনো অ্যাকসেন্ট রং নেই, তাই নিচের রং ব্যবহার করা হচ্ছে।
settings-motion = গতি
settings-animations = অ্যানিমেশন
settings-animations-note = কিছু বদলালে জিনিসগুলো লাফিয়ে আর পিছলে আসে।
settings-animations-reduced = ডেস্কটপ কম গতি চেয়েছে, তাই triib স্থির থাকে।
settings-outside-control = বাইরের নিয়ন্ত্রণ
settings-allow-outside-control = বাইরের নিয়ন্ত্রণের অনুমতি দিন
settings-allow-outside-control-note = Claude Code-এর মতো এজেন্ট triib --mcp ব্যবহার করে আপনার AVB নেটওয়ার্ক দেখতে ও পরিবর্তন করতে পারে। প্রতিটি নতুন এজেন্টের আগে triib জিজ্ঞাসা করে।
settings-allowed-agents = অনুমোদিত: { $agents }
settings-forget-agents = ভুলে যান
agent-prompt-title = { $agent }-কে triib নিয়ন্ত্রণ করার অনুমতি দেবেন?
agent-prompt-body = { $agent } triib-এর বাইরের নিয়ন্ত্রণ ব্যবহার করতে চায়: আপনার নেটওয়ার্কের entities দেখতে ও পরিবর্তন করতে, streams সংযোগ করতে এবং formats, clock ও নাম সেট করতে। সেটিংসে গিয়ে বাইরের নিয়ন্ত্রণ বন্ধ করতে পারেন।
agent-prompt-allow = অনুমতি দিন
agent-prompt-deny = অনুমতি দেবেন না

common-cancel = বাতিল করুন
common-save = সংরক্ষণ করুন
common-not-set = সেট করা নেই
common-unnamed = নামহীন
common-none = কোনোটিই নয়
common-mac-address = MAC ঠিকানা
common-list-separator = {", "}

## Network interfaces

interface-up = চালু
interface-link-down = link বন্ধ
interface-wireless = ওয়্যারলেস
interface-hardware-clock = hardware clock
interface-hardware-clock-named = hardware clock { $clock }
interface-virtual = virtual

## Toolbar

toolbar-choose-interface = Interface বেছে নিন
toolbar-interface = নেটওয়ার্ক interface
toolbar-show-virtual = Virtual interfaces দেখান
toolbar-hide-virtual = Virtual interfaces লুকান
toolbar-connections = সংযোগ
toolbar-network = নেটওয়ার্ক
toolbar-entities = Entities
toolbar-rediscover = প্রতিটি entity-কে নিজেকে advertise করতে বলুন
toolbar-rescan = সব entities সাফ করে আবার স্ক্যান করুন
toolbar-search = Entities ও streams খুঁজুন
toolbar-presets = Presets
toolbar-log = লগ
toolbar-inspector = পরিদর্শক
toolbar-settings = সেটিংস

## The network's state, in place of a view

state-no-interface = কোনো interface নেই
state-no-interface-note = Entities খুঁজতে AVB নেটওয়ার্কের interface বেছে নিন।
state-starting = শুরু হচ্ছে
state-starting-note = { $interface } খোলা হচ্ছে।
state-listening = শুনছে
state-listening-note = { $interface }-এ entities নিজেদের advertise করলেই এখানে দেখা যায়।
state-permission-needed = অনুমতি দরকার
state-npcap-needed = Npcap দরকার
state-get-npcap = Npcap নিন
state-copy-command = কমান্ড কপি করুন
state-cannot-use = { $interface } ব্যবহার করা যাচ্ছে না
state-try-again = আবার চেষ্টা করুন

## Entity list

entities-none-yet = এখনও কোনো entity নেই
entities-none-yet-note = নেটওয়ার্কের প্রতিটি entity, তার ভূমিকা, SR classes ও clock সহ।

## Inspector

inspector-title = পরিদর্শক
inspector-entity = Entity
inspector-streams = Streams
inspector-controls = Controls
inspector-diagnostics = ডায়াগনস্টিকস
inspector-descriptors = Descriptors
inspector-select = বিস্তারিত দেখতে একটি entity বেছে নিন।
inspector-offline = { $entity } অফলাইন।
inspector-rename = নাম বদলান
inspector-name = নাম
inspector-identify = Identify
inspector-model-not-read = এর entity model পড়া হয়নি।
inspector-no-streams = কোনো stream নেই।
inspector-no-controls = দেখানোর মতো কোনো control নেই।
inspector-no-diagnostics = কোনো interface বা counter জানানো হয়নি।
inspector-reading = Descriptors পড়া হচ্ছে, এ পর্যন্ত { $count }টি।
inspector-read-failed = Entity model পড়া যায়নি: { $reason }।

entity-section = Entity
entity-name = নাম
entity-group = গ্রুপ
entity-product = পণ্য
entity-firmware = ফার্মওয়্যার
entity-serial-number = সিরিয়াল নম্বর
entity-configuration = কনফিগারেশন
entity-configuration-of = { $name } ({ $count }টির মধ্যে { $number })
entity-milan = Milan
entity-media-clock = Media clock
entity-clock-domain = Clock domain
entity-sampling-rate = Sampling rate
clock-source-numbered = Source { $index }
rate-pull = pull { $pull }

stream-inputs = Stream inputs
stream-outputs = Stream outputs
stream-max-transit-time = সর্বোচ্চ transit time { $time }

avb-interfaces = AVB interfaces
avb-interface = Interface
avb-interface-clock-identity = Clock identity
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domain { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = চলছে
avb-interface-none-reported = কিছু জানানো হয়নি
avb-interface-path = Path
avb-interface-own-grandmaster = নিজেই Grandmaster
avb-interface-hops = { $count ->
    [one] Grandmaster থেকে { $count } hop দূরে
   *[other] Grandmaster থেকে { $count } hops দূরে
}
avb-interface-link-up = Link চালু
avb-interface-link-down = Link বন্ধ
avb-interface-grandmaster-changes = Grandmaster পরিবর্তন
avb-interface-frames-sent = পাঠানো frames
avb-interface-frames-received = প্রাপ্ত frames
avb-interface-crc-errors = CRC ত্রুটি

tree-firmware = ফার্মওয়্যার { $version }
tree-descriptor-types = { $count ->
    [one] { $count } ধরনের descriptor
   *[other] { $count } ধরনের descriptor
}
tree-clock = Clock
tree-clock-source-from = { $kind }, { $location } { $index } থেকে
tree-clock-domain-using = { $source } ব্যবহার করছে
tree-clusters = { $count ->
    [one] { $count }টি cluster
   *[other] { $count }টি cluster
}
tree-maps = { $count ->
    [one] { $count }টি map
   *[other] { $count }টি map
}

advert-not-advertised = Advertise করা হয়নি
advert-identity = পরিচয়
advert-entity-id = Entity ID
advert-entity-model = Entity model
advert-roles = ভূমিকা
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
advert-valid-time = বৈধতার সময়
advert-available-index = Available index
advert-association = Association
advert-capabilities = সক্ষমতা

## Status bar

status-entities = { $count ->
    [one] { $count }টি entity
   *[other] { $count }টি entity
}
status-not-discovering = খোঁজা হচ্ছে না
status-discovering = খোঁজা হচ্ছে
status-discovering-as = { $controller } হিসেবে খোঁজা হচ্ছে
status-stopped = ত্রুটির কারণে বন্ধ
status-alarm = অ্যালার্ম
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } এবং আরও { $count }টি

## Descriptions of entities, shared by the views

role-talker = Talker { $count }
role-listener = Listener { $count }
role-controller = controller
role-none = কোনো ভূমিকা নেই
classes-a-and-b = A ও B
clock-no-gptp = gPTP নেই

read-not-read = পড়া হয়নি
read-reading = পড়া হচ্ছে, এ পর্যন্ত { $count }টি
read-ready-unreadable = প্রস্তুত, { $count }টি পড়া যায়নি
read-ready-cached = প্রস্তুত, ক্যাশ থেকে
read-ready = প্রস্তুত
read-failed = ব্যর্থ: { $reason }

milan-no = না
milan-before-1-3 = 1.3-এর আগে
milan-certified = { $version }, { $certification } প্রত্যয়িত
milan-not-certified = { $version }, প্রত্যয়িত নয়

outcome-status = স্ট্যাটাস { $status }
outcome-no-response = কোনো সাড়া নেই
outcome-not-possible = সম্ভব নয়
outcome-connect = { $talker } থেকে { $listener }-এ সংযোগ করা যায়নি: { $reason }।
outcome-disconnect = { $listener }-এর সংযোগ বিচ্ছিন্ন করা যায়নি: { $reason }।
outcome-identify = { $entity }-কে identify করা যায়নি: { $reason }।
outcome-rename = { $what }-এর নাম বদলে “{ $name }” করা যায়নি: { $reason }।
outcome-rename-group = { $entity }-এর গ্রুপের নাম বদলে “{ $name }” করা যায়নি: { $reason }।
outcome-format-streaming = { $stream }-এর format বদলানো যায়নি: এতে streaming চলছে। আগে এর সংযোগ বিচ্ছিন্ন করুন।
outcome-format = { $stream }-এর format বদলানো যায়নি: { $reason }।
outcome-sampling-rate = { $entity }-এর sampling rate বদলানো যায়নি: { $reason }।
outcome-clock-source = { $entity }-এর clock source বদলানো যায়নি: { $reason }।
outcome-map = { $entity }-এ channel map করা যায়নি: { $reason }।
outcome-unmap = { $entity }-এ channel unmap করা যায়নি: { $reason }।
outcome-control = { $entity }-এ “{ $control }” সেট করা যায়নি: { $reason }।
outcome-control-numbered = { $entity }-এ control { $index } সেট করা যায়নি: { $reason }।

stream-not-connected = সংযুক্ত নয়
stream-from = { $stream } থেকে
stream-from-receiving = { $stream } থেকে, গ্রহণ করছে
stream-from-waiting = { $stream } থেকে, Talker-এর অপেক্ষায়
stream-from-failed = { $stream } থেকে, Talker-এর reservation ব্যর্থ: { $reason }
stream-sending-to = { $destination }-এ পাঠাচ্ছে

failure-no-response = এটি সাড়া দেয়নি
failure-refused = এটি { $status } দিয়ে প্রত্যাখ্যান করেছে
failure-malformed = এর উত্তর decode হয়নি
failure-on-this-computer = এটি এই কম্পিউটারেই চলে; অন্য কম্পিউটার থেকে পড়ুন

msrp-failure-1 = যথেষ্ট bandwidth নেই
msrp-failure-2 = bridge-এর যথেষ্ট রিসোর্স নেই
msrp-failure-3 = traffic class-এর জন্য যথেষ্ট bandwidth নেই
msrp-failure-4 = stream ID অন্য একটি Talker ব্যবহার করছে
msrp-failure-5 = destination address আগে থেকেই ব্যবহৃত হচ্ছে
msrp-failure-6 = উচ্চতর rank-এর একটি stream এটিকে সরিয়ে দিয়েছে
msrp-failure-7 = জানানো latency বদলে গেছে
msrp-failure-8 = egress port AVB সক্ষম নয়
msrp-failure-9 = অন্য একটি destination address ব্যবহার করুন
msrp-failure-10 = MSRP রিসোর্স শেষ
msrp-failure-11 = MMRP রিসোর্স শেষ
msrp-failure-12 = destination address সংরক্ষণ করা যাচ্ছে না
msrp-failure-13 = অগ্রাধিকারটি SR class-এর অগ্রাধিকার নয়
msrp-failure-14 = frames মাধ্যমের জন্য খুব বড়
msrp-failure-15 = fan-in port-এর সীমা পূর্ণ
msrp-failure-16 = নিবন্ধিত stream-এর প্রথম মান বদলেছে
msrp-failure-17 = egress port-এ VLAN ব্লক করা
msrp-failure-18 = egress port-এ VLAN tagging বন্ধ
msrp-failure-19 = SR class-এর অগ্রাধিকার মেলে না
msrp-failure-unknown = অজানা কারণ
msrp-failure-at = { $reason }, bridge { $bridge }-এ

## Entity list columns

column-vendor = নির্মাতা
column-model = মডেল
column-state = অবস্থা
column-entity-model-id = Entity model ID
column-talker-streams = Talker streams
column-listener-streams = Listener streams
column-avb-lite = AVB Lite
column-egress = Egress
column-wireless = Wireless

## Settings file

settings-no-place = সেটিংস রাখার কোনো জায়গা নেই: হোম ফোল্ডার জানা নেই।
settings-unusable = { $path } ব্যবহার করা যায়নি: { $error }।
settings-unsaved = { $path } সংরক্ষণ করা যায়নি: { $error }।

column-remove = কলাম সরান
column-move-left = বাঁয়ে সরান
column-move-right = ডানে সরান
column-add = কলাম যোগ করুন
common-percent = { $value }%

## Network view

netmap-empty = দেখানোর মতো এখনও কোনো নেটওয়ার্ক নেই
netmap-empty-note = Entities পড়া হলে এবং gPTP tree-তে নিজেদের অবস্থান জানালে এখানে দেখা যায়।
netmap-focus-clock-path = { $name }-এর clock path
netmap-focus-streams = { $name }-এর streams
netmap-showing = দেখানো হচ্ছে: { $what }
netmap-devices = { $count ->
    [one] { $count }টি ডিভাইস
   *[other] { $count }টি ডিভাইস
}
netmap-bridges = { $count ->
    [one] { $count }টি bridge
   *[other] { $count }টি bridge
}
netmap-show-map = ম্যাপ দেখান
netmap-show-details = বিস্তারিত দেখান
stream-numbered = Stream { $index }
netmap-bridge = Bridge
netmap-access-point = Access point
netmap-device = ডিভাইস
netmap-this-computer = এই কম্পিউটার
netmap-connected = সংযুক্ত
netmap-advertised = Advertise করা হয়েছে, কোনো Listener প্রস্তুত নেই
netmap-advertised-off-tree = Advertise করা হয়েছে, কোনো Listener প্রস্তুত নেই ({ $listener } gPTP tree-তে নেই)
netmap-failed-at = { $bridge }-এ reservation ব্যর্থ: { $reason }
netmap-failed = Reservation ব্যর্থ: { $reason }
netmap-no-bridge-on = { $interface }-এ কোনো bridge শোনা যায়নি
netmap-cannot-listen-on = { $interface }-এ gPTP শোনা সম্ভব নয়
netmap-on-this-computer = এই কম্পিউটারে
netmap-path-not-reported = Path জানানো হয়নি
netmap-gptp-not-reported = gPTP জানানো হয়নি
netmap-off-tree = gPTP tree-তে নেই
netmap-off-ptp = PTP tree-তে নেই
netmap-not-lite = AVB Lite-এ নেই
netmap-lite-not-reported = AVB Lite জানানো হয়নি
netmap-synced = সিঙ্ক হয়েছে
netmap-not-synced = সিঙ্ক নেই
netmap-triib-on = { $interface }-এ triib
netmap-through-count = { $count }টি পার হচ্ছে
netmap-out = { $count }টি বাইরে
netmap-in = { $count }টি ভেতরে
netmap-failed-count = { $count }টি ব্যর্থ
netmap-advertised-only = শুধু advertise করা হয়েছে
netmap-failed-state = ব্যর্থ
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = gPTP tree-তে নেই: এটি নিজেই Grandmaster
netmap-apart-no-path = এর path জানানো হয়নি; এটি Grandmaster { $grandmaster }-কে অনুসরণ করে
netmap-apart-unreported = এটি নিজের gPTP অবস্থা জানায়নি
netmap-apart-no-neighbor = এই কম্পিউটারের interface-এ কোনো bridge শোনা যায়নি
netmap-apart-cannot-listen = এই কম্পিউটার তার interface-এ gPTP শুনতে পারে না
netmap-apart-on-this-computer = এটি এই কম্পিউটারেই চলে; এর gPTP অবস্থা দেখতে অন্য কম্পিউটার থেকে পড়ুন
netmap-apart-not-lite = এটি AVB Lite চালায় না, তাই Grandmaster-কে অনুসরণ করে না
netmap-apart-lite-unreported = এটি AVB Lite সম্পর্কে কিছুই জানায় না, তাই এটি কাকে অনুসরণ করে তা জানা নেই
netmap-clock-tree = Clock tree
netmap-no-grandmaster = কোনো Grandmaster শোনা যায়নি
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = মনোযোগ দরকার
netmap-nodes-below = নিচের nodes
netmap-bridges-below = নিচের bridges
netmap-clock-path = Clock path
netmap-hops = Grandmaster থেকে hops
netmap-link-delay = Link delay
netmap-bridge-port = Bridge port
netmap-link-drops = Link drops
netmap-synced-to-grandmaster = Grandmaster-এর সঙ্গে সিঙ্ক হয়েছে
netmap-host-no-gptp = সিঙ্ক নেই: এই কম্পিউটারে gPTP চলে না
netmap-link-no-gptp = সিঙ্ক নেই: এর link-এ gPTP চলে না
netmap-ptp-offset-high = সিঙ্ক নেই: Grandmaster থেকে { $offset }, AVB Lite-এর অনুমোদিত 50 µs ছাড়িয়ে
netmap-ptp-no-offset = সিঙ্ক নেই: এটি Grandmaster থেকে কোনো offset মাপেনি
netmap-audio = অডিও
netmap-media-clock-streams = Media clock streams
netmap-audio-streams = অডিও streams
netmap-bound = { $count }টি আবদ্ধ
netmap-flowing = চলছে
netmap-advertised-state = Advertise করা হয়েছে
netmap-media-clock-stream = Media clock stream
netmap-audio-stream = অডিও stream
netmap-reaches = পৌঁছেছে
netmap-passing-count = { $count ->
    [one] { $count }টি stream পার হচ্ছে
   *[other] { $count }টি stream পার হচ্ছে
}
netmap-through = পার হচ্ছে
netmap-passing-through = পার হওয়া streams
netmap-sending = পাঠাচ্ছে
netmap-receiving = গ্রহণ করছে
netmap-problems = সমস্যা
netmap-help-back = পুরো ম্যাপে ফিরতে ফাঁকা জায়গায় ক্লিক করুন।
netmap-help-stream = কোনো stream পরীক্ষা করতে তাতে ক্লিক করুন, বা পুরো ম্যাপে ফিরতে ফাঁকা জায়গায়।
netmap-help-ptp = AVB Lite-এ clock প্রান্ত থেকে প্রান্তে, Grandmaster থেকে প্রতিটি ডিভাইসে যায়, এমন switch-এর মধ্য দিয়ে যেগুলো এতে অংশ নেয় না, তাই কোনো switch দেখানো হয় না। কোনো ডিভাইস যতক্ষণ 50 µs-এর মধ্যে Grandmaster-কে অনুসরণ করে, ততক্ষণ সেটি সিঙ্কে থাকে। কোনো ডিভাইস বা তার তারে ক্লিক করে তার clock পরীক্ষা করুন; নির্বাচন মুছতে ফাঁকা জায়গায় ক্লিক করুন।
netmap-help-clock = Clock, Grandmaster থেকে প্রতিটি bridge হয়ে tree-র প্রতিটি node-এ যায়। ভাঙা ধূসর রেখা এমন একটি link, যাতে gPTP চলে না। কোনো ডিভাইস বা তার তারে ক্লিক করে তার clock path পরীক্ষা করুন; নির্বাচন মুছতে ফাঁকা জায়গায় ক্লিক করুন।
netmap-help-media-clock = শুধু media clock (CRF) streams, অডিওর মতোই আঁকা: প্রতিটি stream-এর একটি তার, Talker অনুযায়ী রঙিন। কোনো তারে ক্লিক করে তার stream পরীক্ষা করুন, বা কোনো ডিভাইসে ক্লিক করে তার streams দেখুন; নির্বাচন মুছতে ফাঁকা জায়গায় ক্লিক করুন।
netmap-help-audio = প্রতিটি stream-এর নিজস্ব তার আছে, যা পথের প্রতিটি bridge-এ ঢোকে ও বেরোয়। রং Talker অনুযায়ী: প্রতিটি Talker-এর একটি রং, আর তার streams সেই রঙেরই নানা শেড। চলমান বিন্দু মানে অডিও চলছে; স্থির লাল রেখা মানে ব্যর্থ reservation, আর স্থির ধূসর রেখা মানে advertise করা এমন stream, যার জন্য কোনো Listener প্রস্তুত নেই; দুটোই সেখানে থামে যেখানে reservation থামে। মাঝের কলামের ডিভাইসগুলো সরাসরি Grandmaster-এর bridge-এ যুক্ত। কোনো তারে ক্লিক করে তার stream পরীক্ষা করুন, বা কোনো ডিভাইসে ক্লিক করে তার streams দেখুন; নির্বাচন মুছতে ফাঁকা জায়গায় ক্লিক করুন।

## Connections

matrix-nothing-shown = দেখানোর মতো কোনো stream নেই
matrix-nothing-shown-note = আরও streams দেখতে খোঁজ বা ফিল্টার বদলান।
matrix-empty = সংযোগ করার মতো কোনো stream নেই
matrix-empty-note = Talker streams ও Listener streams থাকা entities পড়া হলে সেগুলো এখানে মুখোমুখি হয়।
matrix-all-streams = সব streams
matrix-connectable-only = যেগুলো সংযোগ করা যায় না, সেগুলো লুকান
matrix-none-hidden = দেখানো প্রতিটি stream সংযোগ করা যায়
matrix-hidden = { $count ->
    [one] { $count }টি stream লুকানো
   *[other] { $count }টি stream লুকানো
}
matrix-own = কোনো entity-র outputs তার নিজের inputs-এ সংযুক্ত হয় না।
matrix-working = কাজ চলছে।
matrix-waiting-change = এই input-এ শেষ পরিবর্তনের অপেক্ষা।
matrix-connected = সংযুক্ত এবং গ্রহণ করছে। সংযোগ বিচ্ছিন্ন করতে ক্লিক করুন।
matrix-bound-waiting = আবদ্ধ, Talker-এর stream-এর অপেক্ষায়। সংযোগ বিচ্ছিন্ন করতে ক্লিক করুন।
matrix-bound-failed = আবদ্ধ, কিন্তু Talker-এর reservation ব্যর্থ: { $reason }। সংযোগ বিচ্ছিন্ন করতে ক্লিক করুন।
matrix-bound-formats-differ = আবদ্ধ, কিন্তু formats আলাদা: Talker পাঠায় { $sent }, input সেট করা আছে { $set }-এ। সংযোগ বিচ্ছিন্ন করতে ক্লিক করুন।
matrix-formats-match = Formats মেলে ({ $format })। সংযোগ করতে ক্লিক করুন।
matrix-format-must-change = Input { $sent } নেয়, কিন্তু সেট করা আছে { $set }-এ, তাই format না বদলানো পর্যন্ত হয়তো বাজবে না। একটি format বেছে নিতে ক্লিক করুন।
matrix-format-input = Input বদলে করুন
matrix-format-both = দুটোই বদলে করুন
matrix-format-talker-busy = Talker অন্য listener-দের কাছে stream করছে, তাই এর format একই থাকবে।
matrix-format-anyway = Format না বদলে সংযোগ করুন
matrix-incompatible = Input { $sent } নেয় না। এটি { $set }-এ সেট করা।
matrix-group-none = সংযুক্ত নয়। একটি একটি করে streams সংযোগ করতে প্রসারিত করুন।
matrix-group-connected = { $count }টি সংযুক্ত। প্রতিটি দেখতে প্রসারিত করুন।
matrix-outputs-expand = { $count ->
    [one] { $count }টি stream output। প্রসারিত করতে তীরে, পরীক্ষা করতে নামে ক্লিক করুন।
   *[other] { $count }টি stream output। প্রসারিত করতে তীরে, পরীক্ষা করতে নামে ক্লিক করুন।
}
matrix-outputs-collapse = { $count ->
    [one] { $count }টি stream output। সংকুচিত করতে তীরে, পরীক্ষা করতে নামে ক্লিক করুন।
   *[other] { $count }টি stream output। সংকুচিত করতে তীরে, পরীক্ষা করতে নামে ক্লিক করুন।
}
matrix-inputs-expand = { $count ->
    [one] { $count }টি stream input। প্রসারিত করতে তীরে, পরীক্ষা করতে নামে ক্লিক করুন।
   *[other] { $count }টি stream input। প্রসারিত করতে তীরে, পরীক্ষা করতে নামে ক্লিক করুন।
}
matrix-inputs-collapse = { $count ->
    [one] { $count }টি stream input। সংকুচিত করতে তীরে, পরীক্ষা করতে নামে ক্লিক করুন।
   *[other] { $count }টি stream input। সংকুচিত করতে তীরে, পরীক্ষা করতে নামে ক্লিক করুন।
}
matrix-stream-inspect = { $detail } { $entity } পরীক্ষা করতে ক্লিক করুন।
matrix-point = কোনো সেলে পয়েন্টার রাখুন
matrix-point-note = তার Talker ও Listener দেখতে এবং তাদের formats মেলে কিনা জানতে।
matrix-legend-waiting = আবদ্ধ, stream-এর অপেক্ষায়
matrix-legend-trouble = আবদ্ধ, কিছু একটা সমস্যা
matrix-legend-open = সংযোগ করা যায়
matrix-legend-change = আগে input format বদলাতে হবে
matrix-legend-incompatible = Formats মিলতে পারে না
matrix-talker-outputs = Talker outputs
matrix-listener-inputs = Listener inputs

common-thousands-separator = {","}

## Diagnostics

diag-since-start = Entity চালু হওয়ার পর থেকে গোনা।
diag-stream-input = Stream input
diag-stream-output = Stream output
diag-locked = { $count ->
    [0] lock হয়নি
    [one] একবার lock হয়েছে
   *[other] { $number } বার lock হয়েছে
}
diag-lost-lock = { $count ->
    [0] lock হারায়নি
    [one] একবার lock হারিয়েছে
   *[other] { $number } বার lock হারিয়েছে
}
diag-frames-in = { $count ->
    [one] { $number }টি frame এসেছে
   *[other] { $number }টি frame এসেছে
}
diag-frames-out = { $count ->
    [one] { $number }টি frame পাঠানো হয়েছে
   *[other] { $number }টি frame পাঠানো হয়েছে
}
diag-media-locked = { $count ->
    [0] media lock হয়নি
    [one] একবার media lock হয়েছে
   *[other] { $number } বার media lock হয়েছে
}
diag-lost-media-lock = { $count ->
    [0] media lock হারায়নি
    [one] একবার media lock হারিয়েছে
   *[other] { $number } বার media lock হারিয়েছে
}
diag-interrupted = { $count ->
    [0] ব্যাহত হয়নি
    [one] একবার ব্যাহত হয়েছে
   *[other] { $number } বার ব্যাহত হয়েছে
}
diag-out-of-sequence = { $count ->
    [one] { $number }টি frame ক্রমের বাইরে
   *[other] { $number }টি frame ক্রমের বাইরে
}
diag-media-resets = { $count ->
    [one] { $number }টি media reset
   *[other] { $number }টি media reset
}
diag-timestamps-uncertain = { $count ->
    [0] timestamps অনিশ্চিত হয়নি
    [one] একবার timestamps অনিশ্চিত
   *[other] { $number } বার timestamps অনিশ্চিত
}
diag-no-timestamp = { $count ->
    [one] timestamp ছাড়া { $number }টি frame
   *[other] timestamp ছাড়া { $number }টি frame
}
diag-unsupported-format = { $count ->
    [one] অসমর্থিত format-এ { $number }টি frame
   *[other] অসমর্থিত format-এ { $number }টি frame
}
diag-late = { $count ->
    [one] { $number }টি frame দেরিতে
   *[other] { $number }টি frame দেরিতে
}
diag-early = { $count ->
    [one] { $number }টি frame আগেভাগে
   *[other] { $number }টি frame আগেভাগে
}
diag-started = { $count ->
    [0] শুরু হয়নি
    [one] একবার শুরু হয়েছে
   *[other] { $number } বার শুরু হয়েছে
}
diag-stopped = { $count ->
    [0] থামেনি
    [one] একবার থেমেছে
   *[other] { $number } বার থেমেছে
}
diag-reservation-failed = Talker-এর reservation ব্যর্থ: { $reason }
diag-latency = { $microseconds } µs সঞ্চিত latency

## AVB Lite

lite-active = সক্রিয়
lite-active-untagged = সক্রিয়, untagged
lite-active-vlan = সক্রিয়, VLAN { $vlan }
lite-capable = সক্ষম
lite-mode = মোড
lite-mode-capable = AVB (AVB Lite সক্ষম)
lite-because = কারণ
lite-fallback-none = কোনো কারণ জানানো হয়নি
lite-fallback-endpoint = অন্য একটি endpoint-এর declaration এসে পৌঁছেছে, তাই তাদের মাঝে কোনো AVB bridge নেই
lite-fallback-unanswered = নয়টি peer delay অনুরোধের উত্তর আসেনি
lite-fallback-responders = একটি peer delay অনুরোধে দুই বা তার বেশি সাড়া এসেছে, তাই switch-টি AVB bridge নয়
lite-fallback-configured = অপারেটর বা কোনো controller এটি সেট করেছে
lite-fallback-other = এমন কারণ, যার নাম profile-এ নেই
lite-other-profile = অন্য profile
lite-ptp-domain = { $profile }, domain { $domain }
lite-offset = Offset
lite-offset-from = { $grandmaster } থেকে { $offset }
lite-media-vlan = Media VLAN
lite-untagged = Untagged
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] প্রতি stream { $count }টি Listener পর্যন্ত, তারপর multicast
   *[other] প্রতি stream { $count }টি Listener পর্যন্ত, তারপর multicast
}
lite-link = Link
lite-bandwidth = Bandwidth
lite-egress-of = { $link }-এর মধ্যে { $used }, { $share }
lite-egress-of-assumed = { $link }-এর মধ্যে { $used }, { $share }, gigabit link ধরে নিয়ে
lite-egress-reported = Entity নিজে তার গৃহীত streams যেভাবে গোনে।
lite-egress-worked-out = এর সংযুক্ত stream outputs-এর formats থেকে হিসাব করা।
lite-alarm-offset = PTP offset { $offset }, AVB Lite-এর অনুমোদিত 50 µs ছাড়িয়ে
lite-alarm-egress = Egress link-এর { $share }, streams-এর জন্য অনুমোদিত { $limit } ছাড়িয়ে

## AVB Wireless

wireless-station = Station
wireless-access-point = Access point
wireless-role = ভূমিকা
wireless-mode = মোড
wireless-time = সময়
wireless-mode-a-ftm = Mode A, FTM-এর মাধ্যমে 802.1AS
wireless-mode-a-tm = Mode A, TM-এর মাধ্যমে 802.1AS
wireless-mode-b = Mode B, beacon থেকে
wireless-no-time = সময় নেই
wireless-other-mode = এমন মোড, যার নাম profile-এ নেই
wireless-locked = Lock হয়েছে
wireless-holdover = Holdover-এ আছে
wireless-not-locked = Lock হয়নি
wireless-row-locked = Station, lock হয়েছে
wireless-row-holdover = Station, holdover-এ
wireless-row-not-locked = Station, lock হয়নি
wireless-row-access-point = { $count ->
    [one] Access point, { $count }টি station
   *[other] Access point, { $count }টি station
}
wireless-link = Link
wireless-channel = চ্যানেল { $channel }
wireless-not-known = অজানা
wireless-signal = সিগন্যাল
wireless-rate = Rate
wireless-ftm-valid = { $share } বৈধ
wireless-rtt = round trip { $rtt }
wireless-bursts = { $count ->
    [one] { $count }টি frame-এর burst
   *[other] { $count }টি frame-এর burst
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = access point তিন বা দুই ছাড়া অন্য সংখ্যক frame-এর FTM burst দেয়
wireless-reason-measurement = access point-এর সঙ্গে FTM বা TM কোনোটিই নেই
wireless-reason-signaling = access point থেকে কোনো gPTP-capable Signaling আসেনি
wireless-reason-other = এমন কারণ, যার নাম profile-এ নেই
wireless-servo = Servo ত্রুটি
wireless-stations = Stations
wireless-station-count = { $count ->
    [one] { $count }টি station
   *[other] { $count }টি station
}
wireless-no-ftm = FTM ছাড়া
wireless-unserved = সেবা না পাওয়া Listeners
wireless-frame-budget = Frame budget
wireless-frame-use = প্রতি সেকেন্ডে { $budget }টির মধ্যে { $used }টি frame
wireless-stream-frames = Stream frames
wireless-frames-of = { $readdressed } station-এ, { $unmapped } Listener ছাড়া, { $dropped } drop হয়েছে, { $restored } station থেকে
wireless-class-a-allowed = অনুমোদিত
wireless-class-a-not-allowed = অনুমোদিত নয়
wireless-alarm-not-locked = Wi-Fi সময় access point-এর সঙ্গে lock হয়নি
wireless-alarm-holdover = Wi-Fi সময় holdover-এ, access point-এর সঙ্গে lock হারিয়েছে
wireless-alarm-unserved = { $count ->
    [one] Wi-Fi port-এ { $count }টি Listener সেবা পায়নি, unicast সীমা ছাড়িয়ে
   *[other] Wi-Fi port-এ { $count }টি Listener সেবা পায়নি, unicast সীমা ছাড়িয়ে
}

## Log

log-all = সব
log-warnings = সতর্কতা
log-pause = থামান
log-resume = আবার চালু করুন
log-clear = সাফ করুন
log-empty = triib যত ATDECC frame পাঠায় ও শোনে, সবই এখানে দেখা যায়, নতুনটি সবার আগে।
log-none-match = রাখা কোনো frame ফিল্টারের সঙ্গে মেলে না।
log-frames = { $count ->
    [one] { $count }টি frame
   *[other] { $count }টি frame
}
log-shown-of = { $all }টির মধ্যে { $shown }টি frame
log-sent = পাঠানো
log-heard = শোনা
log-not-decoded = Decode হয়নি
log-warning-short = এর control_data_length frame-এর শেষের পরেও { $missing } octets দাবি করে।
log-warning-undecodable = এটি decode হয় না: { $error }।
log-warning-long-acmp = এটি দীর্ঘ ACMP রূপে আছে, যা পাঠানোর অনুমতি Milan entity-র নেই (Milan 1.3, 5.5.2.2)।

## Channel mappings

mapping-section = Channel mappings
mapping-inputs = Inputs
mapping-outputs = Outputs
mapping-port = port { $number }
mapping-fixed = স্থির
mapping-not-read = এখনও পড়া হয়নি।
mapping-no-clusters = কোনো cluster নেই।
mapping-no-streams = কোনো অডিও stream নেই।
mapping-none = কোনো mapping নেই।
mapping-not-mapped = Map করা হয়নি
mapping-cluster-numbered = Cluster { $index }

## Presets

presets-note = Preset প্রতিটি entity-র clock sources, sampling rates, stream formats, controls ও সংযোগ রাখে। Recall করলে যা আলাদা, তা বদলে যায়।
presets-none = এখনও কোনো preset সংরক্ষণ করা হয়নি।
presets-connections = { $count ->
    [one] { $count }টি সংযোগ
   *[other] { $count }টি সংযোগ
}
presets-recall = Recall
presets-delete = মুছুন
presets-no-place = Presets রাখার কোনো জায়গা নেই: হোম ফোল্ডার জানা নেই।
presets-undeletable = { $path } মোছা যায়নি: { $error }।
presets-saved = { $count ->
    [one] { $count }টি entity সহ “{ $name }” সংরক্ষণ করা হয়েছে।
   *[other] { $count }টি entity সহ “{ $name }” সংরক্ষণ করা হয়েছে।
}
presets-nothing-differs = “{ $name }” থেকে কিছুই আলাদা নয়।
presets-recalling = { $count ->
    [one] “{ $name }” recall করা হচ্ছে: { $count }টি পরিবর্তন।
   *[other] “{ $name }” recall করা হচ্ছে: { $count }টি পরিবর্তন।
}
presets-missing = { $report } এখানে নেই বা পড়া হয়নি: { $missing }।
presets-deleted = “{ $name }” মোছা হয়েছে।
presets-host-note = এটি এই কম্পিউটারের নিজস্ব Talker ও Listener-ও রাখে, এবং recall করলে সেগুলো আবার চালু করে।
presets-host-endpoints = এই কম্পিউটারে { $count }টি
presets-starting-host = “{ $name }”-এর জন্য এই কম্পিউটারের Talker ও Listener চালু হচ্ছে; সেগুলো ফিরে এলে বাকিটা হবে।

## Controls

control-numbered = Control { $index }
control-not-shown = এখানে দেখানো হয় না
control-option = বিকল্প { $number }

## Network errors

network-permission = raw Ethernet frames পাঠাতে ও গ্রহণ করতে triib-এর অনুমতি দরকার।
network-needs-npcap = raw Ethernet frames পাঠাতে ও গ্রহণ করতে triib-এর Npcap দরকার।
network-npcap-administrators = Npcap শুধু administrator-দের raw Ethernet frames পাঠাতে ও গ্রহণ করতে দেয়। triib administrator হিসেবে চালান, অথবা শুধু-administrator বিকল্প ছাড়া Npcap আবার ইনস্টল করুন।

matrix-stream-format = { $format }।
matrix-stream-format-state = { $format }। { $state }।

common-decimal-separator = {"."}

## This computer's own talkers and listeners

host-add-talker = Talker যোগ করুন
host-add-listener = Listener যোগ করুন
host-add-device = Device যোগ করুন
host-show-mine = শুধু এই কম্পিউটারের নিজস্ব Talker ও Listener দেখান
host-show-all = সব entity দেখান
host-new-talker = হোস্ট Talker { $number }
host-new-listener = হোস্ট Listener { $number }
host-new-device = হোস্ট Device { $number }
host-failed = এই কম্পিউটারে যোগ করা যায়নি: { $reason }
host-needs-clock = এই কম্পিউটারের নিজস্ব Talker ও Listener-এর জন্য PTP hardware clock-সহ তারযুক্ত interface দরকার
host-no-ptp4l = ptp4l সাড়া দিচ্ছে না, তাই এই কম্পিউটারের stream gPTP সময় ধরে রাখতে পারে না
host-elsewhere = triib-endpointd { $interface }-এ অন্য একজন ব্যবহারকারীর জন্য বা root হিসেবে চলছে, তাই এই কম্পিউটারের Talker ও Listener সেখানে চলে, এখানে নয়
host-foreign-mrp = অন্য একটি প্রোগ্রাম এই কম্পিউটারের ঠিকানা থেকে { $interface }-এ MSRP বা MVRP declaration পাঠাচ্ছে, তাতে এই কম্পিউটারের stream-এর যা দরকার তা প্রত্যাহার হয়ে যেতে পারে
host-alarm-foreign-mrp = এই কম্পিউটারের অন্য একটি প্রোগ্রাম { $interface }-এ MSRP বা MVRP declaration পাঠাচ্ছে
host-state = অবস্থা
host-streaming = stream চলছে
host-waiting = Listener-এর অপেক্ষায়
host-listening = শুনছে
host-bound = বাঁধা, Talker-এর অপেক্ষায়
host-unbound = বাঁধা নয়
host-audio-from = অডিওর উৎস
host-audio-to = অডিওর গন্তব্য
host-channels = চ্যানেল
host-virtual-device = ভার্চুয়াল device
host-add-output-stream = Output stream যোগ করুন
host-add-input-stream = Input stream যোগ করুন
host-remove-stream = Stream সরান
host-silence = নীরবতা
host-tone = পরীক্ষামূলক টোন
host-nowhere = কোথাও নয়
host-default-device = ডিফল্ট ডিভাইস
host-remove = এই কম্পিউটার থেকে সরান
