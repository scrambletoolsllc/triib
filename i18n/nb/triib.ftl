# triib's interface text in Norwegian Bokmål.

## Language

language-name = Norsk bokmål

## Common

common-close = Lukk
common-more = Mer
common-keep-toolbar-shown = Vis alltid verktøylinjen
common-auto-hide-toolbar = Skjul verktøylinjen automatisk

## Settings

settings-title = Innstillinger
settings-general = Generelt
settings-appearance = Utseende
settings-language = Språk
settings-language-system = Systemstandard: { $language }
settings-language-note = Tekstfelt skriver med systemets inndataspråk.
settings-appearance-system = System
settings-appearance-light = Lyst
settings-appearance-dark = Mørkt
settings-colors = Farger
settings-system-accent = Bruk systemets aksentfarge
settings-accent-picked = Fargen nedenfor danner grunnlaget for fargene i triib.
settings-accent-omarchy = Fra Omarchy-temaet, { $theme }.
settings-accent-desktop = Fra skrivebordets aksentfarge.
settings-accent-none = Skrivebordet har ingen aksentfarge, så fargen nedenfor brukes.
settings-motion = Bevegelse
settings-animations = Animasjoner
settings-animations-note = Fjærende og glidende overganger når noe endres.
settings-animations-reduced = Skrivebordet ber om redusert bevegelse, så triib holder seg i ro.

common-cancel = Avbryt
common-save = Lagre
common-not-set = Ikke angitt
common-unnamed = Uten navn
common-none = Ingen
common-mac-address = MAC-adresse
common-list-separator = {", "}

## Network interfaces

interface-up = aktivt
interface-link-down = link nede
interface-wireless = trådløst
interface-hardware-clock = maskinvareklokke
interface-hardware-clock-named = maskinvareklokke { $clock }
interface-virtual = virtuelt

## Toolbar

toolbar-choose-interface = Velg et grensesnitt
toolbar-interface = Nettverksgrensesnitt
toolbar-show-virtual = Vis virtuelle grensesnitt
toolbar-hide-virtual = Skjul virtuelle grensesnitt
toolbar-connections = Tilkoblinger
toolbar-network = Nettverk
toolbar-entities = Entiteter
toolbar-rediscover = Be alle entiteter om å annonsere seg
toolbar-rescan = Tøm og skann alle entiteter på nytt
toolbar-search = Søk i entiteter og streamer
toolbar-presets = Presets
toolbar-log = Logg
toolbar-inspector = Inspektør
toolbar-settings = Innstillinger

## The network's state, in place of a view

state-no-interface = Ingen grensesnitt
state-no-interface-note = Velg grensesnittet på AVB-nettverket for å oppdage entiteter.
state-starting = Starter
state-starting-note = Åpner { $interface }.
state-listening = Lytter
state-listening-note = Entiteter på { $interface } vises her etter hvert som de annonserer seg.
state-permission-needed = Tillatelse kreves
state-npcap-needed = Npcap kreves
state-get-npcap = Hent Npcap
state-copy-command = Kopier kommandoen
state-cannot-use = Kan ikke bruke { $interface }
state-try-again = Prøv igjen

## Entity list

entities-none-yet = Ingen entiteter ennå
entities-none-yet-note = Alle entiteter på nettverket, med roller, SR-klasser og klokke.

## Inspector

inspector-title = Inspektør
inspector-entity = Entitet
inspector-streams = Streamer
inspector-controls = Kontroller
inspector-diagnostics = Diagnostikk
inspector-descriptors = Deskriptorer
inspector-select = Velg en entitet for å se detaljene.
inspector-offline = { $entity } er offline.
inspector-rename = Gi nytt navn
inspector-name = Navn
inspector-identify = Identifiser
inspector-model-not-read = Entitetsmodellen er ikke lest.
inspector-no-streams = Ingen streamer.
inspector-no-controls = Ingen kontroller å vise.
inspector-no-diagnostics = Ingen grensesnitt eller tellere rapportert.
inspector-reading = Leser deskriptorer, { $count } så langt.
inspector-read-failed = Kunne ikke lese entitetsmodellen: { $reason }.

