# triib's interface text in Danish.

## Language

language-name = Dansk

## Common

common-close = Luk
common-more = Mere
common-keep-toolbar-shown = Vis altid værktøjslinjen
common-auto-hide-toolbar = Skjul værktøjslinjen automatisk

## Settings

settings-title = Indstillinger
settings-general = Generelt
settings-appearance = Udseende
settings-language = Sprog
settings-language-system = Systemstandard: { $language }
settings-language-note = Tekstfelter skriver på systemets inputsprog.
settings-appearance-system = System
settings-appearance-light = Lyst
settings-appearance-dark = Mørkt
settings-colors = Farver
settings-system-accent = Brug systemets accentfarve
settings-accent-picked = Farven nedenfor danner grundlag for farverne i triib.
settings-accent-omarchy = Fra Omarchy-temaet, { $theme }.
settings-accent-desktop = Fra skrivebordets accentfarve.
settings-accent-none = Skrivebordet har ingen accentfarve, så farven nedenfor bruges.
settings-motion = Bevægelse
settings-animations = Animationer
settings-animations-note = Fjedrende og glidende overgange, når noget ændrer sig.
settings-animations-reduced = Skrivebordet beder om mindre bevægelse, så triib holder sig i ro.
settings-outside-control = Ekstern styring
settings-allow-outside-control = Tillad ekstern styring
settings-allow-outside-control-note = Agenter som Claude Code kan se og ændre dit AVB-netværk via triib --mcp. triib spørger før hver ny agent.
settings-allowed-agents = Tilladt: { $agents }
settings-forget-agents = Glem
agent-prompt-title = Vil du give { $agent } lov til at styre triib?
agent-prompt-body = { $agent } beder om at bruge ekstern styring af triib: at se entiteterne på dit netværk og ændre dem, forbinde streams og indstille formater, clocks og navne. Du kan slå ekstern styring fra i Indstillinger.
agent-prompt-allow = Tillad
agent-prompt-deny = Tillad ikke

common-cancel = Annuller
common-save = Gem
common-not-set = Ikke angivet
common-unnamed = Unavngivet
common-none = Ingen
common-mac-address = MAC-adresse
common-list-separator = {", "}

## Network interfaces

interface-up = aktivt
interface-link-down = link nede
interface-wireless = trådløst
interface-hardware-clock = hardwareclock
interface-hardware-clock-named = hardwareclock { $clock }
interface-virtual = virtuelt

## Toolbar

toolbar-choose-interface = Vælg et interface
toolbar-interface = Netværksinterface
toolbar-show-virtual = Vis virtuelle interfaces
toolbar-hide-virtual = Skjul virtuelle interfaces
toolbar-connections = Forbindelser
toolbar-network = Netværk
toolbar-entities = Entiteter
toolbar-rediscover = Bed alle entiteter om at annoncere sig
toolbar-rescan = Ryd og scan alle entiteter igen
toolbar-search = Søg i entiteter og streams
toolbar-presets = Presets
toolbar-log = Log
toolbar-inspector = Inspektør
toolbar-settings = Indstillinger

## The network's state, in place of a view

state-no-interface = Intet interface
state-no-interface-note = Vælg interfacet på AVB-netværket for at opdage entiteter.
state-starting = Starter
state-starting-note = Åbner { $interface }.
state-listening = Lytter
state-listening-note = Entiteter på { $interface } vises her, efterhånden som de annoncerer sig.
state-permission-needed = Tilladelse påkrævet
state-npcap-needed = Npcap påkrævet
state-get-npcap = Hent Npcap
state-copy-command = Kopiér kommandoen
state-cannot-use = Kan ikke bruge { $interface }
state-try-again = Prøv igen

## Entity list

entities-none-yet = Ingen entiteter endnu
entities-none-yet-note = Alle entiteter på netværket med deres roller, SR-klasser og clock.

## Inspector

