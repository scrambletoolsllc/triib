# triib's interface text in Dutch.

## Language

language-name = Nederlands

## Common

common-close = Sluiten
common-more = Meer
common-keep-toolbar-shown = Werkbalk altijd tonen
common-auto-hide-toolbar = Werkbalk automatisch verbergen

## Settings

settings-title = Instellingen
settings-general = Algemeen
settings-appearance = Weergave
settings-language = Taal
settings-language-system = Systeemstandaard: { $language }
settings-language-note = Tekstvelden typen in de invoertaal van het systeem.
settings-appearance-system = Systeem
settings-appearance-light = Licht
settings-appearance-dark = Donker
settings-colors = Kleuren
settings-system-accent = Accentkleur van het systeem gebruiken
settings-accent-picked = De kleur hieronder is de basis voor de kleuren van triib.
settings-accent-omarchy = Uit het Omarchy-thema, { $theme }.
settings-accent-desktop = Uit de accentkleur van de desktop.
settings-accent-none = De desktop heeft geen accentkleur, dus wordt de kleur hieronder gebruikt.
settings-motion = Beweging
settings-animations = Animaties
settings-animations-note = Verende en glijdende overgangen als er iets verandert.
settings-animations-reduced = De desktop vraagt om minder beweging, dus triib blijft stil.

common-cancel = Annuleren
common-save = Opslaan
common-not-set = Niet ingesteld
common-unnamed = Naamloos
common-none = Geen
common-mac-address = MAC-adres
common-list-separator = {", "}

## Network interfaces

interface-up = actief
interface-link-down = link verbroken
interface-wireless = draadloos
interface-hardware-clock = hardwareklok
interface-hardware-clock-named = hardwareklok { $clock }
interface-virtual = virtueel

## Toolbar

toolbar-choose-interface = Kies een interface
toolbar-interface = Netwerkinterface
toolbar-show-virtual = Virtuele interfaces tonen
toolbar-hide-virtual = Virtuele interfaces verbergen
toolbar-connections = Verbindingen
toolbar-network = Netwerk
toolbar-entities = Entiteiten
toolbar-rediscover = Elke entiteit vragen zich aan te kondigen
toolbar-rescan = Alle entiteiten wissen en opnieuw scannen
toolbar-search = Entiteiten en streams zoeken
toolbar-presets = Presets
toolbar-log = Log
toolbar-inspector = Inspector
toolbar-settings = Instellingen

## The network's state, in place of a view

state-no-interface = Geen interface
state-no-interface-note = Kies de interface op het AVB-netwerk om entiteiten te detecteren.
state-starting = Starten
state-starting-note = { $interface } wordt geopend.
state-listening = Luisteren
state-listening-note = Entiteiten op { $interface } verschijnen hier zodra ze zich aankondigen.
state-permission-needed = Toestemming nodig
state-npcap-needed = Npcap nodig
state-get-npcap = Npcap downloaden
state-copy-command = Commando kopiëren
state-cannot-use = Kan { $interface } niet gebruiken
state-try-again = Opnieuw proberen

## Entity list

entities-none-yet = Nog geen entiteiten
entities-none-yet-note = Elke entiteit op het netwerk, met haar rollen, SR-klassen en klok.

## Inspector

inspector-title = Inspector
inspector-entity = Entiteit
inspector-streams = Streams
inspector-controls = Regelaars
inspector-diagnostics = Diagnose
inspector-descriptors = Descriptors
inspector-select = Selecteer een entiteit om de details te zien.
inspector-offline = { $entity } is offline.
inspector-rename = Hernoemen
inspector-name = Naam
inspector-identify = Identificeren
inspector-model-not-read = Het entiteitsmodel is niet gelezen.
inspector-no-streams = Geen streams.
inspector-no-controls = Geen regelaars om te tonen.
inspector-no-diagnostics = Geen interfaces of tellers gemeld.
inspector-reading = Descriptors lezen, { $count } tot nu toe.
inspector-read-failed = Kon het entiteitsmodel niet lezen: { $reason }.

