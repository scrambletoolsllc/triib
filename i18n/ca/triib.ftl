# triib's interface text in Catalan. The source is i18n/en/triib.ftl; term
# choices are in docs/glossary/ca.md.

## Language

language-name = Català

## Common

common-close = Tanca
common-more = Més
common-keep-toolbar-shown = Mantén visible la barra d’eines
common-auto-hide-toolbar = Amaga automàticament la barra d’eines

## Settings

settings-title = Configuració
settings-general = General
settings-appearance = Aparença
settings-language = Llengua
settings-language-system = Llengua del sistema: { $language }
settings-language-note = Els camps de text fan servir la llengua d’entrada del sistema.
settings-appearance-system = Sistema
settings-appearance-light = Clar
settings-appearance-dark = Fosc
settings-colors = Colors
settings-system-accent = Fes servir el color d’èmfasi del sistema
settings-accent-picked = El color de sota és la base dels colors de triib.
settings-accent-omarchy = Del tema Omarchy, { $theme }.
settings-accent-desktop = Del color d’èmfasi de l’escriptori.
settings-accent-none = L’escriptori no té color d’èmfasi, així que es fa servir el color de sota.
settings-motion = Moviment
settings-animations = Animacions
settings-animations-note = Efectes elàstics i lliscaments a cada canvi.
settings-animations-reduced = L’escriptori demana menys moviment, així que triib es queda quiet.

common-cancel = Cancel·la
common-save = Desa
common-not-set = Sense definir
common-unnamed = Sense nom
common-none = Cap
common-mac-address = Adreça MAC
common-list-separator = {", "}

## Network interfaces

interface-up = activa
interface-link-down = enllaç inactiu
interface-wireless = sense fil
interface-hardware-clock = rellotge de maquinari
interface-hardware-clock-named = rellotge de maquinari { $clock }
interface-virtual = virtual

## Toolbar

toolbar-choose-interface = Tria una interfície
toolbar-interface = Interfície de xarxa
toolbar-show-virtual = Mostra les interfícies virtuals
toolbar-hide-virtual = Amaga les interfícies virtuals
toolbar-connections = Connexions
toolbar-network = Xarxa
toolbar-entities = Entitats
toolbar-rediscover = Demana a cada entitat que s’anunciï
toolbar-rescan = Esborra i torna a escanejar totes les entitats
toolbar-search = Cerca entitats i fluxos
toolbar-presets = Presets
toolbar-log = Registre
toolbar-inspector = Inspector
toolbar-settings = Configuració

## The network's state, in place of a view

state-no-interface = Cap interfície
state-no-interface-note = Trieu la interfície de la xarxa AVB per descobrir-hi les entitats.
state-starting = S’està iniciant
state-starting-note = S’està obrint { $interface }.
state-listening = A l’escolta
state-listening-note = Les entitats de { $interface } apareixen aquí a mesura que s’anuncien.
state-permission-needed = Cal permís
state-npcap-needed = Cal Npcap
state-get-npcap = Obtén Npcap
state-copy-command = Copia l’ordre
state-cannot-use = No es pot fer servir { $interface }
state-try-again = Torna-ho a provar

## Entity list

entities-none-yet = Encara no hi ha entitats
entities-none-yet-note = Totes les entitats de la xarxa, amb els seus rols, classes SR i rellotge.

## Inspector

inspector-title = Inspector
inspector-entity = Entitat
inspector-streams = Fluxos
inspector-controls = Controls
inspector-diagnostics = Diagnòstic
inspector-descriptors = Descriptors
inspector-select = Seleccioneu una entitat per veure’n els detalls.
inspector-offline = { $entity } està fora de línia.
inspector-rename = Canvia el nom
inspector-name = Nom
inspector-identify = Identifica
inspector-model-not-read = El seu model d’entitat no s’ha llegit.
inspector-no-streams = Cap flux.
inspector-no-controls = No hi ha controls per mostrar.
inspector-no-diagnostics = No s’ha notificat cap interfície ni comptador.
inspector-reading = S’estan llegint els descriptors, { $count } fins ara.
inspector-read-failed = No s’ha pogut llegir el model d’entitat: { $reason }.

entity-section = Entitat
entity-name = Nom
entity-group = Grup
entity-product = Producte
entity-firmware = Firmware
entity-serial-number = Número de sèrie
entity-configuration = Configuració
entity-configuration-of = { $name } ({ $number } de { $count })
entity-milan = Milan
entity-media-clock = Rellotge de mitjans
entity-clock-domain = Domini de rellotge
entity-sampling-rate = Freqüència de mostreig
clock-source-numbered = Font { $index }
rate-pull = pull { $pull }

