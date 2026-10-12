# triib's interface text in Romanian. The source is i18n/en/triib.ftl; term
# choices are in docs/glossary/ro.md.

## Language

language-name = Română

## Common

common-close = Închide
common-more = Mai multe
common-keep-toolbar-shown = Păstrează bara de instrumente vizibilă
common-auto-hide-toolbar = Ascunde automat bara de instrumente

## Settings

settings-title = Setări
settings-general = General
settings-appearance = Aspect
settings-language = Limbă
settings-language-system = Limba sistemului: { $language }
settings-language-note = Câmpurile de text folosesc limba de introducere a sistemului.
settings-appearance-system = Sistem
settings-appearance-light = Luminos
settings-appearance-dark = Întunecat
settings-colors = Culori
settings-system-accent = Folosește culoarea de accent a sistemului
settings-accent-picked = Culoarea de mai jos stă la baza culorilor triib.
settings-accent-omarchy = Din tema Omarchy, { $theme }.
settings-accent-desktop = Din culoarea de accent a desktopului.
settings-accent-none = Desktopul nu are o culoare de accent, așa că se folosește culoarea de mai jos.
settings-motion = Mișcare
settings-animations = Animații
settings-animations-note = Efecte elastice și glisări la fiecare schimbare.
settings-animations-reduced = Desktopul cere mișcare redusă, așa că triib rămâne nemișcat.
settings-outside-control = Control extern
settings-allow-outside-control = Permite controlul extern
settings-allow-outside-control-note = Agenții precum Claude Code pot vedea și modifica rețeaua ta AVB, prin triib --mcp. triib întreabă înaintea fiecărui agent nou.
settings-allowed-agents = Permiși: { $agents }
settings-forget-agents = Uită
agent-prompt-title = Permiți ca { $agent } să controleze triib?
agent-prompt-body = { $agent } cere să folosească controlul extern al triib: să vadă entitățile din rețeaua ta și să le modifice, conectând fluxuri și setând formate, ceasuri și nume. Poți dezactiva controlul extern în Setări.
agent-prompt-allow = Permite
agent-prompt-deny = Nu permite

common-cancel = Anulează
common-save = Salvează
common-not-set = Nesetat
common-unnamed = Fără nume
common-none = Niciunul
common-mac-address = Adresă MAC
common-list-separator = {", "}

## Network interfaces

interface-up = activă
interface-link-down = legătură inactivă
interface-wireless = wireless
interface-hardware-clock = ceas hardware
interface-hardware-clock-named = ceas hardware { $clock }
interface-virtual = virtuală

## Toolbar

toolbar-choose-interface = Alege o interfață
toolbar-interface = Interfață de rețea
toolbar-show-virtual = Afișează interfețele virtuale
toolbar-hide-virtual = Ascunde interfețele virtuale
toolbar-connections = Conexiuni
toolbar-network = Rețea
toolbar-entities = Entități
toolbar-rediscover = Cere fiecărei entități să se anunțe
toolbar-rescan = Șterge și scanează din nou toate entitățile
toolbar-search = Caută entități și fluxuri
toolbar-presets = Preseturi
toolbar-log = Jurnal
toolbar-inspector = Inspector
toolbar-settings = Setări

## The network's state, in place of a view

state-no-interface = Nicio interfață
state-no-interface-note = Alege interfața din rețeaua AVB pentru a descoperi entitățile.
state-starting = Pornire
state-starting-note = Se deschide { $interface }.
state-listening = În ascultare
state-listening-note = Entitățile de pe { $interface } apar aici pe măsură ce se anunță.
state-permission-needed = Permisiune necesară
state-npcap-needed = Npcap necesar
state-get-npcap = Obține Npcap
state-copy-command = Copiază comanda
state-cannot-use = Nu se poate folosi { $interface }
state-try-again = Încearcă din nou

## Entity list

entities-none-yet = Încă nicio entitate
entities-none-yet-note = Fiecare entitate din rețea, cu rolurile, clasele SR și ceasul ei.

## Inspector

inspector-title = Inspector
inspector-entity = Entitate
inspector-streams = Fluxuri
inspector-controls = Controale
inspector-diagnostics = Diagnosticare
inspector-descriptors = Descriptori
inspector-select = Selectează o entitate pentru a-i vedea detaliile.
inspector-offline = { $entity } este offline.
inspector-rename = Redenumește
inspector-name = Nume
inspector-identify = Identifică
inspector-model-not-read = Modelul entității nu a fost citit.
inspector-no-streams = Niciun flux.
inspector-no-controls = Niciun control de afișat.
inspector-no-diagnostics = Nu s-au raportat interfețe sau contoare.
inspector-reading = Se citesc descriptorii, { $count } până acum.
inspector-read-failed = Modelul entității nu a putut fi citit: { $reason }.

