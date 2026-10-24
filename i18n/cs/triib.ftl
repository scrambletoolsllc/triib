## Language

language-name = Čeština

## Common

common-close = Zavřít
common-more = Více
common-keep-toolbar-shown = Nechat panel nástrojů zobrazený
common-auto-hide-toolbar = Automaticky skrývat panel nástrojů

## Settings

settings-title = Nastavení
settings-general = Obecné
settings-appearance = Vzhled
settings-language = Jazyk
settings-language-system = Jazyk systému: { $language }
settings-language-note = Textová pole používají vstupní jazyk systému.
settings-appearance-system = Podle systému
settings-appearance-light = Světlý
settings-appearance-dark = Tmavý
settings-colors = Barvy
settings-system-accent = Použít systémovou barvu zvýraznění
settings-accent-picked = Barvy triib vycházejí z barvy níže.
settings-accent-omarchy = Z motivu Omarchy: { $theme }.
settings-accent-desktop = Z barvy zvýraznění pracovní plochy.
settings-accent-none = Pracovní plocha nemá barvu zvýraznění, proto se použije barva níže.
settings-motion = Pohyb
settings-animations = Animace
settings-animations-note = Pružné a plynulé přechody při změnách.
settings-animations-reduced = Pracovní plocha žádá omezení pohybu, proto se triib obejde bez animací.
settings-outside-control = Vnější ovládání
settings-allow-outside-control = Povolit vnější ovládání
settings-allow-outside-control-note = Agenti, například Claude Code, mohou přes triib --mcp vidět a měnit vaši síť AVB. Před každým novým agentem se triib zeptá.
settings-allowed-agents = Povoleno: { $agents }
settings-forget-agents = Zapomenout
settings-storage = Úložiště
settings-storage-presets = Složka presetů
settings-storage-entities = Soubor entit
settings-storage-apply = Použít
settings-storage-choose = Vybrat…
settings-storage-note = Presety uložené na původním místě tam zůstanou; chcete-li je dál používat, přesuňte je. Nastavení aplikace triib se ukládá do { $file }.
settings-storage-entities-note = Soubor entit, který ještě neexistuje, začne jako kopie používaného souboru a triib-endpointd se na něm znovu spustí.
dialog-choose-presets = Vyberte složku s presety
dialog-choose-entities = Vyberte složku pro soubor entit
settings-full-path = Použijte úplnou cestu, např. ~/Documents/triib.
settings-path-is-folder = { $path } je složka, ne soubor.
settings-folder-missing = Složka { $path } neexistuje. Nejprve ji vytvořte, nebo vyberte jinou.
settings-path-is-file = { $path } je soubor, ne složka.
settings-cannot-write = triib nemůže zapisovat do { $path }: { $error }.
app-file-dialog-failed = Dialog souborů se nepodařilo zobrazit: { $error }
agent-prompt-title = Povolit agentovi { $agent } ovládat triib?
agent-prompt-body = { $agent } žádá o použití vnějšího ovládání triib: aby viděl entity ve vaší síti a měnil je, propojoval streamy a nastavoval formáty, hodiny a názvy. Vnější ovládání můžete vypnout v Nastavení.
agent-prompt-allow = Povolit
agent-prompt-deny = Nepovolit

common-cancel = Zrušit
common-save = Uložit
common-not-set = Nenastaveno
common-unnamed = Bez názvu
common-none = Žádné
common-mac-address = MAC adresa
common-list-separator = {", "}

## Network interfaces

interface-up = aktivní
interface-link-down = linka neaktivní
interface-wireless = bezdrátové
interface-hardware-clock = hardwarové hodiny
interface-hardware-clock-named = hardwarové hodiny { $clock }
interface-virtual = virtuální

## Toolbar

toolbar-choose-interface = Vyberte rozhraní
toolbar-interface = Síťové rozhraní
toolbar-filter-interfaces = Filtrovat rozhraní
toolbar-hide-virtual = Skrýt virtuální rozhraní
toolbar-hide-wireless = Skrýt bezdrátová rozhraní
toolbar-hide-unclocked = Skrýt rozhraní bez hardwarových hodin
toolbar-connections = Připojení
toolbar-network = Síť
toolbar-entities = Entity
toolbar-rediscover = Požádat všechny entity, aby se ohlásily
toolbar-rescan = Vymazat a znovu načíst všechny entity
toolbar-search = Hledat entity a streamy
toolbar-presets = Presety
toolbar-log = Protokol
toolbar-inspector = Inspektor
toolbar-settings = Nastavení
toolbar-close-window = Zavřít okno

## The macOS menu bar, named as in macOS's own apps

menu-about = O aplikaci triib
menu-settings = Nastavení…
menu-services = Služby
menu-hide = Skrýt triib
menu-hide-others = Skrýt ostatní
menu-show-all = Zobrazit vše
menu-quit = Ukončit triib
menu-file = Soubor
menu-presets = Presety…
menu-close-window = Zavřít okno
menu-edit = Úpravy
menu-cut = Vyjmout
menu-copy = Kopírovat
menu-paste = Vložit
menu-select-all = Vybrat vše
menu-view = Zobrazení
menu-full-screen = Přejít do režimu celé obrazovky
menu-window = Okno
menu-minimize = Minimalizovat
menu-zoom = Zvětšit/zmenšit
menu-bring-all-to-front = Přenést vše do popředí
menu-help = Nápověda
menu-website = Webové stránky triib
menu-release-notes = Poznámky k verzi

## The network's state, in place of a view

