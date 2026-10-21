## Language

language-name = हिन्दी

## Common

common-close = बंद करें
common-more = ज़्यादा
common-keep-toolbar-shown = टूलबार हमेशा दिखाएँ
common-auto-hide-toolbar = टूलबार अपने-आप छिपाएँ

## Settings

settings-title = सेटिंग्स
settings-general = सामान्य
settings-appearance = रूप-रंग
settings-language = भाषा
settings-language-system = सिस्टम डिफ़ॉल्ट: { $language }
settings-language-note = टेक्स्ट फ़ील्ड में सिस्टम की इनपुट भाषा में टाइप होता है।
settings-appearance-system = सिस्टम
settings-appearance-light = हल्का
settings-appearance-dark = गहरा
settings-colors = रंग
settings-system-accent = सिस्टम का एक्सेंट रंग इस्तेमाल करें
settings-accent-picked = नीचे वाले रंग से triib के रंग बनते हैं।
settings-accent-omarchy = Omarchy थीम { $theme } से।
settings-accent-desktop = डेस्कटॉप के एक्सेंट रंग से।
settings-accent-none = डेस्कटॉप में कोई एक्सेंट रंग नहीं है, इसलिए नीचे वाला रंग इस्तेमाल होता है।
settings-motion = गति
settings-animations = ऐनिमेशन
settings-animations-note = बदलाव होने पर चीज़ें उछलकर और सरककर आती हैं।
settings-animations-reduced = डेस्कटॉप ने कम गति माँगी है, इसलिए triib स्थिर रहता है।
settings-outside-control = बाहरी नियंत्रण
settings-allow-outside-control = बाहरी नियंत्रण की अनुमति दें
settings-allow-outside-control-note = Claude Code जैसे एजेंट triib --mcp के ज़रिए आपका AVB नेटवर्क देख और बदल सकते हैं। हर नए एजेंट से पहले triib पूछता है।
settings-allowed-agents = अनुमति प्राप्त: { $agents }
settings-forget-agents = भूल जाएँ
settings-storage = स्टोरेज
settings-storage-presets = Presets फ़ोल्डर
settings-storage-entities = Entity फ़ाइल
settings-storage-apply = लागू करें
settings-storage-choose = चुनें…
settings-storage-note = पुरानी जगह पर पहले से रखे Presets वहीं रहते हैं; उनका उपयोग जारी रखने के लिए उन्हें नई जगह पर ले जाएँ। triib की सेटिंग्स { $file } में सहेजी जाती हैं।
settings-storage-entities-note = जो Entity फ़ाइल अभी नहीं है वह इस्तेमाल हो रही फ़ाइल की कॉपी के रूप में शुरू होती है, और triib-endpointd उस पर फिर से शुरू होता है।
dialog-choose-presets = Presets फ़ोल्डर चुनें
dialog-choose-entities = Entity फ़ाइल का फ़ोल्डर चुनें
settings-full-path = पूरा पाथ इस्तेमाल करें, जैसे ~/Documents/triib।
settings-path-is-folder = { $path } एक फ़ोल्डर है, फ़ाइल नहीं।
settings-folder-missing = { $path } नाम का कोई फ़ोल्डर नहीं है। पहले उसे बनाएँ, या कोई फ़ोल्डर चुनें।
settings-path-is-file = { $path } एक फ़ाइल है, फ़ोल्डर नहीं।
settings-cannot-write = triib { $path } में नहीं लिख सकता: { $error }।
app-file-dialog-failed = फ़ाइल डायलॉग नहीं दिखाया जा सका: { $error }
agent-prompt-title = { $agent } को triib नियंत्रित करने की अनुमति दें?
agent-prompt-body = { $agent } triib के बाहरी नियंत्रण का उपयोग करना चाहता है: आपके नेटवर्क की entities देखने और बदलने, streams जोड़ने और formats, clock व नाम सेट करने के लिए। आप सेटिंग्स में बाहरी नियंत्रण बंद कर सकते हैं।
agent-prompt-allow = अनुमति दें
agent-prompt-deny = अनुमति न दें

common-cancel = रद्द करें
common-save = सहेजें
common-not-set = सेट नहीं है
common-unnamed = बेनाम
common-none = कोई नहीं
common-mac-address = MAC पता
common-list-separator = {", "}

## Network interfaces

interface-up = चालू
interface-link-down = link बंद
interface-wireless = वायरलेस
interface-hardware-clock = hardware clock
interface-hardware-clock-named = hardware clock { $clock }
interface-virtual = virtual

## Toolbar

toolbar-choose-interface = Interface चुनें
toolbar-interface = नेटवर्क interface
toolbar-filter-interfaces = Interfaces फ़िल्टर करें
toolbar-hide-virtual = Virtual interfaces छिपाएँ
toolbar-hide-wireless = वायरलेस interfaces छिपाएँ
toolbar-hide-unclocked = बिना hardware clock वाले interfaces छिपाएँ
toolbar-connections = कनेक्शन
toolbar-network = नेटवर्क
toolbar-entities = Entities
toolbar-rediscover = हर entity से खुद को advertise करने को कहें
toolbar-rescan = सभी entities साफ़ करके दोबारा स्कैन करें
toolbar-search = Entities और streams खोजें
toolbar-presets = Presets
toolbar-log = लॉग
toolbar-inspector = निरीक्षक
toolbar-settings = सेटिंग्स
toolbar-close-window = विंडो बंद करें

## The macOS menu bar, named as in macOS's own apps

menu-about = triib के बारे में
menu-settings = सेटिंग्स…
menu-services = सेवाएँ
menu-hide = triib छिपाएँ
menu-hide-others = अन्य को छिपाएँ
menu-show-all = सभी दिखाएँ
menu-quit = triib बंद करें
menu-file = फ़ाइल
menu-presets = Presets…
menu-close-window = विंडो बंद करें
menu-edit = संपादित करें
menu-cut = कट करें
menu-copy = कॉपी करें
menu-paste = पेस्ट करें
menu-select-all = सभी चुनें
menu-view = देखें
menu-full-screen = पूर्ण स्क्रीन में प्रवेश करें
menu-window = विंडो
menu-minimize = छोटा करें
menu-zoom = ज़ूम करें
menu-bring-all-to-front = सभी को सामने लाएँ
menu-help = सहायता
menu-website = triib वेबसाइट
menu-release-notes = रिलीज़ नोट्स