entity-section = Entitate
entity-name = Nume
entity-group = Grup
entity-product = Produs
entity-firmware = Firmware
entity-serial-number = Număr de serie
entity-configuration = Configurație
entity-configuration-of = { $name } ({ $number } din { $count })
entity-milan = Milan
entity-media-clock = Ceas media
entity-clock-domain = Domeniu de ceas
entity-sampling-rate = Frecvență de eșantionare
clock-source-numbered = Sursa { $index }
rate-pull = pull { $pull }

stream-inputs = Intrări de flux
stream-outputs = Ieșiri de flux
stream-max-transit-time = Timp maxim de tranzit { $time }

avb-interfaces = Interfețe AVB
avb-interface = Interfață
avb-interface-clock-identity = Identitatea ceasului
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domeniul { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = În funcțiune
avb-interface-none-reported = Nimic raportat
avb-interface-path = Cale
avb-interface-own-grandmaster = Este propriul grandmaster
avb-interface-hops = { $count ->
    [one] La { $count } salt de grandmaster
    [few] La { $count } salturi de grandmaster
   *[other] La { $count } de salturi de grandmaster
}
avb-interface-link-up = Legătură activă
avb-interface-link-down = Legătură inactivă
avb-interface-grandmaster-changes = Schimbări de grandmaster
avb-interface-frames-sent = Cadre trimise
avb-interface-frames-received = Cadre primite
avb-interface-crc-errors = Erori CRC

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } tip de descriptor
    [few] { $count } tipuri de descriptori
   *[other] { $count } de tipuri de descriptori
}
tree-clock = Ceas
tree-clock-source-from = { $kind }, de la { $location } { $index }
tree-clock-domain-using = Folosește { $source }
tree-clusters = { $count ->
    [one] { $count } cluster
    [few] { $count } clustere
   *[other] { $count } de clustere
}
tree-maps = { $count ->
    [one] { $count } mapare
    [few] { $count } mapări
   *[other] { $count } de mapări
}

advert-not-advertised = Neanunțat
advert-identity = Identitate
advert-entity-id = ID entitate
advert-entity-model = Model de entitate
advert-roles = Roluri
advert-talker = Talker
advert-listener = Listener
advert-clock = Ceas
advert-btc = BTC
advert-gptp-domain = Domeniu gPTP
advert-sr-classes = Clase SR
advert-indexes = Indici în modelul de entitate
advert-identify-control = Control de identificare
advert-avb-interface = Interfață AVB
advert-advertising = Anunțare
advert-valid-time = Durată de valabilitate
advert-available-index = Index de disponibilitate
advert-association = Asociere
advert-capabilities = Capabilități

## Status bar

status-entities = { $count ->
    [one] { $count } entitate
    [few] { $count } entități
   *[other] { $count } de entități
}
status-not-discovering = Descoperire inactivă
status-discovering = Descoperire în curs
status-discovering-as = Descoperire ca { $controller }
status-stopped = Oprit din cauza unei erori
status-alarm = Alarmă
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } și încă { $count }

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = controler
role-none = niciun rol
classes-a-and-b = A și B
clock-no-gptp = Fără gPTP

read-not-read = Necitit
read-reading = Se citește, { $count } până acum
read-ready-unreadable = Gata, ilizibili: { $count }
read-ready-cached = Gata, din cache
read-ready = Gata
read-failed = Eșuat: { $reason }

milan-no = Nu
milan-before-1-3 = înainte de 1.3
milan-certified = { $version }, certificat { $certification }
milan-not-certified = { $version }, necertificat

outcome-status = stare { $status }
outcome-no-response = niciun răspuns
outcome-not-possible = imposibil
outcome-connect = Nu s-a putut conecta { $talker } la { $listener }: { $reason }.
outcome-disconnect = Nu s-a putut deconecta { $listener }: { $reason }.
outcome-identify = Nu s-a putut identifica { $entity }: { $reason }.
outcome-rename = Nu s-a putut redenumi { $what } în „{ $name }”: { $reason }.
outcome-rename-group = Nu s-a putut redenumi grupul entității { $entity } în „{ $name }”: { $reason }.
outcome-format-streaming = Nu s-a putut schimba formatul pentru { $stream }: fluxul este în transmisie. Deconectează-l mai întâi.
outcome-format = Nu s-a putut schimba formatul pentru { $stream }: { $reason }.
outcome-sampling-rate = Nu s-a putut schimba frecvența de eșantionare pentru { $entity }: { $reason }.
outcome-clock-source = Nu s-a putut schimba sursa de ceas pentru { $entity }: { $reason }.
outcome-map = Nu s-a putut mapa canalul pe { $entity }: { $reason }.
outcome-unmap = Nu s-a putut elimina maparea canalului pe { $entity }: { $reason }.
outcome-control = Nu s-a putut seta „{ $control }” pe { $entity }: { $reason }.
outcome-control-numbered = Nu s-a putut seta controlul { $index } pe { $entity }: { $reason }.