state-no-interface = Není vybráno rozhraní
state-no-interface-note = Vyberte rozhraní v síti AVB, na kterém se mají hledat entity.
state-starting = Spouštění
state-starting-note = Otevírání { $interface }.
state-listening = Naslouchání
state-listening-note = Entity na { $interface } se zde objeví, jakmile se ohlásí.
state-permission-needed = Je potřeba oprávnění
state-npcap-needed = Je potřeba Npcap
state-get-npcap = Získat Npcap
state-copy-command = Zkopírovat příkaz
state-permission-watch = Jakmile příkaz proběhne, triib pokračuje sám.
state-cannot-use = Nelze použít { $interface }
state-try-again = Zkusit znovu

## Entity list

entities-none-yet = Zatím žádné entity
entities-none-yet-note = Všechny entity v síti s jejich rolemi, třídami SR a hodinami.

## Inspector

inspector-title = Inspektor
inspector-entity = Entita
inspector-streams = Streamy
inspector-controls = Ovládací prvky
inspector-diagnostics = Diagnostika
inspector-descriptors = Deskriptory
inspector-select = Vyberte entitu a zobrazte její podrobnosti.
inspector-offline = { $entity } je offline.
inspector-heard = Neohlašuje se: triib ho zaslechl v příkazech, které poslal, a ponechá ho v seznamu, dokud neuplyne 5 minut bez příkazu.
inspector-rename = Přejmenovat
inspector-name = Název
inspector-identify = Identifikovat
inspector-model-not-read = Model entity není načten.
inspector-no-streams = Žádné streamy.
inspector-no-controls = Žádné ovládací prvky k zobrazení.
inspector-no-diagnostics = Nebyla hlášena žádná rozhraní ani čítače.
inspector-reading = Načítání deskriptorů, zatím { $count }.
inspector-read-failed = Model entity se nepodařilo načíst: { $reason }.

entity-section = Entita
entity-name = Název
entity-triib-controller = Kontrolér triib
entity-triib-on = triib na { $host }
entity-triib-here = triib na tomto počítači
entity-controller-on = Kontrolér na { $host }
entity-controller-here = Kontrolér na tomto počítači
entity-controller-at = Kontrolér na { $mac }
entity-group = Skupina
entity-product = Produkt
entity-firmware = Firmware
entity-serial-number = Sériové číslo
entity-configuration = Konfigurace
entity-configuration-of = { $name } ({ $number } z { $count })
entity-configuration-holds = { $inputs } vstupů, { $outputs } výstupů, streamy po { $channels } kanálech
entity-configuration-holds-mixed = { $inputs } vstupů, { $outputs } výstupů
entity-configuration-numbered = Konfigurace { $number }
entity-milan = Milan
entity-media-clock = Mediální hodiny
entity-clock-domain = Hodinová doména
entity-sampling-rate = Vzorkovací frekvence
clock-source-numbered = Zdroj { $index }
rate-pull = pull { $pull }

stream-format = Formát
stream-id = ID streamu
stream-destination = Cíl
stream-max-transit = Max. doba průchodu
stream-listeners = Posluchači
stream-listener-gone = { $entity } vstup { $input }, není online
stream-disconnect-listener = Odpojit

avb-interfaces = Rozhraní AVB
avb-interface = Rozhraní
avb-interface-clock-identity = Identifikátor hodin
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, doména { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Běží
avb-interface-none-reported = Nehlášeno
avb-interface-path = Cesta
avb-interface-own-grandmaster = Je svým vlastním grandmasterem
avb-interface-hops = { $count ->
    [one] { $count } skok od grandmasteru
    [few] { $count } skoky od grandmasteru
    [many] { $count } skoku od grandmasteru
   *[other] { $count } skoků od grandmasteru
}
avb-interface-link-up = Linka aktivní
avb-interface-link-down = Linka neaktivní
avb-interface-grandmaster-changes = Změny grandmasteru
avb-interface-frames-sent = Odeslané rámce
avb-interface-frames-received = Přijaté rámce
avb-interface-crc-errors = Chyby CRC

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } typ deskriptoru
    [few] { $count } typy deskriptorů
    [many] { $count } typu deskriptorů
   *[other] { $count } typů deskriptorů
}
tree-clock = Hodiny
tree-clock-source-from = { $kind }, z { $location } { $index }
tree-clock-domain-using = Používá { $source }
tree-clusters = { $count ->
    [one] { $count } klastr
    [few] { $count } klastry
    [many] { $count } klastru
   *[other] { $count } klastrů
}
tree-maps = { $count ->
    [one] { $count } mapa
    [few] { $count } mapy
    [many] { $count } mapy
   *[other] { $count } map
}

advert-not-advertised = Neohlášeno
advert-identity = Identita
advert-entity-id = ID entity
advert-entity-model = Model entity
advert-roles = Role
advert-talker = Talker
advert-listener = Listener
advert-clock = Hodiny
advert-gptp-domain = Doména gPTP
advert-sr-classes = Třídy SR
advert-indexes = Indexy v modelu entity
advert-identify-control = Ovládací prvek identifikace
advert-avb-interface = Rozhraní AVB
advert-advertising = Ohlašování
advert-valid-time = Doba platnosti
advert-available-index = Index dostupnosti
advert-association = Asociace
advert-capabilities = Schopnosti

## Status bar

status-entities = { $count ->
    [one] { $count } entita
    [few] { $count } entity
    [many] { $count } entity
   *[other] { $count } entit
}
status-not-discovering = Vyhledávání neprobíhá
status-discovering = Vyhledávání
status-discovering-as = Vyhledávání jako { $controller }
status-stopped = Zastaveno kvůli chybě
status-alarm = Alarm
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } a další { $count }

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = kontrolér
role-none = žádné role
classes-a-and-b = A a B
clock-no-gptp = Bez gPTP

read-not-read = Nenačteno
read-reading = Načítání, zatím { $count }
read-ready-unreadable = Připraveno, nečitelné: { $count }
read-ready-cached = Připraveno, z mezipaměti
read-ready = Připraveno
read-failed = Selhalo: { $reason }