entity-section = Entiteit
entity-name = Naam
entity-group = Groep
entity-product = Product
entity-firmware = Firmware
entity-serial-number = Serienummer
entity-configuration = Configuratie
entity-configuration-of = { $name } ({ $number } van { $count })
entity-milan = Milan
entity-media-clock = Mediaklok
entity-clock-domain = Klokdomein
entity-sampling-rate = Samplefrequentie
clock-source-numbered = Bron { $index }
rate-pull = pull { $pull }

stream-inputs = Stream-ingangen
stream-outputs = Stream-uitgangen
stream-max-transit-time = Max. doorlooptijd { $time }

avb-interfaces = AVB-interfaces
avb-interface = Interface
avb-interface-clock-identity = Klokidentiteit
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domein { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Actief
avb-interface-none-reported = Geen gemeld
avb-interface-path = Pad
avb-interface-own-grandmaster = Zelf grandmaster
avb-interface-hops = { $count ->
    [one] { $count } hop van de grandmaster
   *[other] { $count } hops van de grandmaster
}
avb-interface-link-up = Link verbonden
avb-interface-link-down = Link verbroken
avb-interface-grandmaster-changes = Grandmasterwissels
avb-interface-frames-sent = Verzonden frames
avb-interface-frames-received = Ontvangen frames
avb-interface-crc-errors = CRC-fouten

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } descriptortype
   *[other] { $count } descriptortypen
}
tree-clock = Klok
tree-clock-source-from = { $kind }, van { $location } { $index }
tree-clock-domain-using = Gebruikt { $source }
tree-clusters = { $count ->
    [one] { $count } cluster
   *[other] { $count } clusters
}
tree-maps = { $count ->
    [one] { $count } toewijzing
   *[other] { $count } toewijzingen
}

advert-not-advertised = Niet aangekondigd
advert-identity = Identiteit
advert-entity-id = Entiteits-ID
advert-entity-model = Entiteitsmodel
advert-roles = Rollen
advert-talker = Talker
advert-listener = Listener
advert-clock = Klok
advert-btc = BTC
advert-gptp-domain = gPTP-domein
advert-sr-classes = SR-klassen
advert-indexes = Indexen in het entiteitsmodel
advert-identify-control = Identificatie
advert-avb-interface = AVB-interface
advert-advertising = Aankondiging
advert-valid-time = Geldigheidsduur
advert-available-index = Beschikbaarheidsindex
advert-association = Associatie
advert-capabilities = Mogelijkheden

## Status bar

status-entities = { $count ->
    [one] { $count } entiteit
   *[other] { $count } entiteiten
}
status-not-discovering = Geen detectie
status-discovering = Detectie actief
status-discovering-as = Detectie als { $controller }
status-stopped = Gestopt door een fout
status-alarm = Alarm
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } en nog { $count }

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = controller
role-none = geen rollen
classes-a-and-b = A en B
clock-no-gptp = Geen gPTP

read-not-read = Niet gelezen
read-reading = Lezen, { $count } tot nu toe
read-ready-unreadable = Gereed, { $count } onleesbaar
read-ready-cached = Gereed, uit cache
read-ready = Gereed
read-failed = Mislukt: { $reason }

milan-no = Nee
milan-before-1-3 = vóór 1.3
milan-certified = { $version }, gecertificeerd { $certification }
milan-not-certified = { $version }, niet gecertificeerd

outcome-status = status { $status }
outcome-no-response = geen antwoord
outcome-not-possible = niet mogelijk
outcome-connect = Kon { $talker } niet verbinden met { $listener }: { $reason }.
outcome-disconnect = Kon { $listener } niet loskoppelen: { $reason }.
outcome-identify = Kon { $entity } niet identificeren: { $reason }.
outcome-rename = Kon { $what } niet hernoemen naar “{ $name }”: { $reason }.
outcome-rename-group = Kon de groep van { $entity } niet hernoemen naar “{ $name }”: { $reason }.
outcome-format-streaming = Kon het formaat van { $stream } niet wijzigen: de stream is actief. Koppel hem eerst los.
outcome-format = Kon het formaat van { $stream } niet wijzigen: { $reason }.
outcome-sampling-rate = Kon de samplefrequentie van { $entity } niet wijzigen: { $reason }.
outcome-clock-source = Kon de klokbron van { $entity } niet wijzigen: { $reason }.
outcome-map = Kon het kanaal op { $entity } niet toewijzen: { $reason }.
outcome-unmap = Kon de kanaaltoewijzing op { $entity } niet opheffen: { $reason }.
outcome-control = Kon “{ $control }” op { $entity } niet instellen: { $reason }.
outcome-control-numbered = Kon regelaar { $index } op { $entity } niet instellen: { $reason }.