## The network's state, in place of a view

state-no-interface = कोई interface नहीं
state-no-interface-note = Entities खोजने के लिए AVB नेटवर्क वाला interface चुनें।
state-starting = शुरू हो रहा है
state-starting-note = { $interface } खोला जा रहा है।
state-listening = सुन रहा है
state-listening-note = { $interface } पर entities खुद को advertise करते ही यहाँ दिखती हैं।
state-permission-needed = अनुमति चाहिए
state-npcap-needed = Npcap चाहिए
state-get-npcap = Npcap पाएँ
state-copy-command = कमांड कॉपी करें
state-cannot-use = { $interface } इस्तेमाल नहीं किया जा सकता
state-try-again = फिर से कोशिश करें

## Entity list

entities-none-yet = अभी कोई entity नहीं
entities-none-yet-note = नेटवर्क की हर entity, उसकी भूमिकाओं, SR classes और clock के साथ।

## Inspector

inspector-title = निरीक्षक
inspector-entity = Entity
inspector-streams = Streams
inspector-controls = Controls
inspector-diagnostics = निदान
inspector-descriptors = Descriptors
inspector-select = विवरण देखने के लिए कोई entity चुनें।
inspector-offline = { $entity } ऑफ़लाइन है।
inspector-heard = यह अपनी घोषणा नहीं करता: triib ने इसे इसके भेजे कमांड में सुना, और बिना कमांड के 5 मिनट बीतने तक सूची में रखता है।
inspector-rename = नाम बदलें
inspector-name = नाम
inspector-identify = Identify
inspector-model-not-read = इसका entity model पढ़ा नहीं गया है।
inspector-no-streams = कोई stream नहीं।
inspector-no-controls = दिखाने के लिए कोई control नहीं।
inspector-no-diagnostics = कोई interface या counter रिपोर्ट नहीं हुआ।
inspector-reading = Descriptors पढ़े जा रहे हैं, अब तक { $count }।
inspector-read-failed = Entity model पढ़ा नहीं जा सका: { $reason }।

entity-section = Entity
entity-name = नाम
entity-triib-controller = triib controller
entity-triib-on = { $host } पर triib
entity-triib-here = इस कंप्यूटर पर triib
entity-controller-on = { $host } पर controller
entity-controller-here = इस कंप्यूटर पर controller
entity-controller-at = { $mac } पर controller
entity-group = समूह
entity-product = उत्पाद
entity-firmware = फ़र्मवेयर
entity-serial-number = सीरियल नंबर
entity-configuration = कॉन्फ़िगरेशन
entity-configuration-of = { $name } ({ $count } में से { $number })
entity-configuration-holds = { $inputs } इन, { $outputs } आउट, { $channels } चैनल वाले streams
entity-configuration-holds-mixed = { $inputs } इन, { $outputs } आउट
entity-configuration-numbered = कॉन्फ़िगरेशन { $number }
entity-milan = Milan
entity-media-clock = Media clock
entity-clock-domain = Clock domain
entity-sampling-rate = Sampling rate
clock-source-numbered = Source { $index }
rate-pull = pull { $pull }

stream-format = Format
stream-id = Stream ID
stream-destination = गंतव्य
stream-max-transit = अधिकतम transit time
stream-listeners = Listeners
stream-listener-gone = { $entity } इनपुट { $input }, ऑनलाइन नहीं
stream-disconnect-listener = डिसकनेक्ट करें

avb-interfaces = AVB interfaces
avb-interface = Interface
avb-interface-clock-identity = Clock identity
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domain { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = चल रहे
avb-interface-none-reported = कोई रिपोर्ट नहीं
avb-interface-path = Path
avb-interface-own-grandmaster = खुद ही Grandmaster
avb-interface-hops = { $count ->
    [one] Grandmaster से { $count } hop
   *[other] Grandmaster से { $count } hops
}
avb-interface-link-up = Link चालू
avb-interface-link-down = Link बंद
avb-interface-grandmaster-changes = Grandmaster बदलाव
avb-interface-frames-sent = भेजे गए frames
avb-interface-frames-received = प्राप्त frames
avb-interface-crc-errors = CRC त्रुटियाँ

tree-firmware = फ़र्मवेयर { $version }
tree-descriptor-types = { $count ->
    [one] { $count } प्रकार का descriptor
   *[other] { $count } प्रकार के descriptors
}
tree-clock = Clock
tree-clock-source-from = { $kind }, { $location } { $index } से
tree-clock-domain-using = { $source } का इस्तेमाल
tree-clusters = { $count ->
    [one] { $count } cluster
   *[other] { $count } clusters
}
tree-maps = { $count ->
    [one] { $count } map
   *[other] { $count } maps
}

advert-not-advertised = Advertise नहीं किया गया
advert-identity = पहचान
advert-entity-id = Entity ID
advert-entity-model = Entity model
advert-roles = भूमिकाएँ
advert-talker = Talker
advert-listener = Listener
advert-clock = Clock
advert-gptp-domain = gPTP domain
advert-sr-classes = SR classes
advert-indexes = Entity model indexes
advert-identify-control = Identify control
advert-avb-interface = AVB interface
advert-advertising = Advertising
advert-valid-time = वैधता अवधि
advert-available-index = Available index
advert-association = Association
advert-capabilities = क्षमताएँ

## Status bar

status-entities = { $count ->
    [one] { $count } entity
   *[other] { $count } entities
}
status-not-discovering = खोज नहीं रहा
status-discovering = खोज रहा है
status-discovering-as = { $controller } के रूप में खोज रहा है
status-stopped = त्रुटि के कारण रुका
status-alarm = अलार्म
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } और { $count } अन्य

