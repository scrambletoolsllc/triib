# triib's interface text in Hungarian. Term choices: docs/glossary/hu.md.

## Language

language-name = Magyar

## Common

common-close = Bezárás
common-more = Továbbiak
common-keep-toolbar-shown = Eszköztár állandó megjelenítése
common-auto-hide-toolbar = Eszköztár automatikus elrejtése

## Settings

settings-title = Beállítások
settings-general = Általános
settings-appearance = Megjelenés
settings-language = Nyelv
settings-language-system = Rendszer alapértelmezése: { $language }
settings-language-note = A szövegmezők a rendszer beviteli nyelvét használják.
settings-appearance-system = Rendszer
settings-appearance-light = Világos
settings-appearance-dark = Sötét
settings-colors = Színek
settings-system-accent = A rendszer kiemelőszínének használata
settings-accent-picked = Az alábbi színből származnak a triib színei.
settings-accent-omarchy = Az Omarchy témából: { $theme }.
settings-accent-desktop = Az asztal kiemelőszínéből.
settings-accent-none = Az asztalnak nincs kiemelőszíne, ezért az alábbi szín érvényes.
settings-motion = Mozgás
settings-animations = Animációk
settings-animations-note = Rugózás és csúszás, ahogy a dolgok változnak.
settings-animations-reduced = Az asztal csökkentett mozgást kér, ezért a triib mozdulatlan marad.
settings-outside-control = Külső vezérlés
settings-allow-outside-control = Külső vezérlés engedélyezése
settings-allow-outside-control-note = Az ügynökök, például a Claude Code, a triib --mcp segítségével láthatják és módosíthatják az AVB-hálózatát. A triib minden új ügynök előtt rákérdez.
settings-allowed-agents = Engedélyezve: { $agents }
settings-forget-agents = Elfelejtés
settings-storage = Tárolás
settings-storage-presets = Presetek mappája
settings-storage-entities = Entitásfájl
settings-storage-apply = Alkalmaz
settings-storage-choose = Tallózás…
settings-storage-note = A régi helyen tárolt presetek ott maradnak; helyezze át őket, ha továbbra is használni szeretné őket. A triib beállításainak helye: { $file }.
settings-storage-entities-note = Egy még nem létező entitásfájl a használatban lévő másolataként indul, és a triib-endpointd újraindul vele.
dialog-choose-presets = Válassza ki a presetek mappáját
dialog-choose-entities = Válassza ki az entitásfájl mappáját
settings-full-path = Teljes elérési utat adjon meg, például ~/Documents/triib.
settings-path-is-folder = Ez mappa, nem fájl: { $path }
settings-folder-missing = Nincs ilyen mappa: { $path }. Előbb hozza létre, vagy válasszon egy másikat.
settings-path-is-file = Ez fájl, nem mappa: { $path }
settings-cannot-write = A triib nem tud írni ide: { $path } – { $error }.
app-file-dialog-failed = Nem sikerült megjeleníteni a fájlválasztó ablakot: { $error }
agent-prompt-title = Engedélyezi, hogy { $agent } vezérelje a triib alkalmazást?
agent-prompt-body = { $agent } szeretné használni a triib külső vezérlését: látni és módosítani a hálózata entitásait, streameket csatlakoztatni, formátumokat, órajeleket és neveket beállítani. A külső vezérlést a Beállításokban kapcsolhatja ki.
agent-prompt-allow = Engedélyezés
agent-prompt-deny = Tiltás

common-cancel = Mégse
common-save = Mentés
common-not-set = Nincs megadva
common-unnamed = Névtelen
common-none = Nincs
common-mac-address = MAC-cím
common-list-separator = {", "}

## Network interfaces

interface-up = aktív
interface-link-down = nincs link
interface-wireless = vezeték nélküli
interface-hardware-clock = hardveróra
interface-hardware-clock-named = { $clock } hardveróra
interface-virtual = virtuális

## Toolbar

toolbar-choose-interface = Válasszon interfészt
toolbar-interface = Hálózati interfész
toolbar-filter-interfaces = Interfészek szűrése
toolbar-hide-virtual = Virtuális interfészek elrejtése
toolbar-hide-wireless = Vezeték nélküli interfészek elrejtése
toolbar-hide-unclocked = Hardveróra nélküli interfészek elrejtése
toolbar-connections = Kapcsolatok
toolbar-network = Hálózat
toolbar-entities = Entitások
toolbar-rediscover = Minden entitás felkérése, hogy hirdesse meg magát
toolbar-rescan = Minden entitás törlése és újraolvasása
toolbar-search = Keresés az entitások és streamek között
toolbar-presets = Presetek
toolbar-log = Napló
toolbar-inspector = Vizsgáló
toolbar-settings = Beállítások

## The network's state, in place of a view

state-no-interface = Nincs interfész
state-no-interface-note = Válassza ki az AVB-hálózatra csatlakozó interfészt az entitások felderítéséhez.
state-starting = Indítás
state-starting-note = Megnyitás: { $interface }.
state-listening = Figyelés
state-listening-note = Az entitások itt jelennek meg, amint meghirdetik magukat ezen az interfészen: { $interface }.
state-permission-needed = Engedély szükséges
state-npcap-needed = Npcap szükséges
state-get-npcap = Npcap letöltése
state-copy-command = A parancs másolása
state-cannot-use = Nem használható: { $interface }
state-try-again = Újra