entity-section = Entitet
entity-name = Navn
entity-group = Gruppe
entity-product = Produkt
entity-firmware = Firmware
entity-serial-number = Serienummer
entity-configuration = Konfigurasjon
entity-configuration-of = { $name } ({ $number } av { $count })
entity-milan = Milan
entity-media-clock = Medieklokke
entity-clock-domain = Klokkedomene
entity-sampling-rate = Samplingsfrekvens
clock-source-numbered = Kilde { $index }
rate-pull = pull { $pull }

stream-inputs = Stream-innganger
stream-outputs = Stream-utganger
stream-max-transit-time = Maks. transittid { $time }

avb-interfaces = AVB-grensesnitt
avb-interface = Grensesnitt
avb-interface-clock-identity = Klokkeidentitet
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domene { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Kjører
avb-interface-none-reported = Ingen rapportert
avb-interface-path = Sti
avb-interface-own-grandmaster = Sin egen grandmaster
avb-interface-hops = { $count ->
    [one] { $count } hopp fra grandmasteren
   *[other] { $count } hopp fra grandmasteren
}
avb-interface-link-up = Link oppe
avb-interface-link-down = Link nede
avb-interface-grandmaster-changes = Grandmasterbytter
avb-interface-frames-sent = Sendte rammer
avb-interface-frames-received = Mottatte rammer
avb-interface-crc-errors = CRC-feil

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } deskriptortype
   *[other] { $count } deskriptortyper
}
tree-clock = Klokke
tree-clock-source-from = { $kind }, fra { $location } { $index }
tree-clock-domain-using = Bruker { $source }
tree-clusters = { $count ->
    [one] { $count } klynge
   *[other] { $count } klynger
}
tree-maps = { $count ->
    [one] { $count } tilordning
   *[other] { $count } tilordninger
}

advert-not-advertised = Ikke annonsert
advert-identity = Identitet
advert-entity-id = Entitets-ID
advert-entity-model = Entitetsmodell
advert-roles = Roller
advert-talker = Talker
advert-listener = Listener
advert-clock = Klokke
advert-btc = BTC
advert-gptp-domain = gPTP-domene
advert-sr-classes = SR-klasser
advert-indexes = Indekser i entitetsmodellen
advert-identify-control = Identifisering
advert-avb-interface = AVB-grensesnitt
advert-advertising = Annonsering
advert-valid-time = Gyldighetstid
advert-available-index = Tilgjengelighetsindeks
advert-association = Assosiasjon
advert-capabilities = Egenskaper

## Status bar

status-entities = { $count ->
    [one] { $count } entitet
   *[other] { $count } entiteter
}
status-not-discovering = Oppdager ikke
status-discovering = Oppdager
status-discovering-as = Oppdager som { $controller }
status-stopped = Stoppet av en feil
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

read-not-read = Ikke lest
read-reading = Leser, { $count } så langt
read-ready-unreadable = Klar, { $count } kunne ikke leses
read-ready-cached = Klar, fra cache
read-ready = Klar
read-failed = Mislyktes: { $reason }

milan-no = Nei
milan-before-1-3 = før 1.3
milan-certified = { $version }, sertifisert { $certification }
milan-not-certified = { $version }, ikke sertifisert

outcome-status = status { $status }
outcome-no-response = ikke noe svar
outcome-not-possible = ikke mulig
outcome-connect = Kunne ikke koble { $talker } til { $listener }: { $reason }.
outcome-disconnect = Kunne ikke koble fra { $listener }: { $reason }.
outcome-identify = Kunne ikke identifisere { $entity }: { $reason }.
outcome-rename = Kunne ikke endre navnet på { $what } til «{ $name }»: { $reason }.
outcome-rename-group = Kunne ikke endre navnet på gruppen for { $entity } til «{ $name }»: { $reason }.
outcome-format-streaming = Kunne ikke endre formatet for { $stream }: den streamer. Koble den fra først.
outcome-format = Kunne ikke endre formatet for { $stream }: { $reason }.
outcome-sampling-rate = Kunne ikke endre samplingsfrekvensen for { $entity }: { $reason }.
outcome-clock-source = Kunne ikke endre klokkekilden for { $entity }: { $reason }.
outcome-map = Kunne ikke tilordne kanalen på { $entity }: { $reason }.
outcome-unmap = Kunne ikke fjerne kanaltilordningen på { $entity }: { $reason }.
outcome-control = Kunne ikke stille inn «{ $control }» på { $entity }: { $reason }.
outcome-control-numbered = Kunne ikke stille inn kontroll { $index } på { $entity }: { $reason }.