stream-not-connected = Niet verbonden
stream-from = Van { $stream }
stream-from-receiving = Van { $stream }, ontvangt
stream-from-waiting = Van { $stream }, wacht op de talker
stream-from-failed = Van { $stream }, reservering van de talker mislukt: { $reason }
stream-sending-to = Verzendt naar { $destination }

failure-no-response = geen antwoord
failure-refused = geweigerd met { $status }
failure-malformed = antwoord niet te decoderen
failure-on-this-computer = draait op deze computer; lees deze vanaf een andere computer

msrp-failure-1 = onvoldoende bandbreedte
msrp-failure-2 = onvoldoende switchresources
msrp-failure-3 = onvoldoende bandbreedte voor de verkeersklasse
msrp-failure-4 = stream-ID in gebruik door een andere talker
msrp-failure-5 = bestemmingsadres al in gebruik
msrp-failure-6 = verdrongen door een stream met hogere rang
msrp-failure-7 = gemelde latentie is gewijzigd
msrp-failure-8 = egress-poort is niet AVB-geschikt
msrp-failure-9 = gebruik een ander bestemmingsadres
msrp-failure-10 = MSRP-resources uitgeput
msrp-failure-11 = MMRP-resources uitgeput
msrp-failure-12 = kan het bestemmingsadres niet opslaan
msrp-failure-13 = prioriteit is geen SR-klasseprioriteit
msrp-failure-14 = frames te groot voor het medium
msrp-failure-15 = fan-in-limiet van de poort bereikt
msrp-failure-16 = eerste waarde gewijzigd voor een geregistreerde stream
msrp-failure-17 = VLAN geblokkeerd op de egress-poort
msrp-failure-18 = VLAN-tagging uitgeschakeld op de egress-poort
msrp-failure-19 = SR-klasseprioriteit komt niet overeen
msrp-failure-unknown = onbekende reden
msrp-failure-at = { $reason }, bij switch { $bridge }

## Entity list columns

column-vendor = Fabrikant
column-model = Model
column-state = Status
column-entity-model-id = Entiteitsmodel-ID
column-talker-streams = Talkerstreams
column-listener-streams = Listenerstreams
column-avb-lite = AVB Lite
column-egress = Egress
column-wireless = Draadloos

## Settings file

settings-no-place = De instellingen kunnen nergens worden bewaard: de thuismap is onbekend.
settings-unusable = Kon { $path } niet gebruiken: { $error }.
settings-unsaved = Kon { $path } niet opslaan: { $error }.

column-remove = Kolom verwijderen
column-move-left = Naar links
column-move-right = Naar rechts
column-add = Kolom toevoegen
common-percent = { $value }%

## Network view