## Entity list

entities-none-yet = Még nincsenek entitások
entities-none-yet-note = A hálózat összes entitása a szerepköreivel, SR-osztályaival és órájával.

## Inspector

inspector-title = Vizsgáló
inspector-entity = Entitás
inspector-streams = Streamek
inspector-controls = Vezérlőelemek
inspector-diagnostics = Diagnosztika
inspector-descriptors = Leírók
inspector-select = Válasszon ki egy entitást a részletek megtekintéséhez.
inspector-offline = Az entitás offline: { $entity }.
inspector-heard = Nem hirdeti magát: a triib az általa küldött parancsokból ismerte fel, és a listán tartja, amíg 5 perc parancs nélkül el nem telik.
inspector-rename = Átnevezés
inspector-name = Név
inspector-identify = Azonosítás
inspector-model-not-read = Az entitásmodell még nincs beolvasva.
inspector-no-streams = Nincsenek streamek.
inspector-no-controls = Nincs megjeleníthető vezérlőelem.
inspector-no-diagnostics = Nem jelentett interfészeket vagy számlálókat.
inspector-reading = Leírók olvasása, eddig { $count }.
inspector-read-failed = Nem sikerült beolvasni az entitásmodellt: { $reason }.

entity-section = Entitás
entity-name = Név
entity-triib-controller = triib-vezérlő
entity-triib-on = triib ezen: { $host }
entity-triib-here = triib ezen a számítógépen
entity-controller-on = Vezérlő ezen: { $host }
entity-controller-here = Vezérlő ezen a számítógépen
entity-controller-at = Vezérlő itt: { $mac }
entity-group = Csoport
entity-product = Termék
entity-firmware = Firmware
entity-serial-number = Sorozatszám
entity-configuration = Konfiguráció
entity-configuration-of = { $name } ({ $number }/{ $count })
entity-configuration-holds = { $inputs } be, { $outputs } ki, { $channels } csatornás streamek
entity-configuration-holds-mixed = { $inputs } be, { $outputs } ki
entity-configuration-numbered = { $number }. konfiguráció
entity-milan = Milan
entity-media-clock = Médiaórajel
entity-clock-domain = Órajeltartomány
entity-sampling-rate = Mintavételi frekvencia
clock-source-numbered = Forrás { $index }
rate-pull = pull { $pull }

stream-format = Formátum
stream-id = Stream-ID
stream-destination = Cél
stream-max-transit = Max. átfutási idő
stream-listeners = Hallgatók
stream-listener-gone = { $entity } { $input }. bemenet, nem érhető el
stream-disconnect-listener = Leválasztás

avb-interfaces = AVB-interfészek
avb-interface = Interfész
avb-interface-clock-identity = Óraazonosító
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, tartomány: { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Fut
avb-interface-none-reported = Nincs jelentve
avb-interface-path = Útvonal
avb-interface-own-grandmaster = Saját maga a grandmaster
avb-interface-hops = { $count ->
    [one] { $count } ugrás a grandmastertől
   *[other] { $count } ugrás a grandmastertől
}
avb-interface-link-up = Link létrejött
avb-interface-link-down = Link megszakadt
avb-interface-grandmaster-changes = Grandmaster-váltások
avb-interface-frames-sent = Elküldött keretek
avb-interface-frames-received = Fogadott keretek
avb-interface-crc-errors = CRC-hibák

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } leírótípus
   *[other] { $count } leírótípus
}
tree-clock = Órajel
tree-clock-source-from = { $kind }, forrás: { $location } { $index }
tree-clock-domain-using = Használt forrás: { $source }
tree-clusters = { $count ->
    [one] { $count } klaszter
   *[other] { $count } klaszter
}
tree-maps = { $count ->
    [one] { $count } kiosztás
   *[other] { $count } kiosztás
}

advert-not-advertised = Nincs meghirdetve
advert-identity = Identitás
advert-entity-id = Entitás-ID
advert-entity-model = Entitásmodell
advert-roles = Szerepkörök
advert-talker = Talker
advert-listener = Listener
advert-clock = Óra
advert-btc = BTC
advert-gptp-domain = gPTP-tartomány
advert-sr-classes = SR-osztályok
advert-indexes = Entitásmodell-indexek
advert-identify-control = Azonosítási vezérlőelem
advert-avb-interface = AVB-interfész
advert-advertising = Hirdetés
advert-valid-time = Érvényességi idő
advert-available-index = Elérhetőségi index
advert-association = Társítás
advert-capabilities = Képességek

## Status bar

status-entities = { $count ->
    [one] { $count } entitás
   *[other] { $count } entitás
}
status-not-discovering = Nincs felderítés
status-discovering = Felderítés
status-discovering-as = Felderítés, saját ID: { $controller }
status-stopped = Hiba miatt leállt
status-alarm = Riasztás
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } és még { $count }

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = vezérlő
role-none = nincs szerepkör
classes-a-and-b = A és B
clock-no-gptp = Nincs gPTP