stream-not-connected = Ikke tilkoblet
stream-from = Fra { $stream }
stream-from-receiving = Fra { $stream }, mottar
stream-from-waiting = Fra { $stream }, venter på talkeren
stream-from-failed = Fra { $stream }, talkerens reservasjon mislyktes: { $reason }
stream-sending-to = Sender til { $destination }

failure-no-response = ikke noe svar
failure-refused = avvist med { $status }
failure-malformed = svaret kunne ikke dekodes
failure-on-this-computer = kjører på denne datamaskinen; les den fra en annen

msrp-failure-1 = utilstrekkelig båndbredde
msrp-failure-2 = utilstrekkelige switchressurser
msrp-failure-3 = utilstrekkelig båndbredde for trafikklassen
msrp-failure-4 = stream-ID brukes av en annen talker
msrp-failure-5 = måladressen er allerede i bruk
msrp-failure-6 = fortrengt av en stream med høyere rang
msrp-failure-7 = rapportert latens er endret
msrp-failure-8 = egress-porten er ikke AVB-kompatibel
msrp-failure-9 = bruk en annen måladresse
msrp-failure-10 = tom for MSRP-ressurser
msrp-failure-11 = tom for MMRP-ressurser
msrp-failure-12 = kan ikke lagre måladressen
msrp-failure-13 = prioriteten er ikke en SR-klasseprioritet
msrp-failure-14 = rammene er for store for mediet
msrp-failure-15 = portens fan-in-grense er nådd
msrp-failure-16 = første verdi endret for en registrert stream
msrp-failure-17 = VLAN blokkert på egress-porten
msrp-failure-18 = VLAN-tagging deaktivert på egress-porten
msrp-failure-19 = SR-klasseprioriteten stemmer ikke
msrp-failure-unknown = ukjent årsak
msrp-failure-at = { $reason }, ved switchen { $bridge }

## Entity list columns

column-vendor = Produsent
column-model = Modell
column-state = Status
column-entity-model-id = Entitetsmodell-ID
column-talker-streams = Talker-streamer
column-listener-streams = Listener-streamer
column-avb-lite = AVB Lite
column-egress = Egress
column-wireless = Trådløst

## Settings file

settings-no-place = Innstillingene kan ikke lagres noe sted: hjemmemappen er ukjent.
settings-unusable = Kunne ikke bruke { $path }: { $error }.
settings-unsaved = Kunne ikke lagre { $path }: { $error }.

column-remove = Fjern kolonne
column-move-left = Flytt til venstre
column-move-right = Flytt til høyre
column-add = Legg til en kolonne
common-percent = { $value }{" "}%

## Network view