stream-inputs = Entrades de flux
stream-outputs = Sortides de flux
stream-max-transit-time = Temps de trànsit màx. { $time }

avb-interfaces = Interfícies AVB
avb-interface = Interfície
avb-interface-clock-identity = Identitat del rellotge
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domini { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = En funcionament
avb-interface-none-reported = No n’ha notificat cap
avb-interface-path = Camí
avb-interface-own-grandmaster = És el seu propi grandmaster
avb-interface-hops = { $count ->
    [one] A { $count } salt del grandmaster
    [many] A { $count } salts del grandmaster
   *[other] A { $count } salts del grandmaster
}
avb-interface-link-up = Enllaç actiu
avb-interface-link-down = Enllaç inactiu
avb-interface-grandmaster-changes = Canvis de grandmaster
avb-interface-frames-sent = Trames enviades
avb-interface-frames-received = Trames rebudes
avb-interface-crc-errors = Errors CRC

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } tipus de descriptor
    [many] { $count } tipus de descriptors
   *[other] { $count } tipus de descriptors
}
tree-clock = Rellotge
tree-clock-source-from = { $kind }, des de { $location } { $index }
tree-clock-domain-using = Fa servir { $source }
tree-clusters = { $count ->
    [one] { $count } clúster
    [many] { $count } clústers
   *[other] { $count } clústers
}
tree-maps = { $count ->
    [one] { $count } mapatge
    [many] { $count } mapatges
   *[other] { $count } mapatges
}

advert-not-advertised = No anunciat
advert-identity = Identitat
advert-entity-id = ID d’entitat
advert-entity-model = Model d’entitat
advert-roles = Rols
advert-talker = Talker
advert-listener = Listener
advert-clock = Rellotge
advert-btc = BTC
advert-gptp-domain = Domini gPTP
advert-sr-classes = Classes SR
advert-indexes = Índexs del model d’entitat
advert-identify-control = Control d’identificació
advert-avb-interface = Interfície AVB
advert-advertising = Anunci
advert-valid-time = Temps de validesa
advert-available-index = Índex de disponibilitat
advert-association = Associació
advert-capabilities = Capacitats

## Status bar

status-entities = { $count ->
    [one] { $count } entitat
    [many] { $count } entitats
   *[other] { $count } entitats
}
status-not-discovering = Descobriment inactiu
status-discovering = S’està descobrint
status-discovering-as = S’està descobrint com a { $controller }
status-stopped = Aturat per un error
status-alarm = Alarma
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } i { $count } més

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = controlador
role-none = cap rol
classes-a-and-b = A i B
clock-no-gptp = Sense gPTP

read-not-read = No llegit
read-reading = S’està llegint, { $count } fins ara
read-ready-unreadable = { $count ->
    [one] Llest, { $count } il·legible
    [many] Llest, { $count } il·legibles
   *[other] Llest, { $count } il·legibles
}
read-ready-cached = Llest, de la memòria cau
read-ready = Llest
read-failed = Ha fallat: { $reason }

milan-no = No
milan-before-1-3 = anterior a la 1.3
milan-certified = { $version }, certificat { $certification }
milan-not-certified = { $version }, no certificat

outcome-status = estat { $status }
outcome-no-response = sense resposta
outcome-not-possible = no és possible
outcome-connect = No s’ha pogut connectar { $talker } a { $listener }: { $reason }.
outcome-disconnect = No s’ha pogut desconnectar { $listener }: { $reason }.
outcome-identify = No s’ha pogut identificar { $entity }: { $reason }.
outcome-rename = No s’ha pogut canviar el nom de { $what } a «{ $name }»: { $reason }.
outcome-rename-group = No s’ha pogut canviar el nom del grup de { $entity } a «{ $name }»: { $reason }.
outcome-format-streaming = No s’ha pogut canviar el format de { $stream }: està transmetent. Primer desconnecteu-lo.
outcome-format = No s’ha pogut canviar el format de { $stream }: { $reason }.
outcome-sampling-rate = No s’ha pogut canviar la freqüència de mostreig de { $entity }: { $reason }.
outcome-clock-source = No s’ha pogut canviar la font de rellotge de { $entity }: { $reason }.
outcome-map = No s’ha pogut mapar el canal a { $entity }: { $reason }.
outcome-unmap = No s’ha pogut treure el mapatge del canal a { $entity }: { $reason }.
outcome-control = No s’ha pogut ajustar «{ $control }» a { $entity }: { $reason }.
outcome-control-numbered = No s’ha pogut ajustar el control { $index } a { $entity }: { $reason }.