stream-not-connected = Neconectat
stream-from = De la { $stream }
stream-from-receiving = De la { $stream }, recepție în curs
stream-from-waiting = De la { $stream }, se așteaptă talkerul
stream-from-failed = De la { $stream }, rezervarea talkerului a eșuat: { $reason }
stream-sending-to = Trimitere către { $destination }

failure-no-response = entitatea nu a răspuns
failure-refused = entitatea a refuzat cu { $status }
failure-malformed = răspunsul ei nu a putut fi decodat
failure-on-this-computer = entitatea rulează pe acest computer; citește-o de pe altul

msrp-failure-1 = lățime de bandă insuficientă
msrp-failure-2 = resurse insuficiente pe switch
msrp-failure-3 = lățime de bandă insuficientă pentru clasa de trafic
msrp-failure-4 = ID de flux folosit de alt talker
msrp-failure-5 = adresă de destinație deja folosită
msrp-failure-6 = înlocuit de un flux de rang superior
msrp-failure-7 = latența raportată s-a schimbat
msrp-failure-8 = portul de ieșire nu este compatibil AVB
msrp-failure-9 = folosește altă adresă de destinație
msrp-failure-10 = resurse MSRP epuizate
msrp-failure-11 = resurse MMRP epuizate
msrp-failure-12 = adresa de destinație nu poate fi stocată
msrp-failure-13 = prioritatea nu este o prioritate de clasă SR
msrp-failure-14 = cadre prea mari pentru mediu
msrp-failure-15 = limita de fan-in a portului a fost atinsă
msrp-failure-16 = prima valoare s-a schimbat pentru un flux înregistrat
msrp-failure-17 = VLAN blocat pe portul de ieșire
msrp-failure-18 = etichetarea VLAN este dezactivată pe portul de ieșire
msrp-failure-19 = nepotrivire de prioritate a clasei SR
msrp-failure-unknown = motiv necunoscut
msrp-failure-at = { $reason }, la switch-ul { $bridge }

## Entity list columns

column-vendor = Producător
column-model = Model
column-state = Stare
column-entity-model-id = ID model de entitate
column-talker-streams = Fluxuri talker
column-listener-streams = Fluxuri listener
column-avb-lite = AVB Lite
column-egress = Trafic de ieșire
column-wireless = Wireless

## Settings file

settings-no-place = Setările nu pot fi păstrate nicăieri: dosarul personal nu este cunoscut.
settings-unusable = Nu s-a putut folosi { $path }: { $error }.
settings-unsaved = Nu s-a putut salva { $path }: { $error }.

column-remove = Elimină coloana
column-move-left = Mută la stânga
column-move-right = Mută la dreapta
column-add = Adaugă o coloană
common-percent = { $value }{"\u00A0"}%

## Network view

