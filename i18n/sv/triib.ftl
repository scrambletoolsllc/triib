# triib's interface text in Swedish.

## Language

language-name = Svenska

## Common

common-close = Stäng
common-more = Mer
common-keep-toolbar-shown = Visa alltid verktygsfältet
common-auto-hide-toolbar = Dölj verktygsfältet automatiskt

## Settings

settings-title = Inställningar
settings-general = Allmänt
settings-appearance = Utseende
settings-language = Språk
settings-language-system = Systemstandard: { $language }
settings-language-note = Textfält skriver på systemets inmatningsspråk.
settings-appearance-system = System
settings-appearance-light = Ljust
settings-appearance-dark = Mörkt
settings-colors = Färger
settings-system-accent = Använd systemets accentfärg
settings-accent-picked = Färgen nedan ligger till grund för färgerna i triib.
settings-accent-omarchy = Från Omarchy-temat, { $theme }.
settings-accent-desktop = Från skrivbordets accentfärg.
settings-accent-none = Skrivbordet har ingen accentfärg, så färgen nedan används.
settings-motion = Rörelse
settings-animations = Animationer
settings-animations-note = Fjädrande och glidande övergångar när något ändras.
settings-animations-reduced = Skrivbordet ber om minskad rörelse, så triib står still.
settings-outside-control = Extern styrning
settings-allow-outside-control = Tillåt extern styrning
settings-allow-outside-control-note = Agenter som Claude Code kan se och ändra ditt AVB-nätverk via triib --mcp. triib frågar före varje ny agent.
settings-allowed-agents = Tillåtna: { $agents }
settings-forget-agents = Glöm
settings-storage = Lagring
settings-storage-presets = Mapp för presets
settings-storage-entities = Entitetsfil
settings-storage-apply = Verkställ
settings-storage-choose = Välj…
settings-storage-note = Presets som redan finns på en gammal plats ligger kvar där; flytta dem om du vill fortsätta använda dem. Inställningarna för triib sparas i { $file }.
settings-storage-entities-note = En entitetsfil som inte finns än börjar som en kopia av den som används, och triib-endpointd startar om med den.
dialog-choose-presets = Välj mappen för presets
dialog-choose-entities = Välj mappen för entitetsfilen
settings-full-path = Använd en fullständig sökväg, till exempel ~/Documents/triib.
settings-path-is-folder = { $path } är en mapp, inte en fil.
settings-folder-missing = Mappen { $path } finns inte. Skapa den först eller välj en annan.
settings-path-is-file = { $path } är en fil, inte en mapp.
settings-cannot-write = triib kan inte skriva i { $path }: { $error }.
app-file-dialog-failed = Det gick inte att visa fildialogen: { $error }
agent-prompt-title = Vill du låta { $agent } styra triib?
agent-prompt-body = { $agent } ber om att få använda extern styrning av triib: att se entiteterna i ditt nätverk och ändra dem, ansluta streamar och ställa in format, klockor och namn. Du kan stänga av extern styrning i Inställningar.
agent-prompt-allow = Tillåt
agent-prompt-deny = Tillåt inte

common-cancel = Avbryt
common-save = Spara
common-not-set = Inte angivet
common-unnamed = Namnlös
common-none = Ingen
common-mac-address = MAC-adress
common-list-separator = {", "}

## Network interfaces

interface-up = aktivt
interface-link-down = länk nere
interface-wireless = trådlöst
interface-hardware-clock = hårdvaruklocka
interface-hardware-clock-named = hårdvaruklocka { $clock }
interface-virtual = virtuellt

## Toolbar

toolbar-choose-interface = Välj ett gränssnitt
toolbar-interface = Nätverksgränssnitt
toolbar-filter-interfaces = Filtrera gränssnitt
toolbar-hide-virtual = Dölj virtuella gränssnitt
toolbar-hide-wireless = Dölj trådlösa gränssnitt
toolbar-hide-unclocked = Dölj gränssnitt utan hårdvaruklocka
toolbar-connections = Anslutningar
toolbar-network = Nätverk
toolbar-entities = Entiteter
toolbar-rediscover = Be alla entiteter att annonsera sig
toolbar-rescan = Rensa och skanna om alla entiteter
toolbar-search = Sök entiteter och streamar
toolbar-presets = Presets
toolbar-log = Logg
toolbar-inspector = Inspektör
toolbar-settings = Inställningar

## The network's state, in place of a view