## Descriptions of entities, shared by the views

role-talker = Talker { $count }
role-listener = Listener { $count }
role-controller = controller
role-none = कोई भूमिका नहीं
classes-a-and-b = A और B
clock-no-gptp = gPTP नहीं

read-not-read = पढ़ा नहीं गया
read-reading = पढ़ा जा रहा है, अब तक { $count }
read-ready-unreadable = तैयार, { $count } पढ़े नहीं जा सके
read-ready-cached = तैयार, कैश से
read-ready = तैयार
read-failed = विफल: { $reason }

milan-no = नहीं
milan-before-1-3 = 1.3 से पहले
milan-certified = { $version }, { $certification } प्रमाणित
milan-not-certified = { $version }, प्रमाणित नहीं

outcome-status = स्थिति { $status }
outcome-no-response = कोई जवाब नहीं
outcome-not-possible = संभव नहीं
outcome-connect = { $talker } को { $listener } से कनेक्ट नहीं किया जा सका: { $reason }।
outcome-disconnect = { $listener } को डिसकनेक्ट नहीं किया जा सका: { $reason }।
outcome-disconnect-listener = { $listener } को { $talker } से डिसकनेक्ट नहीं किया जा सका: { $reason }।
outcome-identify = { $entity } को identify नहीं किया जा सका: { $reason }।
outcome-rename = { $what } का नाम बदलकर “{ $name }” नहीं किया जा सका: { $reason }।
outcome-rename-group = { $entity } के समूह का नाम बदलकर “{ $name }” नहीं किया जा सका: { $reason }।
outcome-format-streaming = { $stream } का format नहीं बदला जा सका: इस पर streaming चल रही है। पहले इसे डिसकनेक्ट करें।
outcome-format = { $stream } का format नहीं बदला जा सका: { $reason }।
outcome-max-transit-time = { $stream } का अधिकतम transit time नहीं बदला जा सका: { $reason }।
outcome-max-transit-time-streaming = { $stream } का अधिकतम transit time नहीं बदला जा सका: इस पर streaming चल रही है। पहले इसे डिसकनेक्ट करें।
outcome-sampling-rate = { $entity } का sampling rate नहीं बदला जा सका: { $reason }।
outcome-clock-source = { $entity } का clock source नहीं बदला जा सका: { $reason }।
outcome-configuration = { $entity } का कॉन्फ़िगरेशन नहीं बदला जा सका: { $reason }।
outcome-configuration-streaming = { $entity } अपने streams चलते रहने तक अपना कॉन्फ़िगरेशन नहीं बदलता: पहले उन्हें डिसकनेक्ट करें।
outcome-map = { $entity } पर channel map नहीं किया जा सका: { $reason }।
outcome-unmap = { $entity } पर channel unmap नहीं किया जा सका: { $reason }।
outcome-control = { $entity } पर “{ $control }” सेट नहीं किया जा सका: { $reason }।
outcome-control-numbered = { $entity } पर control { $index } सेट नहीं किया जा सका: { $reason }।

stream-not-connected = कनेक्ट नहीं है
stream-from = { $stream } से
stream-from-receiving = { $stream } से, प्राप्त कर रहा है
stream-from-waiting = { $stream } से, Talker की प्रतीक्षा में
stream-from-failed = { $stream } से, Talker का reservation विफल: { $reason }
stream-sending-to = { $destination } को भेज रहा है

failure-no-response = उसने जवाब नहीं दिया
failure-refused = उसने { $status } के साथ मना कर दिया
failure-malformed = उसका जवाब decode नहीं हुआ
failure-on-this-computer = यह इसी कंप्यूटर पर चलती है; इसे किसी दूसरे कंप्यूटर से पढ़ें

msrp-failure-1 = पर्याप्त bandwidth नहीं
msrp-failure-2 = bridge के संसाधन पर्याप्त नहीं
msrp-failure-3 = traffic class के लिए पर्याप्त bandwidth नहीं
msrp-failure-4 = stream ID किसी दूसरे Talker के इस्तेमाल में है
msrp-failure-5 = destination address पहले से इस्तेमाल में है
msrp-failure-6 = ऊँची rank वाली stream ने इसे हटा दिया
msrp-failure-7 = रिपोर्ट की गई latency बदल गई है
msrp-failure-8 = egress port AVB सक्षम नहीं है
msrp-failure-9 = कोई दूसरा destination address इस्तेमाल करें
msrp-failure-10 = MSRP संसाधन खत्म हो गए
msrp-failure-11 = MMRP संसाधन खत्म हो गए
msrp-failure-12 = destination address सहेजा नहीं जा सकता
msrp-failure-13 = प्राथमिकता SR class की प्राथमिकता नहीं है
msrp-failure-14 = frames माध्यम के लिए बहुत बड़े हैं
msrp-failure-15 = fan-in port की सीमा पूरी हो गई
msrp-failure-16 = पंजीकृत stream का पहला मान बदल गया
msrp-failure-17 = egress port पर VLAN ब्लॉक है
msrp-failure-18 = egress port पर VLAN tagging बंद है
msrp-failure-19 = SR class की प्राथमिकता मेल नहीं खाती
msrp-failure-unknown = अज्ञात कारण
msrp-failure-at = { $reason }, bridge { $bridge } पर

## Entity list columns

column-vendor = निर्माता
column-entity-image = Entity की छवि
column-vendor-image = निर्माता की छवि
column-model = मॉडल
column-state = स्थिति
column-entity-model-id = Entity model ID
column-talker-streams = Talker streams
column-listener-streams = Listener streams
column-avb-lite = AVB Lite
column-egress = Egress
column-wireless = Wireless

## Settings file

settings-no-place = सेटिंग्स रखने की कोई जगह नहीं: होम फ़ोल्डर ज्ञात नहीं है।
settings-unusable = { $path } इस्तेमाल नहीं किया जा सका: { $error }।
settings-unsaved = { $path } सहेजा नहीं जा सका: { $error }।