read-not-read = Nincs beolvasva
read-reading = Olvasás, eddig { $count }
read-ready-unreadable = Kész, { $count } nem olvasható
read-ready-cached = Kész, gyorsítótárból
read-ready = Kész
read-failed = Sikertelen: { $reason }

milan-no = Nem
milan-before-1-3 = 1.3 előtti
milan-certified = { $version }, tanúsítva: { $certification }
milan-not-certified = { $version }, nem tanúsított

outcome-status = állapot: { $status }
outcome-no-response = nincs válasz
outcome-not-possible = nem lehetséges
outcome-connect = A csatlakoztatás nem sikerült ({ $talker } → { $listener }): { $reason }.
outcome-disconnect = A leválasztás nem sikerült ({ $listener }): { $reason }.
outcome-disconnect-listener = { $listener } nem választható le erről: { $talker }: { $reason }.
outcome-identify = Az azonosítás nem sikerült ({ $entity }): { $reason }.
outcome-rename = Az átnevezés nem sikerült ({ $what } → „{ $name }”): { $reason }.
outcome-rename-group = A csoport átnevezése nem sikerült ({ $entity } → „{ $name }”): { $reason }.
outcome-format-streaming = A formátum nem módosítható ({ $stream }): a stream fut. Előbb válassza le.
outcome-format = A formátum módosítása nem sikerült ({ $stream }): { $reason }.
outcome-max-transit-time = A max. átfutási idő módosítása nem sikerült ({ $stream }): { $reason }.
outcome-max-transit-time-streaming = A max. átfutási idő módosítása nem sikerült ({ $stream }): a stream fut. Előbb válassza le.
outcome-sampling-rate = A mintavételi frekvencia módosítása nem sikerült ({ $entity }): { $reason }.
outcome-clock-source = Az órajelforrás módosítása nem sikerült ({ $entity }): { $reason }.
outcome-configuration = A konfiguráció nem módosítható ({ $entity }): { $reason }.
outcome-configuration-streaming = { $entity } megtartja a konfigurációját, amíg a streamjei futnak: előbb válassza le őket.
outcome-map = A csatorna kiosztása nem sikerült ({ $entity }): { $reason }.
outcome-unmap = A csatornakiosztás megszüntetése nem sikerült ({ $entity }): { $reason }.
outcome-control = A beállítás nem sikerült ({ $entity }, „{ $control }”): { $reason }.
outcome-control-numbered = A beállítás nem sikerült ({ $entity }, vezérlőelem { $index }): { $reason }.

stream-not-connected = Nincs csatlakoztatva
stream-from = Forrás: { $stream }
stream-from-receiving = Forrás: { $stream }, vétel
stream-from-waiting = Forrás: { $stream }, várakozás a talkerre
stream-from-failed = Forrás: { $stream }, a talker foglalása sikertelen: { $reason }
stream-sending-to = Küldés ide: { $destination }

failure-no-response = nem válaszolt
failure-refused = elutasította: { $status }
failure-malformed = a válasza nem dekódolható
failure-on-this-computer = ezen a számítógépen fut; olvassa be egy másikról

msrp-failure-1 = nincs elég sávszélesség
msrp-failure-2 = a hídnak nincs elég erőforrása
msrp-failure-3 = nincs elég sávszélesség a forgalmi osztály számára
msrp-failure-4 = a stream-ID-t egy másik talker használja
msrp-failure-5 = a célcím már használatban van
msrp-failure-6 = egy magasabb rangú stream kiszorította
msrp-failure-7 = a jelentett késleltetés megváltozott
msrp-failure-8 = a kimenő port nem AVB-képes
msrp-failure-9 = használjon másik célcímet
msrp-failure-10 = elfogytak az MSRP-erőforrások
msrp-failure-11 = elfogytak az MMRP-erőforrások
msrp-failure-12 = a célcím nem tárolható
msrp-failure-13 = a prioritás nem SR-osztály-prioritás
msrp-failure-14 = a keretek túl nagyok az átviteli közeghez
msrp-failure-15 = elérte a port fan-in korlátját
msrp-failure-16 = egy regisztrált stream első értéke megváltozott
msrp-failure-17 = a VLAN blokkolva van a kimenő porton
msrp-failure-18 = a VLAN-címkézés le van tiltva a kimenő porton
msrp-failure-19 = nem egyező SR-osztály-prioritás
msrp-failure-unknown = ismeretlen ok
msrp-failure-at = { $reason } (híd: { $bridge })

## Entity list columns

column-vendor = Gyártó
column-model = Modell
column-state = Állapot
column-entity-model-id = Entitásmodell-ID
column-talker-streams = Talker-streamek
column-listener-streams = Listener-streamek
column-avb-lite = AVB Lite
column-egress = Kimenő forgalom
column-wireless = Vezeték nélküli

## Settings file

settings-no-place = A beállításokat nincs hová menteni: a saját mappa nem ismert.
settings-unusable = A fájl nem használható ({ $path }): { $error }.
settings-unsaved = A mentés nem sikerült ({ $path }): { $error }.