inspector-title = Inspektør
inspector-entity = Entitet
inspector-streams = Streams
inspector-controls = Kontroller
inspector-diagnostics = Diagnostik
inspector-descriptors = Deskriptorer
inspector-select = Vælg en entitet for at se dens detaljer.
inspector-offline = { $entity } er offline.
inspector-rename = Omdøb
inspector-name = Navn
inspector-identify = Identificer
inspector-model-not-read = Entitetsmodellen er ikke læst.
inspector-no-streams = Ingen streams.
inspector-no-controls = Ingen kontroller at vise.
inspector-no-diagnostics = Ingen interfaces eller tællere rapporteret.
inspector-reading = Læser deskriptorer, { $count } indtil videre.
inspector-read-failed = Kunne ikke læse entitetsmodellen: { $reason }.

entity-section = Entitet
entity-name = Navn
entity-group = Gruppe
entity-product = Produkt
entity-firmware = Firmware
entity-serial-number = Serienummer
entity-configuration = Konfiguration
entity-configuration-of = { $name } ({ $number } af { $count })
entity-milan = Milan
entity-media-clock = Medieclock
entity-clock-domain = Clockdomæne
entity-sampling-rate = Samplingfrekvens
clock-source-numbered = Kilde { $index }
rate-pull = pull { $pull }

stream-format = Format
stream-id = Stream-ID
stream-destination = Destination
stream-max-transit = Maks. transittid

avb-interfaces = AVB-interfaces
avb-interface = Interface
avb-interface-clock-identity = Clockidentitet
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domæne { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Kører
avb-interface-none-reported = Ingen rapporteret
avb-interface-path = Sti
avb-interface-own-grandmaster = Sin egen grandmaster
avb-interface-hops = { $count ->
    [one] { $count } hop fra grandmasteren
   *[other] { $count } hop fra grandmasteren
}
avb-interface-link-up = Link oppe
avb-interface-link-down = Link nede
avb-interface-grandmaster-changes = Grandmasterskift
avb-interface-frames-sent = Sendte rammer
avb-interface-frames-received = Modtagne rammer
avb-interface-crc-errors = CRC-fejl

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } deskriptortype
   *[other] { $count } deskriptortyper
}
tree-clock = Clock
tree-clock-source-from = { $kind }, fra { $location } { $index }
tree-clock-domain-using = Bruger { $source }
tree-clusters = { $count ->
    [one] { $count } klynge
   *[other] { $count } klynger
}
tree-maps = { $count ->
    [one] { $count } tildeling
   *[other] { $count } tildelinger
}

advert-not-advertised = Ikke annonceret
advert-identity = Identitet
advert-entity-id = Entitets-ID
advert-entity-model = Entitetsmodel
advert-roles = Roller
advert-talker = Talker
advert-listener = Listener
advert-clock = Clock
advert-btc = BTC
advert-gptp-domain = gPTP-domæne
advert-sr-classes = SR-klasser
advert-indexes = Indeks i entitetsmodellen
advert-identify-control = Identifikation
advert-avb-interface = AVB-interface
advert-advertising = Annoncering
advert-valid-time = Gyldighedstid
advert-available-index = Tilgængelighedsindeks
advert-association = Association
advert-capabilities = Egenskaber

## Status bar

status-entities = { $count ->
    [one] { $count } entitet
   *[other] { $count } entiteter
}
status-not-discovering = Opdager ikke
status-discovering = Opdager
status-discovering-as = Opdager som { $controller }
status-stopped = Stoppet af en fejl
status-alarm = Alarm
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } og { $count } til

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = controller
role-none = ingen roller
classes-a-and-b = A og B
clock-no-gptp = Ingen gPTP

read-not-read = Ikke læst
read-reading = Læser, { $count } indtil videre
read-ready-unreadable = Klar, { $count } kunne ikke læses
read-ready-cached = Klar, fra cache
read-ready = Klar
read-failed = Mislykkedes: { $reason }

milan-no = Nej
milan-before-1-3 = før 1.3
milan-certified = { $version }, certificeret { $certification }
milan-not-certified = { $version }, ikke certificeret

