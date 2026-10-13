## Language

language-name = Polski

## Common

common-close = Zamknij
common-more = Więcej
common-keep-toolbar-shown = Nie ukrywaj paska narzędzi
common-auto-hide-toolbar = Automatycznie ukrywaj pasek narzędzi

## Settings

settings-title = Ustawienia
settings-general = Ogólne
settings-appearance = Wygląd
settings-language = Język
settings-language-system = Język systemu: { $language }
settings-language-note = Pola tekstowe używają systemowego języka wprowadzania.
settings-appearance-system = Jak w systemie
settings-appearance-light = Jasny
settings-appearance-dark = Ciemny
settings-colors = Kolory
settings-system-accent = Użyj systemowego koloru akcentu
settings-accent-picked = Kolory triib powstają na bazie koloru poniżej.
settings-accent-omarchy = Z motywu Omarchy: { $theme }.
settings-accent-desktop = Z koloru akcentu pulpitu.
settings-accent-none = Pulpit nie ma koloru akcentu, więc używany jest kolor poniżej.
settings-motion = Ruch
settings-animations = Animacje
settings-animations-note = Sprężyste i płynne przejścia przy zmianach.
settings-animations-reduced = Pulpit prosi o ograniczenie ruchu, więc triib obywa się bez animacji.
settings-outside-control = Sterowanie zewnętrzne
settings-allow-outside-control = Zezwalaj na sterowanie zewnętrzne
settings-allow-outside-control-note = Agenci, tacy jak Claude Code, mogą przez triib --mcp widzieć i zmieniać Twoją sieć AVB. triib pyta przed każdym nowym agentem.
settings-allowed-agents = Dozwolone: { $agents }
settings-forget-agents = Zapomnij
agent-prompt-title = Zezwolić agentowi { $agent } na sterowanie triib?
agent-prompt-body = { $agent } prosi o użycie sterowania zewnętrznego triib: aby widzieć encje w Twojej sieci i zmieniać je, łączyć strumienie oraz ustawiać formaty, zegary i nazwy. Sterowanie zewnętrzne możesz wyłączyć w Ustawieniach.
agent-prompt-allow = Zezwól
agent-prompt-deny = Nie zezwalaj

common-cancel = Anuluj
common-save = Zapisz
common-not-set = Nie ustawiono
common-unnamed = Bez nazwy
common-none = Brak
common-mac-address = Adres MAC
common-list-separator = {", "}

## Network interfaces

interface-up = aktywny
interface-link-down = brak łącza
interface-wireless = bezprzewodowy
interface-hardware-clock = zegar sprzętowy
interface-hardware-clock-named = zegar sprzętowy { $clock }
interface-virtual = wirtualny

## Toolbar

toolbar-choose-interface = Wybierz interfejs
toolbar-interface = Interfejs sieciowy
toolbar-show-virtual = Pokaż interfejsy wirtualne
toolbar-hide-virtual = Ukryj interfejsy wirtualne
toolbar-connections = Połączenia
toolbar-network = Sieć
toolbar-entities = Encje
toolbar-rediscover = Poproś wszystkie encje o ogłoszenie się
toolbar-rescan = Wyczyść i ponownie przeskanuj wszystkie encje
toolbar-search = Szukaj encji i strumieni
toolbar-presets = Presety
toolbar-log = Dziennik
toolbar-inspector = Inspektor
toolbar-settings = Ustawienia

## The network's state, in place of a view

state-no-interface = Nie wybrano interfejsu
state-no-interface-note = Wybierz interfejs w sieci AVB, aby wykrywać encje.
state-starting = Uruchamianie
state-starting-note = Otwieranie { $interface }.
state-listening = Nasłuchiwanie
state-listening-note = Encje na { $interface } pojawiają się tutaj, gdy się ogłoszą.
state-permission-needed = Wymagane uprawnienia
state-npcap-needed = Wymagany Npcap
state-get-npcap = Pobierz Npcap
state-copy-command = Kopiuj polecenie
state-cannot-use = Nie można użyć { $interface }
state-try-again = Spróbuj ponownie

## Entity list

entities-none-yet = Brak encji
entities-none-yet-note = Każda encja w sieci wraz z rolami, klasami SR i zegarem.

## Inspector

inspector-title = Inspektor
inspector-entity = Encja
inspector-streams = Strumienie
inspector-controls = Regulatory
inspector-diagnostics = Diagnostyka
inspector-descriptors = Deskryptory
inspector-select = Wybierz encję, aby zobaczyć szczegóły.
inspector-offline = { $entity } jest offline.
inspector-rename = Zmień nazwę
inspector-name = Nazwa
inspector-identify = Identyfikuj
inspector-model-not-read = Model encji nie został odczytany.
inspector-no-streams = Brak strumieni.
inspector-no-controls = Brak regulatorów do wyświetlenia.
inspector-no-diagnostics = Brak zgłoszonych interfejsów i liczników.
inspector-reading = Odczyt deskryptorów, odczytano { $count }.
inspector-read-failed = Nie udało się odczytać modelu encji: { $reason }.