netmap-empty = Ikke noe nettverk å vise ennå
netmap-empty-note = Entiteter vises her når de er lest og har oppgitt hvor de sitter i gPTP-treet.
netmap-focus-clock-path = Klokkesti for { $name }
netmap-focus-streams = Streamer fra { $name }
netmap-showing = Viser { $what }
netmap-devices = { $count ->
    [one] { $count } enhet
   *[other] { $count } enheter
}
netmap-bridges = { $count ->
    [one] { $count } switch
   *[other] { $count } switcher
}
netmap-show-map = Vis kartet
netmap-show-details = Vis detaljene
stream-numbered = Stream { $index }
netmap-bridge = Switch
netmap-access-point = Tilgangspunkt
netmap-device = Enhet
netmap-this-computer = Denne datamaskinen
netmap-connected = Tilkoblet
netmap-advertised = Annonsert, ingen listener klar
netmap-advertised-off-tree = Annonsert, ingen listener klar ({ $listener } er ikke i gPTP-treet)
netmap-failed-at = Reservasjonen mislyktes ved { $bridge }: { $reason }
netmap-failed = Reservasjonen mislyktes: { $reason }
netmap-no-bridge-on = Ingen switch funnet på { $interface }
netmap-cannot-listen-on = Kan ikke lytte etter gPTP på { $interface }
netmap-on-this-computer = På denne datamaskinen
netmap-path-not-reported = Stien er ikke rapportert
netmap-gptp-not-reported = gPTP er ikke rapportert
netmap-off-tree = Ikke i gPTP-treet
netmap-off-ptp = Ikke i PTP-treet
netmap-not-lite = Ikke i AVB Lite
netmap-lite-not-reported = AVB Lite er ikke rapportert
netmap-synced = Synkronisert
netmap-not-synced = Ikke synkronisert
netmap-triib-on = triib på { $interface }
netmap-through-count = { $count } gjennom
netmap-out = { $count } ut
netmap-in = { $count } inn
netmap-failed-count = { $count } mislyktes
netmap-advertised-only = Bare annonsert
netmap-failed-state = Mislyktes
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Ikke i gPTP-treet: den er sin egen grandmaster
netmap-apart-no-path = Stien er ikke rapportert; følger grandmaster { $grandmaster }
netmap-apart-unreported = Har ikke rapportert gPTP-tilstanden sin
netmap-apart-no-neighbor = Ingen switch funnet på grensesnittet til denne datamaskinen
netmap-apart-cannot-listen = Denne datamaskinen kan ikke lytte etter gPTP på grensesnittet sitt
netmap-apart-on-this-computer = Den kjører på denne datamaskinen; les den fra en annen datamaskin for å se gPTP-tilstanden
netmap-apart-not-lite = Den kjører ikke AVB Lite, så den følger ikke grandmasteren
netmap-apart-lite-unreported = Rapporterer ingenting om AVB Lite, så det er ukjent hva den følger
netmap-clock-tree = Klokketre
netmap-no-grandmaster = Ingen grandmaster funnet
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Trenger oppmerksomhet
netmap-nodes-below = Noder under
netmap-bridges-below = Switcher under
netmap-clock-path = Klokkesti
netmap-hops = Hopp fra grandmasteren
netmap-link-delay = Linkforsinkelse
netmap-bridge-port = Switchport
netmap-link-drops = Linkbrudd
netmap-synced-to-grandmaster = Synkronisert med grandmasteren
netmap-host-no-gptp = Ikke synkronisert: denne datamaskinen kjører ikke gPTP
netmap-link-no-gptp = Ikke synkronisert: gPTP kjører ikke på linken
netmap-ptp-offset-high = Ikke synkronisert: { $offset } fra grandmasteren, over de 50 µs som AVB Lite tillater
netmap-ptp-no-offset = Ikke synkronisert: den har ikke målt noen offset fra grandmasteren
netmap-audio = Lyd
netmap-media-clock-streams = Medieklokkestreamer
netmap-audio-streams = Lydstreamer
netmap-bound = { $count } bundet
netmap-flowing = Flyter
netmap-advertised-state = Annonsert
netmap-media-clock-stream = Medieklokkestream
netmap-audio-stream = Lydstream
netmap-reaches = Når til
netmap-passing-count = { $count ->
    [one] { $count } stream gjennom
   *[other] { $count } streamer gjennom
}
netmap-through = Gjennom
netmap-passing-through = Går gjennom
netmap-sending = Sender
netmap-receiving = Mottar
netmap-problems = Problemer
netmap-help-back = Klikk på bakgrunnen for å gå tilbake til oversikten.
netmap-help-stream = Klikk på en stream for å inspisere den, eller på bakgrunnen for å gå tilbake til oversikten.
netmap-help-ptp = I AVB Lite går klokken ende til ende fra grandmasteren til hver enhet gjennom switcher som ikke deltar, så ingen av dem vises. En enhet er synkronisert så lenge den følger grandmasteren innenfor 50 µs. Klikk på en enhet eller ledningen dens for å inspisere klokken; klikk på bakgrunnen for å fjerne markeringen.
netmap-help-clock = Klokken går fra grandmasteren gjennom hver switch til hver node i treet. En stiplet grå linje er en link der gPTP ikke kjører. Klikk på en enhet eller ledningen dens for å inspisere klokkestien; klikk på bakgrunnen for å fjerne markeringen.
netmap-help-media-clock = Bare medieklokkestreamer (CRF), tegnet på samme måte som lyd: én ledning per stream, farget etter talker. Klikk på en ledning for å inspisere streamen, eller på en enhet for å se streamene dens; klikk på bakgrunnen for å fjerne markeringen.
netmap-help-audio = Hver stream har sin egen ledning, som går inn i og ut av hver switch den passerer. Fargen følger talkeren: hver talker har sin egen fargetone, og streamene dens er nyanser av den. Bevegelige prikker betyr at lyd flyter; en stillestående rød linje er en mislykket reservasjon, og en stillestående grå linje er annonsert uten at noen listener er klar; begge stopper der reservasjonen stopper. Enheter i den midterste kolonnen er koblet direkte til grandmasterens switch. Klikk på en ledning for å inspisere streamen, eller på en enhet for å se streamene dens; klikk på bakgrunnen for å fjerne markeringen.