column-remove = कॉलम हटाएँ
column-move-left = बाएँ ले जाएँ
column-move-right = दाएँ ले जाएँ
column-add = कॉलम जोड़ें
common-percent = { $value }%

## Network view

netmap-empty = दिखाने के लिए अभी कोई नेटवर्क नहीं
netmap-empty-note = Entities पढ़ी जाने और gPTP tree में अपनी जगह बताने के बाद यहाँ दिखती हैं।
netmap-focus-clock-path = { $name } का clock path
netmap-focus-streams = { $name } की streams
netmap-showing = दिखाया जा रहा है: { $what }
netmap-devices = { $count ->
    [one] { $count } डिवाइस
   *[other] { $count } डिवाइस
}
netmap-bridges = { $count ->
    [one] { $count } bridge
   *[other] { $count } bridges
}
netmap-show-map = नक्शा दिखाएँ
netmap-show-details = विवरण दिखाएँ
stream-numbered = Stream { $index }
netmap-bridge = Bridge
netmap-access-point = Access point
netmap-plain-switch = बिना AVB वाला switch
netmap-device = डिवाइस
netmap-this-computer = यह कंप्यूटर
netmap-connected = कनेक्ट है
netmap-advertised = Advertise हुई, कोई Listener तैयार नहीं
netmap-advertised-off-tree = Advertise हुई, कोई Listener तैयार नहीं ({ $listener } gPTP tree पर नहीं है)
netmap-failed-at = { $bridge } पर reservation विफल: { $reason }
netmap-failed = Reservation विफल: { $reason }
netmap-no-bridge-on = { $interface } पर कोई bridge सुनाई नहीं दिया
netmap-cannot-listen-on = { $interface } पर gPTP नहीं सुना जा सकता
netmap-on-this-computer = इसी कंप्यूटर पर
netmap-path-not-reported = Path रिपोर्ट नहीं हुआ
netmap-gptp-not-reported = gPTP रिपोर्ट नहीं हुआ
netmap-off-tree = gPTP tree पर नहीं
netmap-off-ptp = PTP tree पर नहीं
netmap-not-lite = AVB Lite में नहीं
netmap-lite-not-reported = AVB Lite रिपोर्ट नहीं हुआ
netmap-domain = gPTP domain { $domain }
netmap-lite-only = केवल AVB Lite
netmap-lite-set = AVB Lite, हाथ से सेट किया गया
netmap-synced = सिंक है
netmap-not-synced = सिंक नहीं है
netmap-triib-on = { $interface } पर triib
netmap-through-count = { $count } आर-पार
netmap-out = { $count } बाहर
netmap-in = { $count } अंदर
netmap-failed-count = { $count } विफल
netmap-advertised-only = केवल advertise हुई
netmap-failed-state = विफल
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-sending-unheard = भेज रहा है, इससे बँधा कोई ज्ञात Listener नहीं
netmap-no-listener = कोई ज्ञात Listener नहीं
netmap-apart-own-grandmaster = gPTP tree पर नहीं: यह खुद ही Grandmaster है
netmap-apart-no-path = इसका path रिपोर्ट नहीं हुआ; यह Grandmaster { $grandmaster } का अनुसरण करता है
netmap-apart-unreported = इसने अपनी gPTP स्थिति रिपोर्ट नहीं की है
netmap-apart-no-neighbor = इस कंप्यूटर के interface पर कोई bridge सुनाई नहीं दिया
netmap-apart-cannot-listen = यह कंप्यूटर अपने interface पर gPTP नहीं सुन सकता
netmap-apart-on-this-computer = यह इसी कंप्यूटर पर चलती है; इसकी gPTP स्थिति देखने के लिए इसे किसी दूसरे कंप्यूटर से पढ़ें
netmap-apart-not-lite = यह AVB Lite नहीं चलाता, इसलिए यह Grandmaster का अनुसरण नहीं करता
netmap-apart-lite-unreported = यह AVB Lite के बारे में कुछ रिपोर्ट नहीं करता, इसलिए पता नहीं कि यह किसका अनुसरण करता है
netmap-apart-other-domain = यह domain { $domain } पर gPTP चलाता है और tree domain { $network } पर है, इसलिए यह tree के Grandmaster का अनुसरण नहीं करता
netmap-apart-lite-only = यह केवल AVB Lite चलाता है और gPTP या SRP में हिस्सा नहीं लेता, इसलिए AVB bridges इसकी streams के लिए कुछ भी reserve नहीं करते
netmap-apart-lite-set = यह AVB Lite चलाता है क्योंकि ऑपरेटर या किसी controller ने इसे ऐसा सेट किया है, और यह gPTP या SRP में हिस्सा नहीं लेता
netmap-switch-state = AVB में हिस्सा नहीं लेता: यह नीचे के डिवाइसों के आगे के switches को दर्शाता है, जिन्हें कोई AVB bridge नहीं मिला और जो AVB Lite पर चले गए, और एक ही PTP Grandmaster का अनुसरण करते हैं
netmap-clock-tree = Clock tree
netmap-no-grandmaster = कोई Grandmaster सुनाई नहीं दिया
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = ध्यान देने की ज़रूरत
netmap-nodes-below = नीचे के nodes
netmap-bridges-below = नीचे के bridges
netmap-clock-path = Clock path
netmap-hops = Grandmaster से hops
netmap-link-delay = Link delay
netmap-bridge-port = Bridge port
netmap-link-drops = Link drops
netmap-synced-to-grandmaster = Grandmaster से सिंक है
netmap-host-no-gptp = सिंक नहीं: यह कंप्यूटर gPTP नहीं चलाता
netmap-link-no-gptp = सिंक नहीं: इसके link पर gPTP नहीं चलता
netmap-ptp-offset-high = सिंक नहीं: Grandmaster से { $offset }, AVB Lite की अनुमत 50 µs सीमा से ज़्यादा
netmap-ptp-no-offset = सिंक नहीं: इसने Grandmaster से कोई offset नहीं मापा है
netmap-audio = ऑडियो
netmap-media-clock-streams = Media clock streams
netmap-audio-streams = ऑडियो streams
netmap-bound = { $count } बँधी
netmap-flowing = चल रही है
netmap-advertised-state = Advertise हुई
netmap-media-clock-stream = Media clock stream
netmap-audio-stream = ऑडियो stream
netmap-reaches = पहुँच
netmap-passing-count = { $count ->
    [one] { $count } stream आर-पार जा रही है
   *[other] { $count } streams आर-पार जा रही हैं
}
netmap-through = आर-पार
netmap-passing-through = आर-पार जा रही streams
netmap-sending = भेज रहा है
netmap-receiving = प्राप्त कर रहा है
netmap-problems = समस्याएँ
netmap-help-back = पूरे नक्शे पर लौटने के लिए खाली जगह पर क्लिक करें।
netmap-help-stream = किसी stream को जाँचने के लिए उस पर क्लिक करें, या पूरे नक्शे पर लौटने के लिए खाली जगह पर।
netmap-help-ptp = AVB Lite में clock, Grandmaster से हर डिवाइस तक end to end जाती है, ऐसे switches से होकर जो इसमें हिस्सा नहीं लेते, इसलिए कोई switch नहीं दिखाया जाता। कोई डिवाइस तब तक सिंक है, जब तक वह 50 µs के भीतर Grandmaster का अनुसरण करता है। किसी डिवाइस या उसके तार पर क्लिक करके उसकी clock जाँचें; चुनाव हटाने के लिए खाली जगह पर क्लिक करें।
netmap-help-clock = Clock, Grandmaster से हर bridge से होकर tree के हर node तक जाती है। टूटी हुई स्लेटी रेखा ऐसा link है जिस पर gPTP नहीं चलता। किसी डिवाइस या उसके तार पर क्लिक करके उसका clock path जाँचें; चुनाव हटाने के लिए खाली जगह पर क्लिक करें।
netmap-help-lite = जिन डिवाइसों को कोई AVB bridge नहीं मिला और जो AVB Lite पर चले गए, वे टूटी रेखा वाले एक कार्ड के नीचे लटकते हैं जो उनके आगे के switches को दर्शाता है, हर उस PTP Grandmaster के लिए एक, जिसका वे अनुसरण करते हैं। जो डिवाइस केवल AVB Lite चलाते हैं या उस पर सेट किए गए हैं, और किसी दूसरे gPTP domain के डिवाइस, अपने अलग क्षेत्रों में रहते हैं।
netmap-help-media-clock = केवल media clock (CRF) streams, ऑडियो की तरह ही बनी हुई: हर link पर हर stream के frames का एक तार, Talker के हिसाब से रंगा हुआ। किसी तार पर क्लिक करके उसकी stream जाँचें, या किसी डिवाइस पर क्लिक करके उसकी streams देखें; चुनाव हटाने के लिए खाली जगह पर क्लिक करें।
netmap-help-audio = हर तार किसी link पर एक stream के frames दिखाता है, जो रास्ते के हर bridge में आता है और उससे निकलता है। किसी multicast stream का तार तब तक एक ही रहता है जब तक उसके Listeners तक के रास्ते साथ चलते हैं, और जहाँ वे अलग होते हैं वहाँ शाखाओं में बँट जाता है; किसी unicast stream का, जैसी AVB Lite भेज सकता है, Talker से हर Listener तक अपना अलग तार होता है। रंग Talker के हिसाब से है: हर Talker का एक रंग है, और उसकी हर stream हर शाखा पर उसी रंग की एक छटा है। चलते बिंदुओं का मतलब है कि ऑडियो चल रहा है, तब भी जब Talker बिना किसी ज्ञात Listener के भेज रहा हो; रुकी हुई लाल रेखा विफल reservation है, और रुकी हुई स्लेटी रेखा advertise हुई ऐसी stream है जिसके लिए कोई Listener तैयार नहीं; दोनों वहीं रुकती हैं जहाँ reservation रुकता है। बीच वाले कॉलम के डिवाइस सीधे Grandmaster के bridge से जुड़े हैं। किसी तार पर क्लिक करके उसकी stream जाँचें, किसी साझा तार पर क्लिक करके उसके सभी Listeners देखें, या किसी डिवाइस पर क्लिक करके उसकी streams देखें; चुनाव हटाने के लिए खाली जगह पर क्लिक करें।