stream-not-connected = No connectat
stream-from = Des de { $stream }
stream-from-receiving = Des de { $stream }, rebent
stream-from-waiting = Des de { $stream }, esperant el talker
stream-from-failed = Des de { $stream }, ha fallat la reserva del talker: { $reason }
stream-sending-to = Enviant a { $destination }

failure-no-response = l’entitat no ha respost
failure-refused = l’entitat ho ha rebutjat amb { $status }
failure-malformed = la seva resposta no s’ha pogut descodificar
failure-on-this-computer = l’entitat s’executa en aquest ordinador; llegiu-la des d’un altre

msrp-failure-1 = amplada de banda insuficient
msrp-failure-2 = recursos del commutador insuficients
msrp-failure-3 = amplada de banda insuficient per a la classe de trànsit
msrp-failure-4 = ID de flux en ús per un altre talker
msrp-failure-5 = adreça de destinació ja en ús
msrp-failure-6 = desplaçat per un flux de rang superior
msrp-failure-7 = la latència notificada ha canviat
msrp-failure-8 = el port de sortida no és compatible amb AVB
msrp-failure-9 = feu servir una altra adreça de destinació
msrp-failure-10 = recursos MSRP esgotats
msrp-failure-11 = recursos MMRP esgotats
msrp-failure-12 = no es pot desar l’adreça de destinació
msrp-failure-13 = la prioritat no és una prioritat de classe SR
msrp-failure-14 = trames massa grans per al medi
msrp-failure-15 = límit de fan-in del port assolit
msrp-failure-16 = primer valor canviat per a un flux registrat
msrp-failure-17 = VLAN bloquejada al port de sortida
msrp-failure-18 = etiquetatge VLAN desactivat al port de sortida
msrp-failure-19 = la prioritat de classe SR no coincideix
msrp-failure-unknown = motiu desconegut
msrp-failure-at = { $reason }, al commutador { $bridge }

## Entity list columns

column-vendor = Fabricant
column-model = Model
column-state = Estat
column-entity-model-id = ID del model d’entitat
column-talker-streams = Fluxos talker
column-listener-streams = Fluxos listener
column-avb-lite = AVB Lite
column-egress = Trànsit de sortida
column-wireless = Sense fil

## Settings file

settings-no-place = No hi ha on desar la configuració: no es coneix la carpeta personal.
settings-unusable = No s’ha pogut fer servir { $path }: { $error }.
settings-unsaved = No s’ha pogut desar { $path }: { $error }.

column-remove = Elimina la columna
column-move-left = Mou a l’esquerra
column-move-right = Mou a la dreta
column-add = Afegeix una columna
common-percent = { $value }{"\u00A0"}%

## Network view