outcome-status = status { $status }
outcome-no-response = intet svar
outcome-not-possible = ikke muligt
outcome-connect = Kunne ikke forbinde { $talker } til { $listener }: { $reason }.
outcome-disconnect = Kunne ikke frakoble { $listener }: { $reason }.
outcome-identify = Kunne ikke identificere { $entity }: { $reason }.
outcome-rename = Kunne ikke omdøbe { $what } til »{ $name }«: { $reason }.
outcome-rename-group = Kunne ikke omdøbe gruppen for { $entity } til »{ $name }«: { $reason }.
outcome-format-streaming = Kunne ikke ændre formatet for { $stream }: den streamer. Frakobl den først.
outcome-format = Kunne ikke ændre formatet for { $stream }: { $reason }.
outcome-max-transit-time = Kunne ikke ændre maks. transittid for { $stream }: { $reason }.
outcome-max-transit-time-streaming = Kunne ikke ændre maks. transittid for { $stream }: den streamer. Frakobl den først.
outcome-sampling-rate = Kunne ikke ændre samplingfrekvensen for { $entity }: { $reason }.
outcome-clock-source = Kunne ikke ændre clockkilden for { $entity }: { $reason }.
outcome-map = Kunne ikke tildele kanalen på { $entity }: { $reason }.
outcome-unmap = Kunne ikke fjerne kanaltildelingen på { $entity }: { $reason }.
outcome-control = Kunne ikke indstille »{ $control }« på { $entity }: { $reason }.
outcome-control-numbered = Kunne ikke indstille kontrol { $index } på { $entity }: { $reason }.

stream-not-connected = Ikke forbundet
stream-from = Fra { $stream }
stream-from-receiving = Fra { $stream }, modtager
stream-from-waiting = Fra { $stream }, venter på talkeren
stream-from-failed = Fra { $stream }, talkerens reservation mislykkedes: { $reason }
stream-sending-to = Sender til { $destination }

failure-no-response = intet svar
failure-refused = afvist med { $status }
failure-malformed = svaret kunne ikke afkodes
failure-on-this-computer = kører på denne computer; læs den fra en anden

msrp-failure-1 = utilstrækkelig båndbredde
msrp-failure-2 = utilstrækkelige switchressourcer
msrp-failure-3 = utilstrækkelig båndbredde til trafikklassen
msrp-failure-4 = stream-ID bruges af en anden talker
msrp-failure-5 = destinationsadressen er allerede i brug
msrp-failure-6 = fortrængt af en stream med højere rang
msrp-failure-7 = den rapporterede latens er ændret
msrp-failure-8 = egress-porten er ikke AVB-kompatibel
msrp-failure-9 = brug en anden destinationsadresse
msrp-failure-10 = ikke flere MSRP-ressourcer
msrp-failure-11 = ikke flere MMRP-ressourcer
msrp-failure-12 = kan ikke gemme destinationsadressen
msrp-failure-13 = prioriteten er ikke en SR-klasseprioritet
msrp-failure-14 = rammerne er for store til mediet
msrp-failure-15 = portens fan-in-grænse er nået
msrp-failure-16 = første værdi ændret for en registreret stream
msrp-failure-17 = VLAN blokeret på egress-porten
msrp-failure-18 = VLAN-tagging deaktiveret på egress-porten
msrp-failure-19 = SR-klasseprioriteten stemmer ikke overens
msrp-failure-unknown = ukendt årsag
msrp-failure-at = { $reason }, ved switchen { $bridge }

## Entity list columns

column-vendor = Producent
column-model = Model
column-state = Status
column-entity-model-id = Entitetsmodel-ID
column-talker-streams = Talker-streams
column-listener-streams = Listener-streams
column-avb-lite = AVB Lite
column-egress = Egress
column-wireless = Trådløst

## Settings file

settings-no-place = Indstillingerne kan ikke gemmes nogen steder: hjemmemappen er ukendt.
settings-unusable = Kunne ikke bruge { $path }: { $error }.
settings-unsaved = Kunne ikke gemme { $path }: { $error }.

column-remove = Fjern kolonne
column-move-left = Flyt til venstre
column-move-right = Flyt til højre
column-add = Tilføj en kolonne
common-percent = { $value }{" "}%

## Network view