netmap-empty = Nog geen netwerk om te tonen
netmap-empty-note = Entiteiten verschijnen hier zodra ze gelezen zijn en hebben gemeld waar ze in de gPTP-boom zitten.
netmap-focus-clock-path = Klokpad van { $name }
netmap-focus-streams = Streams van { $name }
netmap-showing = Toont { $what }
netmap-devices = { $count ->
    [one] { $count } apparaat
   *[other] { $count } apparaten
}
netmap-bridges = { $count ->
    [one] { $count } switch
   *[other] { $count } switches
}
netmap-show-map = Kaart tonen
netmap-show-details = Details tonen
stream-numbered = Stream { $index }
netmap-bridge = Switch
netmap-access-point = Access point
netmap-device = Apparaat
netmap-this-computer = Deze computer
netmap-connected = Verbonden
netmap-advertised = Aangekondigd, geen listener gereed
netmap-advertised-off-tree = Aangekondigd, geen listener gereed ({ $listener } zit niet in de gPTP-boom)
netmap-failed-at = Reservering mislukt bij { $bridge }: { $reason }
netmap-failed = Reservering mislukt: { $reason }
netmap-no-bridge-on = Geen switch gedetecteerd op { $interface }
netmap-cannot-listen-on = Kan niet naar gPTP luisteren op { $interface }
netmap-on-this-computer = Op deze computer
netmap-path-not-reported = Pad niet gemeld
netmap-gptp-not-reported = gPTP niet gemeld
netmap-off-tree = Niet in de gPTP-boom
netmap-off-ptp = Niet in de PTP-boom
netmap-not-lite = Niet in AVB Lite
netmap-lite-not-reported = AVB Lite niet gemeld
netmap-synced = Gesynchroniseerd
netmap-not-synced = Niet gesynchroniseerd
netmap-triib-on = triib op { $interface }
netmap-through-count = { $count } doorgeleid
netmap-out = { $count } uit
netmap-in = { $count } in
netmap-failed-count = { $count } mislukt
netmap-advertised-only = Alleen aangekondigd
netmap-failed-state = Mislukt
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Niet in de gPTP-boom: zelf grandmaster
netmap-apart-no-path = Pad niet gemeld; volgt grandmaster { $grandmaster }
netmap-apart-unreported = gPTP-status niet gemeld
netmap-apart-no-neighbor = Geen switch gedetecteerd op de interface van deze computer
netmap-apart-cannot-listen = Deze computer kan op zijn interface niet naar gPTP luisteren
netmap-apart-on-this-computer = Draait op deze computer; lees deze vanaf een andere computer om de gPTP-status te zien
netmap-apart-not-lite = Draait geen AVB Lite, dus volgt de grandmaster niet
netmap-apart-lite-unreported = Meldt niets over AVB Lite, dus het is onbekend wat het volgt
netmap-clock-tree = Klokboom
netmap-no-grandmaster = Geen grandmaster gedetecteerd
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Vraagt aandacht
netmap-nodes-below = Knooppunten eronder
netmap-bridges-below = Switches eronder
netmap-clock-path = Klokpad
netmap-hops = Hops vanaf de grandmaster
netmap-link-delay = Linkvertraging
netmap-bridge-port = Switchpoort
netmap-link-drops = Linkonderbrekingen
netmap-synced-to-grandmaster = Gesynchroniseerd met de grandmaster
netmap-host-no-gptp = Niet gesynchroniseerd: deze computer draait geen gPTP
netmap-link-no-gptp = Niet gesynchroniseerd: op de link draait geen gPTP
netmap-ptp-offset-high = Niet gesynchroniseerd: { $offset } ten opzichte van de grandmaster, meer dan de 50 µs die AVB Lite toestaat
netmap-ptp-no-offset = Niet gesynchroniseerd: geen offset ten opzichte van de grandmaster gemeten
netmap-audio = Audio
netmap-media-clock-streams = Mediaklokstreams
netmap-audio-streams = Audiostreams
netmap-bound = { $count } gekoppeld
netmap-flowing = Stroomt
netmap-advertised-state = Aangekondigd
netmap-media-clock-stream = Mediaklokstream
netmap-audio-stream = Audiostream
netmap-reaches = Reikt tot
netmap-passing-count = { $count ->
    [one] { $count } doorgeleide stream
   *[other] { $count } doorgeleide streams
}
netmap-through = Doorgeleid
netmap-passing-through = Doorgeleide streams
netmap-sending = Verzendt
netmap-receiving = Ontvangt
netmap-problems = Problemen
netmap-help-back = Klik op de achtergrond om terug te gaan naar het overzicht.
netmap-help-stream = Klik op een stream om hem te inspecteren, of op de achtergrond om terug te gaan naar het overzicht.
netmap-help-ptp = In AVB Lite loopt de klok end-to-end van de grandmaster naar elk apparaat, via switches die er niet aan deelnemen, dus die worden niet getoond. Een apparaat is gesynchroniseerd zolang het de grandmaster binnen 50 µs volgt. Klik op een apparaat of zijn draad om zijn klok te inspecteren; klik op de achtergrond om de selectie op te heffen.
netmap-help-clock = De klok loopt van de grandmaster via elke switch naar elk knooppunt in de boom. Een gestreepte grijze lijn is een link waarop gPTP niet draait. Klik op een apparaat of zijn draad om zijn klokpad te inspecteren; klik op de achtergrond om de selectie op te heffen.
netmap-help-media-clock = Alleen mediaklokstreams (CRF), getekend zoals audio: één draad per stream, gekleurd per talker. Klik op een draad om de stream te inspecteren, of op een apparaat om zijn streams te zien; klik op de achtergrond om de selectie op te heffen.
netmap-help-audio = Elke stream heeft een eigen draad, die elke switch die hij passeert in- en uitgaat. De kleur volgt de talker: elke talker heeft een eigen tint, en zijn streams zijn schakeringen daarvan. Bewegende stippen betekenen dat er audio stroomt; een stilstaande rode lijn is een mislukte reservering en een stilstaande grijze lijn is aangekondigd terwijl er geen listener gereed is; beide stoppen waar de reservering stopt. Apparaten in de middelste kolom zijn direct verbonden met de switch van de grandmaster. Klik op een draad om de stream te inspecteren, of op een apparaat om zijn streams te zien; klik op de achtergrond om de selectie op te heffen.

