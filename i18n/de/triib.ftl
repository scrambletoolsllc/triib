# triib's interface text in German.

## Language

language-name = Deutsch

## Common

common-close = Schließen
common-more = Mehr
common-keep-toolbar-shown = Werkzeugleiste immer anzeigen
common-auto-hide-toolbar = Werkzeugleiste automatisch ausblenden

## Settings

settings-title = Einstellungen
settings-general = Allgemein
settings-appearance = Darstellung
settings-language = Sprache
settings-language-system = Systemstandard: { $language }
settings-language-note = Textfelder verwenden die Eingabesprache des Systems.
settings-appearance-system = System
settings-appearance-light = Hell
settings-appearance-dark = Dunkel
settings-colors = Farben
settings-system-accent = Akzentfarbe des Systems verwenden
settings-accent-picked = Die Farbe unten bestimmt die Farben von triib.
settings-accent-omarchy = Aus dem Omarchy-Theme { $theme }.
settings-accent-desktop = Aus der Akzentfarbe des Desktops.
settings-accent-none = Der Desktop hat keine Akzentfarbe, daher wird die Farbe unten verwendet.
settings-motion = Bewegung
settings-animations = Animationen
settings-animations-note = Federnde und gleitende Übergänge, wenn sich etwas ändert.
settings-animations-reduced = Der Desktop wünscht reduzierte Bewegung, daher bleibt triib ruhig.
settings-outside-control = Steuerung von außen
settings-allow-outside-control = Steuerung von außen erlauben
settings-allow-outside-control-note = Agenten wie Claude Code können über triib --mcp Ihr AVB-Netzwerk sehen und ändern. triib fragt vor jedem neuen Agenten nach.
settings-allowed-agents = Erlaubt: { $agents }
settings-forget-agents = Vergessen
agent-prompt-title = { $agent } erlauben, triib zu steuern?
agent-prompt-body = { $agent } möchte die Steuerung von außen in triib nutzen: die Entitäten in Ihrem Netzwerk sehen und ändern, Streams verbinden und Formate, Clocks und Namen einstellen. Sie können die Steuerung von außen in den Einstellungen ausschalten.
agent-prompt-allow = Erlauben
agent-prompt-deny = Nicht erlauben

common-cancel = Abbrechen
common-save = Speichern
common-not-set = Nicht gesetzt
common-unnamed = Unbenannt
common-none = Keine
common-mac-address = MAC-Adresse
common-list-separator = {", "}

## Network interfaces

interface-up = aktiv
interface-link-down = Link getrennt
interface-wireless = drahtlos
interface-hardware-clock = Hardware-Clock
interface-hardware-clock-named = Hardware-Clock { $clock }
interface-virtual = virtuell

## Toolbar

toolbar-choose-interface = Schnittstelle wählen
toolbar-interface = Netzwerkschnittstelle
toolbar-show-virtual = Virtuelle Schnittstellen anzeigen
toolbar-hide-virtual = Virtuelle Schnittstellen ausblenden
toolbar-connections = Verbindungen
toolbar-network = Netzwerk
toolbar-entities = Entitäten
toolbar-rediscover = Alle Entitäten auffordern, sich anzukündigen
toolbar-rescan = Alle Entitäten verwerfen und neu einlesen
toolbar-search = Entitäten und Streams suchen
toolbar-presets = Presets
toolbar-log = Log
toolbar-inspector = Inspektor
toolbar-settings = Einstellungen

## The network's state, in place of a view

state-no-interface = Keine Schnittstelle
state-no-interface-note = Schnittstelle im AVB-Netzwerk wählen, um Entitäten zu erkennen.
state-starting = Wird gestartet
state-starting-note = { $interface } wird geöffnet.
state-listening = Empfangsbereit
state-listening-note = Entitäten auf { $interface } erscheinen hier, sobald sie sich ankündigen.
state-permission-needed = Berechtigung erforderlich
state-npcap-needed = Npcap erforderlich
state-get-npcap = Npcap herunterladen
state-copy-command = Befehl kopieren
state-cannot-use = { $interface } kann nicht verwendet werden
state-try-again = Erneut versuchen

## Entity list

entities-none-yet = Noch keine Entitäten
entities-none-yet-note = Jede Entität im Netzwerk, mit ihren Rollen, SR-Klassen und ihrer Clock.

## Inspector