## Connections

matrix-nothing-shown = दिखाने के लिए कोई stream नहीं
matrix-nothing-shown-note = और streams देखने के लिए खोज या फ़िल्टर बदलें।
matrix-empty = कनेक्ट करने के लिए कोई stream नहीं
matrix-empty-note = Talker streams और Listener streams वाली entities पढ़ी जाने के बाद ये streams यहाँ आमने-सामने आती हैं।
matrix-all-streams = सभी streams
matrix-connectable-only = जो कनेक्ट नहीं हो सकतीं, उन्हें छिपाएँ
matrix-none-hidden = दिखाई गई हर stream कनेक्ट हो सकती है
matrix-hidden = { $count ->
    [one] { $count } stream छिपी है
   *[other] { $count } streams छिपी हैं
}
matrix-own = किसी entity के outputs उसके अपने inputs से कनेक्ट नहीं होते।
matrix-working = काम चल रहा है।
matrix-waiting-change = इस input में किए गए पिछले बदलाव की प्रतीक्षा है।
matrix-connected = कनेक्ट है और प्राप्त कर रहा है। डिसकनेक्ट करने के लिए क्लिक करें।
matrix-bound-waiting = बँधा है, Talker की stream की प्रतीक्षा में। डिसकनेक्ट करने के लिए क्लिक करें।
matrix-bound-failed = बँधा है, पर Talker का reservation विफल हुआ: { $reason }। डिसकनेक्ट करने के लिए क्लिक करें।
matrix-bound-formats-differ = बँधा है, पर formats अलग हैं: Talker { $sent } भेजता है, input { $set } पर सेट है। डिसकनेक्ट करने के लिए क्लिक करें।
matrix-formats-match = Formats मेल खाते हैं ({ $format })। कनेक्ट करने के लिए क्लिक करें।
matrix-format-must-change = Input { $sent } लेता है पर { $set } पर सेट है, इसलिए format बदलने तक शायद न बजे। format चुनने के लिए क्लिक करें।
matrix-format-input = Input को इसमें बदलें
matrix-format-both = दोनों को इसमें बदलें
matrix-format-talker-busy = Talker दूसरे listeners को stream कर रहा है, इसलिए उसका format नहीं बदलेगा।
matrix-format-anyway = format बदले बिना कनेक्ट करें
matrix-incompatible = Input { $sent } नहीं लेता। यह { $set } पर सेट है।
matrix-group-none = कनेक्ट नहीं है। Streams को एक-एक करके कनेक्ट करने के लिए फैलाएँ।
matrix-group-connected = { $count } कनेक्ट हैं। हर एक को देखने के लिए फैलाएँ।
matrix-outputs-expand = { $count ->
    [one] { $count } stream output। फैलाने के लिए तीर पर, जाँचने के लिए नाम पर क्लिक करें।
   *[other] { $count } stream outputs। फैलाने के लिए तीर पर, जाँचने के लिए नाम पर क्लिक करें।
}
matrix-outputs-collapse = { $count ->
    [one] { $count } stream output। समेटने के लिए तीर पर, जाँचने के लिए नाम पर क्लिक करें।
   *[other] { $count } stream outputs। समेटने के लिए तीर पर, जाँचने के लिए नाम पर क्लिक करें।
}
matrix-inputs-expand = { $count ->
    [one] { $count } stream input। फैलाने के लिए तीर पर, जाँचने के लिए नाम पर क्लिक करें।
   *[other] { $count } stream inputs। फैलाने के लिए तीर पर, जाँचने के लिए नाम पर क्लिक करें।
}
matrix-inputs-collapse = { $count ->
    [one] { $count } stream input। समेटने के लिए तीर पर, जाँचने के लिए नाम पर क्लिक करें।
   *[other] { $count } stream inputs। समेटने के लिए तीर पर, जाँचने के लिए नाम पर क्लिक करें।
}
matrix-stream-inspect = { $detail } { $entity } को जाँचने के लिए क्लिक करें।
matrix-point = किसी सेल पर पॉइंटर ले जाएँ
matrix-point-note = ताकि उसका Talker और Listener दिखें और पता चले कि उनके formats मेल खाते हैं या नहीं।
matrix-legend-waiting = बँधा, stream की प्रतीक्षा में
matrix-legend-trouble = बँधा, कुछ गड़बड़ है
matrix-legend-open = कनेक्ट हो सकता है
matrix-legend-change = पहले input का format बदलना होगा
matrix-legend-incompatible = Formats मेल नहीं खा सकते
matrix-talker-outputs = Talker outputs
matrix-listener-inputs = Listener inputs