netmap-empty = Intet netværk at vise endnu
netmap-empty-note = Entiteter vises her, når de er læst og har oplyst, hvor de sidder i gPTP-træet.
netmap-focus-clock-path = Clocksti for { $name }
netmap-focus-streams = Streams fra { $name }
netmap-showing = Viser { $what }
netmap-devices = { $count ->
    [one] { $count } enhed
   *[other] { $count } enheder
}
netmap-bridges = { $count ->
    [one] { $count } switch
   *[other] { $count } switche
}
netmap-show-map = Vis kortet
netmap-show-details = Vis detaljerne
stream-numbered = Stream { $index }
netmap-bridge = Switch
netmap-access-point = Adgangspunkt
netmap-device = Enhed
netmap-this-computer = Denne computer
netmap-connected = Forbundet
netmap-advertised = Annonceret, ingen listener klar
netmap-advertised-off-tree = Annonceret, ingen listener klar ({ $listener } er ikke i gPTP-træet)
netmap-failed-at = Reservation mislykkedes ved { $bridge }: { $reason }
netmap-failed = Reservation mislykkedes: { $reason }
netmap-no-bridge-on = Ingen switch fundet på { $interface }
netmap-cannot-listen-on = Kan ikke lytte efter gPTP på { $interface }
netmap-on-this-computer = På denne computer
netmap-path-not-reported = Sti ikke rapporteret
netmap-gptp-not-reported = gPTP ikke rapporteret
netmap-off-tree = Ikke i gPTP-træet
netmap-off-ptp = Ikke i PTP-træet
netmap-not-lite = Ikke i AVB Lite
netmap-lite-not-reported = AVB Lite ikke rapporteret
netmap-synced = Synkroniseret
netmap-not-synced = Ikke synkroniseret
netmap-triib-on = triib på { $interface }
netmap-through-count = { $count } igennem
netmap-out = { $count } ud
netmap-in = { $count } ind
netmap-failed-count = { $count } mislykkedes
netmap-advertised-only = Kun annonceret
netmap-failed-state = Mislykkedes
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Ikke i gPTP-træet: den er sin egen grandmaster
netmap-apart-no-path = Stien er ikke rapporteret; følger grandmaster { $grandmaster }
netmap-apart-unreported = Har ikke rapporteret sin gPTP-tilstand
netmap-apart-no-neighbor = Ingen switch fundet på denne computers interface
netmap-apart-cannot-listen = Denne computer kan ikke lytte efter gPTP på sit interface
netmap-apart-on-this-computer = Den kører på denne computer; læs den fra en anden computer for at se dens gPTP-tilstand
netmap-apart-not-lite = Den kører ikke AVB Lite, så den følger ikke grandmasteren
netmap-apart-lite-unreported = Rapporterer intet om AVB Lite, så det vides ikke, hvad den følger
netmap-clock-tree = Clocktræ
netmap-no-grandmaster = Ingen grandmaster fundet
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Kræver opmærksomhed
netmap-nodes-below = Knuder nedenunder
netmap-bridges-below = Switche nedenunder
netmap-clock-path = Clocksti
netmap-hops = Hop fra grandmasteren
netmap-link-delay = Linkforsinkelse
netmap-bridge-port = Switchport
netmap-link-drops = Linkafbrud
netmap-synced-to-grandmaster = Synkroniseret med grandmasteren
netmap-host-no-gptp = Ikke synkroniseret: denne computer kører ikke gPTP
netmap-link-no-gptp = Ikke synkroniseret: gPTP kører ikke på dens link
netmap-ptp-offset-high = Ikke synkroniseret: { $offset } fra grandmasteren, over de 50 µs, som AVB Lite tillader
netmap-ptp-no-offset = Ikke synkroniseret: den har ikke målt noget offset fra grandmasteren
netmap-audio = Lyd
netmap-media-clock-streams = Medieclock-streams
netmap-audio-streams = Lydstreams
netmap-bound = { $count } bundet
netmap-flowing = Flyder
netmap-advertised-state = Annonceret
netmap-media-clock-stream = Medieclock-stream
netmap-audio-stream = Lydstream
netmap-reaches = Når til
netmap-passing-count = { $count ->
    [one] { $count } stream igennem
   *[other] { $count } streams igennem
}
netmap-through = Igennem
netmap-passing-through = Går igennem
netmap-sending = Sender
netmap-receiving = Modtager
netmap-problems = Problemer
netmap-help-back = Klik på baggrunden for at gå tilbage til oversigten.
netmap-help-stream = Klik på en stream for at inspicere den, eller på baggrunden for at gå tilbage til oversigten.
netmap-help-ptp = I AVB Lite går clocken end-to-end fra grandmasteren til hver enhed gennem switche, der ikke deltager, så ingen af dem vises. En enhed er synkroniseret, så længe den følger grandmasteren inden for 50 µs. Klik på en enhed eller dens ledning for at inspicere dens clock; klik på baggrunden for at fjerne markeringen.
netmap-help-clock = Clocken går fra grandmasteren gennem hver switch til hver knude i træet. En stiplet grå linje er et link, som gPTP ikke kører på. Klik på en enhed eller dens ledning for at inspicere dens clocksti; klik på baggrunden for at fjerne markeringen.
netmap-help-media-clock = Kun medieclock-streams (CRF), tegnet på samme måde som lyd: én ledning pr. stream, farvet efter talker. Klik på en ledning for at inspicere dens stream, eller på en enhed for at se dens streams; klik på baggrunden for at fjerne markeringen.
netmap-help-audio = Hver stream har sin egen ledning, der går ind i og ud af hver switch, den passerer. Farven følger talkeren: hver talker har sin egen farvetone, og dens streams er nuancer af den. Bevægelige prikker betyder, at der flyder lyd; en stillestående rød linje er en mislykket reservation, og en stillestående grå linje er annonceret, uden at nogen listener er klar; begge stopper, hvor reservationen stopper. Enheder i den midterste kolonne forbinder direkte til grandmasterens switch. Klik på en ledning for at inspicere dens stream, eller på en enhed for at se dens streams; klik på baggrunden for at fjerne markeringen.