state-no-interface = Inget gränssnitt
state-no-interface-note = Välj gränssnittet på AVB-nätverket för att upptäcka entiteter.
state-starting = Startar
state-starting-note = Öppnar { $interface }.
state-listening = Lyssnar
state-listening-note = Entiteter på { $interface } visas här när de annonserar sig.
state-permission-needed = Behörighet krävs
state-npcap-needed = Npcap krävs
state-get-npcap = Hämta Npcap
state-copy-command = Kopiera kommandot
state-cannot-use = Kan inte använda { $interface }
state-try-again = Försök igen

## Entity list

entities-none-yet = Inga entiteter än
entities-none-yet-note = Varje entitet i nätverket, med dess roller, SR-klasser och klocka.

## Inspector

inspector-title = Inspektör
inspector-entity = Entitet
inspector-streams = Streamar
inspector-controls = Reglage
inspector-diagnostics = Diagnostik
inspector-descriptors = Deskriptorer
inspector-select = Välj en entitet för att se dess detaljer.
inspector-offline = { $entity } är offline.
inspector-rename = Byt namn
inspector-name = Namn
inspector-identify = Identifiera
inspector-model-not-read = Entitetsmodellen är inte läst.
inspector-no-streams = Inga streamar.
inspector-no-controls = Inga reglage att visa.
inspector-no-diagnostics = Inga gränssnitt eller räknare rapporterade.
inspector-reading = Läser deskriptorer, { $count } hittills.
inspector-read-failed = Kunde inte läsa entitetsmodellen: { $reason }.

entity-section = Entitet
entity-name = Namn
entity-triib-controller = triib-controller
entity-triib-on = triib på { $host }
entity-triib-here = triib på den här datorn
entity-group = Grupp
entity-product = Produkt
entity-firmware = Firmware
entity-serial-number = Serienummer
entity-configuration = Konfiguration
entity-configuration-of = { $name } ({ $number } av { $count })
entity-milan = Milan
entity-media-clock = Mediaklocka
entity-clock-domain = Klockdomän
entity-sampling-rate = Samplingsfrekvens
clock-source-numbered = Källa { $index }
rate-pull = pull { $pull }

stream-format = Format
stream-id = Stream-ID
stream-destination = Destination
stream-max-transit = Max. transittid

avb-interfaces = AVB-gränssnitt
avb-interface = Gränssnitt
avb-interface-clock-identity = Klockidentitet
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domän { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Körs
avb-interface-none-reported = Inga rapporterade
avb-interface-path = Väg
avb-interface-own-grandmaster = Sin egen grandmaster
avb-interface-hops = { $count ->
    [one] { $count } hopp från grandmastern
   *[other] { $count } hopp från grandmastern
}
avb-interface-link-up = Länk uppe
avb-interface-link-down = Länk nere
avb-interface-grandmaster-changes = Grandmasterbyten
avb-interface-frames-sent = Skickade ramar
avb-interface-frames-received = Mottagna ramar
avb-interface-crc-errors = CRC-fel

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } deskriptortyp
   *[other] { $count } deskriptortyper
}
tree-clock = Klocka
tree-clock-source-from = { $kind }, från { $location } { $index }
tree-clock-domain-using = Använder { $source }
tree-clusters = { $count ->
    [one] { $count } kluster
   *[other] { $count } kluster
}
tree-maps = { $count ->
    [one] { $count } tilldelning
   *[other] { $count } tilldelningar
}

advert-not-advertised = Inte annonserad
advert-identity = Identitet
advert-entity-id = Entitets-ID
advert-entity-model = Entitetsmodell
advert-roles = Roller
advert-talker = Talker
advert-listener = Listener
advert-clock = Klocka
advert-btc = BTC
advert-gptp-domain = gPTP-domän
advert-sr-classes = SR-klasser
advert-indexes = Index i entitetsmodellen
advert-identify-control = Identifiering
advert-avb-interface = AVB-gränssnitt
advert-advertising = Annonsering
advert-valid-time = Giltighetstid
advert-available-index = Tillgänglighetsindex
advert-association = Association
advert-capabilities = Egenskaper

## Status bar

status-entities = { $count ->
    [one] { $count } entitet
   *[other] { $count } entiteter
}
status-not-discovering = Upptäcker inte
status-discovering = Upptäcker
status-discovering-as = Upptäcker som { $controller }
status-stopped = Stoppad av ett fel
status-alarm = Larm
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } och { $count } till

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = controller
role-none = inga roller
classes-a-and-b = A och B
clock-no-gptp = Ingen gPTP