common-thousands-separator = {","}

## Diagnostics

diag-since-start = Entity के शुरू होने से अब तक की गिनती।
diag-stream-input = Stream input
diag-stream-output = Stream output
diag-locked = { $count ->
    [0] lock नहीं हुआ
    [one] एक बार lock हुआ
   *[other] { $number } बार lock हुआ
}
diag-lost-lock = { $count ->
    [0] lock नहीं टूटा
    [one] एक बार lock टूटा
   *[other] { $number } बार lock टूटा
}
diag-frames-in = { $count ->
    [one] { $number } frame प्राप्त
   *[other] { $number } frames प्राप्त
}
diag-frames-out = { $count ->
    [one] { $number } frame भेजा गया
   *[other] { $number } frames भेजे गए
}
diag-media-locked = { $count ->
    [0] media lock नहीं हुआ
    [one] एक बार media lock हुआ
   *[other] { $number } बार media lock हुआ
}
diag-lost-media-lock = { $count ->
    [0] media lock नहीं टूटा
    [one] एक बार media lock टूटा
   *[other] { $number } बार media lock टूटा
}
diag-interrupted = { $count ->
    [0] बाधित नहीं हुआ
    [one] एक बार बाधित हुआ
   *[other] { $number } बार बाधित हुआ
}
diag-out-of-sequence = { $count ->
    [one] { $number } frame क्रम से बाहर
   *[other] { $number } frames क्रम से बाहर
}
diag-media-resets = { $count ->
    [one] { $number } media reset
   *[other] { $number } media resets
}
diag-timestamps-uncertain = { $count ->
    [0] timestamps अनिश्चित नहीं रहे
    [one] एक बार timestamps अनिश्चित रहे
   *[other] { $number } बार timestamps अनिश्चित रहे
}
diag-no-timestamp = { $count ->
    [one] { $number } frame बिना timestamp के
   *[other] { $number } frames बिना timestamp के
}
diag-unsupported-format = { $count ->
    [one] { $number } frame असमर्थित format में
   *[other] { $number } frames असमर्थित format में
}
diag-late = { $count ->
    [one] { $number } frame देर से
   *[other] { $number } frames देर से
}
diag-early = { $count ->
    [one] { $number } frame जल्दी
   *[other] { $number } frames जल्दी
}
diag-started = { $count ->
    [0] शुरू नहीं हुआ
    [one] एक बार शुरू हुआ
   *[other] { $number } बार शुरू हुआ
}
diag-stopped = { $count ->
    [0] रुका नहीं
    [one] एक बार रुका
   *[other] { $number } बार रुका
}
diag-reservation-failed = Talker का reservation विफल: { $reason }
diag-latency = { $microseconds } µs संचित latency

## AVB Lite