entity-section = Encja
entity-name = Nazwa
entity-group = Grupa
entity-product = Produkt
entity-firmware = Firmware
entity-serial-number = Numer seryjny
entity-configuration = Konfiguracja
entity-configuration-of = { $name } ({ $number } z { $count })
entity-milan = Milan
entity-media-clock = Zegar mediów
entity-clock-domain = Domena zegara
entity-sampling-rate = Częstotliwość próbkowania
clock-source-numbered = Źródło { $index }
rate-pull = pull { $pull }

stream-format = Format
stream-id = ID strumienia
stream-destination = Cel
stream-max-transit-time = Maks. czas przejścia { $time }

avb-interfaces = Interfejsy AVB
avb-interface = Interfejs
avb-interface-clock-identity = Identyfikator zegara
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domena { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Działa
avb-interface-none-reported = Brak danych
avb-interface-path = Ścieżka
avb-interface-own-grandmaster = Sam jest grandmasterem
avb-interface-hops = { $count ->
    [one] { $count } przeskok od grandmastera
    [few] { $count } przeskoki od grandmastera
    [many] { $count } przeskoków od grandmastera
   *[other] { $count } przeskoku od grandmastera
}
avb-interface-link-up = Łącze aktywne
avb-interface-link-down = Brak łącza
avb-interface-grandmaster-changes = Zmiany grandmastera
avb-interface-frames-sent = Wysłane ramki
avb-interface-frames-received = Odebrane ramki
avb-interface-crc-errors = Błędy CRC

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } typ deskryptora
    [few] { $count } typy deskryptorów
    [many] { $count } typów deskryptorów
   *[other] { $count } typu deskryptorów
}
tree-clock = Zegar
tree-clock-source-from = { $kind }, z { $location } { $index }
tree-clock-domain-using = Używa: { $source }
tree-clusters = { $count ->
    [one] { $count } klaster
    [few] { $count } klastry
    [many] { $count } klastrów
   *[other] { $count } klastra
}
tree-maps = { $count ->
    [one] { $count } mapa
    [few] { $count } mapy
    [many] { $count } map
   *[other] { $count } mapy
}

advert-not-advertised = Nie ogłoszono
advert-identity = Tożsamość
advert-entity-id = ID encji
advert-entity-model = Model encji
advert-roles = Role
advert-talker = Talker
advert-listener = Listener
advert-clock = Zegar
advert-btc = BTC
advert-gptp-domain = Domena gPTP
advert-sr-classes = Klasy SR
advert-indexes = Indeksy w modelu encji
advert-identify-control = Regulator identyfikacji
advert-avb-interface = Interfejs AVB
advert-advertising = Ogłaszanie
advert-valid-time = Czas ważności
advert-available-index = Indeks dostępności
advert-association = Asocjacja
advert-capabilities = Możliwości

## Status bar

status-entities = { $count ->
    [one] { $count } encja
    [few] { $count } encje
    [many] { $count } encji
   *[other] { $count } encji
}
status-not-discovering = Wykrywanie wyłączone
status-discovering = Wykrywanie
status-discovering-as = Wykrywanie jako { $controller }
status-stopped = Zatrzymano z powodu błędu
status-alarm = Alarm
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } i jeszcze { $count }

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = kontroler
role-none = brak ról
classes-a-and-b = A i B
clock-no-gptp = Brak gPTP

read-not-read = Nie odczytano
read-reading = Odczyt, odczytano { $count }
read-ready-unreadable = Gotowe, nieczytelne: { $count }
read-ready-cached = Gotowe, z pamięci podręcznej
read-ready = Gotowe
read-failed = Błąd: { $reason }

milan-no = Nie
milan-before-1-3 = przed 1.3
milan-certified = { $version }, certyfikacja { $certification }
milan-not-certified = { $version }, bez certyfikacji

outcome-status = status { $status }
outcome-no-response = brak odpowiedzi
outcome-not-possible = niemożliwe
outcome-connect = Nie udało się połączyć { $talker } z { $listener }: { $reason }.
outcome-disconnect = Nie udało się rozłączyć { $listener }: { $reason }.
outcome-identify = Nie udało się zidentyfikować { $entity }: { $reason }.
outcome-rename = Nie udało się zmienić nazwy { $what } na „{ $name }”: { $reason }.
outcome-rename-group = Nie udało się zmienić nazwy grupy encji { $entity } na „{ $name }”: { $reason }.
outcome-format-streaming = Nie udało się zmienić formatu strumienia { $stream }: trwa transmisja. Najpierw go rozłącz.
outcome-format = Nie udało się zmienić formatu strumienia { $stream }: { $reason }.
outcome-sampling-rate = Nie udało się zmienić częstotliwości próbkowania encji { $entity }: { $reason }.
outcome-clock-source = Nie udało się zmienić źródła zegara encji { $entity }: { $reason }.
outcome-map = Nie udało się zmapować kanału w encji { $entity }: { $reason }.
outcome-unmap = Nie udało się usunąć mapowania kanału w encji { $entity }: { $reason }.
outcome-control = Nie udało się ustawić „{ $control }” w encji { $entity }: { $reason }.
outcome-control-numbered = Nie udało się ustawić regulatora { $index } w encji { $entity }: { $reason }.