## Connections

matrix-nothing-shown = Geen streams om te tonen
matrix-nothing-shown-note = Wijzig de zoekopdracht of de filters om meer streams te zien.
matrix-empty = Geen streams om te verbinden
matrix-empty-note = Talkerstreams en listenerstreams komen hier samen zodra entiteiten met streams gelezen zijn.
matrix-all-streams = Alle streams
matrix-connectable-only = Verbergen wat niet kan verbinden
matrix-none-hidden = Elke getoonde stream kan verbinden
matrix-hidden = { $count ->
    [one] { $count } stream verborgen
   *[other] { $count } streams verborgen
}
matrix-own = De uitgangen van een entiteit verbinden niet met haar eigen ingangen.
matrix-working = Bezig.
matrix-waiting-change = Wacht op de laatste wijziging aan deze ingang.
matrix-connected = Verbonden en ontvangt. Klik om los te koppelen.
matrix-bound-waiting = Gekoppeld, wacht op de stream van de talker. Klik om los te koppelen.
matrix-bound-failed = Gekoppeld, maar de reservering van de talker is mislukt: { $reason }. Klik om los te koppelen.
matrix-bound-formats-differ = Gekoppeld, maar de formaten verschillen: de talker verzendt { $sent }, de ingang staat op { $set }. Klik om los te koppelen.
matrix-formats-match = Formaten komen overeen ({ $format }). Klik om te verbinden.
matrix-format-must-change = De ingang accepteert { $sent } maar staat op { $set }, dus speelt mogelijk pas af als het formaat verandert. Klik om een formaat te kiezen.
matrix-format-input = Ingang wijzigen naar
matrix-format-both = Beide wijzigen naar
matrix-format-talker-busy = De talker streamt naar andere listeners, dus zijn formaat blijft.
matrix-format-anyway = Verbinden zonder formaten te wijzigen
matrix-incompatible = De ingang accepteert { $sent } niet. Hij staat op { $set }.
matrix-group-none = Niet verbonden. Klap uit om streams één voor één te verbinden.
matrix-group-connected = { $count } verbonden. Klap uit om ze afzonderlijk te zien.
matrix-outputs-expand = { $count ->
    [one] { $count } stream-uitgang. Klik op de pijl om uit te klappen, op de naam om te inspecteren.
   *[other] { $count } stream-uitgangen. Klik op de pijl om uit te klappen, op de naam om te inspecteren.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } stream-uitgang. Klik op de pijl om in te klappen, op de naam om te inspecteren.
   *[other] { $count } stream-uitgangen. Klik op de pijl om in te klappen, op de naam om te inspecteren.
}
matrix-inputs-expand = { $count ->
    [one] { $count } stream-ingang. Klik op de pijl om uit te klappen, op de naam om te inspecteren.
   *[other] { $count } stream-ingangen. Klik op de pijl om uit te klappen, op de naam om te inspecteren.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } stream-ingang. Klik op de pijl om in te klappen, op de naam om te inspecteren.
   *[other] { $count } stream-ingangen. Klik op de pijl om in te klappen, op de naam om te inspecteren.
}
matrix-stream-inspect = { $detail } Klik om { $entity } te inspecteren.
matrix-point = Wijs een cel aan
matrix-point-note = om de talker en listener te zien en of hun formaten overeenkomen.
matrix-legend-waiting = Gekoppeld, wacht op de stream
matrix-legend-trouble = Gekoppeld, er is iets mis
matrix-legend-open = Kan verbinden
matrix-legend-change = Formaat van de ingang moet eerst veranderen
matrix-legend-incompatible = Formaten passen niet bij elkaar
matrix-talker-outputs = Talker-uitgangen
matrix-listener-inputs = Listener-ingangen