inspector-title = Inspektor
inspector-entity = Entität
inspector-streams = Streams
inspector-controls = Bedienelemente
inspector-diagnostics = Diagnose
inspector-descriptors = Deskriptoren
inspector-select = Entität auswählen, um ihre Details zu sehen.
inspector-offline = { $entity } ist offline.
inspector-rename = Umbenennen
inspector-name = Name
inspector-identify = Identifizieren
inspector-model-not-read = Das Entitätsmodell wurde nicht gelesen.
inspector-no-streams = Keine Streams.
inspector-no-controls = Keine Bedienelemente vorhanden.
inspector-no-diagnostics = Keine Schnittstellen oder Zähler gemeldet.
inspector-reading = Deskriptoren werden gelesen, bisher { $count }.
inspector-read-failed = Das Entitätsmodell konnte nicht gelesen werden: { $reason }.

entity-section = Entität
entity-name = Name
entity-group = Gruppe
entity-product = Produkt
entity-firmware = Firmware
entity-serial-number = Seriennummer
entity-configuration = Konfiguration
entity-configuration-of = { $name } ({ $number } von { $count })
entity-milan = Milan
entity-media-clock = Media-Clock
entity-clock-domain = Clock-Domäne
entity-sampling-rate = Abtastrate
clock-source-numbered = Quelle { $index }
rate-pull = Pull { $pull }

stream-format = Format
stream-id = Stream-ID
stream-destination = Ziel
stream-max-transit = Max. Laufzeit

avb-interfaces = AVB-Schnittstellen
avb-interface = Schnittstelle
avb-interface-clock-identity = Clock-Identität
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, Domäne { $domain }
avb-interface-peer-delay = Peer-Delay
avb-interface-running = Aktiv
avb-interface-none-reported = Keine gemeldet
avb-interface-path = Pfad
avb-interface-own-grandmaster = Selbst Grandmaster
avb-interface-hops = { $count ->
    [one] { $count } Hop vom Grandmaster
   *[other] { $count } Hops vom Grandmaster
}
avb-interface-link-up = Link verbunden
avb-interface-link-down = Link getrennt
avb-interface-grandmaster-changes = Grandmaster-Wechsel
avb-interface-frames-sent = Gesendete Frames
avb-interface-frames-received = Empfangene Frames
avb-interface-crc-errors = CRC-Fehler

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } Deskriptortyp
   *[other] { $count } Deskriptortypen
}
tree-clock = Clock
tree-clock-source-from = { $kind }, von { $location } { $index }
tree-clock-domain-using = Nutzt { $source }
tree-clusters = { $count ->
    [one] { $count } Cluster
   *[other] { $count } Cluster
}
tree-maps = { $count ->
    [one] { $count } Zuordnung
   *[other] { $count } Zuordnungen
}

advert-not-advertised = Nicht angekündigt
advert-identity = Identität
advert-entity-id = Entitäts-ID
advert-entity-model = Entitätsmodell
advert-roles = Rollen
advert-talker = Talker
advert-listener = Listener
advert-clock = Clock
advert-btc = BTC
advert-gptp-domain = gPTP-Domäne
advert-sr-classes = SR-Klassen
advert-indexes = Indizes im Entitätsmodell
advert-identify-control = Identifizierung
advert-avb-interface = AVB-Schnittstelle
advert-advertising = Ankündigung
advert-valid-time = Gültigkeitsdauer
advert-available-index = Verfügbarkeitsindex
advert-association = Assoziation
advert-capabilities = Fähigkeiten

## Status bar

status-entities = { $count ->
    [one] { $count } Entität
   *[other] { $count } Entitäten
}
status-not-discovering = Keine Erkennung
status-discovering = Erkennung läuft
status-discovering-as = Erkennung läuft als { $controller }
status-stopped = Durch einen Fehler gestoppt
status-alarm = Alarm
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } und { $count } weitere

## Descriptions of entities, shared by the views

role-talker = Talker { $count }
role-listener = Listener { $count }
role-controller = Controller
role-none = keine Rollen
classes-a-and-b = A und B
clock-no-gptp = Kein gPTP

read-not-read = Nicht gelesen
read-reading = Wird gelesen, bisher { $count }
read-ready-unreadable = Bereit, { $count } nicht lesbar
read-ready-cached = Bereit, aus dem Cache
read-ready = Bereit
read-failed = Fehlgeschlagen: { $reason }

milan-no = Nein
milan-before-1-3 = vor 1.3
milan-certified = { $version }, zertifiziert { $certification }
milan-not-certified = { $version }, nicht zertifiziert