## Connections

matrix-nothing-shown = Ingen streamer å vise
matrix-nothing-shown-note = Endre søket eller filtrene for å se flere streamer.
matrix-empty = Ingen streamer å koble til
matrix-empty-note = Talker-streamer og listener-streamer møtes her når entiteter med streamer er lest.
matrix-all-streams = Alle streamer
matrix-connectable-only = Skjul det som ikke kan kobles til
matrix-none-hidden = Alle viste streamer kan kobles til
matrix-hidden = { $count ->
    [one] { $count } stream skjult
   *[other] { $count } streamer skjult
}
matrix-own = En entitets utganger kan ikke kobles til dens egne innganger.
matrix-working = Jobber med saken.
matrix-waiting-change = Venter på den siste endringen av denne inngangen.
matrix-connected = Tilkoblet og mottar. Klikk for å koble fra.
matrix-bound-waiting = Bundet, venter på talkerens stream. Klikk for å koble fra.
matrix-bound-failed = Bundet, men talkerens reservasjon mislyktes: { $reason }. Klikk for å koble fra.
matrix-bound-formats-differ = Bundet, men formatene er ulike: talkeren sender { $sent }, inngangen er satt til { $set }. Klikk for å koble fra.
matrix-formats-match = Formatene stemmer ({ $format }). Klikk for å koble til.
matrix-format-must-change = Inngangen tar { $sent }, men er satt til { $set }, så den spiller kanskje ikke før formatet endres. Klikk for å velge et format.
matrix-format-input = Endre inngangen til
matrix-format-both = Endre begge til
matrix-format-talker-busy = Talkeren strømmer til andre listenere, så formatet beholdes.
matrix-format-anyway = Koble til uten å endre formater
matrix-incompatible = Inngangen tar ikke { $sent }. Den er satt til { $set }.
matrix-group-none = Ikke tilkoblet. Utvid for å koble til streamer én etter én.
matrix-group-connected = { $count } tilkoblet. Utvid for å se hver enkelt.
matrix-outputs-expand = { $count ->
    [one] { $count } stream-utgang. Klikk på pilen for å utvide, på navnet for å inspisere den.
   *[other] { $count } stream-utganger. Klikk på pilen for å utvide, på navnet for å inspisere den.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } stream-utgang. Klikk på pilen for å skjule, på navnet for å inspisere den.
   *[other] { $count } stream-utganger. Klikk på pilen for å skjule, på navnet for å inspisere den.
}
matrix-inputs-expand = { $count ->
    [one] { $count } stream-inngang. Klikk på pilen for å utvide, på navnet for å inspisere den.
   *[other] { $count } stream-innganger. Klikk på pilen for å utvide, på navnet for å inspisere den.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } stream-inngang. Klikk på pilen for å skjule, på navnet for å inspisere den.
   *[other] { $count } stream-innganger. Klikk på pilen for å skjule, på navnet for å inspisere den.
}
matrix-stream-inspect = { $detail } Klikk for å inspisere { $entity }.
matrix-point = Pek på en celle
matrix-point-note = for å se talkeren og listeneren og om formatene deres passer sammen.
matrix-legend-waiting = Bundet, venter på streamen
matrix-legend-trouble = Bundet, noe er galt
matrix-legend-open = Kan kobles til
matrix-legend-change = Inngangsformatet må endres først
matrix-legend-incompatible = Formatene passer ikke sammen
matrix-talker-outputs = Talker-utganger
matrix-listener-inputs = Listener-innganger