## Connections

matrix-nothing-shown = Ingen streams at vise
matrix-nothing-shown-note = Ændr søgningen eller filtrene for at se flere streams.
matrix-empty = Ingen streams at forbinde
matrix-empty-note = Talker-streams og listener-streams mødes her, når entiteter med streams er læst.
matrix-all-streams = Alle streams
matrix-connectable-only = Skjul det, der ikke kan forbindes
matrix-none-hidden = Alle viste streams kan forbindes
matrix-hidden = { $count ->
    [one] { $count } stream skjult
   *[other] { $count } streams skjult
}
matrix-own = En entitets udgange kan ikke forbindes til dens egne indgange.
matrix-working = Arbejder på det.
matrix-waiting-change = Venter på den seneste ændring af denne indgang.
matrix-connected = Forbundet og modtager. Klik for at frakoble.
matrix-bound-waiting = Bundet, venter på talkerens stream. Klik for at frakoble.
matrix-bound-failed = Bundet, men talkerens reservation mislykkedes: { $reason }. Klik for at frakoble.
matrix-bound-formats-differ = Bundet, men formaterne er forskellige: talkeren sender { $sent }, indgangen er sat til { $set }. Klik for at frakoble.
matrix-formats-match = Formaterne passer ({ $format }). Klik for at forbinde.
matrix-format-must-change = Indgangen kan tage { $sent }, men er sat til { $set }, så den spiller muligvis ikke, før formatet ændres. Klik for at vælge et format.
matrix-format-input = Skift indgangen til
matrix-format-both = Skift begge til
matrix-format-talker-busy = Talkeren streamer til andre listeners, så dens format bevares.
matrix-format-anyway = Forbind uden at ændre formater
matrix-incompatible = Indgangen kan ikke tage { $sent }. Den er sat til { $set }.
matrix-group-none = Ikke forbundet. Fold ud for at forbinde streams én ad gangen.
matrix-group-connected = { $count } forbundet. Fold ud for at se hver enkelt.
matrix-outputs-expand = { $count ->
    [one] { $count } stream-udgang. Klik på pilen for at folde ud, på navnet for at inspicere den.
   *[other] { $count } stream-udgange. Klik på pilen for at folde ud, på navnet for at inspicere den.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } stream-udgang. Klik på pilen for at folde sammen, på navnet for at inspicere den.
   *[other] { $count } stream-udgange. Klik på pilen for at folde sammen, på navnet for at inspicere den.
}
matrix-inputs-expand = { $count ->
    [one] { $count } stream-indgang. Klik på pilen for at folde ud, på navnet for at inspicere den.
   *[other] { $count } stream-indgange. Klik på pilen for at folde ud, på navnet for at inspicere den.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } stream-indgang. Klik på pilen for at folde sammen, på navnet for at inspicere den.
   *[other] { $count } stream-indgange. Klik på pilen for at folde sammen, på navnet for at inspicere den.
}
matrix-stream-inspect = { $detail } Klik for at inspicere { $entity }.
matrix-point = Peg på en celle
matrix-point-note = for at se dens talker og listener, og om deres formater passer sammen.
matrix-legend-waiting = Bundet, venter på streamen
matrix-legend-trouble = Bundet, noget er galt
matrix-legend-open = Kan forbindes
matrix-legend-change = Indgangens format skal ændres først
matrix-legend-incompatible = Formaterne passer ikke sammen
matrix-talker-outputs = Talker-udgange
matrix-listener-inputs = Listener-indgange