milan-no = Ne
milan-before-1-3 = před 1.3
milan-certified = { $version }, certifikace { $certification }
milan-not-certified = { $version }, bez certifikace

outcome-status = stav { $status }
outcome-no-response = žádná odpověď
outcome-not-possible = není možné
outcome-connect = Nepodařilo se připojit { $talker } k { $listener }: { $reason }.
outcome-disconnect = Nepodařilo se odpojit { $listener }: { $reason }.
outcome-disconnect-listener = Nepodařilo se odpojit { $listener } od { $talker }: { $reason }.
outcome-identify = Nepodařilo se identifikovat { $entity }: { $reason }.
outcome-rename = Nepodařilo se přejmenovat { $what } na „{ $name }“: { $reason }.
outcome-rename-group = Nepodařilo se přejmenovat skupinu entity { $entity } na „{ $name }“: { $reason }.
outcome-format-streaming = Nepodařilo se změnit formát streamu { $stream }: právě se vysílá. Nejprve stream odpojte.
outcome-format = Nepodařilo se změnit formát streamu { $stream }: { $reason }.
outcome-max-transit-time = Nepodařilo se změnit max. dobu průchodu streamu { $stream }: { $reason }.
outcome-max-transit-time-streaming = Nepodařilo se změnit max. dobu průchodu streamu { $stream }: právě se vysílá. Nejprve stream odpojte.
outcome-sampling-rate = Nepodařilo se změnit vzorkovací frekvenci entity { $entity }: { $reason }.
outcome-clock-source = Nepodařilo se změnit zdroj hodin entity { $entity }: { $reason }.
outcome-configuration = Nepodařilo se změnit konfiguraci entity { $entity }: { $reason }.
outcome-configuration-streaming = Entita { $entity } si během běhu svých streamů ponechává konfiguraci: nejprve je odpojte.
outcome-map = Nepodařilo se namapovat kanál na entitě { $entity }: { $reason }.
outcome-unmap = Nepodařilo se zrušit mapování kanálu na entitě { $entity }: { $reason }.
outcome-control = Nepodařilo se nastavit „{ $control }“ na entitě { $entity }: { $reason }.
outcome-control-numbered = Nepodařilo se nastavit ovládací prvek { $index } na entitě { $entity }: { $reason }.

stream-not-connected = Nepřipojeno
stream-from = Z { $stream }
stream-from-receiving = Z { $stream }, příjem
stream-from-waiting = Z { $stream }, čeká se na talker
stream-from-failed = Z { $stream }, rezervace talkeru selhala: { $reason }
stream-sending-to = Odesílání na { $destination }

failure-no-response = entita neodpověděla
failure-refused = odmítnuto se stavem { $status }
failure-malformed = odpověď nešlo dekódovat
failure-on-this-computer = entita běží na tomto počítači; načtěte ji z jiného

msrp-failure-1 = nedostatečná šířka pásma
msrp-failure-2 = nedostatek prostředků přepínače
msrp-failure-3 = nedostatečná šířka pásma pro třídu provozu
msrp-failure-4 = ID streamu používá jiný talker
msrp-failure-5 = cílová adresa se již používá
msrp-failure-6 = vytlačeno streamem s vyšším pořadím
msrp-failure-7 = hlášená latence se změnila
msrp-failure-8 = odchozí port nepodporuje AVB
msrp-failure-9 = použijte jinou cílovou adresu
msrp-failure-10 = došly prostředky MSRP
msrp-failure-11 = došly prostředky MMRP
msrp-failure-12 = cílovou adresu nelze uložit
msrp-failure-13 = priorita není prioritou třídy SR
msrp-failure-14 = rámce jsou pro médium příliš velké
msrp-failure-15 = dosažen limit vstupních portů (fan-in)
msrp-failure-16 = u registrovaného streamu se změnila první hodnota
msrp-failure-17 = VLAN je na odchozím portu blokována
msrp-failure-18 = značkování VLAN je na odchozím portu vypnuto
msrp-failure-19 = nesoulad priority třídy SR
msrp-failure-unknown = neznámý důvod
msrp-failure-at = { $reason }, na přepínači { $bridge }

## Entity list columns

column-vendor = Výrobce
column-entity-image = Obrázek entity
column-vendor-image = Obrázek výrobce
column-model = Model
column-state = Stav
column-entity-model-id = ID modelu entity
column-talker-streams = Streamy talkeru
column-listener-streams = Streamy listeneru
column-avb-lite = AVB Lite
column-egress = Odchozí provoz
column-wireless = Bezdrátová síť

## Settings file

settings-no-place = Nastavení není kam uložit: domovská složka není známa.
settings-unusable = Nelze použít { $path }: { $error }.
settings-unsaved = Nelze uložit { $path }: { $error }.

column-remove = Odebrat sloupec
column-move-left = Posunout doleva
column-move-right = Posunout doprava
column-add = Přidat sloupec
common-percent = { $value }{" "}%

## Network view