lite-active = सक्रिय
lite-active-untagged = सक्रिय, untagged
lite-active-vlan = सक्रिय, VLAN { $vlan }
lite-capable = सक्षम
lite-mode = मोड
lite-mode-capable = AVB (AVB Lite सक्षम)
lite-because = कारण
lite-fallback-none = कोई कारण नहीं बताया गया
lite-fallback-endpoint = दूसरे endpoint का declaration पहुँचा, इसलिए उनके बीच कोई AVB bridge नहीं है
lite-fallback-unanswered = नौ peer delay अनुरोधों का जवाब नहीं आया
lite-fallback-responders = एक peer delay अनुरोध का जवाब दो या ज़्यादा ने दिया, इसलिए switch AVB bridge नहीं है
lite-fallback-configured = ऑपरेटर या किसी controller ने इसे सेट किया
lite-fallback-other = ऐसा कारण जो profile में नहीं है
lite-other-profile = दूसरा profile
lite-ptp-domain = { $profile }, domain { $domain }
lite-offset = Offset
lite-offset-from = { $grandmaster } से { $offset }
lite-media-vlan = Media VLAN
lite-untagged = Untagged
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] हर stream { $count } Listener तक, फिर multicast
   *[other] हर stream { $count } Listeners तक, फिर multicast
}
lite-link = Link
lite-bandwidth = Bandwidth
lite-egress-of = { $link } में से { $used }, { $share }
lite-egress-of-assumed = { $link } में से { $used }, { $share }, gigabit link मानकर
lite-egress-reported = Entity द्वारा गिनी गई स्वीकृत streams के अनुसार।
lite-egress-worked-out = इसके कनेक्ट हुए stream outputs के formats से निकाला गया।
lite-alarm-offset = PTP offset { $offset }, AVB Lite की अनुमत 50 µs सीमा से ज़्यादा
lite-alarm-egress = Egress link का { $share }, streams के लिए अनुमत { $limit } से ज़्यादा

## AVB Wireless