netmap-empty = Encara no hi ha cap xarxa per mostrar
netmap-empty-note = Les entitats apareixen aquí un cop llegides, quan han indicat on són a l’arbre gPTP.
netmap-focus-clock-path = Camí del rellotge de { $name }
netmap-focus-streams = Fluxos de { $name }
netmap-showing = Visualització: { $what }
netmap-devices = { $count ->
    [one] { $count } dispositiu
    [many] { $count } dispositius
   *[other] { $count } dispositius
}
netmap-bridges = { $count ->
    [one] { $count } commutador
    [many] { $count } commutadors
   *[other] { $count } commutadors
}
netmap-show-map = Mostra el mapa
netmap-show-details = Mostra els detalls
stream-numbered = Flux { $index }
netmap-bridge = Commutador
netmap-access-point = Punt d’accés
netmap-device = Dispositiu
netmap-this-computer = Aquest ordinador
netmap-connected = Connectat
netmap-advertised = Anunciat, cap listener a punt
netmap-advertised-off-tree = Anunciat, cap listener a punt ({ $listener } no és a l’arbre gPTP)
netmap-failed-at = Ha fallat la reserva a { $bridge }: { $reason }
netmap-failed = Ha fallat la reserva: { $reason }
netmap-no-bridge-on = No s’ha detectat cap commutador a { $interface }
netmap-cannot-listen-on = No es pot escoltar gPTP a { $interface }
netmap-on-this-computer = En aquest ordinador
netmap-path-not-reported = Camí no notificat
netmap-gptp-not-reported = gPTP no notificat
netmap-off-tree = Fora de l’arbre gPTP
netmap-off-ptp = Fora de l’arbre PTP
netmap-not-lite = Fora d’AVB Lite
netmap-lite-not-reported = AVB Lite no notificat
netmap-synced = Sincronitzat
netmap-not-synced = No sincronitzat
netmap-triib-on = triib a { $interface }
netmap-through-count = { $count } en trànsit
netmap-out = { $count ->
    [one] { $count } enviat
    [many] { $count } enviats
   *[other] { $count } enviats
}
netmap-in = { $count ->
    [one] { $count } rebut
    [many] { $count } rebuts
   *[other] { $count } rebuts
}
netmap-failed-count = { $count } amb error
netmap-advertised-only = Només anunciat
netmap-failed-state = Error
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Fora de l’arbre gPTP: és el seu propi grandmaster
netmap-apart-no-path = No ha notificat el seu camí; segueix el grandmaster { $grandmaster }
netmap-apart-unreported = No ha notificat el seu estat gPTP
netmap-apart-no-neighbor = No s’ha detectat cap commutador a la interfície d’aquest ordinador
netmap-apart-cannot-listen = Aquest ordinador no pot escoltar gPTP a la seva interfície
netmap-apart-on-this-computer = S’executa en aquest ordinador; llegiu-la des d’un altre ordinador per veure’n l’estat de gPTP
netmap-apart-not-lite = No executa AVB Lite, així que no segueix el grandmaster
netmap-apart-lite-unreported = No notifica res d’AVB Lite, així que no se sap qui segueix
netmap-clock-tree = Arbre de rellotge
netmap-no-grandmaster = No s’ha detectat cap grandmaster
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Requereix atenció
netmap-nodes-below = Nodes per sota
netmap-bridges-below = Commutadors per sota
netmap-clock-path = Camí del rellotge
netmap-hops = Salts des del grandmaster
netmap-link-delay = Retard de l’enllaç
netmap-bridge-port = Port del commutador
netmap-link-drops = Caigudes de l’enllaç
netmap-synced-to-grandmaster = Sincronitzat amb el grandmaster
netmap-host-no-gptp = No sincronitzat: aquest ordinador no executa gPTP
netmap-link-no-gptp = No sincronitzat: gPTP no funciona en el seu enllaç
netmap-ptp-offset-high = No sincronitzat: { $offset } respecte del grandmaster, més dels 50 µs que permet AVB Lite
netmap-ptp-no-offset = No sincronitzat: no ha mesurat cap decalatge respecte del grandmaster
netmap-audio = Àudio
netmap-media-clock-streams = Fluxos de rellotge de mitjans
netmap-audio-streams = Fluxos d’àudio
netmap-bound = { $count ->
    [one] { $count } vinculat
    [many] { $count } vinculats
   *[other] { $count } vinculats
}
netmap-flowing = En circulació
netmap-advertised-state = Anunciat
netmap-media-clock-stream = Flux de rellotge de mitjans
netmap-audio-stream = Flux d’àudio
netmap-reaches = Arriba fins a
netmap-passing-count = { $count ->
    [one] { $count } flux en trànsit
    [many] { $count } fluxos en trànsit
   *[other] { $count } fluxos en trànsit
}
netmap-through = A través de
netmap-passing-through = En trànsit
netmap-sending = Enviament
netmap-receiving = Recepció
netmap-problems = Problemes
netmap-help-back = Feu clic al fons per tornar a la vista general.
netmap-help-stream = Feu clic en un flux per inspeccionar-lo, o al fons per tornar a la vista general.
netmap-help-ptp = A AVB Lite el rellotge va d’extrem a extrem, del grandmaster a cada dispositiu, a través de commutadors que no hi participen, així que no se’n mostra cap. Un dispositiu està sincronitzat mentre segueix el grandmaster amb un marge de 50 µs. Feu clic en un dispositiu o en el seu fil per inspeccionar-ne el rellotge; feu clic al fons per esborrar la selecció.
netmap-help-clock = El rellotge surt del grandmaster i passa per cada commutador fins a cada node de l’arbre. Una línia grisa discontínua és un enllaç on no funciona gPTP. Feu clic en un dispositiu o en el seu fil per inspeccionar-ne el camí del rellotge; feu clic al fons per esborrar la selecció.
netmap-help-media-clock = Només fluxos de rellotge de mitjans (CRF), dibuixats igual que l’àudio: un fil per flux, acolorit segons el talker. Feu clic en un fil per inspeccionar-ne el flux, o en un dispositiu per veure’n els fluxos; feu clic al fons per esborrar la selecció.
netmap-help-audio = Cada flux té el seu propi fil, que entra i surt de cada commutador que travessa. El color depèn del talker: cada talker té un to, i els seus fluxos en són matisos. Els punts en moviment indiquen que l’àudio circula; una línia vermella fixa és una reserva fallida i una línia grisa fixa és un flux anunciat sense cap listener a punt; totes dues s’aturen on s’atura la reserva. Els dispositius de la columna central es connecten directament al commutador del grandmaster. Feu clic en un fil per inspeccionar-ne el flux, o en un dispositiu per veure’n els fluxos; feu clic al fons per esborrar la selecció.