outcome-status = Status { $status }
outcome-no-response = keine Antwort
outcome-not-possible = nicht möglich
outcome-connect = { $talker } konnte nicht mit { $listener } verbunden werden: { $reason }.
outcome-disconnect = { $listener } konnte nicht getrennt werden: { $reason }.
outcome-identify = { $entity } konnte nicht identifiziert werden: { $reason }.
outcome-rename = { $what } konnte nicht in „{ $name }“ umbenannt werden: { $reason }.
outcome-rename-group = Die Gruppe von { $entity } konnte nicht in „{ $name }“ umbenannt werden: { $reason }.
outcome-format-streaming = Das Format von { $stream } konnte nicht geändert werden: Der Stream läuft. Zuerst trennen.
outcome-format = Das Format von { $stream } konnte nicht geändert werden: { $reason }.
outcome-max-transit-time = Die max. Laufzeit von { $stream } konnte nicht geändert werden: { $reason }.
outcome-max-transit-time-streaming = Die max. Laufzeit von { $stream } konnte nicht geändert werden: Der Stream läuft. Zuerst trennen.
outcome-sampling-rate = Die Abtastrate von { $entity } konnte nicht geändert werden: { $reason }.
outcome-clock-source = Die Clock-Quelle von { $entity } konnte nicht geändert werden: { $reason }.
outcome-map = Der Kanal auf { $entity } konnte nicht zugeordnet werden: { $reason }.
outcome-unmap = Die Kanalzuordnung auf { $entity } konnte nicht aufgehoben werden: { $reason }.
outcome-control = „{ $control }“ auf { $entity } konnte nicht gesetzt werden: { $reason }.
outcome-control-numbered = Bedienelement { $index } auf { $entity } konnte nicht gesetzt werden: { $reason }.

stream-not-connected = Nicht verbunden
stream-from = Von { $stream }
stream-from-receiving = Von { $stream }, Empfang läuft
stream-from-waiting = Von { $stream }, wartet auf den Talker
stream-from-failed = Von { $stream }, Reservierung des Talkers fehlgeschlagen: { $reason }
stream-sending-to = Sendet an { $destination }

failure-no-response = keine Antwort
failure-refused = abgelehnt mit { $status }
failure-malformed = Antwort nicht dekodierbar
failure-on-this-computer = läuft auf diesem Computer; von einem anderen Computer aus lesen

msrp-failure-1 = unzureichende Bandbreite
msrp-failure-2 = unzureichende Switch-Ressourcen
msrp-failure-3 = unzureichende Bandbreite für die Verkehrsklasse
msrp-failure-4 = Stream-ID von einem anderen Talker belegt
msrp-failure-5 = Zieladresse bereits belegt
msrp-failure-6 = von einem Stream höheren Rangs verdrängt
msrp-failure-7 = gemeldete Latenz hat sich geändert
msrp-failure-8 = Egress-Port ist nicht AVB-fähig
msrp-failure-9 = andere Zieladresse verwenden
msrp-failure-10 = MSRP-Ressourcen erschöpft
msrp-failure-11 = MMRP-Ressourcen erschöpft
msrp-failure-12 = Zieladresse kann nicht gespeichert werden
msrp-failure-13 = Priorität ist keine SR-Klassen-Priorität
msrp-failure-14 = Frames zu groß für das Medium
msrp-failure-15 = Fan-in-Grenze des Ports erreicht
msrp-failure-16 = erster Wert eines registrierten Streams geändert
msrp-failure-17 = VLAN am Egress-Port blockiert
msrp-failure-18 = VLAN-Tagging am Egress-Port deaktiviert
msrp-failure-19 = SR-Klassen-Priorität stimmt nicht überein
msrp-failure-unknown = unbekannter Grund
msrp-failure-at = { $reason }, am Switch { $bridge }

## Entity list columns

column-vendor = Hersteller
column-model = Modell
column-state = Status
column-entity-model-id = Entitätsmodell-ID
column-talker-streams = Talker-Streams
column-listener-streams = Listener-Streams
column-avb-lite = AVB Lite
column-egress = Egress
column-wireless = WLAN

## Settings file

settings-no-place = Die Einstellungen können nirgends gespeichert werden: Der Home-Ordner ist nicht bekannt.
settings-unusable = { $path } konnte nicht verwendet werden: { $error }.
settings-unsaved = { $path } konnte nicht gespeichert werden: { $error }.

column-remove = Spalte entfernen
column-move-left = Nach links
column-move-right = Nach rechts
column-add = Spalte hinzufügen
common-percent = { $value }{" "}%

## Network view

