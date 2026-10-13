# triib's interface text in Italian. The source is i18n/en/triib.ftl; term
# choices are in docs/glossary/it.md.

## Language

language-name = Italiano

## Common

common-close = Chiudi
common-more = Altro
common-keep-toolbar-shown = Mantieni visibile la barra degli strumenti
common-auto-hide-toolbar = Nascondi automaticamente la barra degli strumenti

## Settings

settings-title = Impostazioni
settings-general = Generali
settings-appearance = Aspetto
settings-language = Lingua
settings-language-system = Lingua di sistema: { $language }
settings-language-note = I campi di testo usano la lingua di input del sistema.
settings-appearance-system = Sistema
settings-appearance-light = Chiaro
settings-appearance-dark = Scuro
settings-colors = Colori
settings-system-accent = Usa il colore di accento del sistema
settings-accent-picked = Il colore qui sotto fa da base ai colori di triib.
settings-accent-omarchy = Dal tema Omarchy, { $theme }.
settings-accent-desktop = Dal colore di accento del desktop.
settings-accent-none = Il desktop non ha un colore di accento, quindi si usa il colore qui sotto.
settings-motion = Movimento
settings-animations = Animazioni
settings-animations-note = Effetti elastici e scorrimenti a ogni cambiamento.
settings-animations-reduced = Il desktop chiede di ridurre il movimento, quindi triib resta fermo.
settings-outside-control = Controllo esterno
settings-allow-outside-control = Consenti il controllo esterno
settings-allow-outside-control-note = Gli agenti come Claude Code possono vedere e modificare la tua rete AVB tramite triib --mcp. triib chiede prima di ogni nuovo agente.
settings-allowed-agents = Consentiti: { $agents }
settings-forget-agents = Dimentica
settings-storage = Archiviazione
settings-storage-presets = Cartella dei preset
settings-storage-entities = File delle entità
settings-storage-apply = Applica
settings-storage-choose = Scegli…
settings-storage-note = I preset già conservati in una posizione precedente restano lì; spostali per continuare a usarli. Le impostazioni di triib vengono salvate in { $file }.
settings-storage-entities-note = Un file delle entità che non esiste ancora parte come copia di quello in uso, e triib-endpointd si riavvia con esso.
dialog-choose-presets = Scegli la cartella dei preset
dialog-choose-entities = Scegli la cartella del file delle entità
settings-full-path = Usa un percorso completo, ad esempio ~/Documents/triib.
settings-path-is-folder = { $path } è una cartella, non un file.
settings-folder-missing = La cartella { $path } non esiste. Creala prima, oppure scegline una.
settings-path-is-file = { $path } è un file, non una cartella.
settings-cannot-write = triib non può scrivere in { $path }: { $error }.
app-file-dialog-failed = Impossibile mostrare la finestra dei file: { $error }
agent-prompt-title = Consentire a { $agent } di controllare triib?
agent-prompt-body = { $agent } chiede di usare il controllo esterno di triib: vedere le entità della tua rete e modificarle, collegando flussi e impostando formati, clock e nomi. Puoi disattivare il controllo esterno in Impostazioni.
agent-prompt-allow = Consenti
agent-prompt-deny = Non consentire

common-cancel = Annulla
common-save = Salva
common-not-set = Non impostato
common-unnamed = Senza nome
common-none = Nessuno
common-mac-address = Indirizzo MAC
common-list-separator = {", "}

## Network interfaces

interface-up = attiva
interface-link-down = collegamento inattivo
interface-wireless = wireless
interface-hardware-clock = clock hardware
interface-hardware-clock-named = clock hardware { $clock }
interface-virtual = virtuale

## Toolbar

toolbar-choose-interface = Scegli un’interfaccia
toolbar-interface = Interfaccia di rete
toolbar-filter-interfaces = Filtra le interfacce
toolbar-hide-virtual = Nascondi le interfacce virtuali
toolbar-hide-wireless = Nascondi le interfacce wireless
toolbar-hide-unclocked = Nascondi le interfacce senza clock hardware
toolbar-connections = Connessioni
toolbar-network = Rete
toolbar-entities = Entità
toolbar-rediscover = Chiedi a ogni entità di annunciarsi
toolbar-rescan = Cancella e riscansiona tutte le entità
toolbar-search = Cerca entità e flussi
toolbar-presets = Preset
toolbar-log = Log
toolbar-inspector = Inspector
toolbar-settings = Impostazioni

## The network's state, in place of a view

state-no-interface = Nessuna interfaccia
state-no-interface-note = Scegli l’interfaccia sulla rete AVB per rilevare le entità.
state-starting = Avvio
state-starting-note = Apertura di { $interface }.
state-listening = In ascolto
state-listening-note = Le entità su { $interface } compaiono qui man mano che si annunciano.
state-permission-needed = Autorizzazione necessaria
state-npcap-needed = Npcap necessario
state-get-npcap = Scarica Npcap
state-copy-command = Copia il comando
state-cannot-use = Impossibile usare { $interface }
state-try-again = Riprova