## Connections

matrix-nothing-shown = No hi ha fluxos per mostrar
matrix-nothing-shown-note = Canvieu la cerca o els filtres per veure més fluxos.
matrix-empty = No hi ha fluxos per connectar
matrix-empty-note = Els fluxos dels talkers i dels listeners es troben aquí un cop llegides les entitats que els tenen.
matrix-all-streams = Tots els fluxos
matrix-connectable-only = Amaga el que no es pot connectar
matrix-none-hidden = Tots els fluxos que es mostren es poden connectar
matrix-hidden = { $count ->
    [one] { $count } flux amagat
    [many] { $count } fluxos amagats
   *[other] { $count } fluxos amagats
}
matrix-own = Les sortides d’una entitat no es connecten a les seves pròpies entrades.
matrix-working = S’hi està treballant.
matrix-waiting-change = S’està esperant l’últim canvi en aquesta entrada.
matrix-connected = Connectat i rebent. Feu clic per desconnectar.
matrix-bound-waiting = Vinculada, esperant el flux del talker. Feu clic per desconnectar.
matrix-bound-failed = Vinculada, però ha fallat la reserva del talker: { $reason }. Feu clic per desconnectar.
matrix-bound-formats-differ = Vinculada, però els formats no coincideixen: el talker envia { $sent } i l’entrada està configurada a { $set }. Feu clic per desconnectar.
matrix-formats-match = Els formats coincideixen ({ $format }). Feu clic per connectar.
matrix-format-must-change = L’entrada accepta { $sent } però està configurada a { $set }, així que potser no sonarà fins que no se’n canviï el format. Feu clic per triar un format.
matrix-format-input = Canvia l’entrada a
matrix-format-both = Canvia tots dos a
matrix-format-talker-busy = El talker transmet a altres listeners, així que el seu format no canvia.
matrix-format-anyway = Connecta sense canviar els formats
matrix-incompatible = L’entrada no accepta { $sent }. Està configurada a { $set }.
matrix-group-none = No connectat. Desplegueu per connectar els fluxos un per un.
matrix-group-connected = { $count ->
    [one] { $count } connectat. Desplegueu per veure’l.
    [many] { $count } connectats. Desplegueu per veure’ls un per un.
   *[other] { $count } connectats. Desplegueu per veure’ls un per un.
}
matrix-outputs-expand = { $count ->
    [one] { $count } sortida de flux. Feu clic a la fletxa per desplegar, o al nom per inspeccionar l’entitat.
    [many] { $count } sortides de flux. Feu clic a la fletxa per desplegar, o al nom per inspeccionar l’entitat.
   *[other] { $count } sortides de flux. Feu clic a la fletxa per desplegar, o al nom per inspeccionar l’entitat.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } sortida de flux. Feu clic a la fletxa per plegar, o al nom per inspeccionar l’entitat.
    [many] { $count } sortides de flux. Feu clic a la fletxa per plegar, o al nom per inspeccionar l’entitat.
   *[other] { $count } sortides de flux. Feu clic a la fletxa per plegar, o al nom per inspeccionar l’entitat.
}
matrix-inputs-expand = { $count ->
    [one] { $count } entrada de flux. Feu clic a la fletxa per desplegar, o al nom per inspeccionar l’entitat.
    [many] { $count } entrades de flux. Feu clic a la fletxa per desplegar, o al nom per inspeccionar l’entitat.
   *[other] { $count } entrades de flux. Feu clic a la fletxa per desplegar, o al nom per inspeccionar l’entitat.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } entrada de flux. Feu clic a la fletxa per plegar, o al nom per inspeccionar l’entitat.
    [many] { $count } entrades de flux. Feu clic a la fletxa per plegar, o al nom per inspeccionar l’entitat.
   *[other] { $count } entrades de flux. Feu clic a la fletxa per plegar, o al nom per inspeccionar l’entitat.
}
matrix-stream-inspect = { $detail } Feu clic per inspeccionar { $entity }.
matrix-point = Apunteu a una cel·la
matrix-point-note = per veure’n el talker i el listener i si els seus formats coincideixen.
matrix-legend-waiting = Vinculada, esperant el flux
matrix-legend-trouble = Vinculada, alguna cosa no va bé
matrix-legend-open = Es pot connectar
matrix-legend-change = Cal canviar abans el format de l’entrada
matrix-legend-incompatible = Formats incompatibles
matrix-talker-outputs = Sortides dels talkers
matrix-listener-inputs = Entrades dels listeners