netmap-empty = Noch kein Netzwerk anzuzeigen
netmap-empty-note = Entitäten erscheinen hier, sobald sie gelesen wurden und ihre Position im gPTP-Baum gemeldet haben.
netmap-focus-clock-path = Clock-Pfad von { $name }
netmap-focus-streams = Streams von { $name }
netmap-showing = Zeigt { $what }
netmap-devices = { $count ->
    [one] { $count } Gerät
   *[other] { $count } Geräte
}
netmap-bridges = { $count ->
    [one] { $count } Switch
   *[other] { $count } Switches
}
netmap-show-map = Karte anzeigen
netmap-show-details = Details anzeigen
stream-numbered = Stream { $index }
netmap-bridge = Switch
netmap-access-point = Access Point
netmap-device = Gerät
netmap-this-computer = Dieser Computer
netmap-connected = Verbunden
netmap-advertised = Angekündigt, kein Listener bereit
netmap-advertised-off-tree = Angekündigt, kein Listener bereit ({ $listener } ist nicht im gPTP-Baum)
netmap-failed-at = Reservierung an { $bridge } fehlgeschlagen: { $reason }
netmap-failed = Reservierung fehlgeschlagen: { $reason }
netmap-no-bridge-on = Kein Switch auf { $interface } erkannt
netmap-cannot-listen-on = gPTP auf { $interface } kann nicht empfangen werden
netmap-on-this-computer = Auf diesem Computer
netmap-path-not-reported = Pfad nicht gemeldet
netmap-gptp-not-reported = gPTP nicht gemeldet
netmap-off-tree = Nicht im gPTP-Baum
netmap-off-ptp = Nicht im PTP-Baum
netmap-not-lite = Nicht in AVB Lite
netmap-lite-not-reported = AVB Lite nicht gemeldet
netmap-synced = Synchronisiert
netmap-not-synced = Nicht synchronisiert
netmap-triib-on = triib auf { $interface }
netmap-through-count = { $count } durchgeleitet
netmap-out = { $count } ausgehend
netmap-in = { $count } eingehend
netmap-failed-count = { $count } fehlgeschlagen
netmap-advertised-only = Nur angekündigt
netmap-failed-state = Fehlgeschlagen
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Nicht im gPTP-Baum: selbst Grandmaster
netmap-apart-no-path = Pfad nicht gemeldet; folgt Grandmaster { $grandmaster }
netmap-apart-unreported = gPTP-Zustand nicht gemeldet
netmap-apart-no-neighbor = Kein Switch an der Schnittstelle dieses Computers erkannt
netmap-apart-cannot-listen = Dieser Computer kann gPTP an seiner Schnittstelle nicht empfangen
netmap-apart-on-this-computer = Läuft auf diesem Computer; von einem anderen Computer aus lesen, um den gPTP-Zustand zu sehen
netmap-apart-not-lite = Nutzt kein AVB Lite und folgt daher nicht dem Grandmaster
netmap-apart-lite-unreported = Meldet nichts zu AVB Lite, daher ist unbekannt, wem es folgt
netmap-clock-tree = Clock-Baum
netmap-no-grandmaster = Kein Grandmaster erkannt
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Erfordert Aufmerksamkeit
netmap-nodes-below = Knoten darunter
netmap-bridges-below = Switches darunter
netmap-clock-path = Clock-Pfad
netmap-hops = Hops vom Grandmaster
netmap-link-delay = Link-Verzögerung
netmap-bridge-port = Switch-Port
netmap-link-drops = Link-Abbrüche
netmap-synced-to-grandmaster = Mit dem Grandmaster synchronisiert
netmap-host-no-gptp = Nicht synchronisiert: Dieser Computer nutzt kein gPTP
netmap-link-no-gptp = Nicht synchronisiert: Auf dem Link läuft kein gPTP
netmap-ptp-offset-high = Nicht synchronisiert: { $offset } zum Grandmaster, mehr als die 50 µs, die AVB Lite erlaubt
netmap-ptp-no-offset = Nicht synchronisiert: Hat keinen Offset zum Grandmaster gemessen
netmap-audio = Audio
netmap-media-clock-streams = Media-Clock-Streams
netmap-audio-streams = Audio-Streams
netmap-bound = { $count } gebunden
netmap-flowing = Fließt
netmap-advertised-state = Angekündigt
netmap-media-clock-stream = Media-Clock-Stream
netmap-audio-stream = Audio-Stream
netmap-reaches = Reicht bis
netmap-passing-count = { $count ->
    [one] { $count } durchgeleiteter Stream
   *[other] { $count } durchgeleitete Streams
}
netmap-through = Durchgeleitet
netmap-passing-through = Durchgeleitete Streams
netmap-sending = Sendet
netmap-receiving = Empfängt
netmap-problems = Probleme
netmap-help-back = Hintergrund anklicken, um zur Übersicht zurückzukehren.
netmap-help-stream = Stream anklicken, um ihn zu untersuchen, oder Hintergrund, um zur Übersicht zurückzukehren.
netmap-help-ptp = In AVB Lite läuft die Clock Ende-zu-Ende vom Grandmaster zu jedem Gerät, über Switches, die nicht beteiligt sind; deshalb wird keiner angezeigt. Ein Gerät ist synchronisiert, solange es dem Grandmaster innerhalb von 50 µs folgt. Ein Gerät oder seine Leitung anklicken, um seine Clock zu untersuchen; Hintergrund anklicken, um die Auswahl aufzuheben.
netmap-help-clock = Die Clock läuft vom Grandmaster über jeden Switch zu jedem Knoten im Baum. Eine gestrichelte graue Linie ist ein Link, auf dem kein gPTP läuft. Ein Gerät oder seine Leitung anklicken, um seinen Clock-Pfad zu untersuchen; Hintergrund anklicken, um die Auswahl aufzuheben.
netmap-help-media-clock = Nur Media-Clock-Streams (CRF), gezeichnet wie Audio: eine Leitung pro Stream, gefärbt nach Talker. Eine Leitung anklicken, um ihren Stream zu untersuchen, oder ein Gerät, um seine Streams zu sehen; Hintergrund anklicken, um die Auswahl aufzuheben.
netmap-help-audio = Jeder Stream hat seine eigene Leitung, die in jeden Switch, den er durchläuft, hinein- und wieder herausführt. Die Farbe richtet sich nach dem Talker: Jeder Talker hat einen Farbton, und seine Streams sind Schattierungen davon. Wandernde Punkte bedeuten, dass Audio fließt; eine stehende rote Linie ist eine fehlgeschlagene Reservierung, eine stehende graue Linie ist angekündigt, aber ohne bereiten Listener; beide enden dort, wo die Reservierung endet. Geräte in der mittleren Spalte sind direkt mit dem Switch des Grandmasters verbunden. Eine Leitung anklicken, um ihren Stream zu untersuchen, oder ein Gerät, um seine Streams zu sehen; Hintergrund anklicken, um die Auswahl aufzuheben.