netmap-empty = Încă nicio rețea de afișat
netmap-empty-note = Entitățile apar aici după ce au fost citite și și-au indicat poziția în arborele gPTP.
netmap-focus-clock-path = Calea ceasului pentru { $name }
netmap-focus-streams = Fluxurile pentru { $name }
netmap-showing = Se afișează: { $what }
netmap-devices = { $count ->
    [one] { $count } dispozitiv
    [few] { $count } dispozitive
   *[other] { $count } de dispozitive
}
netmap-bridges = { $count ->
    [one] { $count } switch
    [few] { $count } switch-uri
   *[other] { $count } de switch-uri
}
netmap-show-map = Afișează harta
netmap-show-details = Afișează detaliile
stream-numbered = Flux { $index }
netmap-bridge = Switch
netmap-access-point = Punct de acces
netmap-device = Dispozitiv
netmap-this-computer = Acest computer
netmap-connected = Conectat
netmap-advertised = Anunțat, niciun listener pregătit
netmap-advertised-off-tree = Anunțat, niciun listener pregătit ({ $listener } nu este în arborele gPTP)
netmap-failed-at = Rezervarea a eșuat la { $bridge }: { $reason }
netmap-failed = Rezervarea a eșuat: { $reason }
netmap-no-bridge-on = Niciun switch detectat pe { $interface }
netmap-cannot-listen-on = gPTP nu poate fi ascultat pe { $interface }
netmap-on-this-computer = Pe acest computer
netmap-path-not-reported = Cale neraportată
netmap-gptp-not-reported = gPTP neraportat
netmap-off-tree = În afara arborelui gPTP
netmap-off-ptp = În afara arborelui PTP
netmap-not-lite = În afara AVB Lite
netmap-lite-not-reported = AVB Lite neraportat
netmap-synced = Sincronizat
netmap-not-synced = Nesincronizat
netmap-triib-on = triib pe { $interface }
netmap-through-count = { $count } în tranzit
netmap-out = { $count ->
    [one] { $count } trimis
    [few] { $count } trimise
   *[other] { $count } de trimise
}
netmap-in = { $count ->
    [one] { $count } primit
    [few] { $count } primite
   *[other] { $count } de primite
}
netmap-failed-count = { $count ->
    [one] { $count } eșuat
    [few] { $count } eșuate
   *[other] { $count } de eșuate
}
netmap-advertised-only = Doar anunțat
netmap-failed-state = Eșuat
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = În afara arborelui gPTP: este propriul grandmaster
netmap-apart-no-path = Calea nu i-a fost raportată; urmează grandmasterul { $grandmaster }
netmap-apart-unreported = Nu și-a raportat starea gPTP
netmap-apart-no-neighbor = Niciun switch detectat pe interfața acestui computer
netmap-apart-cannot-listen = Acest computer nu poate asculta gPTP pe interfața sa
netmap-apart-on-this-computer = Rulează pe acest computer; citește-o de pe alt computer pentru a vedea starea gPTP
netmap-apart-not-lite = Nu rulează AVB Lite, deci nu urmează grandmasterul
netmap-apart-lite-unreported = Nu raportează nimic despre AVB Lite, deci nu se știe pe cine urmează
netmap-clock-tree = Arborele ceasului
netmap-no-grandmaster = Niciun grandmaster detectat
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Necesită atenție
netmap-nodes-below = Noduri în aval
netmap-bridges-below = Switch-uri în aval
netmap-clock-path = Calea ceasului
netmap-hops = Salturi de la grandmaster
netmap-link-delay = Întârziere pe legătură
netmap-bridge-port = Portul switch-ului
netmap-link-drops = Căderi ale legăturii
netmap-synced-to-grandmaster = Sincronizat cu grandmasterul
netmap-host-no-gptp = Nesincronizat: acest computer nu rulează gPTP
netmap-link-no-gptp = Nesincronizat: gPTP nu rulează pe legătura sa
netmap-ptp-offset-high = Nesincronizat: { $offset } față de grandmaster, peste limita de 50 µs permisă de AVB Lite
netmap-ptp-no-offset = Nesincronizat: nu a măsurat niciun decalaj față de grandmaster
netmap-audio = Audio
netmap-media-clock-streams = Fluxuri de ceas media
netmap-audio-streams = Fluxuri audio
netmap-bound = { $count ->
    [one] { $count } asociat
    [few] { $count } asociate
   *[other] { $count } de asociate
}
netmap-flowing = În transmisie
netmap-advertised-state = Anunțat
netmap-media-clock-stream = Flux de ceas media
netmap-audio-stream = Flux audio
netmap-reaches = Ajunge până la
netmap-passing-count = { $count ->
    [one] { $count } flux în tranzit
    [few] { $count } fluxuri în tranzit
   *[other] { $count } de fluxuri în tranzit
}
netmap-through = Prin
netmap-passing-through = În tranzit
netmap-sending = Trimitere
netmap-receiving = Recepție
netmap-problems = Probleme
netmap-help-back = Dă clic pe fundal pentru a reveni la vederea de ansamblu.
netmap-help-stream = Dă clic pe un flux pentru a-l inspecta sau pe fundal pentru a reveni la vederea de ansamblu.
netmap-help-ptp = În AVB Lite, ceasul merge cap la cap, de la grandmaster la fiecare dispozitiv, prin switch-uri care nu participă, deci niciunul nu este afișat. Un dispozitiv este sincronizat cât timp urmează grandmasterul cu un decalaj de cel mult 50 µs. Dă clic pe un dispozitiv sau pe firul lui pentru a-i inspecta ceasul; dă clic pe fundal pentru a anula selecția.
netmap-help-clock = Ceasul pornește de la grandmaster și trece prin fiecare switch până la fiecare nod din arbore. O linie gri întreruptă este o legătură pe care nu rulează gPTP. Dă clic pe un dispozitiv sau pe firul lui pentru a-i inspecta calea ceasului; dă clic pe fundal pentru a anula selecția.
netmap-help-media-clock = Doar fluxuri de ceas media (CRF), desenate la fel ca audio: un fir pentru fiecare flux, colorat după talker. Dă clic pe un fir pentru a-i inspecta fluxul sau pe un dispozitiv pentru a-i vedea fluxurile; dă clic pe fundal pentru a anula selecția.
netmap-help-audio = Fiecare flux are propriul fir, care intră și iese din fiecare switch pe care îl traversează. Culoarea depinde de talker: fiecare talker are o nuanță, iar fluxurile lui sunt tonuri ale ei. Punctele în mișcare arată că audio circulă; o linie roșie fixă este o rezervare eșuată, iar o linie gri fixă este un flux anunțat fără niciun listener pregătit; ambele se opresc acolo unde se oprește rezervarea. Dispozitivele din coloana din mijloc se conectează direct la switch-ul grandmasterului. Dă clic pe un fir pentru a-i inspecta fluxul sau pe un dispozitiv pentru a-i vedea fluxurile; dă clic pe fundal pentru a anula selecția.