column-remove = Oszlop eltávolítása
column-move-left = Mozgatás balra
column-move-right = Mozgatás jobbra
column-add = Oszlop hozzáadása
common-percent = { $value }%

## Network view

netmap-empty = Még nincs megjeleníthető hálózat
netmap-empty-note = Az entitások akkor jelennek meg itt, ha már be lettek olvasva, és jelezték, hol helyezkednek el a gPTP-fán.
netmap-focus-clock-path = { $name } órajelútja
netmap-focus-streams = { $name } streamjei
netmap-showing = Kiemelve: { $what }
netmap-devices = { $count ->
    [one] { $count } eszköz
   *[other] { $count } eszköz
}
netmap-bridges = { $count ->
    [one] { $count } híd
   *[other] { $count } híd
}
netmap-show-map = Térkép megjelenítése
netmap-show-details = Részletek megjelenítése
stream-numbered = Stream { $index }
netmap-bridge = Híd
netmap-access-point = Hozzáférési pont
netmap-device = Eszköz
netmap-this-computer = Ez a számítógép
netmap-connected = Csatlakoztatva
netmap-advertised = Meghirdetve, nincs kész listener
netmap-advertised-off-tree = Meghirdetve, nincs kész listener (nincs a gPTP-fán: { $listener })
netmap-failed-at = A foglalás sikertelen (híd: { $bridge }): { $reason }
netmap-failed = A foglalás sikertelen: { $reason }
netmap-no-bridge-on = Nincs észlelt híd ({ $interface })
netmap-cannot-listen-on = A gPTP nem figyelhető ({ $interface })
netmap-on-this-computer = Ezen a számítógépen
netmap-path-not-reported = Az útvonal nincs jelentve
netmap-gptp-not-reported = A gPTP nincs jelentve
netmap-off-tree = Nincs a gPTP-fán
netmap-off-ptp = Nincs a PTP-fán
netmap-not-lite = AVB Lite nélkül
netmap-lite-not-reported = Az AVB Lite nincs jelentve
netmap-synced = Szinkronban
netmap-not-synced = Nincs szinkronban
netmap-triib-on = triib ({ $interface })
netmap-through-count = { $count } áthaladó
netmap-out = { $count } kimenő
netmap-in = { $count } bejövő
netmap-failed-count = { $count } sikertelen
netmap-advertised-only = Csak meghirdetve
netmap-failed-state = Sikertelen
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Nincs a gPTP-fán: saját maga a grandmaster
netmap-apart-no-path = Az útvonalát nem jelentette; a követett grandmaster: { $grandmaster }
netmap-apart-unreported = Nem jelentette a gPTP-állapotát
netmap-apart-no-neighbor = Nincs észlelt híd a számítógép interfészén
netmap-apart-cannot-listen = Ez a számítógép nem tudja figyelni a gPTP-t az interfészén
netmap-apart-on-this-computer = Ezen a számítógépen fut; a gPTP-állapotához olvassa be egy másik számítógépről
netmap-apart-not-lite = Nem futtat AVB Lite-ot, így nem követi a grandmastert
netmap-apart-lite-unreported = Semmit nem jelent az AVB Lite-ról, így nem tudni, mit követ
netmap-clock-tree = Órajelfa
netmap-no-grandmaster = Nincs észlelt grandmaster
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Figyelmet igényel
netmap-nodes-below = Alatta lévő csomópontok
netmap-bridges-below = Alatta lévő hidak
netmap-clock-path = Órajelút
netmap-hops = Ugrások a grandmastertől
netmap-link-delay = Linkkésleltetés
netmap-bridge-port = Hídport
netmap-link-drops = Linkvesztések
netmap-synced-to-grandmaster = Szinkronban a grandmasterrel
netmap-host-no-gptp = Nincs szinkronban: ez a számítógép nem futtat gPTP-t
netmap-link-no-gptp = Nincs szinkronban: a linkjén nem fut gPTP
netmap-ptp-offset-high = Nincs szinkronban: { $offset } eltérés a grandmastertől, több az AVB Lite által megengedett 50 µs-nál
netmap-ptp-no-offset = Nincs szinkronban: nem mért eltérést a grandmastertől
netmap-audio = Audio
netmap-media-clock-streams = Médiaórajel-streamek
netmap-audio-streams = Audiostreamek
netmap-bound = { $count } hozzárendelt
netmap-flowing = Áramlik
netmap-advertised-state = Meghirdetve
netmap-media-clock-stream = Médiaórajel-stream
netmap-audio-stream = Audiostream
netmap-reaches = Eddig jut
netmap-passing-count = { $count ->
    [one] { $count } áthaladó stream
   *[other] { $count } áthaladó stream
}
netmap-through = Áthaladó
netmap-passing-through = Áthalad
netmap-sending = Küldés
netmap-receiving = Vétel
netmap-problems = Problémák
netmap-help-back = Kattintson a háttérre az áttekintéshez való visszatéréshez.
netmap-help-stream = Kattintson egy streamre a vizsgálatához, vagy a háttérre az áttekintéshez való visszatéréshez.
netmap-help-ptp = AVB Lite esetén az órajel végponttól végpontig jut el a grandmastertől minden eszközig, olyan switcheken át, amelyek nem vesznek részt benne, ezért ezek nem jelennek meg. Egy eszköz addig van szinkronban, amíg 50 µs-on belül követi a grandmastert. Kattintson egy eszközre vagy a vezetékére az órajele vizsgálatához; a háttérre kattintva megszüntetheti a kijelölést.
netmap-help-clock = Az órajel a grandmastertől minden hídon át eljut a fa minden csomópontjához. A szaggatott szürke vonal olyan link, amelyen nem fut gPTP. Kattintson egy eszközre vagy a vezetékére az órajelútja vizsgálatához; a háttérre kattintva megszüntetheti a kijelölést.
netmap-help-media-clock = Csak a médiaórajel-streamek (CRF), az audióval azonos módon rajzolva: streamenként egy vezeték, talkerenként színezve. Kattintson egy vezetékre a streamje vizsgálatához, vagy egy eszközre a streamjei megtekintéséhez; a háttérre kattintva megszüntetheti a kijelölést.
netmap-help-audio = Minden streamnek saját vezetéke van, amely belép minden hídba, amelyen áthalad, majd kilép belőle. A szín a talkert jelöli: minden talkernek saját színárnyalata van, és a streamjei ennek változatai. A mozgó pontok azt jelzik, hogy áramlik az audio; az álló piros vonal sikertelen foglalást, az álló szürke vonal pedig olyan meghirdetett streamet jelöl, amelyhez nincs kész listener; mindkettő ott ér véget, ahol a foglalás. A középső oszlop eszközei közvetlenül a grandmaster hídjához csatlakoznak. Kattintson egy vezetékre a streamje vizsgálatához, vagy egy eszközre a streamjei megtekintéséhez; a háttérre kattintva megszüntetheti a kijelölést.