## Entity list

entities-none-yet = Ancora nessuna entità
entities-none-yet-note = Ogni entità della rete, con i suoi ruoli, le classi SR e il clock.

## Inspector

inspector-title = Inspector
inspector-entity = Entità
inspector-streams = Flussi
inspector-controls = Controlli
inspector-diagnostics = Diagnostica
inspector-descriptors = Descrittori
inspector-select = Seleziona un’entità per vederne i dettagli.
inspector-offline = { $entity } è offline.
inspector-heard = Non si annuncia: triib l’ha sentito nei comandi che ha inviato, e lo tiene nell’elenco finché non passano 5 minuti senza comandi.
inspector-rename = Rinomina
inspector-name = Nome
inspector-identify = Identifica
inspector-model-not-read = Il suo modello di entità non è stato letto.
inspector-no-streams = Nessun flusso.
inspector-no-controls = Nessun controllo da mostrare.
inspector-no-diagnostics = Nessuna interfaccia o contatore segnalato.
inspector-reading = Lettura dei descrittori, { $count } finora.
inspector-read-failed = Impossibile leggere il modello di entità: { $reason }.

entity-section = Entità
entity-name = Nome
entity-triib-controller = Controller triib
entity-triib-on = triib su { $host }
entity-triib-here = triib su questo computer
entity-controller-on = Controller su { $host }
entity-controller-here = Controller su questo computer
entity-controller-at = Controller a { $mac }
entity-group = Gruppo
entity-product = Prodotto
entity-firmware = Firmware
entity-serial-number = Numero di serie
entity-configuration = Configurazione
entity-configuration-of = { $name } ({ $number } di { $count })
entity-milan = Milan
entity-media-clock = Media clock
entity-clock-domain = Dominio di clock
entity-sampling-rate = Frequenza di campionamento
clock-source-numbered = Sorgente { $index }
rate-pull = pull { $pull }

stream-format = Formato
stream-id = ID del flusso
stream-destination = Destinazione
stream-max-transit = Tempo di transito max

avb-interfaces = Interfacce AVB
avb-interface = Interfaccia
avb-interface-clock-identity = Identità del clock
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, dominio { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = In funzione
avb-interface-none-reported = Nessuno segnalato
avb-interface-path = Percorso
avb-interface-own-grandmaster = È il proprio grandmaster
avb-interface-hops = { $count ->
    [one] A { $count } hop dal grandmaster
    [many] A { $count } hop dal grandmaster
   *[other] A { $count } hop dal grandmaster
}
avb-interface-link-up = Collegamento attivo
avb-interface-link-down = Collegamento inattivo
avb-interface-grandmaster-changes = Cambi di grandmaster
avb-interface-frames-sent = Frame inviati
avb-interface-frames-received = Frame ricevuti
avb-interface-crc-errors = Errori CRC

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } tipo di descrittore
    [many] { $count } tipi di descrittore
   *[other] { $count } tipi di descrittore
}
tree-clock = Clock
tree-clock-source-from = { $kind }, da { $location } { $index }
tree-clock-domain-using = Usa { $source }
tree-clusters = { $count ->
    [one] { $count } cluster
    [many] { $count } cluster
   *[other] { $count } cluster
}
tree-maps = { $count ->
    [one] { $count } mappatura
    [many] { $count } mappature
   *[other] { $count } mappature
}

advert-not-advertised = Non annunciato
advert-identity = Identità
advert-entity-id = ID entità
advert-entity-model = Modello di entità
advert-roles = Ruoli
advert-talker = Talker
advert-listener = Listener
advert-clock = Clock
advert-btc = BTC
advert-gptp-domain = Dominio gPTP
advert-sr-classes = Classi SR
advert-indexes = Indici nel modello di entità
advert-identify-control = Controllo di identificazione
advert-avb-interface = Interfaccia AVB
advert-advertising = Annuncio
advert-valid-time = Tempo di validità
advert-available-index = Indice di disponibilità
advert-association = Associazione
advert-capabilities = Capacità

## Status bar

status-entities = { $count ->
    [one] { $count } entità
    [many] { $count } entità
   *[other] { $count } entità
}
status-not-discovering = Rilevamento inattivo
status-discovering = Rilevamento in corso
status-discovering-as = Rilevamento come { $controller }
status-stopped = Interrotto da un errore
status-alarm = Allarme
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $count ->
    [one] { $alarm } e { $count } altro
    [many] { $alarm } e { $count } altri
   *[other] { $alarm } e { $count } altri
}

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = controller
role-none = nessun ruolo
classes-a-and-b = A e B
clock-no-gptp = Nessun gPTP