stream-not-connected = Nie połączono
stream-from = Z { $stream }
stream-from-receiving = Z { $stream }, odbiór
stream-from-waiting = Z { $stream }, oczekiwanie na strumień od talkera
stream-from-failed = Z { $stream }, rezerwacja talkera nie powiodła się: { $reason }
stream-sending-to = Wysyłanie do { $destination }

failure-no-response = encja nie odpowiedziała
failure-refused = odmowa ze statusem { $status }
failure-malformed = nie udało się zdekodować odpowiedzi
failure-on-this-computer = encja działa na tym komputerze; odczytaj ją z innego

msrp-failure-1 = niewystarczające pasmo
msrp-failure-2 = niewystarczające zasoby przełącznika
msrp-failure-3 = niewystarczające pasmo dla klasy ruchu
msrp-failure-4 = ID strumienia używany przez innego talkera
msrp-failure-5 = adres docelowy jest już używany
msrp-failure-6 = wywłaszczony przez strumień o wyższej randze
msrp-failure-7 = zgłoszone opóźnienie zmieniło się
msrp-failure-8 = port wyjściowy nie obsługuje AVB
msrp-failure-9 = użyj innego adresu docelowego
msrp-failure-10 = brak zasobów MSRP
msrp-failure-11 = brak zasobów MMRP
msrp-failure-12 = nie można zapisać adresu docelowego
msrp-failure-13 = priorytet nie jest priorytetem klasy SR
msrp-failure-14 = ramki za duże dla medium
msrp-failure-15 = osiągnięto limit portów wejściowych (fan-in)
msrp-failure-16 = zmieniła się pierwsza wartość zarejestrowanego strumienia
msrp-failure-17 = VLAN zablokowany na porcie wyjściowym
msrp-failure-18 = tagowanie VLAN wyłączone na porcie wyjściowym
msrp-failure-19 = niezgodność priorytetu klasy SR
msrp-failure-unknown = nieznana przyczyna
msrp-failure-at = { $reason }, na przełączniku { $bridge }

## Entity list columns

column-vendor = Producent
column-model = Model
column-state = Stan
column-entity-model-id = ID modelu encji
column-talker-streams = Strumienie talkera
column-listener-streams = Strumienie listenera
column-avb-lite = AVB Lite
column-egress = Ruch wychodzący
column-wireless = Sieć bezprzewodowa

## Settings file

settings-no-place = Nie ma gdzie zapisać ustawień: folder domowy jest nieznany.
settings-unusable = Nie można użyć { $path }: { $error }.
settings-unsaved = Nie udało się zapisać { $path }: { $error }.

column-remove = Usuń kolumnę
column-move-left = Przesuń w lewo
column-move-right = Przesuń w prawo
column-add = Dodaj kolumnę
common-percent = { $value }%

## Network view