common-thousands-separator = {"."}

## Diagnostics

diag-since-start = Geteld sinds de entiteit is gestart.
diag-stream-input = Stream-ingang
diag-stream-output = Stream-uitgang
diag-locked = { $count ->
    [0] niet vergrendeld
    [one] één keer vergrendeld
    [2] twee keer vergrendeld
   *[other] { $number } keer vergrendeld
}
diag-lost-lock = { $count ->
    [0] vergrendeling niet verloren
    [one] vergrendeling één keer verloren
    [2] vergrendeling twee keer verloren
   *[other] vergrendeling { $number } keer verloren
}
diag-frames-in = { $count ->
    [one] { $number } frame ontvangen
   *[other] { $number } frames ontvangen
}
diag-frames-out = { $count ->
    [one] { $number } frame verzonden
   *[other] { $number } frames verzonden
}
diag-media-locked = { $count ->
    [0] niet op de mediaklok vergrendeld
    [one] één keer op de mediaklok vergrendeld
    [2] twee keer op de mediaklok vergrendeld
   *[other] { $number } keer op de mediaklok vergrendeld
}
diag-lost-media-lock = { $count ->
    [0] mediaklokvergrendeling niet verloren
    [one] mediaklokvergrendeling één keer verloren
    [2] mediaklokvergrendeling twee keer verloren
   *[other] mediaklokvergrendeling { $number } keer verloren
}
diag-interrupted = { $count ->
    [0] niet onderbroken
    [one] één keer onderbroken
    [2] twee keer onderbroken
   *[other] { $number } keer onderbroken
}
diag-out-of-sequence = { $count ->
    [one] { $number } frame buiten volgorde
   *[other] { $number } frames buiten volgorde
}
diag-media-resets = { $count ->
    [one] { $number } mediareset
   *[other] { $number } mediaresets
}
diag-timestamps-uncertain = { $count ->
    [0] tijdstempels niet onzeker
    [one] tijdstempels één keer onzeker
    [2] tijdstempels twee keer onzeker
   *[other] tijdstempels { $number } keer onzeker
}
diag-no-timestamp = { $count ->
    [one] { $number } frame zonder tijdstempel
   *[other] { $number } frames zonder tijdstempel
}
diag-unsupported-format = { $count ->
    [one] { $number } frame in een niet-ondersteund formaat
   *[other] { $number } frames in een niet-ondersteund formaat
}
diag-late = { $count ->
    [one] { $number } frame te laat
   *[other] { $number } frames te laat
}
diag-early = { $count ->
    [one] { $number } frame te vroeg
   *[other] { $number } frames te vroeg
}
diag-started = { $count ->
    [0] niet gestart
    [one] één keer gestart
    [2] twee keer gestart
   *[other] { $number } keer gestart
}
diag-stopped = { $count ->
    [0] niet gestopt
    [one] één keer gestopt
    [2] twee keer gestopt
   *[other] { $number } keer gestopt
}
diag-reservation-failed = reservering van de talker mislukt: { $reason }
diag-latency = { $microseconds } µs geaccumuleerde latentie

## AVB Lite