common-thousands-separator = {" "}

## Diagnostics

diag-since-start = Talt siden entiteten startet.
diag-stream-input = Stream-inngang
diag-stream-output = Stream-utgang
diag-locked = { $count ->
    [0] ikke låst
    [one] låst én gang
    [2] låst to ganger
   *[other] låst { $number } ganger
}
diag-lost-lock = { $count ->
    [0] ikke mistet låsing
    [one] mistet låsing én gang
    [2] mistet låsing to ganger
   *[other] mistet låsing { $number } ganger
}
diag-frames-in = { $count ->
    [one] { $number } ramme inn
   *[other] { $number } rammer inn
}
diag-frames-out = { $count ->
    [one] { $number } ramme ut
   *[other] { $number } rammer ut
}
diag-media-locked = { $count ->
    [0] ikke låst til medieklokken
    [one] låst til medieklokken én gang
    [2] låst til medieklokken to ganger
   *[other] låst til medieklokken { $number } ganger
}
diag-lost-media-lock = { $count ->
    [0] ikke mistet låsing til medieklokken
    [one] mistet låsing til medieklokken én gang
    [2] mistet låsing til medieklokken to ganger
   *[other] mistet låsing til medieklokken { $number } ganger
}
diag-interrupted = { $count ->
    [0] ikke avbrutt
    [one] avbrutt én gang
    [2] avbrutt to ganger
   *[other] avbrutt { $number } ganger
}
diag-out-of-sequence = { $count ->
    [one] { $number } ramme i feil rekkefølge
   *[other] { $number } rammer i feil rekkefølge
}
diag-media-resets = { $count ->
    [one] { $number } medienullstilling
   *[other] { $number } medienullstillinger
}
diag-timestamps-uncertain = { $count ->
    [0] tidsstempler ikke usikre
    [one] tidsstempler usikre én gang
    [2] tidsstempler usikre to ganger
   *[other] tidsstempler usikre { $number } ganger
}
diag-no-timestamp = { $count ->
    [one] { $number } ramme uten tidsstempel
   *[other] { $number } rammer uten tidsstempel
}
diag-unsupported-format = { $count ->
    [one] { $number } ramme i et format som ikke støttes
   *[other] { $number } rammer i et format som ikke støttes
}
diag-late = { $count ->
    [one] { $number } ramme for sent
   *[other] { $number } rammer for sent
}
diag-early = { $count ->
    [one] { $number } ramme for tidlig
   *[other] { $number } rammer for tidlig
}
diag-started = { $count ->
    [0] ikke startet
    [one] startet én gang
    [2] startet to ganger
   *[other] startet { $number } ganger
}
diag-stopped = { $count ->
    [0] ikke stoppet
    [one] stoppet én gang
    [2] stoppet to ganger
   *[other] stoppet { $number } ganger
}
diag-reservation-failed = talkerens reservasjon mislyktes: { $reason }
diag-latency = { $microseconds } µs akkumulert latens

## AVB Lite

lite-active = Aktiv
lite-active-untagged = Aktiv, utagget
lite-active-vlan = Aktiv, VLAN { $vlan }
lite-capable = Støttet
lite-mode = Modus
lite-mode-capable = AVB, AVB Lite støttet
lite-because = Årsak
lite-fallback-none = ingen oppgitt
lite-fallback-endpoint = et annet endepunkts erklæring kom gjennom, så det er ingen AVB-switch imellom
lite-fallback-unanswered = ni peer delay-forespørsler ble ikke besvart
lite-fallback-responders = to eller flere svarte på én og samme peer delay-forespørsel, så switchen er ikke AVB-kompatibel
lite-fallback-configured = operatøren eller en controller har stilt det inn
lite-fallback-other = en årsak profilen ikke nevner
lite-other-profile = En annen profil
lite-ptp-domain = { $profile }, domene { $domain }
lite-offset = Offset
lite-offset-from = { $offset } fra { $grandmaster }
lite-media-vlan = Medie-VLAN
lite-untagged = Utagget
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Opptil { $count } listener per stream, deretter multicast
   *[other] Opptil { $count } listenere per stream, deretter multicast
}
lite-link = Link
lite-bandwidth = Båndbredde
lite-egress-of = { $used } av { $link }, { $share }
lite-egress-of-assumed = { $used } av { $link }, { $share }, gigabit-link antatt
lite-egress-reported = Slik entiteten teller sine godkjente streamer.
lite-egress-worked-out = Ut fra formatene på de tilkoblede stream-utgangene.
lite-alarm-offset = PTP-offset { $offset }, over de 50 µs som AVB Lite tillater
lite-alarm-egress = Egress på { $share } av linken, over de { $limit } som streamer kan ta