read-not-read = Inte läst
read-reading = Läser, { $count } hittills
read-ready-unreadable = Klar, { $count } gick inte att läsa
read-ready-cached = Klar, från cache
read-ready = Klar
read-failed = Misslyckades: { $reason }

milan-no = Nej
milan-before-1-3 = före 1.3
milan-certified = { $version }, certifierad { $certification }
milan-not-certified = { $version }, inte certifierad

outcome-status = status { $status }
outcome-no-response = inget svar
outcome-not-possible = inte möjligt
outcome-connect = Kunde inte ansluta { $talker } till { $listener }: { $reason }.
outcome-disconnect = Kunde inte koppla från { $listener }: { $reason }.
outcome-identify = Kunde inte identifiera { $entity }: { $reason }.
outcome-rename = Kunde inte byta namn på { $what } till ”{ $name }”: { $reason }.
outcome-rename-group = Kunde inte byta namn på gruppen för { $entity } till ”{ $name }”: { $reason }.
outcome-format-streaming = Kunde inte ändra formatet för { $stream }: den streamar. Koppla från den först.
outcome-format = Kunde inte ändra formatet för { $stream }: { $reason }.
outcome-max-transit-time = Kunde inte ändra max. transittid för { $stream }: { $reason }.
outcome-max-transit-time-streaming = Kunde inte ändra max. transittid för { $stream }: den streamar. Koppla från den först.
outcome-sampling-rate = Kunde inte ändra samplingsfrekvensen för { $entity }: { $reason }.
outcome-clock-source = Kunde inte ändra klockkällan för { $entity }: { $reason }.
outcome-map = Kunde inte tilldela kanalen på { $entity }: { $reason }.
outcome-unmap = Kunde inte ta bort kanaltilldelningen på { $entity }: { $reason }.
outcome-control = Kunde inte ställa in ”{ $control }” på { $entity }: { $reason }.
outcome-control-numbered = Kunde inte ställa in reglage { $index } på { $entity }: { $reason }.

stream-not-connected = Inte ansluten
stream-from = Från { $stream }
stream-from-receiving = Från { $stream }, tar emot
stream-from-waiting = Från { $stream }, väntar på talkern
stream-from-failed = Från { $stream }, talkerns reservering misslyckades: { $reason }
stream-sending-to = Skickar till { $destination }

failure-no-response = inget svar
failure-refused = nekad med { $status }
failure-malformed = svaret gick inte att avkoda
failure-on-this-computer = körs på den här datorn; läs den från en annan dator

msrp-failure-1 = otillräcklig bandbredd
msrp-failure-2 = otillräckliga switchresurser
msrp-failure-3 = otillräcklig bandbredd för trafikklassen
msrp-failure-4 = stream-ID används av en annan talker
msrp-failure-5 = måladressen används redan
msrp-failure-6 = undanträngd av en stream med högre rang
msrp-failure-7 = rapporterad latens har ändrats
msrp-failure-8 = egress-porten är inte AVB-kompatibel
msrp-failure-9 = använd en annan måladress
msrp-failure-10 = slut på MSRP-resurser
msrp-failure-11 = slut på MMRP-resurser
msrp-failure-12 = kan inte lagra måladressen
msrp-failure-13 = prioriteten är inte en SR-klassprioritet
msrp-failure-14 = ramarna är för stora för mediet
msrp-failure-15 = portens fan-in-gräns nådd
msrp-failure-16 = första värdet ändrat för en registrerad stream
msrp-failure-17 = VLAN blockerat på egress-porten
msrp-failure-18 = VLAN-taggning inaktiverad på egress-porten
msrp-failure-19 = SR-klassprioriteten stämmer inte
msrp-failure-unknown = okänd orsak
msrp-failure-at = { $reason }, vid switchen { $bridge }

## Entity list columns

column-vendor = Tillverkare
column-model = Modell
column-state = Status
column-entity-model-id = Entitetsmodell-ID
column-talker-streams = Talker-streamar
column-listener-streams = Listener-streamar
column-avb-lite = AVB Lite
column-egress = Egress
column-wireless = Trådlöst

## Settings file

settings-no-place = Inställningarna kan inte sparas någonstans: hemmappen är okänd.
settings-unusable = Kunde inte använda { $path }: { $error }.
settings-unsaved = Kunde inte spara { $path }: { $error }.