read-not-read = Non letto
read-reading = Lettura, { $count } finora
read-ready-unreadable = { $count ->
    [one] Pronto, { $count } illeggibile
    [many] Pronto, { $count } illeggibili
   *[other] Pronto, { $count } illeggibili
}
read-ready-cached = Pronto, dalla cache
read-ready = Pronto
read-failed = Non riuscito: { $reason }

milan-no = No
milan-before-1-3 = precedente alla 1.3
milan-certified = { $version }, certificato { $certification }
milan-not-certified = { $version }, non certificato

outcome-status = stato { $status }
outcome-no-response = nessuna risposta
outcome-not-possible = non possibile
outcome-connect = Impossibile connettere { $talker } a { $listener }: { $reason }.
outcome-disconnect = Impossibile disconnettere { $listener }: { $reason }.
outcome-identify = Impossibile identificare { $entity }: { $reason }.
outcome-rename = Impossibile rinominare { $what } in “{ $name }”: { $reason }.
outcome-rename-group = Impossibile rinominare il gruppo di { $entity } in “{ $name }”: { $reason }.
outcome-format-streaming = Impossibile cambiare il formato di { $stream }: è in trasmissione. Disconnettilo prima.
outcome-format = Impossibile cambiare il formato di { $stream }: { $reason }.
outcome-max-transit-time = Impossibile cambiare il tempo di transito max di { $stream }: { $reason }.
outcome-max-transit-time-streaming = Impossibile cambiare il tempo di transito max di { $stream }: è in trasmissione. Disconnettilo prima.
outcome-sampling-rate = Impossibile cambiare la frequenza di campionamento di { $entity }: { $reason }.
outcome-clock-source = Impossibile cambiare la sorgente di clock di { $entity }: { $reason }.
outcome-map = Impossibile mappare il canale su { $entity }: { $reason }.
outcome-unmap = Impossibile rimuovere la mappatura del canale su { $entity }: { $reason }.
outcome-control = Impossibile impostare “{ $control }” su { $entity }: { $reason }.
outcome-control-numbered = Impossibile impostare il controllo { $index } su { $entity }: { $reason }.

stream-not-connected = Non connesso
stream-from = Da { $stream }
stream-from-receiving = Da { $stream }, in ricezione
stream-from-waiting = Da { $stream }, in attesa del talker
stream-from-failed = Da { $stream }, prenotazione del talker non riuscita: { $reason }
stream-sending-to = Invio a { $destination }

failure-no-response = l’entità non ha risposto
failure-refused = l’entità ha rifiutato con { $status }
failure-malformed = la sua risposta non è decodificabile
failure-on-this-computer = l’entità è in esecuzione su questo computer; leggila da un altro

msrp-failure-1 = banda insufficiente
msrp-failure-2 = risorse dello switch insufficienti
msrp-failure-3 = banda insufficiente per la classe di traffico
msrp-failure-4 = ID del flusso già usato da un altro talker
msrp-failure-5 = indirizzo di destinazione già in uso
msrp-failure-6 = prelazione da parte di un flusso di rango superiore
msrp-failure-7 = la latenza segnalata è cambiata
msrp-failure-8 = la porta in uscita non è compatibile AVB
msrp-failure-9 = usare un altro indirizzo di destinazione
msrp-failure-10 = risorse MSRP esaurite
msrp-failure-11 = risorse MMRP esaurite
msrp-failure-12 = impossibile memorizzare l’indirizzo di destinazione
msrp-failure-13 = la priorità non è una priorità di classe SR
msrp-failure-14 = frame troppo grandi per il mezzo
msrp-failure-15 = limite di fan-in della porta raggiunto
msrp-failure-16 = primo valore cambiato per un flusso registrato
msrp-failure-17 = VLAN bloccata sulla porta in uscita
msrp-failure-18 = tagging VLAN disattivato sulla porta in uscita
msrp-failure-19 = priorità di classe SR non corrispondente
msrp-failure-unknown = motivo sconosciuto
msrp-failure-at = { $reason }, presso lo switch { $bridge }

## Entity list columns

column-vendor = Produttore
column-model = Modello
column-state = Stato
column-entity-model-id = ID modello di entità
column-talker-streams = Flussi talker
column-listener-streams = Flussi listener
column-avb-lite = AVB Lite
column-egress = Traffico in uscita
column-wireless = Wireless

## Settings file

settings-no-place = Non c’è un posto dove salvare le impostazioni: la cartella home non è nota.
settings-unusable = Impossibile usare { $path }: { $error }.
settings-unsaved = Impossibile salvare { $path }: { $error }.

column-remove = Rimuovi colonna
column-move-left = Sposta a sinistra
column-move-right = Sposta a destra
column-add = Aggiungi una colonna
common-percent = { $value }%

## Network view