## AVB Wireless

wireless-station = Stasjon
wireless-access-point = Tilgangspunkt
wireless-role = Rolle
wireless-mode = Modus
wireless-time = Tid
wireless-mode-a-ftm = Mode A, 802.1AS over FTM
wireless-mode-a-tm = Mode A, 802.1AS over TM
wireless-mode-b = Mode B, fra beacon-rammer
wireless-no-time = Ingen tid
wireless-other-mode = En modus profilen ikke nevner
wireless-locked = Låst
wireless-holdover = I holdover
wireless-not-locked = Ikke låst
wireless-row-locked = Stasjon, låst
wireless-row-holdover = Stasjon, i holdover
wireless-row-not-locked = Stasjon, ikke låst
wireless-row-access-point = { $count ->
    [one] Tilgangspunkt, { $count } stasjon
   *[other] Tilgangspunkt, { $count } stasjoner
}
wireless-link = Link
wireless-channel = kanal { $channel }
wireless-not-known = Ukjent
wireless-signal = Signal
wireless-rate = Sendehastighet
wireless-ftm-valid = { $share } gyldige
wireless-rtt = tur-retur { $rtt }
wireless-bursts = { $count ->
    [one] burster på { $count } ramme
   *[other] burster på { $count } rammer
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = tilgangspunktet tildeler FTM-burster med et annet antall rammer enn tre eller to
wireless-reason-measurement = verken FTM eller TM med tilgangspunktet
wireless-reason-signaling = ingen gPTP-capable Signaling fra tilgangspunktet
wireless-reason-other = en årsak profilen ikke nevner
wireless-servo = Servofeil
wireless-stations = Stasjoner
wireless-station-count = { $count ->
    [one] { $count } stasjon
   *[other] { $count } stasjoner
}
wireless-no-ftm = uten FTM
wireless-unserved = Ikke-betjente listenere
wireless-frame-budget = Rammebudsjett
wireless-frame-use = { $used } av { $budget } rammer per sekund
wireless-stream-frames = Stream-rammer
wireless-frames-of = { $readdressed } til stasjoner, { $unmapped } uten listener, { $dropped } forkastet, { $restored } fra stasjoner
wireless-class-a-allowed = Tillatt
wireless-class-a-not-allowed = Ikke tillatt
wireless-alarm-not-locked = Wi-Fi-tid ikke låst til tilgangspunktet
wireless-alarm-holdover = Wi-Fi-tid i holdover, mistet låsing til tilgangspunktet
wireless-alarm-unserved = { $count ->
    [one] { $count } listener på Wi-Fi-porten betjenes ikke, over unicast-grensen
   *[other] { $count } listenere på Wi-Fi-porten betjenes ikke, over unicast-grensen
}

## Log

log-all = Alle
log-warnings = Advarsler
log-pause = Pause
log-resume = Fortsett
log-clear = Tøm
log-empty = Alle ATDECC-rammer som triib sender og mottar, vises her, nyeste først.
log-none-match = Ingen lagret ramme samsvarer med filteret.
log-frames = { $count ->
    [one] { $count } ramme
   *[other] { $count } rammer
}
log-shown-of = { $shown } av { $all } rammer
log-sent = Sendt
log-heard = Mottatt
log-not-decoded = Ikke dekodet
log-warning-short = Feltet control_data_length angir { $missing } oktetter forbi slutten av rammen.
log-warning-undecodable = Kan ikke dekodes: { $error }.
log-warning-long-acmp = I den lange ACMP-formen, som en Milan-entitet ikke kan sende (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Kanaltilordninger
mapping-inputs = Innganger
mapping-outputs = Utganger
mapping-port = port { $number }
mapping-fixed = fast
mapping-not-read = Ikke lest ennå.
mapping-no-clusters = Ingen klynger.
mapping-no-streams = Ingen lydstreamer.
mapping-none = Ingen tilordninger.
mapping-not-mapped = Ikke tilordnet
mapping-cluster-numbered = Klynge { $index }

## Presets

presets-note = En preset lagrer klokkekilder, samplingsfrekvenser, streamformater, kontroller og tilkoblinger for hver entitet. Når den hentes, endres det som avviker.
presets-none = Ingen presets lagret ennå.
presets-connections = { $count ->
    [one] { $count } tilkobling
   *[other] { $count } tilkoblinger
}
presets-recall = Hent
presets-delete = Slett
presets-no-place = Presets kan ikke lagres noe sted: hjemmemappen er ukjent.
presets-undeletable = Kunne ikke slette { $path }: { $error }.
presets-saved = { $count ->
    [one] Lagret «{ $name }» med { $count } entitet.
   *[other] Lagret «{ $name }» med { $count } entiteter.
}
presets-nothing-differs = Ingenting avviker fra «{ $name }».
presets-recalling = { $count ->
    [one] Henter «{ $name }»: { $count } endring.
   *[other] Henter «{ $name }»: { $count } endringer.
}
presets-missing = { $report } Ikke til stede eller ikke lest: { $missing }.
presets-deleted = Slettet «{ $name }».
presets-host-note = Den lagrer også denne datamaskinens egne talkere og listenere, og starter dem på nytt når den hentes.
presets-host-endpoints = { $count } på denne datamaskinen
presets-starting-host = Starter denne datamaskinens talkere og listenere for «{ $name }»; resten følger når de er tilbake.

## Controls

control-numbered = Kontroll { $index }
control-not-shown = Vises ikke her
control-option = Alternativ { $number }

## Network errors

network-permission = triib trenger tillatelse til å sende og motta rå Ethernet-rammer.
network-needs-npcap = triib trenger Npcap for å sende og motta rå Ethernet-rammer.
network-npcap-administrators = Npcap lar bare administratorer sende og motta rå Ethernet-rammer. Kjør triib som administrator, eller installer Npcap på nytt uten valget for bare administratorer.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own talkers and listeners

host-add-talker = Legg til talker
host-add-listener = Legg til listener
host-show-mine = Vis bare denne datamaskinens egne talkere og listenere
host-show-all = Vis alle entiteter
host-new-talker = Verts-talker { $number }
host-new-listener = Verts-listener { $number }
host-failed = Kunne ikke legge den til på denne datamaskinen: { $reason }
host-needs-clock = Denne datamaskinens egne talkere og listenere trenger et kablet grensesnitt med en PTP-maskinvareklokke
host-no-ptp4l = ptp4l svarer ikke, så streamene til denne datamaskinen kan ikke holde gPTP-tid
host-elsewhere = triib-endpointd kjører på { $interface } for en annen bruker eller som root, så denne datamaskinens talkere og listenere kjører der, ikke her
host-foreign-mrp = Et annet program erklærer MSRP eller MVRP på { $interface } fra denne datamaskinens adresse og kan dermed trekke tilbake det streamene til denne datamaskinen trenger
host-alarm-foreign-mrp = Et annet program på denne datamaskinen erklærer MSRP eller MVRP på { $interface }
host-state = Tilstand
host-streaming = Strømmer
host-waiting = Venter på en listener
host-listening = Lytter
host-bound = Bundet, venter på talkeren
host-unbound = Ikke bundet
host-audio-from = Lyd fra
host-audio-to = Lyd til
host-channels = Kanaler
host-silence = Stillhet
host-tone = Testtone
host-nowhere = Ingen steder
host-default-device = Standardenhet
host-remove = Fjern fra denne datamaskinen