## Connections

matrix-nothing-shown = Nincs megjeleníthető stream
matrix-nothing-shown-note = Módosítsa a keresést vagy a szűrőket, hogy több stream látsszon.
matrix-empty = Nincs csatlakoztatható stream
matrix-empty-note = A talker- és listener-streamek itt találkoznak, amint a hozzájuk tartozó entitások be vannak olvasva.
matrix-all-streams = Összes stream
matrix-connectable-only = A nem csatlakoztathatók elrejtése
matrix-none-hidden = Minden látható stream csatlakoztatható
matrix-hidden = { $count ->
    [one] { $count } rejtett stream
   *[other] { $count } rejtett stream
}
matrix-own = Egy entitás kimenetei nem csatlakoznak a saját bemeneteihez.
matrix-working = Folyamatban.
matrix-waiting-change = Várakozás a bemenet utolsó módosítására.
matrix-connected = Csatlakoztatva, vétel folyik. Kattintson a leválasztáshoz.
matrix-bound-waiting = Hozzárendelve, várakozás a talker streamjére. Kattintson a leválasztáshoz.
matrix-bound-failed = Hozzárendelve, de a talker foglalása sikertelen: { $reason }. Kattintson a leválasztáshoz.
matrix-bound-formats-differ = Hozzárendelve, de a formátumok eltérnek: a talker által küldött formátum { $sent }, a bemenet beállítása { $set }. Kattintson a leválasztáshoz.
matrix-formats-match = A formátumok egyeznek ({ $format }). Kattintson a csatlakoztatáshoz.
matrix-format-must-change = A bemenet fogadni tudja ezt: { $sent }, de a beállítása { $set }, így a formátuma módosításáig lehet, hogy nem szól. Kattintson egy formátum kiválasztásához.
matrix-format-input = A bemenet módosítása erre
matrix-format-both = Mindkettő módosítása erre
matrix-format-talker-busy = A talker más listenereknek is streamel, ezért a formátuma marad.
matrix-format-anyway = Csatlakoztatás a formátumok módosítása nélkül
matrix-incompatible = A bemenet nem fogadja ezt: { $sent }. A beállítása: { $set }.
matrix-group-none = Nincs csatlakoztatva. Bontsa ki a streamek egyenkénti csatlakoztatásához.
matrix-group-connected = { $count } csatlakoztatva. Bontsa ki az egyenkénti megtekintéshez.
matrix-outputs-expand = { $count ->
    [one] { $count } streamkimenet. Kattintson a nyílra a kibontáshoz, a névre a vizsgálathoz.
   *[other] { $count } streamkimenet. Kattintson a nyílra a kibontáshoz, a névre a vizsgálathoz.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } streamkimenet. Kattintson a nyílra az összecsukáshoz, a névre a vizsgálathoz.
   *[other] { $count } streamkimenet. Kattintson a nyílra az összecsukáshoz, a névre a vizsgálathoz.
}
matrix-inputs-expand = { $count ->
    [one] { $count } streambemenet. Kattintson a nyílra a kibontáshoz, a névre a vizsgálathoz.
   *[other] { $count } streambemenet. Kattintson a nyílra a kibontáshoz, a névre a vizsgálathoz.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } streambemenet. Kattintson a nyílra az összecsukáshoz, a névre a vizsgálathoz.
   *[other] { $count } streambemenet. Kattintson a nyílra az összecsukáshoz, a névre a vizsgálathoz.
}
matrix-stream-inspect = { $detail } Kattintson a vizsgálathoz: { $entity }.
matrix-point = Mutasson egy cellára
matrix-point-note = a talkere, a listenere és a formátumok egyezésének megtekintéséhez.
matrix-legend-waiting = Hozzárendelve, várakozás a streamre
matrix-legend-trouble = Hozzárendelve, valami hibás
matrix-legend-open = Csatlakoztatható
matrix-legend-change = Előbb a bemenet formátumát kell módosítani
matrix-legend-incompatible = A formátumok nem egyeztethetők össze
matrix-talker-outputs = Talker-kimenetek
matrix-listener-inputs = Listener-bemenetek