netmap-empty = Ancora nessuna rete da mostrare
netmap-empty-note = Le entità compaiono qui una volta lette, dopo aver indicato la loro posizione nell’albero gPTP.
netmap-focus-clock-path = Percorso del clock di { $name }
netmap-focus-streams = Flussi di { $name }
netmap-showing = In evidenza: { $what }
netmap-devices = { $count ->
    [one] { $count } dispositivo
    [many] { $count } dispositivi
   *[other] { $count } dispositivi
}
netmap-bridges = { $count ->
    [one] { $count } switch
    [many] { $count } switch
   *[other] { $count } switch
}
netmap-show-map = Mostra la mappa
netmap-show-details = Mostra i dettagli
stream-numbered = Flusso { $index }
netmap-bridge = Switch
netmap-access-point = Access point
netmap-device = Dispositivo
netmap-this-computer = Questo computer
netmap-connected = Connesso
netmap-advertised = Annunciato, nessun listener pronto
netmap-advertised-off-tree = Annunciato, nessun listener pronto ({ $listener } non è nell’albero gPTP)
netmap-failed-at = Prenotazione non riuscita presso { $bridge }: { $reason }
netmap-failed = Prenotazione non riuscita: { $reason }
netmap-no-bridge-on = Nessuno switch rilevato su { $interface }
netmap-cannot-listen-on = Impossibile ascoltare gPTP su { $interface }
netmap-on-this-computer = Su questo computer
netmap-path-not-reported = Percorso non segnalato
netmap-gptp-not-reported = gPTP non segnalato
netmap-off-tree = Fuori dall’albero gPTP
netmap-off-ptp = Fuori dall’albero PTP
netmap-not-lite = Fuori da AVB Lite
netmap-lite-not-reported = AVB Lite non segnalato
netmap-synced = Sincronizzato
netmap-not-synced = Non sincronizzato
netmap-triib-on = triib su { $interface }
netmap-through-count = { $count } in transito
netmap-out = { $count ->
    [one] { $count } inviato
    [many] { $count } inviati
   *[other] { $count } inviati
}
netmap-in = { $count ->
    [one] { $count } ricevuto
    [many] { $count } ricevuti
   *[other] { $count } ricevuti
}
netmap-failed-count = { $count ->
    [one] { $count } non riuscito
    [many] { $count } non riusciti
   *[other] { $count } non riusciti
}
netmap-advertised-only = Solo annunciato
netmap-failed-state = Non riuscito
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Fuori dall’albero gPTP: è il proprio grandmaster
netmap-apart-no-path = Il suo percorso non è stato segnalato; segue il grandmaster { $grandmaster }
netmap-apart-unreported = Non ha segnalato il suo stato gPTP
netmap-apart-no-neighbor = Nessuno switch rilevato sull’interfaccia di questo computer
netmap-apart-cannot-listen = Questo computer non può ascoltare gPTP sulla sua interfaccia
netmap-apart-on-this-computer = È in esecuzione su questo computer; leggila da un altro computer per vederne lo stato gPTP
netmap-apart-not-lite = Non esegue AVB Lite, quindi non segue il grandmaster
netmap-apart-lite-unreported = Non segnala nulla su AVB Lite, quindi non si sa chi segua
netmap-clock-tree = Albero del clock
netmap-no-grandmaster = Nessun grandmaster rilevato
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Richiede attenzione
netmap-nodes-below = Nodi a valle
netmap-bridges-below = Switch a valle
netmap-clock-path = Percorso del clock
netmap-hops = Hop dal grandmaster
netmap-link-delay = Ritardo del collegamento
netmap-bridge-port = Porta dello switch
netmap-link-drops = Cadute del collegamento
netmap-synced-to-grandmaster = Sincronizzato con il grandmaster
netmap-host-no-gptp = Non sincronizzato: questo computer non esegue gPTP
netmap-link-no-gptp = Non sincronizzato: gPTP non è attivo sul suo collegamento
netmap-ptp-offset-high = Non sincronizzato: { $offset } rispetto al grandmaster, oltre i 50 µs consentiti da AVB Lite
netmap-ptp-no-offset = Non sincronizzato: non ha misurato alcun offset rispetto al grandmaster
netmap-audio = Audio
netmap-media-clock-streams = Flussi di media clock
netmap-audio-streams = Flussi audio
netmap-bound = { $count ->
    [one] { $count } associato
    [many] { $count } associati
   *[other] { $count } associati
}
netmap-flowing = In trasmissione
netmap-advertised-state = Annunciato
netmap-media-clock-stream = Flusso di media clock
netmap-audio-stream = Flusso audio
netmap-reaches = Arriva fino a
netmap-passing-count = { $count ->
    [one] { $count } flusso in transito
    [many] { $count } flussi in transito
   *[other] { $count } flussi in transito
}
netmap-through = Attraverso
netmap-passing-through = In transito
netmap-sending = Invio
netmap-receiving = Ricezione
netmap-problems = Problemi
netmap-help-back = Fai clic sullo sfondo per tornare alla panoramica.
netmap-help-stream = Fai clic su un flusso per ispezionarlo, o sullo sfondo per tornare alla panoramica.
netmap-help-ptp = In AVB Lite il clock va da un capo all’altro, dal grandmaster a ogni dispositivo, attraverso switch che non vi prendono parte, quindi non ne viene mostrato nessuno. Un dispositivo è sincronizzato finché segue il grandmaster entro 50 µs. Fai clic su un dispositivo o sul suo filo per ispezionarne il clock; fai clic sullo sfondo per annullare la selezione.
netmap-help-clock = Il clock parte dal grandmaster e attraversa ogni switch fino a ogni nodo dell’albero. Una linea grigia tratteggiata è un collegamento su cui gPTP non è attivo. Fai clic su un dispositivo o sul suo filo per ispezionarne il percorso del clock; fai clic sullo sfondo per annullare la selezione.
netmap-help-media-clock = Solo flussi di media clock (CRF), disegnati come l’audio: un filo per flusso, colorato in base al talker. Fai clic su un filo per ispezionarne il flusso, o su un dispositivo per vederne i flussi; fai clic sullo sfondo per annullare la selezione.
netmap-help-audio = Ogni flusso ha il proprio filo, che entra ed esce da ogni switch che attraversa. Il colore dipende dal talker: ogni talker ha una tinta e i suoi flussi ne sono sfumature. I punti in movimento indicano che l’audio scorre; una linea rossa ferma è una prenotazione non riuscita e una linea grigia ferma è un flusso annunciato senza listener pronti; entrambe si fermano dove si ferma la prenotazione. I dispositivi nella colonna centrale sono connessi direttamente allo switch del grandmaster. Fai clic su un filo per ispezionarne il flusso, o su un dispositivo per vederne i flussi; fai clic sullo sfondo per annullare la selezione.