common-thousands-separator = {"."}
common-decimal-separator = {","}

## Diagnostics

diag-since-start = Comptat des que es va iniciar l’entitat.
diag-stream-input = Entrada de flux
diag-stream-output = Sortida de flux
diag-locked = { $count ->
    [0] no enganxat
    [1] enganxat una vegada
    [2] enganxat dues vegades
   *[other] enganxat { $number } vegades
}
diag-lost-lock = { $count ->
    [0] sense pèrdues d’enganxament
    [1] enganxament perdut una vegada
    [2] enganxament perdut dues vegades
   *[other] enganxament perdut { $number } vegades
}
diag-frames-in = { $count ->
    [one] { $number } trama rebuda
    [many] { $number } trames rebudes
   *[other] { $number } trames rebudes
}
diag-frames-out = { $count ->
    [one] { $number } trama enviada
    [many] { $number } trames enviades
   *[other] { $number } trames enviades
}
diag-media-locked = { $count ->
    [0] no enganxat al rellotge de mitjans
    [1] enganxat al rellotge de mitjans una vegada
    [2] enganxat al rellotge de mitjans dues vegades
   *[other] enganxat al rellotge de mitjans { $number } vegades
}
diag-lost-media-lock = { $count ->
    [0] sense pèrdues d’enganxament al rellotge de mitjans
    [1] enganxament al rellotge de mitjans perdut una vegada
    [2] enganxament al rellotge de mitjans perdut dues vegades
   *[other] enganxament al rellotge de mitjans perdut { $number } vegades
}
diag-interrupted = { $count ->
    [0] no interromput
    [1] interromput una vegada
    [2] interromput dues vegades
   *[other] interromput { $number } vegades
}
diag-out-of-sequence = { $count ->
    [one] { $number } trama fora de seqüència
    [many] { $number } trames fora de seqüència
   *[other] { $number } trames fora de seqüència
}
diag-media-resets = { $count ->
    [one] { $number } reinici de mitjans
    [many] { $number } reinicis de mitjans
   *[other] { $number } reinicis de mitjans
}
diag-timestamps-uncertain = { $count ->
    [0] sense marques de temps incertes
    [1] marques de temps incertes una vegada
    [2] marques de temps incertes dues vegades
   *[other] marques de temps incertes { $number } vegades
}
diag-no-timestamp = { $count ->
    [one] { $number } trama sense marca de temps
    [many] { $number } trames sense marca de temps
   *[other] { $number } trames sense marca de temps
}
diag-unsupported-format = { $count ->
    [one] { $number } trama en un format no admès
    [many] { $number } trames en un format no admès
   *[other] { $number } trames en un format no admès
}
diag-late = { $count ->
    [one] { $number } trama amb retard
    [many] { $number } trames amb retard
   *[other] { $number } trames amb retard
}
diag-early = { $count ->
    [one] { $number } trama abans d’hora
    [many] { $number } trames abans d’hora
   *[other] { $number } trames abans d’hora
}
diag-started = { $count ->
    [0] no iniciat
    [1] iniciat una vegada
    [2] iniciat dues vegades
   *[other] iniciat { $number } vegades
}
diag-stopped = { $count ->
    [0] no aturat
    [1] aturat una vegada
    [2] aturat dues vegades
   *[other] aturat { $number } vegades
}
diag-reservation-failed = ha fallat la reserva del talker: { $reason }
diag-latency = { $microseconds } µs de latència acumulada

## AVB Lite