common-thousands-separator = {" "}

## Diagnostics

diag-since-start = Az entitás indulása óta számolva.
diag-stream-input = Streambemenet
diag-stream-output = Streamkimenet
diag-locked = { $count ->
    [0] nem zárolódott
    [one] egyszer zárolódott
   *[other] { $number } alkalommal zárolódott
}
diag-lost-lock = { $count ->
    [0] nem vesztette el a zárolást
    [one] egyszer vesztette el a zárolást
   *[other] { $number } alkalommal vesztette el a zárolást
}
diag-frames-in = { $count ->
    [one] { $number } bejövő keret
   *[other] { $number } bejövő keret
}
diag-frames-out = { $count ->
    [one] { $number } kimenő keret
   *[other] { $number } kimenő keret
}
diag-media-locked = { $count ->
    [0] nem zárolódott a médiaórajelre
    [one] egyszer zárolódott a médiaórajelre
   *[other] { $number } alkalommal zárolódott a médiaórajelre
}
diag-lost-media-lock = { $count ->
    [0] nem vesztette el a médiaórajel-zárolást
    [one] egyszer vesztette el a médiaórajel-zárolást
   *[other] { $number } alkalommal vesztette el a médiaórajel-zárolást
}
diag-interrupted = { $count ->
    [0] nem szakadt meg
    [one] egyszer szakadt meg
   *[other] { $number } alkalommal szakadt meg
}
diag-out-of-sequence = { $count ->
    [one] { $number } soron kívüli keret
   *[other] { $number } soron kívüli keret
}
diag-media-resets = { $count ->
    [one] { $number } média-visszaállítás
   *[other] { $number } média-visszaállítás
}
diag-timestamps-uncertain = { $count ->
    [0] nem volt bizonytalan időbélyeg
    [one] egyszer volt bizonytalan időbélyeg
   *[other] { $number } alkalommal volt bizonytalan időbélyeg
}
diag-no-timestamp = { $count ->
    [one] { $number } keret időbélyeg nélkül
   *[other] { $number } keret időbélyeg nélkül
}
diag-unsupported-format = { $count ->
    [one] { $number } keret nem támogatott formátumban
   *[other] { $number } keret nem támogatott formátumban
}
diag-late = { $count ->
    [one] { $number } késve érkezett keret
   *[other] { $number } késve érkezett keret
}
diag-early = { $count ->
    [one] { $number } korán érkezett keret
   *[other] { $number } korán érkezett keret
}
diag-started = { $count ->
    [0] nem indult el
    [one] egyszer indult el
   *[other] { $number } alkalommal indult el
}
diag-stopped = { $count ->
    [0] nem állt le
    [one] egyszer állt le
   *[other] { $number } alkalommal állt le
}
diag-reservation-failed = a talker foglalása sikertelen: { $reason }
diag-latency = { $microseconds } µs halmozott késleltetés

## AVB Lite

lite-active = Aktív
lite-active-untagged = Aktív, címkézetlen
lite-active-vlan = Aktív, VLAN { $vlan }
lite-capable = Támogatott
lite-mode = Mód
lite-mode-capable = AVB (AVB Lite-képes)
lite-because = Ok
lite-fallback-none = nincs megadott ok
lite-fallback-endpoint = egy másik végpont deklarációja átjutott, tehát nincs köztük AVB-híd
lite-fallback-unanswered = kilenc peer delay kérés válasz nélkül maradt
lite-fallback-responders = egy peer delay kérésre ketten vagy többen válaszoltak, tehát a switch nem AVB-híd
lite-fallback-configured = a kezelő vagy egy vezérlő állította be
lite-fallback-other = a profil által meg nem nevezett ok
lite-other-profile = Más profil
lite-ptp-domain = { $profile }, tartomány: { $domain }
lite-offset = Eltérés
lite-offset-from = { $offset } (grandmaster: { $grandmaster })
lite-media-vlan = Média-VLAN
lite-untagged = Címkézetlen
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Streamenként legfeljebb { $count } listener, utána multicast
   *[other] Streamenként legfeljebb { $count } listener, utána multicast
}
lite-link = Link
lite-bandwidth = Sávszélesség
lite-egress-of = { $used } / { $link }, { $share }
lite-egress-of-assumed = { $used } / { $link }, { $share }, gigabites linket feltételezve
lite-egress-reported = Ahogy az entitás a befogadott streamjeit számolja.
lite-egress-worked-out = A csatlakoztatott streamkimenetei formátumaiból számolva.
lite-alarm-offset = PTP-eltérés: { $offset }, több az AVB Lite által megengedett 50 µs-nál
lite-alarm-egress = Kimenő forgalom: { $share } a link kapacitásából, több a streamek számára megengedett { $limit } értéknél