netmap-empty = Zatím není co zobrazit
netmap-empty-note = Entity se zde objeví, jakmile budou načteny a oznámí, ve které doméně hodin jsou.
netmap-focus-clock-path = Cesta hodin { $name }
netmap-focus-streams = Streamy { $name }
netmap-showing = Zobrazeno: { $what }
netmap-devices = { $count ->
    [one] { $count } zařízení
    [few] { $count } zařízení
    [many] { $count } zařízení
   *[other] { $count } zařízení
}
netmap-bridges = { $count ->
    [one] { $count } přepínač
    [few] { $count } přepínače
    [many] { $count } přepínače
   *[other] { $count } přepínačů
}
netmap-show-map = Zobrazit mapu
netmap-show-details = Zobrazit podrobnosti
stream-numbered = Stream { $index }
netmap-bridge = Přepínač
netmap-access-point = Přístupový bod
netmap-plain-switch = Přepínač bez AVB
netmap-device = Zařízení
netmap-this-computer = Tento počítač
netmap-connected = Připojeno
netmap-advertised = Ohlášeno, žádný listener není připraven
netmap-advertised-unknown-domain = Ohlášeno, žádný listener není připraven (doména hodin zařízení { $listener } není známa)
netmap-failed-at = Rezervace selhala na { $bridge }: { $reason }
netmap-failed = Rezervace selhala: { $reason }
netmap-no-bridge-on = Na { $interface } není slyšet žádný přepínač
netmap-cannot-listen-on = Na { $interface } nelze naslouchat gPTP
netmap-behind-plain-switch = Za přepínačem bez AVB
netmap-on-this-computer = Na tomto počítači
netmap-path-not-reported = Cesta nehlášena
netmap-gptp-not-reported = gPTP nehlášeno
netmap-domain = Doména gPTP { $domain }
netmap-ptp-domain = Doména PTP { $domain }
netmap-domain-unknown = Doména neznámá
netmap-synced = Synchronizováno
netmap-sync = Synchronizace
netmap-not-synced = Nesynchronizováno
netmap-triib-on = triib na { $interface }
netmap-through-count = průchozí: { $count }
netmap-out = odchozí: { $count }
netmap-in = příchozí: { $count }
netmap-failed-count = selhané: { $count }
netmap-advertised-only = Pouze ohlášeno
netmap-failed-state = Selhalo
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-sending-unheard = Odesílá, žádný známý listener není přiřazen
netmap-no-listener = Žádný známý listener
netmap-apart-no-path = Cesta nebyla hlášena; řídí se grandmasterem { $grandmaster }
netmap-apart-unreported = Stav gPTP nebyl hlášen
netmap-apart-no-neighbor = Na rozhraní tohoto počítače není slyšet žádný přepínač
netmap-apart-cannot-listen = Tento počítač nemůže na svém rozhraní naslouchat gPTP
netmap-apart-plain-switch = Tento počítač je za přepínačem bez účasti v AVB a žádné zařízení, které jej sdílí, na něm nespouští AVB Lite, takže není v žádné doméně
netmap-apart-on-this-computer = Běží na tomto počítači; stav gPTP uvidíte, když ji načtete z jiného počítače
netmap-switch-state = Nepodílí se na AVB: zastupuje přepínače před zařízeními níže, která nenašla žádný přepínač AVB, přešla na AVB Lite a řídí se stejným grandmasterem PTP
netmap-clock-tree = Strom hodin
netmap-no-grandmaster = Žádný grandmaster není slyšet
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Vyžaduje pozornost
netmap-nodes-below = Uzly níže
netmap-bridges-below = Přepínače níže
netmap-clock-path = Cesta hodin
netmap-hops = Skoky od grandmasteru
netmap-link-delay = Zpoždění linky
netmap-bridge-port = Port přepínače
netmap-link-drops = Výpadky linky
netmap-synced-to-grandmaster = Synchronizováno s grandmasterem
netmap-host-no-gptp = Nesynchronizováno: na tomto počítači neběží gPTP
netmap-host-no-ptp = Nesynchronizováno: tento počítač se neřídí grandmasterem této domény
netmap-link-no-gptp = Nesynchronizováno: na jeho lince neběží gPTP
netmap-ptp-offset-high = Nesynchronizováno: { $offset } od grandmasteru, nad 50 µs, které AVB Lite povoluje
netmap-ptp-no-offset = Nesynchronizováno: nebyla naměřena žádná odchylka od grandmasteru
netmap-audio = Audio
netmap-media-clock-streams = Streamy mediálních hodin
netmap-audio-streams = Audio streamy
netmap-bound = přiřazené: { $count }
netmap-flowing = Přenáší se
netmap-advertised-state = Ohlášeno
netmap-media-clock-stream = Stream mediálních hodin
netmap-audio-stream = Audio stream
netmap-reaches = Dosahuje až k
netmap-passing-count = { $count ->
    [one] { $count } průchozí stream
    [few] { $count } průchozí streamy
    [many] { $count } průchozího streamu
   *[other] { $count } průchozích streamů
}
netmap-through = Přes
netmap-passing-through = Průchozí
netmap-sending = Odesílá
netmap-receiving = Přijímá
netmap-problems = Problémy
netmap-help-back = Kliknutím na pozadí se vrátíte na přehled.
netmap-help-stream = Kliknutím na stream ho otevřete v inspektoru, kliknutím na pozadí se vrátíte na přehled.
netmap-help-clock = Hodiny vycházejí z grandmasteru přes každý přepínač ke všem uzlům stromu. Přerušovaná šedá čára je linka, na které neběží gPTP. Kliknutím na zařízení nebo jeho vodič otevřete v inspektoru jeho cestu hodin; kliknutím na pozadí výběr zrušíte.
netmap-help-domains = Každá oblast je doménou hodin. V doméně gPTP hodiny sestupují z grandmasteru přes přepínače AVB. V doméně PTP běží na zařízeních AVB Lite a řídí se svým grandmasterem od konce ke konci, synchronizována v rozmezí 50 µs; přerušovaná karta zastupuje přepínače bez účasti v AVB, za nimiž některá z nich přešla na AVB Lite. Zařízení, které svou doménu nesdílí s žádným jiným, má vlastní oblast.
netmap-help-media-clock = Pouze streamy mediálních hodin (CRF), kreslené stejně jako audio: jeden vodič pro rámce každého streamu na každé lince, barva podle talkeru. Kliknutím na vodič otevřete jeho stream v inspektoru, kliknutím na zařízení zobrazíte jeho streamy; kliknutím na pozadí výběr zrušíte.
netmap-help-audio = Každý vodič představuje rámce jednoho streamu na jedné lince; vstupuje do každého přepínače na své cestě a zase z něj vystupuje. Multicastový stream má jeden vodič, dokud cesty k jeho listenerům vedou společně, a větví se tam, kde se rozcházejí; unicastový stream, jaký umí posílat AVB Lite, má od talkeru ke každému listeneru vlastní vodič. Barva je podle talkeru: každý talker má svůj odstín a každý z jeho streamů je na všech větvích jednou jeho variantou. Pohybující se tečky znamenají, že audio proudí, i od talkeru, který odesílá bez známého listeneru; nehybná červená čára je selhaná rezervace a nehybná šedá je ohlášený stream bez připraveného listeneru; obě končí tam, kde končí rezervace. Zařízení v prostředním sloupci jsou připojena přímo k přepínači grandmasteru. Kliknutím na vodič otevřete jeho stream v inspektoru, kliknutím na sdílený vodič zobrazíte všechny jeho listenery a kliknutím na zařízení jeho streamy; kliknutím na pozadí výběr zrušíte.