wireless-station = Station
wireless-access-point = Access point
wireless-role = भूमिका
wireless-mode = मोड
wireless-time = समय
wireless-mode-a-ftm = Mode A, FTM पर 802.1AS
wireless-mode-a-tm = Mode A, TM पर 802.1AS
wireless-mode-b = Mode B, beacons से
wireless-no-time = समय नहीं
wireless-other-mode = ऐसा मोड जो profile में नहीं है
wireless-locked = Lock है
wireless-holdover = Holdover में
wireless-not-locked = Lock नहीं है
wireless-row-locked = Station, lock है
wireless-row-holdover = Station, holdover में
wireless-row-not-locked = Station, lock नहीं है
wireless-row-access-point = { $count ->
    [one] Access point, { $count } station
   *[other] Access point, { $count } stations
}
wireless-link = Link
wireless-channel = चैनल { $channel }
wireless-not-known = अज्ञात
wireless-signal = सिग्नल
wireless-rate = Rate
wireless-ftm-valid = { $share } मान्य
wireless-rtt = round trip { $rtt }
wireless-bursts = { $count ->
    [one] { $count } frame के bursts
   *[other] { $count } frames के bursts
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = access point तीन या दो के अलावा किसी और संख्या के frames वाले FTM bursts देता है
wireless-reason-measurement = access point के साथ न FTM है, न TM
wireless-reason-signaling = access point से कोई gPTP-capable Signaling नहीं आया
wireless-reason-other = ऐसा कारण जो profile में नहीं है
wireless-servo = Servo त्रुटि
wireless-stations = Stations
wireless-station-count = { $count ->
    [one] { $count } station
   *[other] { $count } stations
}
wireless-no-ftm = FTM के बिना
wireless-unserved = बिना सेवा के Listeners
wireless-frame-budget = Frame budget
wireless-frame-use = प्रति सेकंड { $budget } में से { $used } frames
wireless-stream-frames = Stream frames
wireless-frames-of = { $readdressed } stations को, { $unmapped } बिना Listener के, { $dropped } drop हुए, { $restored } stations से
wireless-class-a-allowed = अनुमत
wireless-class-a-not-allowed = अनुमत नहीं
wireless-alarm-not-locked = Wi-Fi समय access point से lock नहीं है
wireless-alarm-holdover = Wi-Fi समय holdover में, access point से lock टूटा
wireless-alarm-unserved = { $count ->
    [one] Wi-Fi port पर { $count } Listener को सेवा नहीं मिली, unicast सीमा से ज़्यादा
   *[other] Wi-Fi port पर { $count } Listeners को सेवा नहीं मिली, unicast सीमा से ज़्यादा
}

## Log

log-all = सभी
log-warnings = चेतावनियाँ
log-pause = रोकें
log-resume = फिर शुरू करें
log-clear = साफ़ करें
log-empty = triib जो भी ATDECC frame भेजता और सुनता है, वह यहाँ दिखता है, सबसे नया सबसे ऊपर।
log-none-match = रखा गया कोई भी frame फ़िल्टर से मेल नहीं खाता।
log-frames = { $count ->
    [one] { $count } frame
   *[other] { $count } frames
}
log-shown-of = { $all } में से { $shown } frames
log-sent = भेजा
log-heard = सुना
log-not-decoded = Decode नहीं हुआ
log-warning-short = इसका control_data_length frame के अंत से { $missing } octets आगे तक का दावा करता है।
log-warning-undecodable = यह decode नहीं होता: { $error }।
log-warning-long-acmp = यह लंबे ACMP रूप में है, जिसे भेजने की अनुमति Milan entity को नहीं है (Milan 1.3, 5.5.2.2)।

## Channel mappings

mapping-section = Channel mappings
mapping-inputs = Inputs
mapping-outputs = Outputs
mapping-port = port { $number }
mapping-fixed = स्थिर
mapping-not-read = अभी पढ़ा नहीं गया।
mapping-no-clusters = कोई cluster नहीं।
mapping-no-streams = कोई ऑडियो stream नहीं।
mapping-none = कोई mapping नहीं।
mapping-not-mapped = Map नहीं किया गया
mapping-cluster-numbered = Cluster { $index }

## Presets

presets-note = Preset हर entity के clock sources, sampling rates, stream formats, controls और कनेक्शन सहेजता है। इसे recall करने पर जो अलग है, वह बदल दिया जाता है।
presets-none = अभी कोई preset सहेजा नहीं गया।
presets-connections = { $count ->
    [one] { $count } कनेक्शन
   *[other] { $count } कनेक्शन
}
presets-recall = Recall
presets-delete = हटाएँ
presets-no-place = Presets रखने की कोई जगह नहीं: होम फ़ोल्डर ज्ञात नहीं है।
presets-undeletable = { $path } हटाया नहीं जा सका: { $error }।
presets-saved = { $count ->
    [one] “{ $name }” { $count } entity के साथ सहेजा गया।
   *[other] “{ $name }” { $count } entities के साथ सहेजा गया।
}
presets-nothing-differs = “{ $name }” से कुछ भी अलग नहीं है।
presets-recalling = { $count ->
    [one] “{ $name }” recall हो रहा है: { $count } बदलाव।
   *[other] “{ $name }” recall हो रहा है: { $count } बदलाव।
}
presets-missing = { $report } यहाँ नहीं हैं या पढ़ी नहीं गईं: { $missing }।
presets-deleted = “{ $name }” हटाया गया।
presets-host-note = यह इस कंप्यूटर के अपने Entity भी सहेजता है, और recall करने पर उन्हें फिर से शुरू करता है।
presets-host-endpoints = इस कंप्यूटर पर { $count }
presets-starting-host = “{ $name }” के लिए इस कंप्यूटर के Entity शुरू हो रहे हैं; उनके लौटने पर बाकी होगा।

## Controls

control-numbered = Control { $index }
control-not-shown = यहाँ नहीं दिखाया गया
control-option = विकल्प { $number }

## Network errors

network-permission = triib को raw Ethernet frames भेजने और प्राप्त करने की अनुमति चाहिए।
network-needs-npcap = triib को raw Ethernet frames भेजने और प्राप्त करने के लिए Npcap चाहिए।
network-npcap-administrators = Npcap केवल administrators को raw Ethernet frames भेजने और प्राप्त करने देता है। triib को administrator के रूप में चलाएँ, या Npcap को उसके केवल-administrators विकल्प के बिना फिर से इंस्टॉल करें।

matrix-stream-format = { $format }।
matrix-stream-format-state = { $format }। { $state }।

common-decimal-separator = {"."}

## This computer's own entities

host-add-entity = Entity जोड़ें
host-show-mine = केवल इस कंप्यूटर के अपने Entity दिखाएँ
host-show-all = सभी entities दिखाएँ
host-new-entity = होस्ट Entity { $number }
host-failed = इस कंप्यूटर में नहीं जोड़ा जा सका: { $reason }
host-needs-clock = इस कंप्यूटर के अपने Entity को PTP hardware clock वाले तारयुक्त interface की ज़रूरत है
host-no-ptp4l = ptp4l जवाब नहीं दे रहा, इसलिए इस कंप्यूटर की stream gPTP समय नहीं रख सकतीं
host-elsewhere = triib-endpointd { $interface } पर किसी दूसरे उपयोगकर्ता के लिए या root के रूप में चल रहा है, इसलिए इस कंप्यूटर के Entity वहीं चलते हैं, यहाँ नहीं
host-foreign-mrp = कोई दूसरा प्रोग्राम इस कंप्यूटर के पते से { $interface } पर MSRP या MVRP declarations भेज रहा है, और इससे इस कंप्यूटर की streams के लिए ज़रूरी चीज़ें वापस ली जा सकती हैं
host-alarm-foreign-mrp = इस कंप्यूटर पर कोई दूसरा प्रोग्राम { $interface } पर MSRP या MVRP declarations भेज रहा है
host-state = स्थिति
host-streaming = stream चल रही है
host-waiting = Listener की प्रतीक्षा
host-listening = सुन रहा है
host-bound = बंधा, Talker की प्रतीक्षा
host-unbound = बंधा नहीं
host-audio-from = ऑडियो कहाँ से
host-audio-to = ऑडियो कहाँ तक
host-channels = चैनल
host-virtual-device = वर्चुअल device
host-add-stream = Stream जोड़ें
host-add-audio-input = Audio input
host-add-audio-output = Audio output
host-add-clock-input = Media clock input
host-add-clock-output = Media clock output
host-remove-stream = Stream हटाएँ
host-silence = मौन
host-tone = परीक्षण टोन
host-nowhere = कहीं नहीं
host-default-device = डिफ़ॉल्ट डिवाइस
host-remove = इस कंप्यूटर से हटाएँ

mac-audio-device = ऑडियो डिवाइस
mac-no-audio-device = कोई stream जुड़ा न होने तक कोई नहीं
mac-entity-note = macOS इस Entity को चलाता है। जब तक इसका कोई stream जुड़ा रहता है, macOS वह ऑडियो डिवाइस बनाता है जिससे ऐप्स चलाते और रिकॉर्ड करते हैं, हर दिशा में उतने चैनल के साथ जितने कॉन्फ़िगरेशन तय करता है।
mac-open-audio-midi-setup = Audio MIDI Setup खोलें
mac-avb-off = { $interface } पर AVB बंद है, इसलिए इस Mac की अपनी कोई AVB Entity नहीं है। इसे Audio MIDI Setup के Network Device Browser में चालू करें।

listen-section = सुनें
listen-start = सुनें
listen-stop = सुनना बंद करें
listen-source = सुनता है
listen-received = जो यह प्राप्त करता है
listen-sent = stream { $number } पर जो यह भेजता है ({ $channels } चैनल)
listen-sent-needs-macos = यह Mac जो भेजता है उसे सुनने के लिए macOS 14.2 या बाद का संस्करण चाहिए।
listen-channels = चैनल
listen-channel = चैनल { $number }
listen-channel-pair = चैनल { $first }–{ $last }
listen-output = आउटपुट
listen-default-output = डिफ़ॉल्ट आउटपुट
listen-gain = गेन
listen-gain-db = { $db } dB
listen-starting = शुरू हो रहा है…
listen-waiting-input = { $device } की प्रतीक्षा, जिसे macOS तब बनाता है जब Entity का कोई stream जुड़ा हो।
listen-waiting-output = { $device } की प्रतीक्षा।
listen-playing = चल रहा है, शिखर { $db } dB
listen-playing-silent = चल रहा है, मौन