## Connections

matrix-nothing-shown = Nessun flusso da mostrare
matrix-nothing-shown-note = Modifica la ricerca o i filtri per vedere più flussi.
matrix-empty = Nessun flusso da connettere
matrix-empty-note = I flussi dei talker e dei listener si incontrano qui una volta lette le entità che li possiedono.
matrix-all-streams = Tutti i flussi
matrix-connectable-only = Nascondi ciò che non può connettersi
matrix-none-hidden = Ogni flusso mostrato può connettersi
matrix-hidden = { $count ->
    [one] { $count } flusso nascosto
    [many] { $count } flussi nascosti
   *[other] { $count } flussi nascosti
}
matrix-own = Le uscite di un’entità non si connettono ai suoi stessi ingressi.
matrix-working = In corso.
matrix-waiting-change = In attesa dell’ultima modifica a questo ingresso.
matrix-connected = Connesso e in ricezione. Fai clic per disconnettere.
matrix-bound-waiting = Associato, in attesa del flusso del talker. Fai clic per disconnettere.
matrix-bound-failed = Associato, ma la prenotazione del talker non è riuscita: { $reason }. Fai clic per disconnettere.
matrix-bound-formats-differ = Associato, ma i formati sono diversi: il talker invia { $sent }, l’ingresso è impostato su { $set }. Fai clic per disconnettere.
matrix-formats-match = I formati corrispondono ({ $format }). Fai clic per connettere.
matrix-format-must-change = L’ingresso accetta { $sent } ma è impostato su { $set }, quindi potrebbe non suonare finché il suo formato non cambia. Fai clic per scegliere un formato.
matrix-format-input = Cambia l’ingresso in
matrix-format-both = Cambia entrambi in
matrix-format-talker-busy = Il talker trasmette ad altri listener, quindi il suo formato resta invariato.
matrix-format-anyway = Connetti senza cambiare i formati
matrix-incompatible = L’ingresso non accetta { $sent }. È impostato su { $set }.
matrix-group-none = Non connesso. Espandi per connettere i flussi uno per uno.
matrix-group-connected = { $count ->
    [one] { $count } connesso. Espandi per vederlo.
    [many] { $count } connessi. Espandi per vederli uno per uno.
   *[other] { $count } connessi. Espandi per vederli uno per uno.
}
matrix-outputs-expand = { $count ->
    [one] { $count } uscita di flusso. Fai clic sulla freccia per espandere, sul nome per ispezionare l’entità.
    [many] { $count } uscite di flusso. Fai clic sulla freccia per espandere, sul nome per ispezionare l’entità.
   *[other] { $count } uscite di flusso. Fai clic sulla freccia per espandere, sul nome per ispezionare l’entità.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } uscita di flusso. Fai clic sulla freccia per comprimere, sul nome per ispezionare l’entità.
    [many] { $count } uscite di flusso. Fai clic sulla freccia per comprimere, sul nome per ispezionare l’entità.
   *[other] { $count } uscite di flusso. Fai clic sulla freccia per comprimere, sul nome per ispezionare l’entità.
}
matrix-inputs-expand = { $count ->
    [one] { $count } ingresso di flusso. Fai clic sulla freccia per espandere, sul nome per ispezionare l’entità.
    [many] { $count } ingressi di flusso. Fai clic sulla freccia per espandere, sul nome per ispezionare l’entità.
   *[other] { $count } ingressi di flusso. Fai clic sulla freccia per espandere, sul nome per ispezionare l’entità.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } ingresso di flusso. Fai clic sulla freccia per comprimere, sul nome per ispezionare l’entità.
    [many] { $count } ingressi di flusso. Fai clic sulla freccia per comprimere, sul nome per ispezionare l’entità.
   *[other] { $count } ingressi di flusso. Fai clic sulla freccia per comprimere, sul nome per ispezionare l’entità.
}
matrix-stream-inspect = { $detail } Fai clic per ispezionare { $entity }.
matrix-point = Punta una cella
matrix-point-note = per vederne il talker e il listener e se i loro formati sono compatibili.
matrix-legend-waiting = Associato, in attesa del flusso
matrix-legend-trouble = Associato, qualcosa non va
matrix-legend-open = Può connettersi
matrix-legend-change = Prima va cambiato il formato dell’ingresso
matrix-legend-incompatible = Formati incompatibili
matrix-talker-outputs = Uscite dei talker
matrix-listener-inputs = Ingressi dei listener