## Connections

matrix-nothing-shown = Žádné streamy k zobrazení
matrix-nothing-shown-note = Změňte hledání nebo filtry, abyste viděli více streamů.
matrix-empty = Žádné streamy k připojení
matrix-empty-note = Streamy talkerů a listenerů se zde setkají, jakmile budou načteny entity, které je mají.
matrix-all-streams = Všechny streamy
matrix-connectable-only = Skrýt, co nelze připojit
matrix-none-hidden = Každý zobrazený stream lze připojit
matrix-hidden = { $count ->
    [one] { $count } stream skrytý
    [few] { $count } streamy skryté
    [many] { $count } streamu skryto
   *[other] { $count } streamů skryto
}
matrix-own = Výstupy entity se nepřipojují k jejím vlastním vstupům.
matrix-working = Pracuje se na tom.
matrix-waiting-change = Čeká se na poslední změnu tohoto vstupu.
matrix-connected = Připojeno, probíhá příjem. Kliknutím odpojíte.
matrix-bound-waiting = Přiřazeno, čeká se na stream talkeru. Kliknutím odpojíte.
matrix-bound-failed = Přiřazeno, ale rezervace talkeru selhala: { $reason }. Kliknutím odpojíte.
matrix-bound-formats-differ = Přiřazeno, ale formáty se liší: talker posílá { $sent }, vstup je nastaven na { $set }. Kliknutím odpojíte.
matrix-formats-match = Formáty se shodují ({ $format }). Kliknutím připojíte.
matrix-format-must-change = Vstup přijímá { $sent }, ale je nastaven na { $set }, takže nemusí hrát, dokud se jeho formát nezmění. Kliknutím vyberete formát.
matrix-format-input = Změnit vstup na
matrix-format-both = Změnit oba na
matrix-format-talker-busy = Talker streamuje i dalším listenerům, takže jeho formát zůstane.
matrix-format-anyway = Připojit beze změny formátů
matrix-incompatible = Vstup nepřijímá { $sent }. Je nastaven na { $set }.
matrix-other-domain = Řídí se různými doménami hodin, takže mezi nimi nemůže projít žádný stream.
matrix-group-none = Nepřipojeno. Rozbalte a připojujte streamy jeden po druhém.
matrix-group-connected = Připojeno: { $count }. Rozbalením zobrazíte jednotlivá připojení.
matrix-outputs-expand = { $count ->
    [one] { $count } výstup streamu. Kliknutím na šipku rozbalíte, kliknutím na název otevřete v inspektoru.
    [few] { $count } výstupy streamů. Kliknutím na šipku rozbalíte, kliknutím na název otevřete v inspektoru.
    [many] { $count } výstupu streamu. Kliknutím na šipku rozbalíte, kliknutím na název otevřete v inspektoru.
   *[other] { $count } výstupů streamů. Kliknutím na šipku rozbalíte, kliknutím na název otevřete v inspektoru.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } výstup streamu. Kliknutím na šipku sbalíte, kliknutím na název otevřete v inspektoru.
    [few] { $count } výstupy streamů. Kliknutím na šipku sbalíte, kliknutím na název otevřete v inspektoru.
    [many] { $count } výstupu streamu. Kliknutím na šipku sbalíte, kliknutím na název otevřete v inspektoru.
   *[other] { $count } výstupů streamů. Kliknutím na šipku sbalíte, kliknutím na název otevřete v inspektoru.
}
matrix-inputs-expand = { $count ->
    [one] { $count } vstup streamu. Kliknutím na šipku rozbalíte, kliknutím na název otevřete v inspektoru.
    [few] { $count } vstupy streamů. Kliknutím na šipku rozbalíte, kliknutím na název otevřete v inspektoru.
    [many] { $count } vstupu streamu. Kliknutím na šipku rozbalíte, kliknutím na název otevřete v inspektoru.
   *[other] { $count } vstupů streamů. Kliknutím na šipku rozbalíte, kliknutím na název otevřete v inspektoru.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } vstup streamu. Kliknutím na šipku sbalíte, kliknutím na název otevřete v inspektoru.
    [few] { $count } vstupy streamů. Kliknutím na šipku sbalíte, kliknutím na název otevřete v inspektoru.
    [many] { $count } vstupu streamu. Kliknutím na šipku sbalíte, kliknutím na název otevřete v inspektoru.
   *[other] { $count } vstupů streamů. Kliknutím na šipku sbalíte, kliknutím na název otevřete v inspektoru.
}
matrix-stream-inspect = { $detail } Kliknutím otevřete { $entity } v inspektoru.
matrix-point = Najeďte na buňku
matrix-point-note = — uvidíte její talker a listener a zda se jejich formáty shodují.
matrix-legend-waiting = Přiřazeno, čeká se na stream
matrix-legend-trouble = Přiřazeno, něco je špatně
matrix-legend-open = Lze připojit
matrix-legend-change = Nejprve je nutné změnit formát vstupu
matrix-legend-incompatible = Formáty nelze sladit
matrix-legend-other-domain = Jiná doména hodin
matrix-talker-outputs = Výstupy talkerů
matrix-listener-inputs = Vstupy listenerů