column-remove = Ta bort kolumn
column-move-left = Flytta åt vänster
column-move-right = Flytta åt höger
column-add = Lägg till en kolumn
common-percent = { $value }{" "}%

## Network view

netmap-empty = Inget nätverk att visa än
netmap-empty-note = Entiteter visas här när de har lästs och har angett var de sitter i gPTP-trädet.
netmap-focus-clock-path = Klockväg för { $name }
netmap-focus-streams = Streamar från { $name }
netmap-showing = Visar { $what }
netmap-devices = { $count ->
    [one] { $count } enhet
   *[other] { $count } enheter
}
netmap-bridges = { $count ->
    [one] { $count } switch
   *[other] { $count } switchar
}
netmap-show-map = Visa kartan
netmap-show-details = Visa detaljerna
stream-numbered = Stream { $index }
netmap-bridge = Switch
netmap-access-point = Åtkomstpunkt
netmap-device = Enhet
netmap-this-computer = Den här datorn
netmap-connected = Ansluten
netmap-advertised = Annonserad, ingen listener redo
netmap-advertised-off-tree = Annonserad, ingen listener redo ({ $listener } finns inte i gPTP-trädet)
netmap-failed-at = Reserveringen misslyckades vid { $bridge }: { $reason }
netmap-failed = Reserveringen misslyckades: { $reason }
netmap-no-bridge-on = Ingen switch hittad på { $interface }
netmap-cannot-listen-on = Kan inte lyssna efter gPTP på { $interface }
netmap-on-this-computer = På den här datorn
netmap-path-not-reported = Vägen inte rapporterad
netmap-gptp-not-reported = gPTP inte rapporterat
netmap-off-tree = Inte i gPTP-trädet
netmap-off-ptp = Inte i PTP-trädet
netmap-not-lite = Inte i AVB Lite
netmap-lite-not-reported = AVB Lite inte rapporterat
netmap-synced = Synkroniserad
netmap-not-synced = Inte synkroniserad
netmap-triib-on = triib på { $interface }
netmap-through-count = { $count } igenom
netmap-out = { $count } ut
netmap-in = { $count } in
netmap-failed-count = { $count ->
    [one] { $count } misslyckad
   *[other] { $count } misslyckade
}
netmap-advertised-only = Endast annonserad
netmap-failed-state = Misslyckad
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Inte i gPTP-trädet: den är sin egen grandmaster
netmap-apart-no-path = Vägen har inte rapporterats; följer grandmaster { $grandmaster }
netmap-apart-unreported = Har inte rapporterat sitt gPTP-tillstånd
netmap-apart-no-neighbor = Ingen switch hittad på den här datorns gränssnitt
netmap-apart-cannot-listen = Den här datorn kan inte lyssna efter gPTP på sitt gränssnitt
netmap-apart-on-this-computer = Den körs på den här datorn; läs den från en annan dator för att se dess gPTP-läge
netmap-apart-not-lite = Den kör inte AVB Lite, så den följer inte grandmastern
netmap-apart-lite-unreported = Rapporterar inget om AVB Lite, så det är okänt vad den följer
netmap-clock-tree = Klockträd
netmap-no-grandmaster = Ingen grandmaster hittad
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Behöver åtgärdas
netmap-nodes-below = Noder under
netmap-bridges-below = Switchar under
netmap-clock-path = Klockväg
netmap-hops = Hopp från grandmastern
netmap-link-delay = Länkfördröjning
netmap-bridge-port = Switchport
netmap-link-drops = Länkavbrott
netmap-synced-to-grandmaster = Synkroniserad med grandmastern
netmap-host-no-gptp = Inte synkroniserad: den här datorn kör inte gPTP
netmap-link-no-gptp = Inte synkroniserad: gPTP körs inte på dess länk
netmap-ptp-offset-high = Inte synkroniserad: { $offset } från grandmastern, över de 50 µs som AVB Lite tillåter
netmap-ptp-no-offset = Inte synkroniserad: den har inte mätt någon offset från grandmastern
netmap-audio = Ljud
netmap-media-clock-streams = Mediaklockstreamar
netmap-audio-streams = Ljudstreamar
netmap-bound = { $count ->
    [one] { $count } bunden
   *[other] { $count } bundna
}
netmap-flowing = Flödar
netmap-advertised-state = Annonserad
netmap-media-clock-stream = Mediaklockstream
netmap-audio-stream = Ljudstream
netmap-reaches = Når till
netmap-passing-count = { $count ->
    [one] { $count } stream passerar
   *[other] { $count } streamar passerar
}
netmap-through = Igenom
netmap-passing-through = Passerar
netmap-sending = Skickar
netmap-receiving = Tar emot
netmap-problems = Problem
netmap-help-back = Klicka på bakgrunden för att gå tillbaka till översikten.
netmap-help-stream = Klicka på en stream för att inspektera den, eller på bakgrunden för att gå tillbaka till översikten.
netmap-help-ptp = I AVB Lite går klockan ände till ände från grandmastern till varje enhet, via switchar som inte deltar, så ingen av dem visas. En enhet är synkroniserad så länge den följer grandmastern inom 50 µs. Klicka på en enhet eller dess ledning för att inspektera dess klocka; klicka på bakgrunden för att avmarkera.
netmap-help-clock = Klockan går från grandmastern via varje switch till varje nod i trädet. En streckad grå linje är en länk där gPTP inte körs. Klicka på en enhet eller dess ledning för att inspektera dess klockväg; klicka på bakgrunden för att avmarkera.
netmap-help-media-clock = Endast mediaklockstreamar (CRF), ritade på samma sätt som ljud: en ledning per stream, färgad efter talker. Klicka på en ledning för att inspektera dess stream, eller på en enhet för att se dess streamar; klicka på bakgrunden för att avmarkera.
netmap-help-audio = Varje stream har sin egen ledning, som går in i och ut ur varje switch den passerar. Färgen följer talkern: varje talker har en egen färgton, och dess streamar är nyanser av den. Rörliga prickar betyder att ljud flödar; en stillastående röd linje är en misslyckad reservering och en stillastående grå linje är annonserad utan att någon listener är redo; båda slutar där reserveringen slutar. Enheter i mittenkolumnen ansluter direkt till grandmasterns switch. Klicka på en ledning för att inspektera dess stream, eller på en enhet för att se dess streamar; klicka på bakgrunden för att avmarkera.