## Connections

matrix-nothing-shown = Keine Streams anzuzeigen
matrix-nothing-shown-note = Suche oder Filter ändern, um mehr Streams zu sehen.
matrix-empty = Keine Streams zum Verbinden
matrix-empty-note = Talker-Streams und Listener-Streams treffen hier aufeinander, sobald Entitäten mit Streams gelesen wurden.
matrix-all-streams = Alle Streams
matrix-connectable-only = Nicht Verbindbares ausblenden
matrix-none-hidden = Jeder angezeigte Stream ist verbindbar
matrix-hidden = { $count ->
    [one] { $count } Stream ausgeblendet
   *[other] { $count } Streams ausgeblendet
}
matrix-own = Die Ausgänge einer Entität lassen sich nicht mit ihren eigenen Eingängen verbinden.
matrix-working = Wird bearbeitet.
matrix-waiting-change = Wartet auf die letzte Änderung an diesem Eingang.
matrix-connected = Verbunden, Empfang läuft. Zum Trennen klicken.
matrix-bound-waiting = Gebunden, wartet auf den Stream des Talkers. Zum Trennen klicken.
matrix-bound-failed = Gebunden, aber die Reservierung des Talkers ist fehlgeschlagen: { $reason }. Zum Trennen klicken.
matrix-bound-formats-differ = Gebunden, aber die Formate unterscheiden sich: Der Talker sendet { $sent }, der Eingang ist auf { $set } eingestellt. Zum Trennen klicken.
matrix-formats-match = Formate passen ({ $format }). Zum Verbinden klicken.
matrix-format-must-change = Der Eingang akzeptiert { $sent }, ist aber auf { $set } eingestellt; er spielt eventuell erst, wenn sein Format geändert wird. Klicken, um ein Format zu wählen.
matrix-format-input = Eingang ändern auf
matrix-format-both = Beide ändern auf
matrix-format-talker-busy = Der Talker streamt an andere Listener, daher bleibt sein Format.
matrix-format-anyway = Verbinden, ohne Formate zu ändern
matrix-incompatible = Der Eingang akzeptiert { $sent } nicht. Er ist auf { $set } eingestellt.
matrix-group-none = Nicht verbunden. Aufklappen, um Streams einzeln zu verbinden.
matrix-group-connected = { $count } verbunden. Aufklappen, um jede Verbindung zu sehen.
matrix-outputs-expand = { $count ->
    [one] { $count } Stream-Ausgang. Pfeil anklicken zum Aufklappen, Namen zum Untersuchen.
   *[other] { $count } Stream-Ausgänge. Pfeil anklicken zum Aufklappen, Namen zum Untersuchen.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } Stream-Ausgang. Pfeil anklicken zum Zuklappen, Namen zum Untersuchen.
   *[other] { $count } Stream-Ausgänge. Pfeil anklicken zum Zuklappen, Namen zum Untersuchen.
}
matrix-inputs-expand = { $count ->
    [one] { $count } Stream-Eingang. Pfeil anklicken zum Aufklappen, Namen zum Untersuchen.
   *[other] { $count } Stream-Eingänge. Pfeil anklicken zum Aufklappen, Namen zum Untersuchen.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } Stream-Eingang. Pfeil anklicken zum Zuklappen, Namen zum Untersuchen.
   *[other] { $count } Stream-Eingänge. Pfeil anklicken zum Zuklappen, Namen zum Untersuchen.
}
matrix-stream-inspect = { $detail } Klicken, um { $entity } zu untersuchen.
matrix-point = Auf eine Zelle zeigen
matrix-point-note = um ihren Talker und Listener zu sehen und ob ihre Formate zusammenpassen.
matrix-legend-waiting = Gebunden, wartet auf den Stream
matrix-legend-trouble = Gebunden, etwas stimmt nicht
matrix-legend-open = Verbindbar
matrix-legend-change = Eingangsformat muss erst geändert werden
matrix-legend-incompatible = Formate passen nicht zusammen
matrix-talker-outputs = Talker-Ausgänge
matrix-listener-inputs = Listener-Eingänge