## Connections

matrix-nothing-shown = Niciun flux de afișat
matrix-nothing-shown-note = Schimbă căutarea sau filtrele pentru a vedea mai multe fluxuri.
matrix-empty = Niciun flux de conectat
matrix-empty-note = Fluxurile de talker și de listener se întâlnesc aici după ce entitățile care le au au fost citite.
matrix-all-streams = Toate fluxurile
matrix-connectable-only = Ascunde ce nu se poate conecta
matrix-none-hidden = Toate fluxurile afișate se pot conecta
matrix-hidden = { $count ->
    [one] { $count } flux ascuns
    [few] { $count } fluxuri ascunse
   *[other] { $count } de fluxuri ascunse
}
matrix-own = Ieșirile unei entități nu se conectează la propriile intrări.
matrix-working = Se lucrează.
matrix-waiting-change = Se așteaptă ultima modificare a acestei intrări.
matrix-connected = Conectat și în recepție. Dă clic pentru a deconecta.
matrix-bound-waiting = Asociată, se așteaptă fluxul talkerului. Dă clic pentru a deconecta.
matrix-bound-failed = Asociată, dar rezervarea talkerului a eșuat: { $reason }. Dă clic pentru a deconecta.
matrix-bound-formats-differ = Asociată, dar formatele diferă: talkerul trimite { $sent }, intrarea este setată la { $set }. Dă clic pentru a deconecta.
matrix-formats-match = Formatele se potrivesc ({ $format }). Dă clic pentru a conecta.
matrix-format-must-change = Intrarea acceptă { $sent }, dar este setată la { $set }, deci s-ar putea să nu redea nimic până nu i se schimbă formatul. Dă clic pentru a alege un format.
matrix-format-input = Schimbă intrarea la
matrix-format-both = Schimbă-le pe amândouă la
matrix-format-talker-busy = Talkerul transmite către alți listeneri, așa că formatul său rămâne.
matrix-format-anyway = Conectează fără a schimba formatele
matrix-incompatible = Intrarea nu acceptă { $sent }. Este setată la { $set }.
matrix-group-none = Neconectat. Extinde pentru a conecta fluxurile unul câte unul.
matrix-group-connected = { $count ->
    [one] { $count } conexiune. Extinde pentru a o vedea.
    [few] { $count } conexiuni. Extinde pentru a vedea fiecare conexiune.
   *[other] { $count } de conexiuni. Extinde pentru a vedea fiecare conexiune.
}
matrix-outputs-expand = { $count ->
    [one] { $count } ieșire de flux. Dă clic pe săgeată pentru a extinde, pe nume pentru a inspecta entitatea.
    [few] { $count } ieșiri de flux. Dă clic pe săgeată pentru a extinde, pe nume pentru a inspecta entitatea.
   *[other] { $count } de ieșiri de flux. Dă clic pe săgeată pentru a extinde, pe nume pentru a inspecta entitatea.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } ieșire de flux. Dă clic pe săgeată pentru a restrânge, pe nume pentru a inspecta entitatea.
    [few] { $count } ieșiri de flux. Dă clic pe săgeată pentru a restrânge, pe nume pentru a inspecta entitatea.
   *[other] { $count } de ieșiri de flux. Dă clic pe săgeată pentru a restrânge, pe nume pentru a inspecta entitatea.
}
matrix-inputs-expand = { $count ->
    [one] { $count } intrare de flux. Dă clic pe săgeată pentru a extinde, pe nume pentru a inspecta entitatea.
    [few] { $count } intrări de flux. Dă clic pe săgeată pentru a extinde, pe nume pentru a inspecta entitatea.
   *[other] { $count } de intrări de flux. Dă clic pe săgeată pentru a extinde, pe nume pentru a inspecta entitatea.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } intrare de flux. Dă clic pe săgeată pentru a restrânge, pe nume pentru a inspecta entitatea.
    [few] { $count } intrări de flux. Dă clic pe săgeată pentru a restrânge, pe nume pentru a inspecta entitatea.
   *[other] { $count } de intrări de flux. Dă clic pe săgeată pentru a restrânge, pe nume pentru a inspecta entitatea.
}
matrix-stream-inspect = { $detail } Dă clic pentru a inspecta { $entity }.
matrix-point = Indică o celulă
matrix-point-note = pentru a-i vedea talkerul și listenerul și dacă formatele lor se potrivesc.
matrix-legend-waiting = Asociată, se așteaptă fluxul
matrix-legend-trouble = Asociată, ceva nu este în regulă
matrix-legend-open = Se poate conecta
matrix-legend-change = Mai întâi trebuie schimbat formatul intrării
matrix-legend-incompatible = Formatele nu se potrivesc
matrix-talker-outputs = Ieșiri de talker
matrix-listener-inputs = Intrări de listener