## Connections

matrix-nothing-shown = Inga streamar att visa
matrix-nothing-shown-note = Ändra sökningen eller filtren för att se fler streamar.
matrix-empty = Inga streamar att ansluta
matrix-empty-note = Talker-streamar och listener-streamar möts här när entiteter med streamar har lästs.
matrix-all-streams = Alla streamar
matrix-connectable-only = Dölj det som inte kan anslutas
matrix-none-hidden = Alla visade streamar kan anslutas
matrix-hidden = { $count ->
    [one] { $count } stream dold
   *[other] { $count } streamar dolda
}
matrix-own = En entitets utgångar kan inte anslutas till dess egna ingångar.
matrix-working = Arbetar på det.
matrix-waiting-change = Väntar på den senaste ändringen av den här ingången.
matrix-connected = Ansluten och tar emot. Klicka för att koppla från.
matrix-bound-waiting = Bunden, väntar på talkerns stream. Klicka för att koppla från.
matrix-bound-failed = Bunden, men talkerns reservering misslyckades: { $reason }. Klicka för att koppla från.
matrix-bound-formats-differ = Bunden, men formaten skiljer sig: talkern skickar { $sent }, ingången är inställd på { $set }. Klicka för att koppla från.
matrix-formats-match = Formaten stämmer ({ $format }). Klicka för att ansluta.
matrix-format-must-change = Ingången tar { $sent } men är inställd på { $set }, så den kanske inte spelar förrän formatet ändras. Klicka för att välja ett format.
matrix-format-input = Ändra ingången till
matrix-format-both = Ändra båda till
matrix-format-talker-busy = Talkern strömmar till andra listeners, så dess format behålls.
matrix-format-anyway = Anslut utan att ändra format
matrix-incompatible = Ingången tar inte { $sent }. Den är inställd på { $set }.
matrix-group-none = Inte ansluten. Fäll ut för att ansluta streamar en i taget.
matrix-group-connected = { $count ->
    [one] { $count } ansluten. Fäll ut för att se var och en.
   *[other] { $count } anslutna. Fäll ut för att se var och en.
}
matrix-outputs-expand = { $count ->
    [one] { $count } stream-utgång. Klicka på pilen för att fälla ut, på namnet för att inspektera den.
   *[other] { $count } stream-utgångar. Klicka på pilen för att fälla ut, på namnet för att inspektera den.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } stream-utgång. Klicka på pilen för att fälla ihop, på namnet för att inspektera den.
   *[other] { $count } stream-utgångar. Klicka på pilen för att fälla ihop, på namnet för att inspektera den.
}
matrix-inputs-expand = { $count ->
    [one] { $count } stream-ingång. Klicka på pilen för att fälla ut, på namnet för att inspektera den.
   *[other] { $count } stream-ingångar. Klicka på pilen för att fälla ut, på namnet för att inspektera den.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } stream-ingång. Klicka på pilen för att fälla ihop, på namnet för att inspektera den.
   *[other] { $count } stream-ingångar. Klicka på pilen för att fälla ihop, på namnet för att inspektera den.
}
matrix-stream-inspect = { $detail } Klicka för att inspektera { $entity }.
matrix-point = Peka på en cell
matrix-point-note = för att se dess talker och listener och om deras format passar ihop.
matrix-legend-waiting = Bunden, väntar på streamen
matrix-legend-trouble = Bunden, något är fel
matrix-legend-open = Kan anslutas
matrix-legend-change = Ingångens format måste ändras först
matrix-legend-incompatible = Formaten passar inte ihop
matrix-talker-outputs = Talker-utgångar
matrix-listener-inputs = Listener-ingångar