common-thousands-separator = {"."}

## Diagnostics

diag-since-start = Gezählt seit dem Start der Entität.
diag-stream-input = Stream-Eingang
diag-stream-output = Stream-Ausgang
diag-locked = { $count ->
    [0] nicht eingerastet
    [one] einmal eingerastet
    [2] zweimal eingerastet
   *[other] { $number }-mal eingerastet
}
diag-lost-lock = { $count ->
    [0] Einrastung nicht verloren
    [one] Einrastung einmal verloren
    [2] Einrastung zweimal verloren
   *[other] Einrastung { $number }-mal verloren
}
diag-frames-in = { $count ->
    [one] { $number } Frame empfangen
   *[other] { $number } Frames empfangen
}
diag-frames-out = { $count ->
    [one] { $number } Frame gesendet
   *[other] { $number } Frames gesendet
}
diag-media-locked = { $count ->
    [0] nicht auf Media-Clock eingerastet
    [one] einmal auf Media-Clock eingerastet
    [2] zweimal auf Media-Clock eingerastet
   *[other] { $number }-mal auf Media-Clock eingerastet
}
diag-lost-media-lock = { $count ->
    [0] Media-Clock-Einrastung nicht verloren
    [one] Media-Clock-Einrastung einmal verloren
    [2] Media-Clock-Einrastung zweimal verloren
   *[other] Media-Clock-Einrastung { $number }-mal verloren
}
diag-interrupted = { $count ->
    [0] nicht unterbrochen
    [one] einmal unterbrochen
    [2] zweimal unterbrochen
   *[other] { $number }-mal unterbrochen
}
diag-out-of-sequence = { $count ->
    [one] { $number } Frame in falscher Reihenfolge
   *[other] { $number } Frames in falscher Reihenfolge
}
diag-media-resets = { $count ->
    [one] { $number } Media-Reset
   *[other] { $number } Media-Resets
}
diag-timestamps-uncertain = { $count ->
    [0] Zeitstempel nicht unsicher
    [one] Zeitstempel einmal unsicher
    [2] Zeitstempel zweimal unsicher
   *[other] Zeitstempel { $number }-mal unsicher
}
diag-no-timestamp = { $count ->
    [one] { $number } Frame ohne Zeitstempel
   *[other] { $number } Frames ohne Zeitstempel
}
diag-unsupported-format = { $count ->
    [one] { $number } Frame in nicht unterstütztem Format
   *[other] { $number } Frames in nicht unterstütztem Format
}
diag-late = { $count ->
    [one] { $number } Frame zu spät
   *[other] { $number } Frames zu spät
}
diag-early = { $count ->
    [one] { $number } Frame zu früh
   *[other] { $number } Frames zu früh
}
diag-started = { $count ->
    [0] nicht gestartet
    [one] einmal gestartet
    [2] zweimal gestartet
   *[other] { $number }-mal gestartet
}
diag-stopped = { $count ->
    [0] nicht gestoppt
    [one] einmal gestoppt
    [2] zweimal gestoppt
   *[other] { $number }-mal gestoppt
}
diag-reservation-failed = Reservierung des Talkers fehlgeschlagen: { $reason }
diag-latency = { $microseconds } µs akkumulierte Latenz

## AVB Lite