common-thousands-separator = {" "}

## Diagnostics

diag-since-start = Počítáno od spuštění entity.
diag-stream-input = Vstup streamu
diag-stream-output = Výstup streamu
diag-locked = { $count ->
    [0] bez synchronizace
    [one] { $number } synchronizace
    [few] { $number } synchronizace
    [many] { $number } synchronizace
   *[other] { $number } synchronizací
}
diag-lost-lock = { $count ->
    [0] bez ztráty synchronizace
    [one] { $number } ztráta synchronizace
    [few] { $number } ztráty synchronizace
    [many] { $number } ztráty synchronizace
   *[other] { $number } ztrát synchronizace
}
diag-frames-in = { $count ->
    [one] { $number } přijatý rámec
    [few] { $number } přijaté rámce
    [many] { $number } přijatého rámce
   *[other] { $number } přijatých rámců
}
diag-frames-out = { $count ->
    [one] { $number } odeslaný rámec
    [few] { $number } odeslané rámce
    [many] { $number } odeslaného rámce
   *[other] { $number } odeslaných rámců
}
diag-media-locked = { $count ->
    [0] bez synchronizace s mediálními hodinami
    [one] { $number } synchronizace s mediálními hodinami
    [few] { $number } synchronizace s mediálními hodinami
    [many] { $number } synchronizace s mediálními hodinami
   *[other] { $number } synchronizací s mediálními hodinami
}
diag-lost-media-lock = { $count ->
    [0] bez ztráty synchronizace s mediálními hodinami
    [one] { $number } ztráta synchronizace s mediálními hodinami
    [few] { $number } ztráty synchronizace s mediálními hodinami
    [many] { $number } ztráty synchronizace s mediálními hodinami
   *[other] { $number } ztrát synchronizace s mediálními hodinami
}
diag-interrupted = { $count ->
    [0] bez přerušení
    [one] { $number } přerušení
    [few] { $number } přerušení
    [many] { $number } přerušení
   *[other] { $number } přerušení
}
diag-out-of-sequence = { $count ->
    [one] { $number } rámec mimo pořadí
    [few] { $number } rámce mimo pořadí
    [many] { $number } rámce mimo pořadí
   *[other] { $number } rámců mimo pořadí
}
diag-media-resets = { $count ->
    [one] { $number } reset médií
    [few] { $number } resety médií
    [many] { $number } resetu médií
   *[other] { $number } resetů médií
}
diag-timestamps-uncertain = { $count ->
    [0] časové značky spolehlivé
    [1] nejisté časové značky jednou
    [2] nejisté časové značky dvakrát
    [one] nejisté časové značky { $number }krát
    [few] nejisté časové značky { $number }krát
    [many] nejisté časové značky { $number }krát
   *[other] nejisté časové značky { $number }krát
}
diag-no-timestamp = { $count ->
    [one] { $number } rámec bez časové značky
    [few] { $number } rámce bez časové značky
    [many] { $number } rámce bez časové značky
   *[other] { $number } rámců bez časové značky
}
diag-unsupported-format = { $count ->
    [one] { $number } rámec v nepodporovaném formátu
    [few] { $number } rámce v nepodporovaném formátu
    [many] { $number } rámce v nepodporovaném formátu
   *[other] { $number } rámců v nepodporovaném formátu
}
diag-late = { $count ->
    [one] { $number } opožděný rámec
    [few] { $number } opožděné rámce
    [many] { $number } opožděného rámce
   *[other] { $number } opožděných rámců
}
diag-early = { $count ->
    [one] { $number } předčasný rámec
    [few] { $number } předčasné rámce
    [many] { $number } předčasného rámce
   *[other] { $number } předčasných rámců
}
diag-started = { $count ->
    [0] bez spuštění
    [one] { $number } spuštění
    [few] { $number } spuštění
    [many] { $number } spuštění
   *[other] { $number } spuštění
}
diag-stopped = { $count ->
    [0] bez zastavení
    [one] { $number } zastavení
    [few] { $number } zastavení
    [many] { $number } zastavení
   *[other] { $number } zastavení
}
diag-reservation-failed = rezervace talkeru selhala: { $reason }
diag-latency = akumulovaná latence { $microseconds } µs

## AVB Lite

lite-active = Aktivní
lite-active-untagged = Aktivní, bez značky
lite-active-vlan = Aktivní, VLAN { $vlan }
lite-capable = Podporováno
lite-mode = Režim
lite-mode-capable = AVB (podporuje AVB Lite)
lite-because = Důvod
lite-fallback-none = důvod neuveden
lite-fallback-endpoint = prošla deklarace jiného koncového zařízení, takže mezi nimi není žádný přepínač AVB
lite-fallback-unanswered = devět požadavků peer delay zůstalo bez odpovědi
lite-fallback-responders = na jeden požadavek peer delay odpověděla dvě či více zařízení, takže přepínač nepodporuje AVB
lite-fallback-configured = nastavil to operátor nebo kontrolér
lite-fallback-other = důvod, který profil neuvádí
lite-other-profile = Jiný profil
lite-ptp-domain = { $profile }, doména { $domain }
lite-offset = Odchylka
lite-offset-from = { $offset } od { $grandmaster }
lite-media-vlan = VLAN médií
lite-untagged = Bez značky
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Až { $count } listener na stream, pak multicast
    [few] Až { $count } listenery na stream, pak multicast
    [many] Až { $count } listeneru na stream, pak multicast
   *[other] Až { $count } listenerů na stream, pak multicast
}
lite-link = Linka
lite-bandwidth = Šířka pásma
lite-egress-of = { $used } z { $link }, { $share }
lite-egress-of-assumed = { $used } z { $link }, { $share }, předpokládá se gigabitová linka
lite-egress-reported = Podle toho, jak entita sama počítá své povolené streamy.
lite-egress-worked-out = Spočteno z formátů jejích připojených výstupů streamů.
lite-alarm-offset = Odchylka PTP { $offset }, nad 50 µs, které AVB Lite povoluje
lite-alarm-egress = Odchozí provoz na { $share } linky, nad { $limit }, které mohou streamy zabrat