lite-active = Actiu
lite-active-untagged = Actiu, sense etiqueta
lite-active-vlan = Actiu, VLAN { $vlan }
lite-capable = Compatible
lite-mode = Mode
lite-mode-capable = AVB, compatible amb AVB Lite
lite-because = Motiu
lite-fallback-none = no s’ha indicat cap motiu
lite-fallback-endpoint = ha arribat la declaració d’un altre terminal, així que no hi ha cap commutador AVB entre ells
lite-fallback-unanswered = nou sol·licituds de peer delay han quedat sense resposta
lite-fallback-responders = dos o més dispositius han respost a una mateixa sol·licitud de peer delay, així que el commutador no és compatible amb AVB
lite-fallback-configured = ho ha configurat l’operador o un controlador
lite-fallback-other = un motiu que el perfil no preveu
lite-other-profile = Un altre perfil
lite-ptp-domain = { $profile }, domini { $domain }
lite-offset = Decalatge
lite-offset-from = { $offset } respecte de { $grandmaster }
lite-media-vlan = VLAN de mitjans
lite-untagged = Sense etiqueta
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Fins a { $count } listener per flux, després multicast
    [many] Fins a { $count } listeners per flux, després multicast
   *[other] Fins a { $count } listeners per flux, després multicast
}
lite-link = Enllaç
lite-bandwidth = Amplada de banda
lite-egress-of = { $used } de { $link }, { $share }
lite-egress-of-assumed = { $used } de { $link }, { $share }, suposant un enllaç gigabit
lite-egress-reported = Segons el recompte que fa l’entitat dels seus fluxos admesos.
lite-egress-worked-out = Calculat a partir dels formats de les seves sortides de flux connectades.
lite-alarm-offset = Decalatge PTP de { $offset }, més dels 50 µs que permet AVB Lite
lite-alarm-egress = Trànsit de sortida al { $share } de l’enllaç, més del { $limit } que poden ocupar els fluxos

## AVB Wireless