common-thousands-separator = {"."}
common-decimal-separator = {","}

## Diagnostics

diag-since-start = Conteggi dall’avvio dell’entità.
diag-stream-input = Ingresso di flusso
diag-stream-output = Uscita di flusso
diag-locked = { $count ->
    [0] non agganciato
    [1] agganciato una volta
    [2] agganciato due volte
   *[other] agganciato { $number } volte
}
diag-lost-lock = { $count ->
    [0] aggancio mai perso
    [1] aggancio perso una volta
    [2] aggancio perso due volte
   *[other] aggancio perso { $number } volte
}
diag-frames-in = { $count ->
    [one] { $number } frame ricevuto
    [many] { $number } frame ricevuti
   *[other] { $number } frame ricevuti
}
diag-frames-out = { $count ->
    [one] { $number } frame inviato
    [many] { $number } frame inviati
   *[other] { $number } frame inviati
}
diag-media-locked = { $count ->
    [0] non agganciato al media clock
    [1] agganciato al media clock una volta
    [2] agganciato al media clock due volte
   *[other] agganciato al media clock { $number } volte
}
diag-lost-media-lock = { $count ->
    [0] aggancio al media clock mai perso
    [1] aggancio al media clock perso una volta
    [2] aggancio al media clock perso due volte
   *[other] aggancio al media clock perso { $number } volte
}
diag-interrupted = { $count ->
    [0] non interrotto
    [1] interrotto una volta
    [2] interrotto due volte
   *[other] interrotto { $number } volte
}
diag-out-of-sequence = { $count ->
    [one] { $number } frame fuori sequenza
    [many] { $number } frame fuori sequenza
   *[other] { $number } frame fuori sequenza
}
diag-media-resets = { $count ->
    [one] { $number } reset media
    [many] { $number } reset media
   *[other] { $number } reset media
}
diag-timestamps-uncertain = { $count ->
    [0] timestamp mai incerti
    [1] timestamp incerti una volta
    [2] timestamp incerti due volte
   *[other] timestamp incerti { $number } volte
}
diag-no-timestamp = { $count ->
    [one] { $number } frame senza timestamp
    [many] { $number } frame senza timestamp
   *[other] { $number } frame senza timestamp
}
diag-unsupported-format = { $count ->
    [one] { $number } frame in un formato non supportato
    [many] { $number } frame in un formato non supportato
   *[other] { $number } frame in un formato non supportato
}
diag-late = { $count ->
    [one] { $number } frame in ritardo
    [many] { $number } frame in ritardo
   *[other] { $number } frame in ritardo
}
diag-early = { $count ->
    [one] { $number } frame in anticipo
    [many] { $number } frame in anticipo
   *[other] { $number } frame in anticipo
}
diag-started = { $count ->
    [0] non avviato
    [1] avviato una volta
    [2] avviato due volte
   *[other] avviato { $number } volte
}
diag-stopped = { $count ->
    [0] non arrestato
    [1] arrestato una volta
    [2] arrestato due volte
   *[other] arrestato { $number } volte
}
diag-reservation-failed = prenotazione del talker non riuscita: { $reason }
diag-latency = { $microseconds } µs di latenza accumulata

## AVB Lite