common-thousands-separator = {"."}
common-decimal-separator = {","}

## Diagnostics

diag-since-start = Numărat de la pornirea entității.
diag-stream-input = Intrare de flux
diag-stream-output = Ieșire de flux
diag-locked = { $count ->
    [0] necalat
    [1] calat o dată
    [2] calat de două ori
    [few] calat de { $number } ori
   *[other] calat de { $number } de ori
}
diag-lost-lock = { $count ->
    [0] fără pierderi de calare
    [1] calare pierdută o dată
    [2] calare pierdută de două ori
    [few] calare pierdută de { $number } ori
   *[other] calare pierdută de { $number } de ori
}
diag-frames-in = { $count ->
    [one] { $number } cadru primit
    [few] { $number } cadre primite
   *[other] { $number } de cadre primite
}
diag-frames-out = { $count ->
    [one] { $number } cadru trimis
    [few] { $number } cadre trimise
   *[other] { $number } de cadre trimise
}
diag-media-locked = { $count ->
    [0] necalat pe ceasul media
    [1] calat pe ceasul media o dată
    [2] calat pe ceasul media de două ori
    [few] calat pe ceasul media de { $number } ori
   *[other] calat pe ceasul media de { $number } de ori
}
diag-lost-media-lock = { $count ->
    [0] fără pierderi de calare pe ceasul media
    [1] calare pe ceasul media pierdută o dată
    [2] calare pe ceasul media pierdută de două ori
    [few] calare pe ceasul media pierdută de { $number } ori
   *[other] calare pe ceasul media pierdută de { $number } de ori
}
diag-interrupted = { $count ->
    [0] neîntrerupt
    [1] întrerupt o dată
    [2] întrerupt de două ori
    [few] întrerupt de { $number } ori
   *[other] întrerupt de { $number } de ori
}
diag-out-of-sequence = { $count ->
    [one] { $number } cadru în afara secvenței
    [few] { $number } cadre în afara secvenței
   *[other] { $number } de cadre în afara secvenței
}
diag-media-resets = { $count ->
    [one] { $number } resetare media
    [few] { $number } resetări media
   *[other] { $number } de resetări media
}
diag-timestamps-uncertain = { $count ->
    [0] fără marcaje temporale incerte
    [1] marcaje temporale incerte o dată
    [2] marcaje temporale incerte de două ori
    [few] marcaje temporale incerte de { $number } ori
   *[other] marcaje temporale incerte de { $number } de ori
}
diag-no-timestamp = { $count ->
    [one] { $number } cadru fără marcaj temporal
    [few] { $number } cadre fără marcaj temporal
   *[other] { $number } de cadre fără marcaj temporal
}
diag-unsupported-format = { $count ->
    [one] { $number } cadru într-un format neacceptat
    [few] { $number } cadre într-un format neacceptat
   *[other] { $number } de cadre într-un format neacceptat
}
diag-late = { $count ->
    [one] { $number } cadru întârziat
    [few] { $number } cadre întârziate
   *[other] { $number } de cadre întârziate
}
diag-early = { $count ->
    [one] { $number } cadru prea devreme
    [few] { $number } cadre prea devreme
   *[other] { $number } de cadre prea devreme
}
diag-started = { $count ->
    [0] nepornit
    [1] pornit o dată
    [2] pornit de două ori
    [few] pornit de { $number } ori
   *[other] pornit de { $number } de ori
}
diag-stopped = { $count ->
    [0] neoprit
    [1] oprit o dată
    [2] oprit de două ori
    [few] oprit de { $number } ori
   *[other] oprit de { $number } de ori
}
diag-reservation-failed = rezervarea talkerului a eșuat: { $reason }
diag-latency = latență acumulată de { $microseconds } µs

## AVB Lite