common-thousands-separator = {"."}

## Diagnostics

diag-since-start = Talt siden entiteten startede.
diag-stream-input = Stream-indgang
diag-stream-output = Stream-udgang
diag-locked = { $count ->
    [0] ikke låst
    [one] låst én gang
    [2] låst to gange
   *[other] låst { $number } gange
}
diag-lost-lock = { $count ->
    [0] ikke mistet lås
    [one] mistet lås én gang
    [2] mistet lås to gange
   *[other] mistet lås { $number } gange
}
diag-frames-in = { $count ->
    [one] { $number } ramme ind
   *[other] { $number } rammer ind
}
diag-frames-out = { $count ->
    [one] { $number } ramme ud
   *[other] { $number } rammer ud
}
diag-media-locked = { $count ->
    [0] ikke låst til medieclock
    [one] låst til medieclock én gang
    [2] låst til medieclock to gange
   *[other] låst til medieclock { $number } gange
}
diag-lost-media-lock = { $count ->
    [0] ikke mistet medieclock-lås
    [one] mistet medieclock-lås én gang
    [2] mistet medieclock-lås to gange
   *[other] mistet medieclock-lås { $number } gange
}
diag-interrupted = { $count ->
    [0] ikke afbrudt
    [one] afbrudt én gang
    [2] afbrudt to gange
   *[other] afbrudt { $number } gange
}
diag-out-of-sequence = { $count ->
    [one] { $number } ramme uden for rækkefølge
   *[other] { $number } rammer uden for rækkefølge
}
diag-media-resets = { $count ->
    [one] { $number } medienulstilling
   *[other] { $number } medienulstillinger
}
diag-timestamps-uncertain = { $count ->
    [0] tidsstempler ikke usikre
    [one] tidsstempler usikre én gang
    [2] tidsstempler usikre to gange
   *[other] tidsstempler usikre { $number } gange
}
diag-no-timestamp = { $count ->
    [one] { $number } ramme uden tidsstempel
   *[other] { $number } rammer uden tidsstempel
}
diag-unsupported-format = { $count ->
    [one] { $number } ramme i et ikke-understøttet format
   *[other] { $number } rammer i et ikke-understøttet format
}
diag-late = { $count ->
    [one] { $number } ramme for sent
   *[other] { $number } rammer for sent
}
diag-early = { $count ->
    [one] { $number } ramme for tidligt
   *[other] { $number } rammer for tidligt
}
diag-started = { $count ->
    [0] ikke startet
    [one] startet én gang
    [2] startet to gange
   *[other] startet { $number } gange
}
diag-stopped = { $count ->
    [0] ikke stoppet
    [one] stoppet én gang
    [2] stoppet to gange
   *[other] stoppet { $number } gange
}
diag-reservation-failed = talkerens reservation mislykkedes: { $reason }
diag-latency = { $microseconds } µs akkumuleret latens

## AVB Lite