netmap-empty = Brak sieci do pokazania
netmap-empty-note = Encje pojawią się tutaj, gdy zostaną odczytane i podadzą swoje miejsce w drzewie gPTP.
netmap-focus-clock-path = Ścieżka zegara { $name }
netmap-focus-streams = Strumienie { $name }
netmap-showing = Pokazano: { $what }
netmap-devices = { $count ->
    [one] { $count } urządzenie
    [few] { $count } urządzenia
    [many] { $count } urządzeń
   *[other] { $count } urządzenia
}
netmap-bridges = { $count ->
    [one] { $count } przełącznik
    [few] { $count } przełączniki
    [many] { $count } przełączników
   *[other] { $count } przełącznika
}
netmap-show-map = Pokaż mapę
netmap-show-details = Pokaż szczegóły
stream-numbered = Strumień { $index }
netmap-bridge = Przełącznik
netmap-access-point = Punkt dostępowy
netmap-device = Urządzenie
netmap-this-computer = Ten komputer
netmap-connected = Połączono
netmap-advertised = Ogłoszono, brak gotowego listenera
netmap-advertised-off-tree = Ogłoszono, brak gotowego listenera ({ $listener } poza drzewem gPTP)
netmap-failed-at = Rezerwacja nie powiodła się na { $bridge }: { $reason }
netmap-failed = Rezerwacja nie powiodła się: { $reason }
netmap-no-bridge-on = Na { $interface } nie słychać przełącznika
netmap-cannot-listen-on = Nie można nasłuchiwać gPTP na { $interface }
netmap-on-this-computer = Na tym komputerze
netmap-path-not-reported = Nie podano ścieżki
netmap-gptp-not-reported = Brak danych gPTP
netmap-off-tree = Poza drzewem gPTP
netmap-off-ptp = Poza drzewem PTP
netmap-not-lite = Poza AVB Lite
netmap-lite-not-reported = Brak danych AVB Lite
netmap-synced = Zsynchronizowano
netmap-not-synced = Brak synchronizacji
netmap-triib-on = triib na { $interface }
netmap-through-count = tranzytowe: { $count }
netmap-out = wychodzące: { $count }
netmap-in = przychodzące: { $count }
netmap-failed-count = nieudane: { $count }
netmap-advertised-only = Tylko ogłoszono
netmap-failed-state = Błąd
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Poza drzewem gPTP: sam jest grandmasterem
netmap-apart-no-path = Nie podano ścieżki; synchronizuje się z grandmasterem { $grandmaster }
netmap-apart-unreported = Nie podano stanu gPTP
netmap-apart-no-neighbor = Na interfejsie tego komputera nie słychać przełącznika
netmap-apart-cannot-listen = Ten komputer nie może nasłuchiwać gPTP na swoim interfejsie
netmap-apart-on-this-computer = Działa na tym komputerze; odczytaj ją z innego komputera, aby zobaczyć stan gPTP
netmap-apart-not-lite = Nie działa w AVB Lite, więc nie synchronizuje się z grandmasterem
netmap-apart-lite-unreported = Nie podaje żadnych danych AVB Lite, więc nie wiadomo, z czym się synchronizuje
netmap-clock-tree = Drzewo zegara
netmap-no-grandmaster = Nie słychać grandmastera
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Wymaga uwagi
netmap-nodes-below = Węzły poniżej
netmap-bridges-below = Przełączniki poniżej
netmap-clock-path = Ścieżka zegara
netmap-hops = Przeskoki od grandmastera
netmap-link-delay = Opóźnienie łącza
netmap-bridge-port = Port przełącznika
netmap-link-drops = Zaniki łącza
netmap-synced-to-grandmaster = Zsynchronizowano z grandmasterem
netmap-host-no-gptp = Brak synchronizacji: na tym komputerze nie działa gPTP
netmap-link-no-gptp = Brak synchronizacji: na jego łączu nie działa gPTP
netmap-ptp-offset-high = Brak synchronizacji: { $offset } od grandmastera, powyżej 50 µs dopuszczanych przez AVB Lite
netmap-ptp-no-offset = Brak synchronizacji: nie zmierzono przesunięcia względem grandmastera
netmap-audio = Audio
netmap-media-clock-streams = Strumienie zegara mediów
netmap-audio-streams = Strumienie audio
netmap-bound = powiązane: { $count }
netmap-flowing = Przesyłany
netmap-advertised-state = Ogłoszono
netmap-media-clock-stream = Strumień zegara mediów
netmap-audio-stream = Strumień audio
netmap-reaches = Dociera do
netmap-passing-count = { $count ->
    [one] { $count } strumień tranzytowy
    [few] { $count } strumienie tranzytowe
    [many] { $count } strumieni tranzytowych
   *[other] { $count } strumienia tranzytowego
}
netmap-through = Przez
netmap-passing-through = Tranzyt
netmap-sending = Wysyła
netmap-receiving = Odbiera
netmap-problems = Problemy
netmap-help-back = Kliknij tło, aby wrócić do przeglądu.
netmap-help-stream = Kliknij strumień, aby otworzyć go w inspektorze, lub tło, aby wrócić do przeglądu.
netmap-help-ptp = W AVB Lite zegar płynie end-to-end od grandmastera do każdego urządzenia przez przełączniki, które w tym nie uczestniczą, więc żaden nie jest pokazany. Urządzenie jest zsynchronizowane, dopóki synchronizuje się z grandmasterem z dokładnością do 50 µs. Kliknij urządzenie lub jego przewód, aby otworzyć jego zegar w inspektorze; kliknij tło, aby wyczyścić zaznaczenie.
netmap-help-clock = Zegar płynie od grandmastera przez każdy przełącznik do każdego węzła drzewa. Przerywana szara linia to łącze, na którym nie działa gPTP. Kliknij urządzenie lub jego przewód, aby otworzyć jego ścieżkę zegara w inspektorze; kliknij tło, aby wyczyścić zaznaczenie.
netmap-help-media-clock = Tylko strumienie zegara mediów (CRF), rysowane tak samo jak audio: jeden przewód na strumień, kolor według talkera. Kliknij przewód, aby otworzyć jego strumień w inspektorze, lub urządzenie, aby zobaczyć jego strumienie; kliknij tło, aby wyczyścić zaznaczenie.
netmap-help-audio = Każdy strumień ma własny przewód, który wchodzi do każdego przełącznika na swojej drodze i z niego wychodzi. Kolor zależy od talkera: każdy talker ma swój odcień, a jego strumienie są wariantami tego odcienia. Poruszające się kropki oznaczają, że audio płynie; nieruchoma czerwona linia to nieudana rezerwacja, a nieruchoma szara to strumień ogłoszony bez gotowego listenera; obie kończą się tam, gdzie kończy się rezerwacja. Urządzenia w środkowej kolumnie łączą się bezpośrednio z przełącznikiem grandmastera. Kliknij przewód, aby otworzyć jego strumień w inspektorze, lub urządzenie, aby zobaczyć jego strumienie; kliknij tło, aby wyczyścić zaznaczenie.

## Connections