lite-active = Activ
lite-active-untagged = Activ, fără etichetă
lite-active-vlan = Activ, VLAN { $vlan }
lite-capable = Compatibil
lite-mode = Mod
lite-mode-capable = AVB, compatibil AVB Lite
lite-because = Motiv
lite-fallback-none = niciun motiv indicat
lite-fallback-endpoint = a trecut declarația altui dispozitiv terminal, deci între ele nu există niciun switch AVB
lite-fallback-unanswered = nouă cereri de peer delay au rămas fără răspuns
lite-fallback-responders = două sau mai multe dispozitive au răspuns la aceeași cerere de peer delay, deci switch-ul nu este compatibil AVB
lite-fallback-configured = setat de operator sau de un controler
lite-fallback-other = un motiv pe care profilul nu îl numește
lite-other-profile = Alt profil
lite-ptp-domain = { $profile }, domeniul { $domain }
lite-offset = Decalaj
lite-offset-from = { $offset } față de { $grandmaster }
lite-media-vlan = VLAN media
lite-untagged = Fără etichetă
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Până la { $count } listener per flux, apoi multicast
    [few] Până la { $count } listenere per flux, apoi multicast
   *[other] Până la { $count } de listenere per flux, apoi multicast
}
lite-link = Legătură
lite-bandwidth = Lățime de bandă
lite-egress-of = { $used } din { $link }, { $share }
lite-egress-of-assumed = { $used } din { $link }, { $share }, presupunând o legătură gigabit
lite-egress-reported = Conform numărării de către entitate a fluxurilor ei admise.
lite-egress-worked-out = Calculat din formatele ieșirilor ei de flux conectate.
lite-alarm-offset = Decalaj PTP de { $offset }, peste limita de 50 µs permisă de AVB Lite
lite-alarm-egress = Trafic de ieșire la { $share } din legătură, peste limita de { $limit } permisă fluxurilor

## AVB Wireless