lite-active = Aktiv
lite-active-untagged = Aktiv, ungetaggt
lite-active-vlan = Aktiv, VLAN { $vlan }
lite-capable = Unterstützt
lite-mode = Modus
lite-mode-capable = AVB (AVB Lite unterstützt)
lite-because = Grund
lite-fallback-none = nicht angegeben
lite-fallback-endpoint = die Deklaration eines anderen Endpunkts kam durch, also liegt kein AVB-Switch dazwischen
lite-fallback-unanswered = neun Peer-Delay-Anfragen blieben unbeantwortet
lite-fallback-responders = zwei oder mehr Geräte beantworteten eine Peer-Delay-Anfrage, also ist der Switch nicht AVB-fähig
lite-fallback-configured = vom Bediener oder einem Controller eingestellt
lite-fallback-other = ein Grund, den das Profil nicht nennt
lite-other-profile = Anderes Profil
lite-ptp-domain = { $profile }, Domäne { $domain }
lite-offset = Offset
lite-offset-from = { $offset } zu { $grandmaster }
lite-media-vlan = Medien-VLAN
lite-untagged = Ungetaggt
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Bis zu { $count } Listener je Stream, dann Multicast
   *[other] Bis zu { $count } Listener je Stream, dann Multicast
}
lite-link = Link
lite-bandwidth = Bandbreite
lite-egress-of = { $used } von { $link }, { $share }
lite-egress-of-assumed = { $used } von { $link }, { $share }, Gigabit-Link angenommen
lite-egress-reported = So, wie die Entität ihre zugelassenen Streams zählt.
lite-egress-worked-out = Aus den Formaten ihrer verbundenen Stream-Ausgänge.
lite-alarm-offset = PTP-Offset { $offset }, mehr als die 50 µs, die AVB Lite erlaubt
lite-alarm-egress = Egress bei { $share } des Links, mehr als die { $limit }, die Streams belegen dürfen

## AVB Wireless