## AVB Wireless

wireless-station = Állomás
wireless-access-point = Hozzáférési pont
wireless-role = Szerepkör
wireless-mode = Mód
wireless-time = Idő
wireless-mode-a-ftm = Mode A, 802.1AS FTM-en keresztül
wireless-mode-a-tm = Mode A, 802.1AS TM-en keresztül
wireless-mode-b = Mode B, beacon keretekből
wireless-no-time = Nincs időszinkron
wireless-other-mode = A profil által meg nem nevezett mód
wireless-locked = Zárolódott
wireless-holdover = Holdover módban
wireless-not-locked = Nem zárolódott
wireless-row-locked = Állomás, zárolódott
wireless-row-holdover = Állomás, holdover módban
wireless-row-not-locked = Állomás, nem zárolódott
wireless-row-access-point = { $count ->
    [one] Hozzáférési pont, { $count } állomás
   *[other] Hozzáférési pont, { $count } állomás
}
wireless-link = Link
wireless-channel = csatorna { $channel }
wireless-not-known = Nem ismert
wireless-signal = Jelerősség
wireless-rate = Sebesség
wireless-ftm-valid = { $share } érvényes
wireless-rtt = { $rtt } körülfordulási idő
wireless-bursts = { $count ->
    [one] { $count } keretes sorozatok
   *[other] { $count } keretes sorozatok
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = a hozzáférési pont háromtól és kettőtől eltérő keretszámú FTM-sorozatokat engedélyez
wireless-reason-measurement = nincs sem FTM, sem TM a hozzáférési ponttal
wireless-reason-signaling = nem érkezik gPTP-capable Signaling üzenet a hozzáférési ponttól
wireless-reason-other = a profil által meg nem nevezett ok
wireless-servo = Szabályozási hiba
wireless-stations = Állomások
wireless-station-count = { $count ->
    [one] { $count } állomás
   *[other] { $count } állomás
}
wireless-no-ftm = FTM nélkül
wireless-unserved = Kiszolgálatlan listenerek
wireless-frame-budget = Keretbüdzsé
wireless-frame-use = { $used } / { $budget } keret másodpercenként
wireless-stream-frames = Streamkeretek
wireless-frames-of = { $readdressed } állomásokhoz, { $unmapped } listener nélkül, { $dropped } eldobva, { $restored } állomásoktól
wireless-class-a-allowed = Engedélyezve
wireless-class-a-not-allowed = Nincs engedélyezve
wireless-alarm-not-locked = A Wi-Fi-idő nem zárolódott a hozzáférési pontra
wireless-alarm-holdover = A Wi-Fi-idő holdover módban, a hozzáférési pontra való zárolás elveszett
wireless-alarm-unserved = { $count ->
    [one] { $count } kiszolgálatlan listener a Wi-Fi-porton, az unicast-korláton felül
   *[other] { $count } kiszolgálatlan listener a Wi-Fi-porton, az unicast-korláton felül
}

## Log

log-all = Mind
log-warnings = Figyelmeztetések
log-pause = Szünet
log-resume = Folytatás
log-clear = Törlés
log-empty = Itt jelenik meg minden ATDECC-keret, amelyet a triib küld és fogad, a legújabbal kezdve.
log-none-match = Egyik megőrzött keret sem felel meg a szűrőnek.
log-frames = { $count ->
    [one] { $count } keret
   *[other] { $count } keret
}
log-shown-of = { $shown } / { $all } keret
log-sent = Küldött
log-heard = Fogadott
log-not-decoded = Nincs dekódolva
log-warning-short = A control_data_length mezője szerint a keret { $missing } oktettel hosszabb a ténylegesnél.
log-warning-undecodable = Nem dekódolható: { $error }.
log-warning-long-acmp = Hosszú ACMP-formátumú, amelyet Milan-entitás nem küldhet (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Csatornakiosztások
mapping-inputs = Bemenetek
mapping-outputs = Kimenetek
mapping-port = port { $number }
mapping-fixed = rögzített
mapping-not-read = Még nincs beolvasva.
mapping-no-clusters = Nincsenek klaszterek.
mapping-no-streams = Nincsenek audiostreamek.
mapping-none = Nincsenek kiosztások.
mapping-not-mapped = Nincs kiosztva
mapping-cluster-numbered = Klaszter { $index }

## Presets

presets-note = A preset megőrzi minden entitás órajelforrásait, mintavételi frekvenciáit, streamformátumait, vezérlőelemeit és kapcsolatait. Visszahíváskor csak az eltérő elemek módosulnak.
presets-none = Még nincsenek mentett presetek.
presets-connections = { $count ->
    [one] { $count } kapcsolat
   *[other] { $count } kapcsolat
}
presets-recall = Visszahívás
presets-delete = Törlés
presets-no-place = A preseteket nincs hová menteni: a saját mappa nem ismert.
presets-undeletable = A törlés nem sikerült ({ $path }): { $error }.
presets-saved = { $count ->
    [one] Mentve: „{ $name }”, { $count } entitással.
   *[other] Mentve: „{ $name }”, { $count } entitással.
}
presets-nothing-differs = Semmi sem tér el ettől: „{ $name }”.
presets-recalling = { $count ->
    [one] Visszahívás: „{ $name }”, { $count } változás.
   *[other] Visszahívás: „{ $name }”, { $count } változás.
}
presets-missing = { $report } Nincs itt vagy nincs beolvasva: { $missing }.
presets-deleted = Törölve: „{ $name }”.
presets-host-note = A számítógép saját entitásait is megőrzi, és visszahíváskor újraindítja őket.
presets-host-endpoints = { $count } ezen a számítógépen
presets-starting-host = A számítógép entitásainak indítása ehhez: „{ $name }”; a többi akkor következik, amikor újra elérhetők.

## Controls

control-numbered = Vezérlőelem { $index }
control-not-shown = Itt nem jelenik meg
control-option = Opció { $number }

## Network errors

network-permission = A triib alkalmazásnak engedély kell nyers Ethernet-keretek küldéséhez és fogadásához.
network-needs-npcap = A triib alkalmazásnak Npcap kell nyers Ethernet-keretek küldéséhez és fogadásához.
network-npcap-administrators = Az Npcap csak rendszergazdáknak engedi a nyers Ethernet-keretek küldését és fogadását. Futtassa a triib alkalmazást rendszergazdaként, vagy telepítse újra az Npcapot a csak rendszergazdáknak szóló beállítás nélkül.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own entities

host-add-entity = Entitás hozzáadása
host-show-mine = Csak a számítógép saját entitásainak megjelenítése
host-show-all = Az összes entitás megjelenítése
host-new-entity = Gazda entitás { $number }
host-failed = Nem sikerült hozzáadni ehhez a számítógéphez: { $reason }
host-needs-clock = A számítógép saját entitásainak PTP-hardverórával rendelkező vezetékes interfész kell
host-no-ptp4l = A ptp4l nem válaszol, így a számítógép streamjei nem tudják tartani a gPTP időt
host-elsewhere = A triib-endpointd egy másik felhasználó vagy a root nevében fut az interfészen ({ $interface }), így a számítógép entitásai ott futnak, nem itt
host-foreign-mrp = Egy másik program a számítógép címéről MSRP- vagy MVRP-deklarációkat küld az interfészen ({ $interface }), és visszavonhatja azt, amire a számítógép streamjeinek szüksége van
host-alarm-foreign-mrp = Egy másik program ezen a számítógépen MSRP- vagy MVRP-deklarációkat küld ({ $interface })
host-state = Állapot
host-streaming = Küldés
host-waiting = Listenerre vár
host-listening = Figyelés
host-bound = Kötve, a talkerre vár
host-unbound = Nincs kötve
host-audio-from = Hang forrása
host-audio-to = Hang célja
host-channels = Csatornák
host-virtual-device = Virtuális eszköz
host-add-stream = Stream hozzáadása
host-add-audio-input = Hangbemenet
host-add-audio-output = Hangkimenet
host-add-clock-input = Médiaórajel-bemenet
host-add-clock-output = Médiaórajel-kimenet
host-remove-stream = Stream eltávolítása
host-silence = Csend
host-tone = Teszthang
host-nowhere = Sehová
host-default-device = Alapértelmezett eszköz
host-remove = Eltávolítás erről a számítógépről

mac-audio-device = Hangeszköz
mac-no-audio-device = Nincs, amíg egy stream sincs csatlakoztatva
mac-entity-note = Ezt az entitást a macOS futtatja. Amíg valamelyik streamje csatlakoztatva van, a macOS létrehozza azt a hangeszközt, amelyen az alkalmazások lejátszanak és felvesznek, mindkét irányban annyi csatornával, amennyit a konfiguráció megad.
mac-open-audio-midi-setup = Audio MIDI Setup megnyitása
mac-avb-off = Az AVB ki van kapcsolva ezen: { $interface }, így ennek a Macnek nincs saját AVB-entitása. Kapcsolja be az Audio MIDI Setup hálózati eszközböngészőjében.

listen-section = Belehallgatás
listen-start = Belehallgatás
listen-stop = Belehallgatás leállítása
listen-channels = Csatornák
listen-channel = { $number }. csatorna
listen-channel-pair = { $first }–{ $last }. csatorna
listen-output = Kimenet
listen-default-output = Alapértelmezett kimenet
listen-gain = Erősítés
listen-gain-db = { $db } dB
listen-starting = Indítás…
listen-waiting-input = Várakozás erre: { $device }, amelyet a macOS akkor hoz létre, amikor az entitás valamelyik streamje csatlakoztatva van.
listen-waiting-output = Várakozás erre: { $device }.
listen-playing = Lejátszás, csúcs { $db } dB
listen-playing-silent = Lejátszás, csend