common-thousands-separator = {" "}

## Diagnostics

diag-since-start = Räknat sedan entiteten startade.
diag-stream-input = Stream-ingång
diag-stream-output = Stream-utgång
diag-locked = { $count ->
    [0] inte låst
    [one] låst en gång
    [2] låst två gånger
   *[other] låst { $number } gånger
}
diag-lost-lock = { $count ->
    [0] låsning inte förlorad
    [one] låsning förlorad en gång
    [2] låsning förlorad två gånger
   *[other] låsning förlorad { $number } gånger
}
diag-frames-in = { $count ->
    [one] { $number } ram in
   *[other] { $number } ramar in
}
diag-frames-out = { $count ->
    [one] { $number } ram ut
   *[other] { $number } ramar ut
}
diag-media-locked = { $count ->
    [0] inte låst till mediaklockan
    [one] låst till mediaklockan en gång
    [2] låst till mediaklockan två gånger
   *[other] låst till mediaklockan { $number } gånger
}
diag-lost-media-lock = { $count ->
    [0] mediaklocklåsning inte förlorad
    [one] mediaklocklåsning förlorad en gång
    [2] mediaklocklåsning förlorad två gånger
   *[other] mediaklocklåsning förlorad { $number } gånger
}
diag-interrupted = { $count ->
    [0] inte avbruten
    [one] avbruten en gång
    [2] avbruten två gånger
   *[other] avbruten { $number } gånger
}
diag-out-of-sequence = { $count ->
    [one] { $number } ram i fel ordning
   *[other] { $number } ramar i fel ordning
}
diag-media-resets = { $count ->
    [one] { $number } mediaåterställning
   *[other] { $number } mediaåterställningar
}
diag-timestamps-uncertain = { $count ->
    [0] tidsstämplar inte osäkra
    [one] tidsstämplar osäkra en gång
    [2] tidsstämplar osäkra två gånger
   *[other] tidsstämplar osäkra { $number } gånger
}
diag-no-timestamp = { $count ->
    [one] { $number } ram utan tidsstämpel
   *[other] { $number } ramar utan tidsstämpel
}
diag-unsupported-format = { $count ->
    [one] { $number } ram i ett format som inte stöds
   *[other] { $number } ramar i ett format som inte stöds
}
diag-late = { $count ->
    [one] { $number } ram för sent
   *[other] { $number } ramar för sent
}
diag-early = { $count ->
    [one] { $number } ram för tidigt
   *[other] { $number } ramar för tidigt
}
diag-started = { $count ->
    [0] inte startad
    [one] startad en gång
    [2] startad två gånger
   *[other] startad { $number } gånger
}
diag-stopped = { $count ->
    [0] inte stoppad
    [one] stoppad en gång
    [2] stoppad två gånger
   *[other] stoppad { $number } gånger
}
diag-reservation-failed = talkerns reservering misslyckades: { $reason }
diag-latency = { $microseconds } µs ackumulerad latens

## AVB Lite