wireless-station = Station
wireless-access-point = Access Point
wireless-role = Rolle
wireless-mode = Modus
wireless-time = Zeit
wireless-mode-a-ftm = Mode A, 802.1AS über FTM
wireless-mode-a-tm = Mode A, 802.1AS über TM
wireless-mode-b = Mode B, aus Beacons
wireless-no-time = Keine Zeitsynchronisation
wireless-other-mode = Ein Modus, den das Profil nicht nennt
wireless-locked = Eingerastet
wireless-holdover = Im Holdover
wireless-not-locked = Nicht eingerastet
wireless-row-locked = Station, eingerastet
wireless-row-holdover = Station, im Holdover
wireless-row-not-locked = Station, nicht eingerastet
wireless-row-access-point = { $count ->
    [one] Access Point, { $count } Station
   *[other] Access Point, { $count } Stationen
}
wireless-link = Link
wireless-channel = Kanal { $channel }
wireless-not-known = Unbekannt
wireless-signal = Signal
wireless-rate = Datenrate
wireless-ftm-valid = { $share } gültig
wireless-rtt = Umlaufzeit { $rtt }
wireless-bursts = { $count ->
    [one] Bursts mit je { $count } Frame
   *[other] Bursts mit je { $count } Frames
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = der Access Point gewährt FTM-Bursts mit einer anderen Länge als drei oder zwei Frames
wireless-reason-measurement = weder FTM noch TM mit dem Access Point
wireless-reason-signaling = kein gPTP-fähiges Signaling vom Access Point
wireless-reason-other = ein Grund, den das Profil nicht nennt
wireless-servo = Servo-Fehler
wireless-stations = Stationen
wireless-station-count = { $count ->
    [one] { $count } Station
   *[other] { $count } Stationen
}
wireless-no-ftm = ohne FTM
wireless-unserved = Nicht bediente Listener
wireless-frame-budget = Frame-Budget
wireless-frame-use = { $used } von { $budget } Frames pro Sekunde
wireless-stream-frames = Stream-Frames
wireless-frames-of = { $readdressed } an Stationen, { $unmapped } ohne Listener, { $dropped } verworfen, { $restored } von Stationen
wireless-class-a-allowed = Erlaubt
wireless-class-a-not-allowed = Nicht erlaubt
wireless-alarm-not-locked = Wi-Fi-Zeit nicht auf den Access Point eingerastet
wireless-alarm-holdover = Wi-Fi-Zeit im Holdover, Bezug zum Access Point verloren
wireless-alarm-unserved = { $count ->
    [one] { $count } Listener am Wi-Fi-Port nicht bedient, über dem Unicast-Limit
   *[other] { $count } Listener am Wi-Fi-Port nicht bedient, über dem Unicast-Limit
}

## Log

log-all = Alle
log-warnings = Warnungen
log-pause = Pause
log-resume = Fortsetzen
log-clear = Leeren
log-empty = Jeder ATDECC-Frame, den triib sendet und empfängt, erscheint hier, neueste zuerst.
log-none-match = Kein gespeicherter Frame passt zum Filter.
log-frames = { $count ->
    [one] { $count } Frame
   *[other] { $count } Frames
}
log-shown-of = { $shown } von { $all } Frames
log-sent = Gesendet
log-heard = Empfangen
log-not-decoded = Nicht dekodiert
log-warning-short = Das Feld control_data_length gibt { $missing } Oktette über das Ende des Frames hinaus an.
log-warning-undecodable = Nicht dekodierbar: { $error }.
log-warning-long-acmp = In der langen ACMP-Form, die eine Milan-Entität nicht senden darf (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Kanalzuordnungen
mapping-inputs = Eingänge
mapping-outputs = Ausgänge
mapping-port = Port { $number }
mapping-fixed = fest
mapping-not-read = Noch nicht gelesen.
mapping-no-clusters = Keine Cluster.
mapping-no-streams = Keine Audio-Streams.
mapping-none = Keine Zuordnungen.
mapping-not-mapped = Nicht zugeordnet
mapping-cluster-numbered = Cluster { $index }

## Presets

presets-note = Ein Preset speichert für jede Entität Clock-Quellen, Abtastraten, Stream-Formate, Bedienelemente und Verbindungen. Beim Abrufen wird geändert, was abweicht.
presets-none = Noch keine Presets gespeichert.
presets-connections = { $count ->
    [one] { $count } Verbindung
   *[other] { $count } Verbindungen
}
presets-recall = Abrufen
presets-delete = Löschen
presets-no-place = Presets können nirgends gespeichert werden: Der Home-Ordner ist nicht bekannt.
presets-undeletable = { $path } konnte nicht gelöscht werden: { $error }.
presets-saved = { $count ->
    [one] „{ $name }“ mit { $count } Entität gespeichert.
   *[other] „{ $name }“ mit { $count } Entitäten gespeichert.
}
presets-nothing-differs = Nichts weicht von „{ $name }“ ab.
presets-recalling = { $count ->
    [one] „{ $name }“ wird abgerufen: { $count } Änderung.
   *[other] „{ $name }“ wird abgerufen: { $count } Änderungen.
}
presets-missing = { $report } Nicht vorhanden oder nicht gelesen: { $missing }.
presets-deleted = „{ $name }“ gelöscht.
presets-host-note = Es speichert auch die eigenen Geräte dieses Computers und startet sie beim Abrufen neu.
presets-host-endpoints = { $count } auf diesem Computer
presets-starting-host = Die Geräte dieses Computers für „{ $name }“ werden gestartet; der Rest folgt, sobald sie wieder da sind.

## Controls

control-numbered = Bedienelement { $index }
control-not-shown = Hier nicht angezeigt
control-option = Option { $number }

## Network errors

network-permission = triib benötigt die Berechtigung, rohe Ethernet-Frames zu senden und zu empfangen.
network-needs-npcap = triib benötigt Npcap, um rohe Ethernet-Frames zu senden und zu empfangen.
network-npcap-administrators = Npcap erlaubt nur Administratoren, rohe Ethernet-Frames zu senden und zu empfangen. triib als Administrator ausführen oder Npcap ohne die Option „nur Administratoren“ neu installieren.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own devices

host-add-endpoint = Endpunkt hinzufügen
host-show-mine = Nur die eigenen Geräte dieses Computers anzeigen
host-show-all = Alle Entitäten anzeigen
host-new-device = Host-Gerät { $number }
host-failed = Konnte nicht zu diesem Computer hinzugefügt werden: { $reason }
host-needs-clock = Die eigenen Geräte dieses Computers brauchen eine kabelgebundene Schnittstelle mit PTP-Hardware-Clock
host-no-ptp4l = ptp4l antwortet nicht, daher können die Streams dieses Computers die gPTP-Zeit nicht halten
host-elsewhere = triib-endpointd läuft auf { $interface } für einen anderen Benutzer oder als root, daher laufen die Geräte dieses Computers dort, nicht hier
host-foreign-mrp = Ein anderes Programm deklariert MSRP oder MVRP auf { $interface } mit der Adresse dieses Computers und kann so zurückziehen, was die Streams dieses Computers brauchen
host-alarm-foreign-mrp = Ein anderes Programm auf diesem Computer deklariert MSRP oder MVRP auf { $interface }
host-state = Zustand
host-streaming = Sendet
host-waiting = Wartet auf einen Listener
host-listening = Empfängt
host-bound = Gebunden, wartet auf den Talker
host-unbound = Nicht gebunden
host-audio-from = Audio von
host-audio-to = Audio an
host-channels = Kanäle
host-virtual-device = Virtuelles Gerät
host-add-stream = Stream hinzufügen
host-add-audio-input = Audioeingang
host-add-audio-output = Audioausgang
host-add-clock-input = Media-Clock-Eingang
host-add-clock-output = Media-Clock-Ausgang
host-remove-stream = Stream entfernen
host-silence = Stille
host-tone = Testton
host-nowhere = Nirgendwohin
host-default-device = Standardgerät
host-remove = Von diesem Computer entfernen