wireless-station = Stație
wireless-access-point = Punct de acces
wireless-role = Rol
wireless-mode = Mod
wireless-time = Timp
wireless-mode-a-ftm = Mode A, 802.1AS prin FTM
wireless-mode-a-tm = Mode A, 802.1AS prin TM
wireless-mode-b = Mode B, din cadrele beacon
wireless-no-time = Fără sincronizarea timpului
wireless-other-mode = Un mod pe care profilul nu îl numește
wireless-locked = Calat
wireless-holdover = În holdover
wireless-not-locked = Necalat
wireless-row-locked = Stație, calată
wireless-row-holdover = Stație, în holdover
wireless-row-not-locked = Stație, necalată
wireless-row-access-point = { $count ->
    [one] Punct de acces, { $count } stație
    [few] Punct de acces, { $count } stații
   *[other] Punct de acces, { $count } de stații
}
wireless-link = Legătură
wireless-channel = canalul { $channel }
wireless-not-known = Necunoscut
wireless-signal = Semnal
wireless-rate = Viteză
wireless-ftm-valid = { $share } valide
wireless-rtt = timp dus-întors { $rtt }
wireless-bursts = { $count ->
    [one] rafale de { $count } cadru
    [few] rafale de { $count } cadre
   *[other] rafale de { $count } de cadre
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = punctul de acces acordă rafale FTM de altă lungime decât trei sau două cadre
wireless-reason-measurement = nici FTM, nici TM cu punctul de acces
wireless-reason-signaling = niciun mesaj Signaling gPTP-capable de la punctul de acces
wireless-reason-other = un motiv pe care profilul nu îl numește
wireless-servo = Eroare servo
wireless-stations = Stații
wireless-station-count = { $count ->
    [one] { $count } stație
    [few] { $count } stații
   *[other] { $count } de stații
}
wireless-no-ftm = fără FTM
wireless-unserved = Listenere neservite
wireless-frame-budget = Buget de cadre
wireless-frame-use = { $budget ->
    [one] { $used } din { $budget } cadru pe secundă
    [few] { $used } din { $budget } cadre pe secundă
   *[other] { $used } din { $budget } de cadre pe secundă
}
wireless-stream-frames = Cadre de flux
wireless-frames-of = către stații: { $readdressed }, fără listener: { $unmapped }, aruncate: { $dropped }, de la stații: { $restored }
wireless-class-a-allowed = Permisă
wireless-class-a-not-allowed = Nepermisă
wireless-alarm-not-locked = Timpul Wi-Fi nu este calat pe punctul de acces
wireless-alarm-holdover = Timpul Wi-Fi în holdover, calarea pe punctul de acces s-a pierdut
wireless-alarm-unserved = { $count ->
    [one] { $count } listener pe portul Wi-Fi neservit, peste limita unicast
    [few] { $count } listenere pe portul Wi-Fi neservite, peste limita unicast
   *[other] { $count } de listenere pe portul Wi-Fi neservite, peste limita unicast
}

## Log

log-all = Toate
log-warnings = Avertismente
log-pause = Pauză
log-resume = Reia
log-clear = Golește
log-empty = Fiecare cadru ATDECC trimis și primit de triib apare aici, cele mai noi primele.
log-none-match = Niciun cadru păstrat nu corespunde filtrului.
log-frames = { $count ->
    [one] { $count } cadru
    [few] { $count } cadre
   *[other] { $count } de cadre
}
log-shown-of = { $all ->
    [one] { $shown } din { $all } cadru
    [few] { $shown } din { $all } cadre
   *[other] { $shown } din { $all } de cadre
}
log-sent = Trimis
log-heard = Primit
log-not-decoded = Nedecodat
log-warning-short = { $missing ->
    [one] control_data_length indică { $missing } octet dincolo de sfârșitul cadrului.
    [few] control_data_length indică { $missing } octeți dincolo de sfârșitul cadrului.
   *[other] control_data_length indică { $missing } de octeți dincolo de sfârșitul cadrului.
}
log-warning-undecodable = Nu poate fi decodat: { $error }.
log-warning-long-acmp = Este în forma ACMP lungă, pe care o entitate Milan nu are voie să o trimită (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Mapări de canale
mapping-inputs = Intrări
mapping-outputs = Ieșiri
mapping-port = port { $number }
mapping-fixed = fixă
mapping-not-read = Necitit încă.
mapping-no-clusters = Niciun cluster.
mapping-no-streams = Niciun flux audio.
mapping-none = Nicio mapare.
mapping-not-mapped = Nemapat
mapping-cluster-numbered = Cluster { $index }

## Presets

presets-note = Un preset păstrează sursele de ceas, frecvențele de eșantionare, formatele fluxurilor, controalele și conexiunile fiecărei entități. La încărcare se schimbă doar ce diferă.
presets-none = Niciun preset salvat încă.
presets-connections = { $count ->
    [one] { $count } conexiune
    [few] { $count } conexiuni
   *[other] { $count } de conexiuni
}
presets-recall = Încarcă
presets-delete = Șterge
presets-no-place = Preseturile nu pot fi păstrate nicăieri: dosarul personal nu este cunoscut.
presets-undeletable = Nu s-a putut șterge { $path }: { $error }.
presets-saved = { $count ->
    [one] S-a salvat „{ $name }” cu { $count } entitate.
    [few] S-a salvat „{ $name }” cu { $count } entități.
   *[other] S-a salvat „{ $name }” cu { $count } de entități.
}
presets-nothing-differs = Nimic nu diferă de „{ $name }”.
presets-recalling = { $count ->
    [one] Se încarcă „{ $name }”: { $count } modificare.
    [few] Se încarcă „{ $name }”: { $count } modificări.
   *[other] Se încarcă „{ $name }”: { $count } de modificări.
}
presets-missing = { $report } Entități absente sau necitite: { $missing }.
presets-deleted = S-a șters „{ $name }”.
presets-host-note = Păstrează și talkerele și listenerele proprii ale acestui computer și le pornește din nou la încărcare.
presets-host-endpoints = { $count } pe acest computer
presets-starting-host = Se pornesc talkerele și listenerele acestui computer pentru „{ $name }”; restul urmează când revin.

## Controls

control-numbered = Control { $index }
control-not-shown = Nu este afișat aici
control-option = Opțiunea { $number }

## Network errors

network-permission = triib are nevoie de permisiune pentru a trimite și a primi cadre Ethernet brute.
network-needs-npcap = triib are nevoie de Npcap pentru a trimite și a primi cadre Ethernet brute.
network-npcap-administrators = Npcap permite doar administratorilor să trimită și să primească cadre Ethernet brute. Rulează triib ca administrator sau reinstalează Npcap fără opțiunea doar pentru administratori.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own talkers and listeners

host-add-talker = Adaugă talker
host-add-listener = Adaugă listener
host-add-device = Adaugă dispozitiv
host-show-mine = Arată doar talkerele și listenerele proprii ale acestui computer
host-show-all = Arată toate entitățile
host-new-talker = Talker gazdă { $number }
host-new-listener = Listener gazdă { $number }
host-new-device = Dispozitiv gazdă { $number }
host-failed = Nu a putut fi adăugat pe acest computer: { $reason }
host-needs-clock = Talkerele și listenerele proprii ale acestui computer au nevoie de o interfață cu fir cu ceas hardware PTP
host-no-ptp4l = ptp4l nu răspunde, așa că fluxurile acestui computer nu pot păstra ora gPTP
host-elsewhere = triib-endpointd rulează pe { $interface } pentru alt utilizator sau ca root, așa că talkerele și listenerele acestui computer rulează acolo, nu aici
host-foreign-mrp = Alt program trimite declarații MSRP sau MVRP pe { $interface } de la adresa acestui computer și poate retrage ce le trebuie fluxurilor acestui computer
host-alarm-foreign-mrp = Alt program de pe acest computer trimite declarații MSRP sau MVRP pe { $interface }
host-state = Stare
host-streaming = Transmite
host-waiting = Așteaptă un listener
host-listening = În ascultare
host-bound = Legat, așteaptă talkerul
host-unbound = Nelegat
host-audio-from = Audio de la
host-audio-to = Audio către
host-channels = Canale
host-virtual-device = Dispozitiv virtual
host-add-output-stream = Adaugă flux de ieșire
host-add-input-stream = Adaugă flux de intrare
host-remove-stream = Elimină fluxul
host-silence = Liniște
host-tone = Ton de test
host-nowhere = Nicăieri
host-default-device = Dispozitiv implicit
host-remove = Elimină de pe acest computer