lite-active = Aktiv
lite-active-untagged = Aktiv, otaggad
lite-active-vlan = Aktiv, VLAN { $vlan }
lite-capable = Stöds
lite-mode = Läge
lite-mode-capable = AVB (AVB Lite stöds)
lite-because = Orsak
lite-fallback-none = ingen angiven
lite-fallback-endpoint = en annan slutpunkts deklaration kom igenom, så det finns ingen AVB-switch emellan
lite-fallback-unanswered = nio peer delay-förfrågningar blev obesvarade
lite-fallback-responders = två eller fler svarade på en och samma peer delay-förfrågan, så switchen är inte AVB-kompatibel
lite-fallback-configured = operatören eller en controller ställde in det
lite-fallback-other = en orsak som profilen inte anger
lite-other-profile = En annan profil
lite-ptp-domain = { $profile }, domän { $domain }
lite-offset = Offset
lite-offset-from = { $offset } från { $grandmaster }
lite-media-vlan = Media-VLAN
lite-untagged = Otaggad
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Upp till { $count } listener per stream, sedan multicast
   *[other] Upp till { $count } listeners per stream, sedan multicast
}
lite-link = Länk
lite-bandwidth = Bandbredd
lite-egress-of = { $used } av { $link }, { $share }
lite-egress-of-assumed = { $used } av { $link }, { $share }, gigabitlänk antagen
lite-egress-reported = Så som entiteten räknar sina godkända streamar.
lite-egress-worked-out = Utifrån formaten på dess anslutna stream-utgångar.
lite-alarm-offset = PTP-offset { $offset }, över de 50 µs som AVB Lite tillåter
lite-alarm-egress = Egress på { $share } av länken, över de { $limit } som streamar får ta

## AVB Wireless