matrix-nothing-shown = Brak strumieni do pokazania
matrix-nothing-shown-note = Zmień wyszukiwanie lub filtry, aby zobaczyć więcej strumieni.
matrix-empty = Brak strumieni do połączenia
matrix-empty-note = Strumienie talkerów i listenerów spotkają się tutaj, gdy encje, które je mają, zostaną odczytane.
matrix-all-streams = Wszystkie strumienie
matrix-connectable-only = Ukryj to, czego nie można połączyć
matrix-none-hidden = Każdy pokazany strumień można połączyć
matrix-hidden = { $count ->
    [one] { $count } strumień ukryty
    [few] { $count } strumienie ukryte
    [many] { $count } strumieni ukrytych
   *[other] { $count } strumienia ukrytego
}
matrix-own = Wyjścia encji nie łączą się z jej własnymi wejściami.
matrix-working = W toku.
matrix-waiting-change = Oczekiwanie na ostatnią zmianę tego wejścia.
matrix-connected = Połączono, trwa odbiór. Kliknij, aby rozłączyć.
matrix-bound-waiting = Powiązano, oczekiwanie na strumień talkera. Kliknij, aby rozłączyć.
matrix-bound-failed = Powiązano, ale rezerwacja talkera nie powiodła się: { $reason }. Kliknij, aby rozłączyć.
matrix-bound-formats-differ = Powiązano, ale formaty się różnią: talker wysyła { $sent }, wejście jest ustawione na { $set }. Kliknij, aby rozłączyć.
matrix-formats-match = Formaty są zgodne ({ $format }). Kliknij, aby połączyć.
matrix-format-must-change = Wejście przyjmuje { $sent }, ale jest ustawione na { $set }, więc może nie odtwarzać dźwięku, dopóki format się nie zmieni. Kliknij, aby wybrać format.
matrix-format-input = Zmień wejście na
matrix-format-both = Zmień oba na
matrix-format-talker-busy = Talker przesyła strumień do innych listenerów, więc jego format zostaje.
matrix-format-anyway = Połącz bez zmiany formatów
matrix-incompatible = Wejście nie przyjmuje { $sent }. Jest ustawione na { $set }.
matrix-group-none = Nie połączono. Rozwiń, aby łączyć strumienie pojedynczo.
matrix-group-connected = Połączono: { $count }. Rozwiń, aby zobaczyć każde połączenie.
matrix-outputs-expand = { $count ->
    [one] { $count } wyjście strumienia. Kliknij strzałkę, aby rozwinąć, lub nazwę, aby otworzyć w inspektorze.
    [few] { $count } wyjścia strumieni. Kliknij strzałkę, aby rozwinąć, lub nazwę, aby otworzyć w inspektorze.
    [many] { $count } wyjść strumieni. Kliknij strzałkę, aby rozwinąć, lub nazwę, aby otworzyć w inspektorze.
   *[other] { $count } wyjścia strumienia. Kliknij strzałkę, aby rozwinąć, lub nazwę, aby otworzyć w inspektorze.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } wyjście strumienia. Kliknij strzałkę, aby zwinąć, lub nazwę, aby otworzyć w inspektorze.
    [few] { $count } wyjścia strumieni. Kliknij strzałkę, aby zwinąć, lub nazwę, aby otworzyć w inspektorze.
    [many] { $count } wyjść strumieni. Kliknij strzałkę, aby zwinąć, lub nazwę, aby otworzyć w inspektorze.
   *[other] { $count } wyjścia strumienia. Kliknij strzałkę, aby zwinąć, lub nazwę, aby otworzyć w inspektorze.
}
matrix-inputs-expand = { $count ->
    [one] { $count } wejście strumienia. Kliknij strzałkę, aby rozwinąć, lub nazwę, aby otworzyć w inspektorze.
    [few] { $count } wejścia strumieni. Kliknij strzałkę, aby rozwinąć, lub nazwę, aby otworzyć w inspektorze.
    [many] { $count } wejść strumieni. Kliknij strzałkę, aby rozwinąć, lub nazwę, aby otworzyć w inspektorze.
   *[other] { $count } wejścia strumienia. Kliknij strzałkę, aby rozwinąć, lub nazwę, aby otworzyć w inspektorze.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } wejście strumienia. Kliknij strzałkę, aby zwinąć, lub nazwę, aby otworzyć w inspektorze.
    [few] { $count } wejścia strumieni. Kliknij strzałkę, aby zwinąć, lub nazwę, aby otworzyć w inspektorze.
    [many] { $count } wejść strumieni. Kliknij strzałkę, aby zwinąć, lub nazwę, aby otworzyć w inspektorze.
   *[other] { $count } wejścia strumienia. Kliknij strzałkę, aby zwinąć, lub nazwę, aby otworzyć w inspektorze.
}
matrix-stream-inspect = { $detail } Kliknij, aby otworzyć { $entity } w inspektorze.
matrix-point = Wskaż komórkę
matrix-point-note = — zobaczysz, który talker i listener się w niej spotykają i czy ich formaty są zgodne.
matrix-legend-waiting = Powiązano, oczekiwanie na strumień
matrix-legend-trouble = Powiązano, coś jest nie tak
matrix-legend-open = Można połączyć
matrix-legend-change = Najpierw trzeba zmienić format wejścia
matrix-legend-incompatible = Formaty niezgodne
matrix-talker-outputs = Wyjścia talkerów
matrix-listener-inputs = Wejścia listenerów

common-thousands-separator = {" "}

## Diagnostics