lite-active = Aktiv
lite-active-untagged = Aktiv, utagget
lite-active-vlan = Aktiv, VLAN { $vlan }
lite-capable = Understøttet
lite-mode = Tilstand
lite-mode-capable = AVB (AVB Lite understøttet)
lite-because = Årsag
lite-fallback-none = ingen angivet
lite-fallback-endpoint = et andet endepunkts erklæring kom igennem, så der er ingen AVB-switch imellem
lite-fallback-unanswered = ni peer delay-forespørgsler blev ikke besvaret
lite-fallback-responders = to eller flere svarede på én peer delay-forespørgsel, så switchen er ikke AVB-kompatibel
lite-fallback-configured = operatøren eller en controller har indstillet det
lite-fallback-other = en årsag, som profilen ikke nævner
lite-other-profile = En anden profil
lite-ptp-domain = { $profile }, domæne { $domain }
lite-offset = Offset
lite-offset-from = { $offset } fra { $grandmaster }
lite-media-vlan = Medie-VLAN
lite-untagged = Utagget
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Op til { $count } listener pr. stream, derefter multicast
   *[other] Op til { $count } listenere pr. stream, derefter multicast
}
lite-link = Link
lite-bandwidth = Båndbredde
lite-egress-of = { $used } af { $link }, { $share }
lite-egress-of-assumed = { $used } af { $link }, { $share }, gigabit-link antaget
lite-egress-reported = Sådan som entiteten tæller sine godkendte streams.
lite-egress-worked-out = Ud fra formaterne på dens forbundne stream-udgange.
lite-alarm-offset = PTP-offset { $offset }, over de 50 µs, som AVB Lite tillader
lite-alarm-egress = Egress på { $share } af linket, over de { $limit }, som streams må optage

## AVB Wireless