lite-active = Attivo
lite-active-untagged = Attivo, senza tag
lite-active-vlan = Attivo, VLAN { $vlan }
lite-capable = Compatibile
lite-mode = Modalità
lite-mode-capable = AVB (compatibile AVB Lite)
lite-because = Motivo
lite-fallback-none = nessun motivo indicato
lite-fallback-endpoint = è arrivata la dichiarazione di un altro endpoint, quindi tra loro non c’è uno switch AVB
lite-fallback-unanswered = nove richieste di peer delay sono rimaste senza risposta
lite-fallback-responders = due o più dispositivi hanno risposto a una stessa richiesta di peer delay, quindi lo switch non è compatibile AVB
lite-fallback-configured = impostato dall’operatore o da un controller
lite-fallback-other = un motivo che il profilo non prevede
lite-other-profile = Altro profilo
lite-ptp-domain = { $profile }, dominio { $domain }
lite-offset = Offset
lite-offset-from = { $offset } rispetto a { $grandmaster }
lite-media-vlan = VLAN media
lite-untagged = Senza tag
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Fino a { $count } listener per flusso, poi multicast
    [many] Fino a { $count } listener per flusso, poi multicast
   *[other] Fino a { $count } listener per flusso, poi multicast
}
lite-link = Collegamento
lite-bandwidth = Banda
lite-egress-of = { $used } di { $link }, { $share }
lite-egress-of-assumed = { $used } di { $link }, { $share }, ipotizzando un collegamento gigabit
lite-egress-reported = Secondo il conteggio dei flussi ammessi fatto dall’entità.
lite-egress-worked-out = Ricavato dai formati delle sue uscite di flusso connesse.
lite-alarm-offset = Offset PTP { $offset }, oltre i 50 µs consentiti da AVB Lite
lite-alarm-egress = Traffico in uscita al { $share } del collegamento, oltre il { $limit } consentito ai flussi

## AVB Wireless