diag-since-start = Zliczono od uruchomienia encji.
diag-stream-input = Wejście strumienia
diag-stream-output = Wyjście strumienia
diag-locked = { $count ->
    [0] bez synchronizacji
    [one] { $number } synchronizacja
    [few] { $number } synchronizacje
    [many] { $number } synchronizacji
   *[other] { $number } synchronizacji
}
diag-lost-lock = { $count ->
    [0] bez utraty synchronizacji
    [one] { $number } utrata synchronizacji
    [few] { $number } utraty synchronizacji
    [many] { $number } utrat synchronizacji
   *[other] { $number } utraty synchronizacji
}
diag-frames-in = { $count ->
    [one] { $number } odebrana ramka
    [few] { $number } odebrane ramki
    [many] { $number } odebranych ramek
   *[other] { $number } odebranej ramki
}
diag-frames-out = { $count ->
    [one] { $number } wysłana ramka
    [few] { $number } wysłane ramki
    [many] { $number } wysłanych ramek
   *[other] { $number } wysłanej ramki
}
diag-media-locked = { $count ->
    [0] bez synchronizacji z zegarem mediów
    [one] { $number } synchronizacja z zegarem mediów
    [few] { $number } synchronizacje z zegarem mediów
    [many] { $number } synchronizacji z zegarem mediów
   *[other] { $number } synchronizacji z zegarem mediów
}
diag-lost-media-lock = { $count ->
    [0] bez utraty synchronizacji z zegarem mediów
    [one] { $number } utrata synchronizacji z zegarem mediów
    [few] { $number } utraty synchronizacji z zegarem mediów
    [many] { $number } utrat synchronizacji z zegarem mediów
   *[other] { $number } utraty synchronizacji z zegarem mediów
}
diag-interrupted = { $count ->
    [0] bez przerw
    [one] { $number } przerwa
    [few] { $number } przerwy
    [many] { $number } przerw
   *[other] { $number } przerwy
}
diag-out-of-sequence = { $count ->
    [one] { $number } ramka poza kolejnością
    [few] { $number } ramki poza kolejnością
    [many] { $number } ramek poza kolejnością
   *[other] { $number } ramki poza kolejnością
}
diag-media-resets = { $count ->
    [one] { $number } reset mediów
    [few] { $number } resety mediów
    [many] { $number } resetów mediów
   *[other] { $number } resetu mediów
}
diag-timestamps-uncertain = { $count ->
    [0] znaczniki czasu pewne
    [1] niepewne znaczniki czasu raz
    [one] niepewne znaczniki czasu { $number } raz
    [few] niepewne znaczniki czasu { $number } razy
    [many] niepewne znaczniki czasu { $number } razy
   *[other] niepewne znaczniki czasu { $number } razy
}
diag-no-timestamp = { $count ->
    [one] { $number } ramka bez znacznika czasu
    [few] { $number } ramki bez znacznika czasu
    [many] { $number } ramek bez znacznika czasu
   *[other] { $number } ramki bez znacznika czasu
}
diag-unsupported-format = { $count ->
    [one] { $number } ramka w nieobsługiwanym formacie
    [few] { $number } ramki w nieobsługiwanym formacie
    [many] { $number } ramek w nieobsługiwanym formacie
   *[other] { $number } ramki w nieobsługiwanym formacie
}
diag-late = { $count ->
    [one] { $number } spóźniona ramka
    [few] { $number } spóźnione ramki
    [many] { $number } spóźnionych ramek
   *[other] { $number } spóźnionej ramki
}
diag-early = { $count ->
    [one] { $number } przedwczesna ramka
    [few] { $number } przedwczesne ramki
    [many] { $number } przedwczesnych ramek
   *[other] { $number } przedwczesnej ramki
}
diag-started = { $count ->
    [0] bez uruchomień
    [one] { $number } uruchomienie
    [few] { $number } uruchomienia
    [many] { $number } uruchomień
   *[other] { $number } uruchomienia
}
diag-stopped = { $count ->
    [0] bez zatrzymań
    [one] { $number } zatrzymanie
    [few] { $number } zatrzymania
    [many] { $number } zatrzymań
   *[other] { $number } zatrzymania
}
diag-reservation-failed = rezerwacja talkera nie powiodła się: { $reason }
diag-latency = skumulowane opóźnienie { $microseconds } µs

## AVB Lite

lite-active = Aktywny
lite-active-untagged = Aktywny, bez tagu
lite-active-vlan = Aktywny, VLAN { $vlan }
lite-capable = Obsługiwany
lite-mode = Tryb
lite-mode-capable = AVB (obsługuje AVB Lite)
lite-because = Przyczyna
lite-fallback-none = nie podano przyczyny
lite-fallback-endpoint = dotarła deklaracja innego urządzenia końcowego, więc nie ma między nimi przełącznika AVB
lite-fallback-unanswered = dziewięć żądań peer delay pozostało bez odpowiedzi
lite-fallback-responders = na jedno żądanie peer delay odpowiedziały co najmniej dwa urządzenia, więc przełącznik nie obsługuje AVB
lite-fallback-configured = ustawił to operator lub kontroler
lite-fallback-other = przyczyna, której profil nie wymienia
lite-other-profile = Inny profil
lite-ptp-domain = { $profile }, domena { $domain }
lite-offset = Przesunięcie
lite-offset-from = { $offset } od { $grandmaster }
lite-media-vlan = VLAN mediów
lite-untagged = Bez tagu
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Do { $count } listenera na strumień, potem multicast
    [few] Do { $count } listenerów na strumień, potem multicast
    [many] Do { $count } listenerów na strumień, potem multicast
   *[other] Do { $count } listenera na strumień, potem multicast
}
lite-link = Łącze
lite-bandwidth = Pasmo
lite-egress-of = { $used } z { $link }, { $share }
lite-egress-of-assumed = { $used } z { $link }, { $share }, przyjęto łącze gigabitowe
lite-egress-reported = Według zliczeń encji dla jej dopuszczonych strumieni.
lite-egress-worked-out = Obliczono z formatów jej połączonych wyjść strumieni.
lite-alarm-offset = Przesunięcie PTP { $offset }, powyżej 50 µs dopuszczanych przez AVB Lite
lite-alarm-egress = Ruch wychodzący na poziomie { $share } łącza, powyżej { $limit } dopuszczalnych dla strumieni