wireless-station = Station
wireless-access-point = Adgangspunkt
wireless-role = Rolle
wireless-mode = Tilstand
wireless-time = Tid
wireless-mode-a-ftm = Mode A, 802.1AS over FTM
wireless-mode-a-tm = Mode A, 802.1AS over TM
wireless-mode-b = Mode B, fra beacon-rammer
wireless-no-time = Ingen tid
wireless-other-mode = En tilstand, som profilen ikke nævner
wireless-locked = Låst
wireless-holdover = I holdover
wireless-not-locked = Ikke låst
wireless-row-locked = Station, låst
wireless-row-holdover = Station, i holdover
wireless-row-not-locked = Station, ikke låst
wireless-row-access-point = { $count ->
    [one] Adgangspunkt, { $count } station
   *[other] Adgangspunkt, { $count } stationer
}
wireless-link = Link
wireless-channel = kanal { $channel }
wireless-not-known = Ukendt
wireless-signal = Signal
wireless-rate = Sendehastighed
wireless-ftm-valid = { $share } gyldige
wireless-rtt = tur-retur { $rtt }
wireless-bursts = { $count ->
    [one] bursts på { $count } ramme
   *[other] bursts på { $count } rammer
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = adgangspunktet tildeler FTM-bursts med et andet antal rammer end tre eller to
wireless-reason-measurement = hverken FTM eller TM med adgangspunktet
wireless-reason-signaling = ingen gPTP-capable Signaling fra adgangspunktet
wireless-reason-other = en årsag, som profilen ikke nævner
wireless-servo = Servofejl
wireless-stations = Stationer
wireless-station-count = { $count ->
    [one] { $count } station
   *[other] { $count } stationer
}
wireless-no-ftm = uden FTM
wireless-unserved = Ikke-betjente listenere
wireless-frame-budget = Rammebudget
wireless-frame-use = { $used } af { $budget } rammer pr. sekund
wireless-stream-frames = Stream-rammer
wireless-frames-of = { $readdressed } til stationer, { $unmapped } uden listener, { $dropped } kasseret, { $restored } fra stationer
wireless-class-a-allowed = Tilladt
wireless-class-a-not-allowed = Ikke tilladt
wireless-alarm-not-locked = Wi-Fi-tid ikke låst til adgangspunktet
wireless-alarm-holdover = Wi-Fi-tid i holdover, mistet lås til adgangspunktet
wireless-alarm-unserved = { $count ->
    [one] { $count } listener på Wi-Fi-porten betjenes ikke, over unicast-grænsen
   *[other] { $count } listenere på Wi-Fi-porten betjenes ikke, over unicast-grænsen
}

## Log

log-all = Alle
log-warnings = Advarsler
log-pause = Pause
log-resume = Genoptag
log-clear = Ryd
log-empty = Alle ATDECC-rammer, som triib sender og modtager, vises her, nyeste først.
log-none-match = Ingen gemt ramme matcher filteret.
log-frames = { $count ->
    [one] { $count } ramme
   *[other] { $count } rammer
}
log-shown-of = { $shown } af { $all } rammer
log-sent = Sendt
log-heard = Modtaget
log-not-decoded = Ikke afkodet
log-warning-short = Feltet control_data_length angiver { $missing } oktetter ud over rammens slutning.
log-warning-undecodable = Kan ikke afkodes: { $error }.
log-warning-long-acmp = I den lange ACMP-form, som en Milan-entitet ikke må sende (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Kanaltildelinger
mapping-inputs = Indgange
mapping-outputs = Udgange
mapping-port = port { $number }
mapping-fixed = fast
mapping-not-read = Ikke læst endnu.
mapping-no-clusters = Ingen klynger.
mapping-no-streams = Ingen lydstreams.
mapping-none = Ingen tildelinger.
mapping-not-mapped = Ikke tildelt
mapping-cluster-numbered = Klynge { $index }

## Presets

presets-note = Et preset gemmer hver entitets clockkilder, samplingfrekvenser, streamformater, kontroller og forbindelser. Når det hentes, ændres det, der afviger.
presets-none = Ingen presets gemt endnu.
presets-connections = { $count ->
    [one] { $count } forbindelse
   *[other] { $count } forbindelser
}
presets-recall = Hent
presets-delete = Slet
presets-no-place = Presets kan ikke gemmes nogen steder: hjemmemappen er ukendt.
presets-undeletable = Kunne ikke slette { $path }: { $error }.
presets-saved = { $count ->
    [one] Gemte »{ $name }« med { $count } entitet.
   *[other] Gemte »{ $name }« med { $count } entiteter.
}
presets-nothing-differs = Intet afviger fra »{ $name }«.
presets-recalling = { $count ->
    [one] Henter »{ $name }«: { $count } ændring.
   *[other] Henter »{ $name }«: { $count } ændringer.
}
presets-missing = { $report } Ikke til stede eller ikke læst: { $missing }.
presets-deleted = Slettede »{ $name }«.
presets-host-note = Det gemmer også denne computers egne talkere og listenere og starter dem igen, når det hentes.
presets-host-endpoints = { $count } på denne computer
presets-starting-host = Starter denne computers talkere og listenere for »{ $name }«; resten følger, når de er tilbage.

## Controls

control-numbered = Kontrol { $index }
control-not-shown = Vises ikke her
control-option = Mulighed { $number }

## Network errors

network-permission = triib skal have tilladelse til at sende og modtage rå Ethernet-rammer.
network-needs-npcap = triib skal bruge Npcap til at sende og modtage rå Ethernet-rammer.
network-npcap-administrators = Npcap lader kun administratorer sende og modtage rå Ethernet-rammer. Kør triib som administrator, eller installér Npcap igen uden indstillingen kun for administratorer.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own talkers and listeners

host-add-talker = Tilføj talker
host-add-listener = Tilføj listener
host-add-device = Tilføj enhed
host-show-mine = Vis kun denne computers egne talkere og listenere
host-show-all = Vis alle entiteter
host-new-talker = Værts-talker { $number }
host-new-listener = Værts-listener { $number }
host-new-device = Værtsenhed { $number }
host-failed = Kunne ikke tilføje den til denne computer: { $reason }
host-needs-clock = Denne computers egne talkere og listenere kræver et kablet interface med en PTP-hardwareclock
host-no-ptp4l = ptp4l svarer ikke, så denne computers streams kan ikke holde gPTP-tid
host-elsewhere = triib-endpointd kører på { $interface } for en anden bruger eller som root, så denne computers talkere og listenere kører der, ikke her
host-foreign-mrp = Et andet program erklærer MSRP eller MVRP på { $interface } fra denne computers adresse og kan dermed trække det tilbage, som denne computers streams har brug for
host-alarm-foreign-mrp = Et andet program på denne computer erklærer MSRP eller MVRP på { $interface }
host-state = Tilstand
host-streaming = Streamer
host-waiting = Venter på en listener
host-listening = Lytter
host-bound = Bundet, venter på talkeren
host-unbound = Ikke bundet
host-audio-from = Lyd fra
host-audio-to = Lyd til
host-channels = Kanaler
host-virtual-device = Virtuel enhed
host-add-output-stream = Tilføj udgangsstream
host-add-input-stream = Tilføj indgangsstream
host-remove-stream = Fjern stream
host-silence = Stilhed
host-tone = Testtone
host-nowhere = Ingen steder
host-default-device = Standardenhed
host-remove = Fjern fra denne computer