wireless-station = Estació
wireless-access-point = Punt d’accés
wireless-role = Rol
wireless-mode = Mode
wireless-time = Temps
wireless-mode-a-ftm = Mode A, 802.1AS sobre FTM
wireless-mode-a-tm = Mode A, 802.1AS sobre TM
wireless-mode-b = Mode B, a partir de les balises
wireless-no-time = Sense temps
wireless-other-mode = Un mode que el perfil no preveu
wireless-locked = Enganxat
wireless-holdover = En holdover
wireless-not-locked = No enganxat
wireless-row-locked = Estació, enganxada
wireless-row-holdover = Estació, en holdover
wireless-row-not-locked = Estació, no enganxada
wireless-row-access-point = { $count ->
    [one] Punt d’accés, { $count } estació
    [many] Punt d’accés, { $count } estacions
   *[other] Punt d’accés, { $count } estacions
}
wireless-link = Enllaç
wireless-channel = canal { $channel }
wireless-not-known = Desconegut
wireless-signal = Senyal
wireless-rate = Velocitat
wireless-ftm-valid = { $share } vàlides
wireless-rtt = anada i tornada { $rtt }
wireless-bursts = { $count ->
    [one] ràfegues de { $count } trama
    [many] ràfegues de { $count } trames
   *[other] ràfegues de { $count } trames
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = el punt d’accés concedeix ràfegues FTM que no són de tres ni de dues trames
wireless-reason-measurement = ni FTM ni TM amb el punt d’accés
wireless-reason-signaling = cap missatge Signaling gPTP-capable del punt d’accés
wireless-reason-other = un motiu que el perfil no preveu
wireless-servo = Error del servo
wireless-stations = Estacions
wireless-station-count = { $count ->
    [one] { $count } estació
    [many] { $count } estacions
   *[other] { $count } estacions
}
wireless-no-ftm = sense FTM
wireless-unserved = Listeners no atesos
wireless-frame-budget = Pressupost de trames
wireless-frame-use = { $used } de { $budget } trames per segon
wireless-stream-frames = Trames de flux
wireless-frames-of = { $readdressed } a les estacions, { $unmapped } sense listener, { $dropped } descartades, { $restored } de les estacions
wireless-class-a-allowed = Permesa
wireless-class-a-not-allowed = No permesa
wireless-alarm-not-locked = Temps Wi-Fi no enganxat al punt d’accés
wireless-alarm-holdover = Temps Wi-Fi en holdover, enganxament al punt d’accés perdut
wireless-alarm-unserved = { $count ->
    [one] { $count } listener al port Wi-Fi no atès, per sobre del límit d’unicast
    [many] { $count } listeners al port Wi-Fi no atesos, per sobre del límit d’unicast
   *[other] { $count } listeners al port Wi-Fi no atesos, per sobre del límit d’unicast
}

## Log

log-all = Tot
log-warnings = Avisos
log-pause = Pausa
log-resume = Reprèn
log-clear = Esborra
log-empty = Aquí apareixen totes les trames ATDECC que triib envia i rep, de la més recent a la més antiga.
log-none-match = Cap trama desada no coincideix amb el filtre.
log-frames = { $count ->
    [one] { $count } trama
    [many] { $count } trames
   *[other] { $count } trames
}
log-shown-of = { $shown } de { $all } trames
log-sent = Enviada
log-heard = Rebuda
log-not-decoded = No descodificada
log-warning-short = { $missing ->
    [one] El seu control_data_length indica { $missing } octet més enllà del final de la trama.
    [many] El seu control_data_length indica { $missing } octets més enllà del final de la trama.
   *[other] El seu control_data_length indica { $missing } octets més enllà del final de la trama.
}
log-warning-undecodable = No es pot descodificar: { $error }.
log-warning-long-acmp = Té la forma ACMP llarga, que una entitat Milan no pot enviar (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Mapatges de canals
mapping-inputs = Entrades
mapping-outputs = Sortides
mapping-port = port { $number }
mapping-fixed = fix
mapping-not-read = Encara no s’ha llegit.
mapping-no-clusters = Cap clúster.
mapping-no-streams = Cap flux d’àudio.
mapping-none = Cap mapatge.
mapping-not-mapped = No mapat
mapping-cluster-numbered = Clúster { $index }

## Presets

presets-note = Un preset desa les fonts de rellotge, les freqüències de mostreig, els formats de flux, els controls i les connexions de cada entitat. En recuperar-lo, només canvia el que és diferent.
presets-none = Encara no hi ha cap preset desat.
presets-connections = { $count ->
    [one] { $count } connexió
    [many] { $count } connexions
   *[other] { $count } connexions
}
presets-recall = Recupera
presets-delete = Suprimeix
presets-no-place = No hi ha on desar els presets: no es coneix la carpeta personal.
presets-undeletable = No s’ha pogut suprimir { $path }: { $error }.
presets-saved = { $count ->
    [one] S’ha desat «{ $name }» amb { $count } entitat.
    [many] S’ha desat «{ $name }» amb { $count } entitats.
   *[other] S’ha desat «{ $name }» amb { $count } entitats.
}
presets-nothing-differs = No hi ha res diferent de «{ $name }».
presets-recalling = { $count ->
    [one] S’està recuperant «{ $name }»: { $count } canvi.
    [many] S’està recuperant «{ $name }»: { $count } canvis.
   *[other] S’està recuperant «{ $name }»: { $count } canvis.
}
presets-missing = { $report } Entitats absents o no llegides: { $missing }.
presets-deleted = S’ha suprimit «{ $name }».
presets-host-note = També desa els talkers i listeners propis d’aquest ordinador, i els torna a iniciar en recuperar-lo.
presets-host-endpoints = { $count } en aquest ordinador
presets-starting-host = S’estan iniciant els talkers i listeners d’aquest ordinador per a «{ $name }»; la resta seguirà quan tornin.

## Controls

control-numbered = Control { $index }
control-not-shown = No es mostra aquí
control-option = Opció { $number }

## Network errors

network-permission = triib necessita permís per enviar i rebre trames Ethernet en brut.
network-needs-npcap = triib necessita Npcap per enviar i rebre trames Ethernet en brut.
network-npcap-administrators = Npcap només permet als administradors enviar i rebre trames Ethernet en brut. Executeu triib com a administrador o torneu a instal·lar Npcap sense l’opció de només administradors.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own talkers and listeners

host-add-talker = Afegeix un talker
host-add-listener = Afegeix un listener
host-show-mine = Mostra només els talkers i listeners propis d’aquest ordinador
host-show-all = Mostra totes les entitats
host-new-talker = Talker de l’amfitrió { $number }
host-new-listener = Listener de l’amfitrió { $number }
host-failed = No s’ha pogut afegir a aquest ordinador: { $reason }
host-needs-clock = Els talkers i listeners propis d’aquest ordinador necessiten una interfície amb fil amb rellotge de maquinari PTP
host-no-ptp4l = ptp4l no respon, de manera que els fluxos d’aquest ordinador no poden mantenir l’hora gPTP
host-elsewhere = triib-endpointd s’executa a { $interface } per a un altre usuari o com a root, de manera que els talkers i listeners d’aquest ordinador s’executen allà, no aquí
host-foreign-mrp = Un altre programa emet declaracions MSRP o MVRP a { $interface } des de l’adreça d’aquest ordinador i pot retirar el que necessiten els fluxos d’aquest ordinador
host-alarm-foreign-mrp = Un altre programa d’aquest ordinador emet declaracions MSRP o MVRP a { $interface }
host-state = Estat
host-streaming = Emetent
host-waiting = Esperant un listener
host-listening = A l’escolta
host-bound = Vinculat, esperant el talker
host-unbound = Sense vincular
host-audio-from = Àudio des de
host-audio-to = Àudio cap a
host-channels = Canals
host-silence = Silenci
host-tone = To de prova
host-nowhere = Enlloc
host-default-device = Dispositiu per defecte
host-remove = Elimina d’aquest ordinador