wireless-station = Stazione
wireless-access-point = Access point
wireless-role = Ruolo
wireless-mode = Modalità
wireless-time = Tempo
wireless-mode-a-ftm = Mode A, 802.1AS su FTM
wireless-mode-a-tm = Mode A, 802.1AS su TM
wireless-mode-b = Mode B, dai beacon
wireless-no-time = Nessuna sincronizzazione
wireless-other-mode = Una modalità che il profilo non prevede
wireless-locked = Agganciato
wireless-holdover = In holdover
wireless-not-locked = Non agganciato
wireless-row-locked = Stazione, agganciata
wireless-row-holdover = Stazione, in holdover
wireless-row-not-locked = Stazione, non agganciata
wireless-row-access-point = { $count ->
    [one] Access point, { $count } stazione
    [many] Access point, { $count } stazioni
   *[other] Access point, { $count } stazioni
}
wireless-link = Collegamento
wireless-channel = canale { $channel }
wireless-not-known = Sconosciuto
wireless-signal = Segnale
wireless-rate = Velocità
wireless-ftm-valid = { $share } valide
wireless-rtt = andata e ritorno { $rtt }
wireless-bursts = { $count ->
    [one] burst di { $count } frame
    [many] burst di { $count } frame
   *[other] burst di { $count } frame
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = l’access point concede burst FTM di lunghezza diversa da tre o due frame
wireless-reason-measurement = né FTM né TM con l’access point
wireless-reason-signaling = nessun messaggio Signaling compatibile gPTP dall’access point
wireless-reason-other = un motivo che il profilo non prevede
wireless-servo = Errore del servo
wireless-stations = Stazioni
wireless-station-count = { $count ->
    [one] { $count } stazione
    [many] { $count } stazioni
   *[other] { $count } stazioni
}
wireless-no-ftm = senza FTM
wireless-unserved = Listener non serviti
wireless-frame-budget = Budget di frame
wireless-frame-use = { $used } di { $budget } frame al secondo
wireless-stream-frames = Frame dei flussi
wireless-frames-of = { $readdressed } alle stazioni, { $unmapped } senza listener, { $dropped } scartati, { $restored } dalle stazioni
wireless-class-a-allowed = Consentita
wireless-class-a-not-allowed = Non consentita
wireless-alarm-not-locked = Tempo Wi-Fi non agganciato all’access point
wireless-alarm-holdover = Tempo Wi-Fi in holdover, aggancio all’access point perso
wireless-alarm-unserved = { $count ->
    [one] { $count } listener sulla porta Wi-Fi non servito, oltre il limite unicast
    [many] { $count } listener sulla porta Wi-Fi non serviti, oltre il limite unicast
   *[other] { $count } listener sulla porta Wi-Fi non serviti, oltre il limite unicast
}

## Log

log-all = Tutti
log-warnings = Avvisi
log-pause = Pausa
log-resume = Riprendi
log-clear = Cancella
log-empty = Ogni frame ATDECC che triib invia e riceve compare qui, a partire dal più recente.
log-none-match = Nessun frame conservato corrisponde al filtro.
log-frames = { $count ->
    [one] { $count } frame
    [many] { $count } frame
   *[other] { $count } frame
}
log-shown-of = { $shown } di { $all } frame
log-sent = Inviato
log-heard = Ricevuto
log-not-decoded = Non decodificato
log-warning-short = { $missing ->
    [one] Il suo control_data_length dichiara { $missing } ottetto oltre la fine del frame.
    [many] Il suo control_data_length dichiara { $missing } ottetti oltre la fine del frame.
   *[other] Il suo control_data_length dichiara { $missing } ottetti oltre la fine del frame.
}
log-warning-undecodable = Non è decodificabile: { $error }.
log-warning-long-acmp = È nella forma ACMP lunga, che un’entità Milan non può inviare (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Mappature dei canali
mapping-inputs = Ingressi
mapping-outputs = Uscite
mapping-port = porta { $number }
mapping-fixed = fissa
mapping-not-read = Non ancora letto.
mapping-no-clusters = Nessun cluster.
mapping-no-streams = Nessun flusso audio.
mapping-none = Nessuna mappatura.
mapping-not-mapped = Non mappato
mapping-cluster-numbered = Cluster { $index }

## Presets

presets-note = Un preset conserva le sorgenti di clock, le frequenze di campionamento, i formati dei flussi, i controlli e le connessioni di ogni entità. Richiamarlo cambia ciò che è diverso.
presets-none = Nessun preset ancora salvato.
presets-connections = { $count ->
    [one] { $count } connessione
    [many] { $count } connessioni
   *[other] { $count } connessioni
}
presets-recall = Richiama
presets-delete = Elimina
presets-no-place = Non c’è un posto dove salvare i preset: la cartella home non è nota.
presets-undeletable = Impossibile eliminare { $path }: { $error }.
presets-saved = { $count ->
    [one] Salvato “{ $name }” con { $count } entità.
    [many] Salvato “{ $name }” con { $count } entità.
   *[other] Salvato “{ $name }” con { $count } entità.
}
presets-nothing-differs = Nulla è diverso da “{ $name }”.
presets-recalling = { $count ->
    [one] Richiamo di “{ $name }”: { $count } modifica.
    [many] Richiamo di “{ $name }”: { $count } modifiche.
   *[other] Richiamo di “{ $name }”: { $count } modifiche.
}
presets-missing = { $report } Entità assenti o non lette: { $missing }.
presets-deleted = Eliminato “{ $name }”.
presets-host-note = Conserva anche le entità di questo computer e le riavvia quando viene richiamato.
presets-host-endpoints = { $count } su questo computer
presets-starting-host = Avvio delle entità di questo computer per “{ $name }”; il resto segue quando tornano disponibili.

## Controls

control-numbered = Controllo { $index }
control-not-shown = Non mostrato qui
control-option = Opzione { $number }

## Network errors

network-permission = triib ha bisogno dell’autorizzazione per inviare e ricevere frame Ethernet raw.
network-needs-npcap = triib ha bisogno di Npcap per inviare e ricevere frame Ethernet raw.
network-npcap-administrators = Npcap consente solo agli amministratori di inviare e ricevere frame Ethernet raw. Esegui triib come amministratore, oppure reinstalla Npcap senza l’opzione riservata agli amministratori.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own entities

host-add-entity = Aggiungi entità
host-show-mine = Mostra solo le entità di questo computer
host-show-all = Mostra tutte le entità
host-new-entity = Entità dell’host { $number }
host-failed = Impossibile aggiungerlo a questo computer: { $reason }
host-needs-clock = Le entità di questo computer richiedono un’interfaccia cablata con un clock hardware PTP
host-no-ptp4l = ptp4l non risponde, quindi gli stream di questo computer non possono mantenere l’ora gPTP
host-elsewhere = triib-endpointd è in esecuzione su { $interface } per un altro utente o come root, quindi le entità di questo computer funzionano lì, non qui
host-foreign-mrp = Un altro programma invia dichiarazioni MSRP o MVRP su { $interface } dall’indirizzo di questo computer e può ritirare ciò di cui hanno bisogno i flussi di questo computer
host-alarm-foreign-mrp = Un altro programma di questo computer invia dichiarazioni MSRP o MVRP su { $interface }
host-state = Stato
host-streaming = In trasmissione
host-waiting = In attesa di un listener
host-listening = In ascolto
host-bound = Associato, in attesa del talker
host-unbound = Non associato
host-audio-from = Audio da
host-audio-to = Audio verso
host-channels = Canali
host-virtual-device = Dispositivo virtuale
host-add-stream = Aggiungi flusso
host-add-audio-input = Ingresso audio
host-add-audio-output = Uscita audio
host-add-clock-input = Ingresso media clock
host-add-clock-output = Uscita media clock
host-remove-stream = Rimuovi flusso
host-silence = Silenzio
host-tone = Tono di prova
host-nowhere = Da nessuna parte
host-default-device = Dispositivo predefinito
host-remove = Rimuovi da questo computer