## AVB Wireless

wireless-station = Stacja
wireless-access-point = Punkt dostępowy
wireless-role = Rola
wireless-mode = Tryb
wireless-time = Czas
wireless-mode-a-ftm = Mode A, 802.1AS przez FTM
wireless-mode-a-tm = Mode A, 802.1AS przez TM
wireless-mode-b = Mode B, z ramek beacon
wireless-no-time = Bez synchronizacji czasu
wireless-other-mode = Tryb, którego profil nie wymienia
wireless-locked = Zsynchronizowano
wireless-holdover = Tryb holdover
wireless-not-locked = Brak synchronizacji
wireless-row-locked = Stacja, zsynchronizowana
wireless-row-holdover = Stacja, tryb holdover
wireless-row-not-locked = Stacja, bez synchronizacji
wireless-row-access-point = { $count ->
    [one] Punkt dostępowy, { $count } stacja
    [few] Punkt dostępowy, { $count } stacje
    [many] Punkt dostępowy, { $count } stacji
   *[other] Punkt dostępowy, { $count } stacji
}
wireless-link = Łącze
wireless-channel = kanał { $channel }
wireless-not-known = Nieznane
wireless-signal = Sygnał
wireless-rate = Szybkość
wireless-ftm-valid = { $share } poprawnych
wireless-rtt = czas obiegu { $rtt }
wireless-bursts = { $count ->
    [one] serie po { $count } ramce
    [few] serie po { $count } ramki
    [many] serie po { $count } ramek
   *[other] serie po { $count } ramki
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = punkt dostępowy przydziela serie FTM o długości innej niż trzy lub dwie ramki
wireless-reason-measurement = ani FTM, ani TM z punktem dostępowym
wireless-reason-signaling = brak komunikatu Signaling gPTP-capable od punktu dostępowego
wireless-reason-other = przyczyna, której profil nie wymienia
wireless-servo = Błąd serwa
wireless-stations = Stacje
wireless-station-count = { $count ->
    [one] { $count } stacja
    [few] { $count } stacje
    [many] { $count } stacji
   *[other] { $count } stacji
}
wireless-no-ftm = bez FTM
wireless-unserved = Nieobsłużone listenery
wireless-frame-budget = Budżet ramek
wireless-frame-use = { $budget ->
    [one] { $used } z { $budget } ramki na sekundę
    [few] { $used } z { $budget } ramek na sekundę
    [many] { $used } z { $budget } ramek na sekundę
   *[other] { $used } z { $budget } ramki na sekundę
}
wireless-stream-frames = Ramki strumieni
wireless-frames-of = do stacji: { $readdressed }, bez listenera: { $unmapped }, odrzucone: { $dropped }, od stacji: { $restored }
wireless-class-a-allowed = Dozwolona
wireless-class-a-not-allowed = Niedozwolona
wireless-alarm-not-locked = Czas Wi-Fi nie jest zsynchronizowany z punktem dostępowym
wireless-alarm-holdover = Czas Wi-Fi w trybie holdover, utracono synchronizację z punktem dostępowym
wireless-alarm-unserved = { $count ->
    [one] { $count } listener na porcie Wi-Fi bez obsługi, ponad limit unicast
    [few] { $count } listenery na porcie Wi-Fi bez obsługi, ponad limit unicast
    [many] { $count } listenerów na porcie Wi-Fi bez obsługi, ponad limit unicast
   *[other] { $count } listenera na porcie Wi-Fi bez obsługi, ponad limit unicast
}

## Log

log-all = Wszystkie
log-warnings = Ostrzeżenia
log-pause = Wstrzymaj
log-resume = Wznów
log-clear = Wyczyść
log-empty = Tutaj pojawia się każda ramka ATDECC, którą triib wysyła i słyszy, od najnowszej.
log-none-match = Żadna zachowana ramka nie pasuje do filtra.
log-frames = { $count ->
    [one] { $count } ramka
    [few] { $count } ramki
    [many] { $count } ramek
   *[other] { $count } ramki
}
log-shown-of = { $all ->
    [one] { $shown } z { $all } ramki
    [few] { $shown } z { $all } ramek
    [many] { $shown } z { $all } ramek
   *[other] { $shown } z { $all } ramki
}
log-sent = Wysłano
log-heard = Odebrano
log-not-decoded = Nie zdekodowano
log-warning-short = { $missing ->
    [one] Pole control_data_length wykracza o { $missing } oktet poza koniec ramki.
    [few] Pole control_data_length wykracza o { $missing } oktety poza koniec ramki.
    [many] Pole control_data_length wykracza o { $missing } oktetów poza koniec ramki.
   *[other] Pole control_data_length wykracza o { $missing } oktetu poza koniec ramki.
}
log-warning-undecodable = Nie da się zdekodować ramki: { $error }.
log-warning-long-acmp = Ramka ma długą postać ACMP, której encja Milan nie może wysyłać (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Mapowanie kanałów
mapping-inputs = Wejścia
mapping-outputs = Wyjścia
mapping-port = port { $number }
mapping-fixed = stałe
mapping-not-read = Jeszcze nie odczytano.
mapping-no-clusters = Brak klastrów.
mapping-no-streams = Brak strumieni audio.
mapping-none = Brak mapowań.
mapping-not-mapped = Nie zmapowano
mapping-cluster-numbered = Klaster { $index }

## Presets

presets-note = Preset zachowuje źródła zegara, częstotliwości próbkowania, formaty strumieni, regulatory i połączenia każdej encji. Przywołanie zmienia to, co się różni.
presets-none = Brak zapisanych presetów.
presets-connections = { $count ->
    [one] { $count } połączenie
    [few] { $count } połączenia
    [many] { $count } połączeń
   *[other] { $count } połączenia
}
presets-recall = Przywołaj
presets-delete = Usuń
presets-no-place = Nie ma gdzie zapisać presetów: folder domowy jest nieznany.
presets-undeletable = Nie udało się usunąć { $path }: { $error }.
presets-saved = { $count ->
    [one] Zapisano „{ $name }” z { $count } encją.
    [few] Zapisano „{ $name }” z { $count } encjami.
    [many] Zapisano „{ $name }” z { $count } encjami.
   *[other] Zapisano „{ $name }” z { $count } encji.
}
presets-nothing-differs = Nic nie różni się od „{ $name }”.
presets-recalling = { $count ->
    [one] Przywoływanie „{ $name }”: { $count } zmiana.
    [few] Przywoływanie „{ $name }”: { $count } zmiany.
    [many] Przywoływanie „{ $name }”: { $count } zmian.
   *[other] Przywoływanie „{ $name }”: { $count } zmiany.
}
presets-missing = { $report } Brak w sieci lub nie odczytano: { $missing }.
presets-deleted = Usunięto „{ $name }”.
presets-host-note = Zachowuje też własne talkery i listenery tego komputera i uruchamia je ponownie przy przywołaniu.
presets-host-endpoints = { $count } na tym komputerze
presets-starting-host = Uruchamianie talkerów i listenerów tego komputera dla „{ $name }”; reszta nastąpi, gdy wrócą.

## Controls

control-numbered = Regulator { $index }
control-not-shown = Nie pokazano tutaj
control-option = Opcja { $number }

## Network errors

network-permission = triib potrzebuje uprawnień do wysyłania i odbierania surowych ramek Ethernet.
network-needs-npcap = triib potrzebuje Npcap do wysyłania i odbierania surowych ramek Ethernet.
network-npcap-administrators = Npcap pozwala wysyłać i odbierać surowe ramki Ethernet tylko administratorom. Uruchom triib jako administrator albo zainstaluj Npcap ponownie bez opcji tylko dla administratorów.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own talkers and listeners

host-add-talker = Dodaj talker
host-add-listener = Dodaj listener
host-add-device = Dodaj urządzenie
host-show-mine = Pokaż tylko własne talkery i listenery tego komputera
host-show-all = Pokaż wszystkie encje
host-new-talker = Talker hosta { $number }
host-new-listener = Listener hosta { $number }
host-new-device = Urządzenie hosta { $number }
host-failed = Nie można dodać do tego komputera: { $reason }
host-needs-clock = Własne talkery i listenery tego komputera wymagają przewodowego interfejsu ze sprzętowym zegarem PTP
host-no-ptp4l = ptp4l nie odpowiada, więc strumienie tego komputera nie mogą trzymać czasu gPTP
host-elsewhere = triib-endpointd działa na { $interface } dla innego użytkownika lub jako root, więc talkery i listenery tego komputera działają tam, nie tutaj
host-foreign-mrp = Inny program deklaruje MSRP lub MVRP na { $interface } z adresu tego komputera i może wycofać to, czego potrzebują strumienie tego komputera
host-alarm-foreign-mrp = Inny program na tym komputerze deklaruje MSRP lub MVRP na { $interface }
host-state = Stan
host-streaming = Nadaje
host-waiting = Czeka na listener
host-listening = Nasłuchuje
host-bound = Powiązany, czeka na talker
host-unbound = Niepowiązany
host-audio-from = Dźwięk z
host-audio-to = Dźwięk do
host-channels = Kanały
host-virtual-device = Urządzenie wirtualne
host-add-output-stream = Dodaj strumień wyjściowy
host-add-input-stream = Dodaj strumień wejściowy
host-remove-stream = Usuń strumień
host-silence = Cisza
host-tone = Ton testowy
host-nowhere = Donikąd
host-default-device = Urządzenie domyślne
host-remove = Usuń z tego komputera