wireless-station = Station
wireless-access-point = Åtkomstpunkt
wireless-role = Roll
wireless-mode = Läge
wireless-time = Tid
wireless-mode-a-ftm = Mode A, 802.1AS över FTM
wireless-mode-a-tm = Mode A, 802.1AS över TM
wireless-mode-b = Mode B, från beacon-ramar
wireless-no-time = Ingen tid
wireless-other-mode = Ett läge som profilen inte anger
wireless-locked = Låst
wireless-holdover = I holdover
wireless-not-locked = Inte låst
wireless-row-locked = Station, låst
wireless-row-holdover = Station, i holdover
wireless-row-not-locked = Station, inte låst
wireless-row-access-point = { $count ->
    [one] Åtkomstpunkt, { $count } station
   *[other] Åtkomstpunkt, { $count } stationer
}
wireless-link = Länk
wireless-channel = kanal { $channel }
wireless-not-known = Okänd
wireless-signal = Signal
wireless-rate = Sändhastighet
wireless-ftm-valid = { $share } giltiga
wireless-rtt = tur och retur { $rtt }
wireless-bursts = { $count ->
    [one] burstar om { $count } ram
   *[other] burstar om { $count } ramar
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = åtkomstpunkten beviljar FTM-burstar med ett annat antal ramar än tre eller två
wireless-reason-measurement = varken FTM eller TM med åtkomstpunkten
wireless-reason-signaling = ingen gPTP-capable Signaling från åtkomstpunkten
wireless-reason-other = en orsak som profilen inte anger
wireless-servo = Servofel
wireless-stations = Stationer
wireless-station-count = { $count ->
    [one] { $count } station
   *[other] { $count } stationer
}
wireless-no-ftm = utan FTM
wireless-unserved = Ej betjänade listeners
wireless-frame-budget = Rambudget
wireless-frame-use = { $used } av { $budget } ramar per sekund
wireless-stream-frames = Streamramar
wireless-frames-of = { $readdressed } till stationer, { $unmapped } utan listener, { $dropped } kastade, { $restored } från stationer
wireless-class-a-allowed = Tillåten
wireless-class-a-not-allowed = Inte tillåten
wireless-alarm-not-locked = Wi-Fi-tid inte låst till åtkomstpunkten
wireless-alarm-holdover = Wi-Fi-tid i holdover, låsning till åtkomstpunkten förlorad
wireless-alarm-unserved = { $count ->
    [one] { $count } listener på Wi-Fi-porten betjänas inte, över unicastgränsen
   *[other] { $count } listeners på Wi-Fi-porten betjänas inte, över unicastgränsen
}

## Log

log-all = Alla
log-warnings = Varningar
log-pause = Pausa
log-resume = Återuppta
log-clear = Rensa
log-empty = Varje ATDECC-ram som triib skickar och tar emot visas här, nyaste först.
log-none-match = Ingen sparad ram matchar filtret.
log-frames = { $count ->
    [one] { $count } ram
   *[other] { $count } ramar
}
log-shown-of = { $shown } av { $all } ramar
log-sent = Skickad
log-heard = Mottagen
log-not-decoded = Inte avkodad
log-warning-short = Fältet control_data_length anger { $missing } oktetter bortom ramens slut.
log-warning-undecodable = Går inte att avkoda: { $error }.
log-warning-long-acmp = I den långa ACMP-formen, som en Milan-entitet inte får skicka (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Kanaltilldelningar
mapping-inputs = Ingångar
mapping-outputs = Utgångar
mapping-port = port { $number }
mapping-fixed = fast
mapping-not-read = Inte läst än.
mapping-no-clusters = Inga kluster.
mapping-no-streams = Inga ljudstreamar.
mapping-none = Inga tilldelningar.
mapping-not-mapped = Inte tilldelad
mapping-cluster-numbered = Kluster { $index }

## Presets

presets-note = En preset sparar varje entitets klockkällor, samplingsfrekvenser, streamformat, reglage och anslutningar. När den hämtas ändras det som skiljer sig.
presets-none = Inga presets sparade än.
presets-connections = { $count ->
    [one] { $count } anslutning
   *[other] { $count } anslutningar
}
presets-recall = Hämta
presets-delete = Ta bort
presets-no-place = Presets kan inte sparas någonstans: hemmappen är okänd.
presets-undeletable = Kunde inte ta bort { $path }: { $error }.
presets-saved = { $count ->
    [one] Sparade ”{ $name }” med { $count } entitet.
   *[other] Sparade ”{ $name }” med { $count } entiteter.
}
presets-nothing-differs = Inget skiljer sig från ”{ $name }”.
presets-recalling = { $count ->
    [one] Hämtar ”{ $name }”: { $count } ändring.
   *[other] Hämtar ”{ $name }”: { $count } ändringar.
}
presets-missing = { $report } Inte här eller inte lästa: { $missing }.
presets-deleted = Tog bort ”{ $name }”.
presets-host-note = Den sparar också den här datorns egna entiteter och startar dem igen när den hämtas.
presets-host-endpoints = { $count } på den här datorn
presets-starting-host = Startar den här datorns entiteter för ”{ $name }”; resten följer när de är tillbaka.

## Controls

control-numbered = Reglage { $index }
control-not-shown = Visas inte här
control-option = Alternativ { $number }

## Network errors

network-permission = triib behöver behörighet att skicka och ta emot råa Ethernet-ramar.
network-needs-npcap = triib behöver Npcap för att skicka och ta emot råa Ethernet-ramar.
network-npcap-administrators = Npcap låter bara administratörer skicka och ta emot råa Ethernet-ramar. Kör triib som administratör, eller installera om Npcap utan alternativet för endast administratörer.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own entities

host-add-entity = Lägg till entitet
host-show-mine = Visa bara den här datorns egna entiteter
host-show-all = Visa alla entiteter
host-new-entity = Värdentitet { $number }
host-failed = Kunde inte lägga till den på den här datorn: { $reason }
host-needs-clock = Den här datorns egna entiteter behöver ett trådbundet gränssnitt med en PTP-hårdvaruklocka
host-no-ptp4l = ptp4l svarar inte, så den här datorns streamar kan inte hålla gPTP-tid
host-elsewhere = triib-endpointd körs på { $interface } för en annan användare eller som root, så den här datorns entiteter körs där, inte här
host-foreign-mrp = Ett annat program deklarerar MSRP eller MVRP på { $interface } från den här datorns adress och kan därmed dra tillbaka det som den här datorns streamar behöver
host-alarm-foreign-mrp = Ett annat program på den här datorn deklarerar MSRP eller MVRP på { $interface }
host-state = Läge
host-streaming = Strömmar
host-waiting = Väntar på en listener
host-listening = Lyssnar
host-bound = Bunden, väntar på talkern
host-unbound = Inte bunden
host-audio-from = Ljud från
host-audio-to = Ljud till
host-channels = Kanaler
host-virtual-device = Virtuell enhet
host-add-stream = Lägg till stream
host-add-audio-input = Ljudingång
host-add-audio-output = Ljudutgång
host-add-clock-input = Mediaklockingång
host-add-clock-output = Mediaklockutgång
host-remove-stream = Ta bort stream
host-silence = Tystnad
host-tone = Testton
host-nowhere = Ingenstans
host-default-device = Standardenhet
host-remove = Ta bort från den här datorn