lite-active = Actief
lite-active-untagged = Actief, ongetagd
lite-active-vlan = Actief, VLAN { $vlan }
lite-capable = Ondersteund
lite-mode = Modus
lite-mode-capable = AVB, AVB Lite ondersteund
lite-because = Reden
lite-fallback-none = niet opgegeven
lite-fallback-endpoint = de declaratie van een ander eindpunt kwam door, dus er zit geen AVB-switch tussen
lite-fallback-unanswered = negen peer-delay-verzoeken bleven onbeantwoord
lite-fallback-responders = twee of meer apparaten beantwoordden één peer-delay-verzoek, dus de switch is niet AVB-geschikt
lite-fallback-configured = ingesteld door de operator of een controller
lite-fallback-other = een reden die het profiel niet noemt
lite-other-profile = Ander profiel
lite-ptp-domain = { $profile }, domein { $domain }
lite-offset = Offset
lite-offset-from = { $offset } ten opzichte van { $grandmaster }
lite-media-vlan = Media-VLAN
lite-untagged = Ongetagd
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Tot { $count } listener per stream, daarna multicast
   *[other] Tot { $count } listeners per stream, daarna multicast
}
lite-link = Link
lite-bandwidth = Bandbreedte
lite-egress-of = { $used } van { $link }, { $share }
lite-egress-of-assumed = { $used } van { $link }, { $share }, gigabitlink aangenomen
lite-egress-reported = Zoals de entiteit haar toegelaten streams telt.
lite-egress-worked-out = Uit de formaten van haar verbonden stream-uitgangen.
lite-alarm-offset = PTP-offset { $offset }, meer dan de 50 µs die AVB Lite toestaat
lite-alarm-egress = Egress op { $share } van de link, meer dan de { $limit } die streams mogen innemen

## AVB Wireless