## AVB Wireless

wireless-station = Stanice
wireless-access-point = Přístupový bod
wireless-role = Role
wireless-mode = Režim
wireless-time = Čas
wireless-mode-a-ftm = Mode A, 802.1AS přes FTM
wireless-mode-a-tm = Mode A, 802.1AS přes TM
wireless-mode-b = Mode B, z rámců beacon
wireless-no-time = Bez synchronizace času
wireless-other-mode = Režim, který profil neuvádí
wireless-locked = Synchronizováno
wireless-holdover = Režim holdover
wireless-not-locked = Nesynchronizováno
wireless-row-locked = Stanice, synchronizovaná
wireless-row-holdover = Stanice, režim holdover
wireless-row-not-locked = Stanice, nesynchronizovaná
wireless-row-access-point = { $count ->
    [one] Přístupový bod, { $count } stanice
    [few] Přístupový bod, { $count } stanice
    [many] Přístupový bod, { $count } stanice
   *[other] Přístupový bod, { $count } stanic
}
wireless-link = Linka
wireless-channel = kanál { $channel }
wireless-not-known = Neznámé
wireless-signal = Signál
wireless-rate = Rychlost
wireless-ftm-valid = { $share } platných
wireless-rtt = doba obrátky { $rtt }
wireless-bursts = { $count ->
    [one] dávky po { $count } rámci
    [few] dávky po { $count } rámcích
    [many] dávky po { $count } rámce
   *[other] dávky po { $count } rámcích
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = přístupový bod přiděluje dávky FTM s jiným počtem rámců než tři nebo dva
wireless-reason-measurement = ani FTM, ani TM s přístupovým bodem
wireless-reason-signaling = žádná zpráva Signaling gPTP-capable od přístupového bodu
wireless-reason-other = důvod, který profil neuvádí
wireless-servo = Chyba serva
wireless-stations = Stanice
wireless-station-count = { $count ->
    [one] { $count } stanice
    [few] { $count } stanice
    [many] { $count } stanice
   *[other] { $count } stanic
}
wireless-no-ftm = bez FTM
wireless-unserved = Neobsloužené listenery
wireless-frame-budget = Rozpočet rámců
wireless-frame-use = { $budget ->
    [one] { $used } z { $budget } rámce za sekundu
    [few] { $used } z { $budget } rámců za sekundu
    [many] { $used } z { $budget } rámce za sekundu
   *[other] { $used } z { $budget } rámců za sekundu
}
wireless-stream-frames = Rámce streamů
wireless-frames-of = do stanic: { $readdressed }, bez listeneru: { $unmapped }, zahozeno: { $dropped }, od stanic: { $restored }
wireless-class-a-allowed = Povoleno
wireless-class-a-not-allowed = Nepovoleno
wireless-alarm-not-locked = Čas Wi-Fi není synchronizován s přístupovým bodem
wireless-alarm-holdover = Čas Wi-Fi v režimu holdover, synchronizace s přístupovým bodem ztracena
wireless-alarm-unserved = { $count ->
    [one] { $count } listener na portu Wi-Fi bez obsluhy, nad limit unicastu
    [few] { $count } listenery na portu Wi-Fi bez obsluhy, nad limit unicastu
    [many] { $count } listeneru na portu Wi-Fi bez obsluhy, nad limit unicastu
   *[other] { $count } listenerů na portu Wi-Fi bez obsluhy, nad limit unicastu
}

## Log

log-all = Vše
log-warnings = Varování
log-pause = Pozastavit
log-resume = Pokračovat
log-clear = Vymazat
log-empty = Zde se objevuje každý rámec ATDECC, který triib odešle a zaslechne, nejnovější nahoře.
log-none-match = Filtru neodpovídá žádný uchovaný rámec.
log-frames = { $count ->
    [one] { $count } rámec
    [few] { $count } rámce
    [many] { $count } rámce
   *[other] { $count } rámců
}
log-shown-of = { $all ->
    [one] { $shown } z { $all } rámce
    [few] { $shown } z { $all } rámců
    [many] { $shown } z { $all } rámce
   *[other] { $shown } z { $all } rámců
}
log-sent = Odesláno
log-heard = Přijato
log-not-decoded = Nedekódováno
log-warning-short = { $missing ->
    [one] Pole control_data_length přesahuje konec rámce o { $missing } oktet.
    [few] Pole control_data_length přesahuje konec rámce o { $missing } oktety.
    [many] Pole control_data_length přesahuje konec rámce o { $missing } oktetu.
   *[other] Pole control_data_length přesahuje konec rámce o { $missing } oktetů.
}
log-warning-undecodable = Rámec nelze dekódovat: { $error }.
log-warning-long-acmp = Rámec je v dlouhé formě ACMP, kterou entita Milan nesmí odesílat (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Mapování kanálů
mapping-inputs = Vstupy
mapping-outputs = Výstupy
mapping-port = port { $number }
mapping-fixed = pevné
mapping-not-read = Zatím nenačteno.
mapping-no-clusters = Žádné klastry.
mapping-no-streams = Žádné audio streamy.
mapping-none = Žádná mapování.
mapping-not-mapped = Nenamapováno
mapping-cluster-numbered = Klastr { $index }

## Presets

presets-note = Preset uchovává zdroje hodin, vzorkovací frekvence, formáty streamů, ovládací prvky a připojení každé entity. Jeho vyvolání změní to, co se liší.
presets-none = Zatím nejsou uloženy žádné presety.
presets-connections = { $count ->
    [one] { $count } připojení
    [few] { $count } připojení
    [many] { $count } připojení
   *[other] { $count } připojení
}
presets-recall = Vyvolat
presets-delete = Smazat
presets-no-place = Presety není kam uložit: domovská složka není známa.
presets-undeletable = Nelze smazat { $path }: { $error }.
presets-saved = { $count ->
    [one] Uloženo „{ $name }“ s { $count } entitou.
    [few] Uloženo „{ $name }“ s { $count } entitami.
    [many] Uloženo „{ $name }“ s { $count } entity.
   *[other] Uloženo „{ $name }“ s { $count } entitami.
}
presets-nothing-differs = Nic se neliší od „{ $name }“.
presets-recalling = { $count ->
    [one] Vyvolávání „{ $name }“: { $count } změna.
    [few] Vyvolávání „{ $name }“: { $count } změny.
    [many] Vyvolávání „{ $name }“: { $count } změny.
   *[other] Vyvolávání „{ $name }“: { $count } změn.
}
presets-missing = { $report } Není v síti nebo nenačteno: { $missing }.
presets-deleted = Smazáno „{ $name }“.
presets-host-note = Uchovává také vlastní entity tohoto počítače a při vyvolání je znovu spustí.
presets-host-endpoints = { $count } na tomto počítači
presets-starting-host = Spouštějí se entity tohoto počítače pro „{ $name }“; zbytek následuje, až budou zpět.

## Controls

control-numbered = Ovládací prvek { $index }
control-not-shown = Zde se nezobrazuje
control-option = Možnost { $number }

## Network errors

network-permission = triib potřebuje oprávnění odesílat a přijímat surové rámce Ethernet.
network-needs-npcap = triib potřebuje Npcap k odesílání a přijímání surových rámců Ethernet.
network-npcap-administrators = Npcap umožňuje odesílat a přijímat surové rámce Ethernet jen správcům. Spusťte triib jako správce, nebo Npcap nainstalujte znovu bez volby pouze pro správce.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own entities

host-add-entity = Přidat entitu
host-show-mine = Zobrazit jen vlastní entity tohoto počítače
host-show-all = Zobrazit všechny entity
host-new-entity = Entita hostitele { $number }
host-failed = Nepodařilo se přidat do tohoto počítače: { $reason }
host-needs-clock = Vlastní entity tohoto počítače potřebují kabelové rozhraní s hardwarovými hodinami PTP
host-no-ptp4l = ptp4l neodpovídá, takže streamy tohoto počítače nemohou držet čas gPTP
host-starting = Spouštění
host-starting-entity = Spouští se { $name }
host-setting-up = Příprava { $interface } pro AVB
host-no-link = Na { $interface } není spojení
host-starting-ptp4l = Spouštění ptp4l
host-no-linuxptp = linuxptp není nainstalován, takže streamy tohoto počítače nemohou držet čas gPTP
host-waiting-network = Čeká na síť
host-elsewhere = triib-endpointd běží na { $interface } pro jiného uživatele nebo jako root, takže entity tohoto počítače běží tam, ne tady
host-foreign-mrp = Jiný program deklaruje MSRP nebo MVRP na { $interface } z adresy tohoto počítače a může tak odvolat to, co potřebují streamy tohoto počítače
host-alarm-foreign-mrp = Jiný program na tomto počítači deklaruje MSRP nebo MVRP na { $interface }
host-state = Stav
host-streaming = Vysílá
host-waiting = Čeká na listener
host-listening = Naslouchá
host-bound = Svázáno, čeká na talker
host-unbound = Nesvázáno
host-audio-from = Zvuk z
host-audio-to = Zvuk do
host-channels = Kanály
host-virtual-device = Virtuální zařízení
host-add-stream = Přidat stream
host-add-audio-input = Zvukový vstup
host-add-audio-output = Zvukový výstup
host-add-clock-input = Vstup mediálních hodin
host-add-clock-output = Výstup mediálních hodin
host-remove-stream = Odebrat stream
host-silence = Ticho
host-tone = Testovací tón
host-nowhere = Nikam
host-default-device = Výchozí zařízení
host-remove = Odebrat z tohoto počítače

mac-audio-device = Zvukové zařízení
mac-no-audio-device = Žádné, dokud není připojen žádný stream
mac-entity-note = Tuto entitu spouští macOS. Dokud je připojen některý z jejích streamů, macOS vytváří zvukové zařízení, do kterého aplikace přehrávají a ze kterého nahrávají, s tolika kanály v každém směru, kolik určuje konfigurace.
mac-open-audio-midi-setup = Otevřít Nastavení Audio MIDI
mac-avb-off = AVB je na rozhraní { $interface } vypnuté, takže tento Mac nemá vlastní entitu AVB. Zapněte ho v prohlížeči síťových zařízení v Nastavení Audio MIDI.

listen-section = Poslech
listen-start = Poslouchat
listen-stop = Přestat poslouchat
listen-source = Poslouchá
listen-received = Co přijímá
listen-sent = Co posílá ve streamu { $number } ({ $channels } kan.)
listen-sent-needs-macos = Poslech toho, co tento Mac posílá, vyžaduje macOS 14.2 nebo novější.
listen-channels = Kanály
listen-channel = Kanál { $number }
listen-channel-pair = Kanály { $first }–{ $last }
listen-output = Výstup
listen-default-output = Výchozí výstup
listen-gain = Zesílení
listen-gain-db = { $db } dB
listen-starting = Spouštění…
listen-waiting-input = Čeká se na { $device }, které macOS vytváří, dokud je připojen některý stream entity.
listen-waiting-output = Čeká se na { $device }.
listen-playing = Přehrává se, špička { $db } dB
listen-playing-silent = Přehrává se, ticho