wireless-station = Station
wireless-access-point = Access point
wireless-role = Rol
wireless-mode = Modus
wireless-time = Tijd
wireless-mode-a-ftm = Mode A, 802.1AS via FTM
wireless-mode-a-tm = Mode A, 802.1AS via TM
wireless-mode-b = Mode B, uit beacons
wireless-no-time = Geen tijd
wireless-other-mode = Een modus die het profiel niet noemt
wireless-locked = Vergrendeld
wireless-holdover = In holdover
wireless-not-locked = Niet vergrendeld
wireless-row-locked = Station, vergrendeld
wireless-row-holdover = Station, in holdover
wireless-row-not-locked = Station, niet vergrendeld
wireless-row-access-point = { $count ->
    [one] Access point, { $count } station
   *[other] Access point, { $count } stations
}
wireless-link = Link
wireless-channel = kanaal { $channel }
wireless-not-known = Onbekend
wireless-signal = Signaal
wireless-rate = Zendsnelheid
wireless-ftm-valid = { $share } geldig
wireless-rtt = retourtijd { $rtt }
wireless-bursts = { $count ->
    [one] bursts van { $count } frame
   *[other] bursts van { $count } frames
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = het access point wijst FTM-bursts toe van een ander aantal frames dan drie of twee
wireless-reason-measurement = noch FTM noch TM met het access point
wireless-reason-signaling = geen gPTP-capable Signaling van het access point
wireless-reason-other = een reden die het profiel niet noemt
wireless-servo = Servofout
wireless-stations = Stations
wireless-station-count = { $count ->
    [one] { $count } station
   *[other] { $count } stations
}
wireless-no-ftm = zonder FTM
wireless-unserved = Niet-bediende listeners
wireless-frame-budget = Framebudget
wireless-frame-use = { $used } van { $budget } frames per seconde
wireless-stream-frames = Streamframes
wireless-frames-of = { $readdressed } naar stations, { $unmapped } zonder listener, { $dropped } verworpen, { $restored } van stations
wireless-class-a-allowed = Toegestaan
wireless-class-a-not-allowed = Niet toegestaan
wireless-alarm-not-locked = Wi-Fi-tijd niet vergrendeld op het access point
wireless-alarm-holdover = Wi-Fi-tijd in holdover, vergrendeling op het access point verloren
wireless-alarm-unserved = { $count ->
    [one] { $count } listener op de Wi-Fi-poort niet bediend, boven de unicastlimiet
   *[other] { $count } listeners op de Wi-Fi-poort niet bediend, boven de unicastlimiet
}

## Log

log-all = Alle
log-warnings = Waarschuwingen
log-pause = Pauzeren
log-resume = Hervatten
log-clear = Wissen
log-empty = Elk ATDECC-frame dat triib verzendt en ontvangt verschijnt hier, nieuwste eerst.
log-none-match = Geen bewaard frame komt overeen met het filter.
log-frames = { $count ->
    [one] { $count } frame
   *[other] { $count } frames
}
log-shown-of = { $shown } van { $all } frames
log-sent = Verzonden
log-heard = Ontvangen
log-not-decoded = Niet gedecodeerd
log-warning-short = Het veld control_data_length claimt { $missing } octetten voorbij het einde van het frame.
log-warning-undecodable = Niet te decoderen: { $error }.
log-warning-long-acmp = In de lange ACMP-vorm, die een Milan-entiteit niet mag verzenden (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Kanaaltoewijzingen
mapping-inputs = Ingangen
mapping-outputs = Uitgangen
mapping-port = poort { $number }
mapping-fixed = vast
mapping-not-read = Nog niet gelezen.
mapping-no-clusters = Geen clusters.
mapping-no-streams = Geen audiostreams.
mapping-none = Geen toewijzingen.
mapping-not-mapped = Niet toegewezen
mapping-cluster-numbered = Cluster { $index }

## Presets

presets-note = Een preset bewaart van elke entiteit de klokbronnen, samplefrequenties, streamformaten, regelaars en verbindingen. Oproepen wijzigt wat verschilt.
presets-none = Nog geen presets opgeslagen.
presets-connections = { $count ->
    [one] { $count } verbinding
   *[other] { $count } verbindingen
}
presets-recall = Oproepen
presets-delete = Verwijderen
presets-no-place = Presets kunnen nergens worden bewaard: de thuismap is onbekend.
presets-undeletable = Kon { $path } niet verwijderen: { $error }.
presets-saved = { $count ->
    [one] “{ $name }” opgeslagen met { $count } entiteit.
   *[other] “{ $name }” opgeslagen met { $count } entiteiten.
}
presets-nothing-differs = Niets verschilt van “{ $name }”.
presets-recalling = { $count ->
    [one] “{ $name }” oproepen: { $count } wijziging.
   *[other] “{ $name }” oproepen: { $count } wijzigingen.
}
presets-missing = { $report } Niet aanwezig of niet gelezen: { $missing }.
presets-deleted = “{ $name }” verwijderd.
presets-host-note = Het bewaart ook de eigen talkers en listeners van deze computer en start ze opnieuw bij oproepen.
presets-host-endpoints = { $count } op deze computer
presets-starting-host = De talkers en listeners van deze computer voor “{ $name }” worden gestart; de rest volgt zodra ze terug zijn.

## Controls

control-numbered = Regelaar { $index }
control-not-shown = Hier niet getoond
control-option = Optie { $number }

## Network errors

network-permission = triib heeft toestemming nodig om ruwe Ethernet-frames te verzenden en te ontvangen.
network-needs-npcap = triib heeft Npcap nodig om ruwe Ethernet-frames te verzenden en te ontvangen.
network-npcap-administrators = Npcap staat alleen beheerders toe ruwe Ethernet-frames te verzenden en te ontvangen. Voer triib uit als beheerder, of installeer Npcap opnieuw zonder de optie voor alleen beheerders.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own talkers and listeners

host-add-talker = Talker toevoegen
host-add-listener = Listener toevoegen
host-show-mine = Alleen de eigen talkers en listeners van deze computer tonen
host-show-all = Alle entiteiten tonen
host-new-talker = Host-talker { $number }
host-new-listener = Host-listener { $number }
host-failed = Kon niet aan deze computer worden toegevoegd: { $reason }
host-needs-clock = De eigen talkers en listeners van deze computer hebben een bekabelde interface met een PTP-hardwareklok nodig
host-no-ptp4l = ptp4l antwoordt niet, dus de streams van deze computer kunnen de gPTP-tijd niet aanhouden
host-elsewhere = triib-endpointd draait op { $interface } voor een andere gebruiker of als root, dus de talkers en listeners van deze computer draaien daar, niet hier
host-foreign-mrp = Een ander programma declareert MSRP of MVRP op { $interface } vanaf het adres van deze computer en kan zo intrekken wat de streams van deze computer nodig hebben
host-alarm-foreign-mrp = Een ander programma op deze computer declareert MSRP of MVRP op { $interface }
host-state = Status
host-streaming = Streamt
host-waiting = Wacht op een listener
host-listening = Luistert
host-bound = Gebonden, wacht op de talker
host-unbound = Niet gebonden
host-audio-from = Audio van
host-audio-to = Audio naar
host-channels = Kanalen
host-silence = Stilte
host-tone = Testtoon
host-nowhere = Nergens heen
host-default-device = Standaardapparaat
host-remove = Verwijderen van deze computer
