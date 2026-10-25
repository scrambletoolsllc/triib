# triib's interface text in Greek. Term choices: docs/glossary/el.md.

## Language

language-name = Ελληνικά

## Common

common-close = Κλείσιμο
common-more = Περισσότερα
common-keep-toolbar-shown = Μόνιμη εμφάνιση γραμμής εργαλείων
common-auto-hide-toolbar = Αυτόματη απόκρυψη γραμμής εργαλείων

## Settings

settings-title = Ρυθμίσεις
settings-general = Γενικά
settings-appearance = Εμφάνιση
settings-language = Γλώσσα
settings-language-system = Προεπιλογή συστήματος: { $language }
settings-language-note = Στα πεδία κειμένου πληκτρολογείτε στη γλώσσα εισαγωγής του συστήματος.
settings-appearance-system = Σύστημα
settings-appearance-light = Φωτεινή
settings-appearance-dark = Σκούρα
settings-colors = Χρώματα
settings-system-accent = Χρήση του χρώματος έμφασης του συστήματος
settings-accent-picked = Από το παρακάτω χρώμα προκύπτουν τα χρώματα του triib.
settings-accent-omarchy = Από το θέμα του Omarchy, { $theme }.
settings-accent-desktop = Από το χρώμα έμφασης της επιφάνειας εργασίας.
settings-accent-none = Η επιφάνεια εργασίας δεν έχει χρώμα έμφασης, οπότε χρησιμοποιείται το παρακάτω χρώμα.
settings-motion = Κίνηση
settings-animations = Κινούμενα εφέ
settings-animations-note = Εφέ ελατηρίου και ολίσθησης όταν κάτι αλλάζει.
settings-animations-reduced = Η επιφάνεια εργασίας ζητά μειωμένη κίνηση, οπότε το triib μένει ακίνητο.
settings-outside-control = Εξωτερικός έλεγχος
settings-allow-outside-control = Να επιτρέπεται ο εξωτερικός έλεγχος
settings-allow-outside-control-note = Πράκτορες όπως το Claude Code μπορούν να βλέπουν και να αλλάζουν το δίκτυο AVB σας, μέσω του triib --mcp. Το triib ρωτά πριν από κάθε νέο πράκτορα.
settings-allowed-agents = Επιτρέπονται: { $agents }
settings-forget-agents = Διαγραφή
settings-storage = Χώρος αποθήκευσης
settings-storage-presets = Φάκελος προρυθμίσεων
settings-storage-entities = Αρχείο οντοτήτων
settings-storage-apply = Εφαρμογή
settings-storage-choose = Επιλογή…
settings-storage-note = Οι προρυθμίσεις που βρίσκονται ήδη σε παλιά θέση μένουν εκεί· μετακινήστε τις για να συνεχίσετε να τις χρησιμοποιείτε. Οι ρυθμίσεις του triib αποθηκεύονται στο { $file }.
settings-storage-entities-note = Ένα αρχείο οντοτήτων που δεν υπάρχει ακόμη ξεκινά ως αντίγραφο αυτού που χρησιμοποιείται, και το triib-endpointd ξεκινά ξανά με αυτό.
dialog-choose-presets = Επιλέξτε τον φάκελο προρυθμίσεων
dialog-choose-entities = Επιλέξτε τον φάκελο για το αρχείο οντοτήτων
settings-full-path = Χρησιμοποιήστε πλήρη διαδρομή, π.χ. ~/Documents/triib.
settings-path-is-folder = Το { $path } είναι φάκελος, όχι αρχείο.
settings-folder-missing = Ο φάκελος { $path } δεν υπάρχει. Δημιουργήστε τον πρώτα ή επιλέξτε έναν άλλο.
settings-path-is-file = Το { $path } είναι αρχείο, όχι φάκελος.
settings-cannot-write = Το triib δεν μπορεί να γράψει στο { $path }: { $error }.
app-file-dialog-failed = Δεν ήταν δυνατή η εμφάνιση του παραθύρου αρχείων: { $error }
agent-prompt-title = Να επιτραπεί στο { $agent } να ελέγχει το triib;
agent-prompt-body = Το { $agent } ζητά να χρησιμοποιήσει τον εξωτερικό έλεγχο του triib: να βλέπει τις οντότητες του δικτύου σας και να τις αλλάζει, συνδέοντας ροές και ορίζοντας μορφές, ρολόγια και ονόματα. Μπορείτε να απενεργοποιήσετε τον εξωτερικό έλεγχο στις Ρυθμίσεις.
agent-prompt-allow = Να επιτραπεί
agent-prompt-deny = Να μην επιτραπεί

common-cancel = Ακύρωση
common-save = Αποθήκευση
common-not-set = Δεν έχει οριστεί
common-unnamed = Χωρίς όνομα
common-none = Κανένα
common-mac-address = Διεύθυνση MAC
common-list-separator = {", "}

## Network interfaces

interface-up = ενεργή
interface-link-down = ζεύξη ανενεργή
interface-wireless = ασύρματη
interface-hardware-clock = ρολόι υλικού
interface-hardware-clock-named = ρολόι υλικού { $clock }
interface-virtual = εικονική

## Toolbar

toolbar-choose-interface = Επιλέξτε διεπαφή
toolbar-interface = Διεπαφή δικτύου
toolbar-filter-interfaces = Φιλτράρισμα διεπαφών
toolbar-hide-virtual = Απόκρυψη εικονικών διεπαφών
toolbar-hide-wireless = Απόκρυψη ασύρματων διεπαφών
toolbar-hide-unclocked = Απόκρυψη διεπαφών χωρίς ρολόι υλικού
toolbar-connections = Συνδέσεις
toolbar-network = Δίκτυο
toolbar-entities = Οντότητες
toolbar-rediscover = Ζητήστε από κάθε οντότητα να ανακοινώσει την παρουσία της
toolbar-rescan = Εκκαθαρίστε και σαρώστε ξανά όλες τις οντότητες
toolbar-search = Αναζήτηση οντοτήτων και ροών
toolbar-presets = Προρυθμίσεις
toolbar-log = Αρχείο καταγραφής
toolbar-inspector = Επιθεωρητής
toolbar-settings = Ρυθμίσεις
toolbar-close-window = Κλείσιμο παραθύρου

## The macOS menu bar, named as in macOS's own apps

menu-about = Πληροφορίες για το triib
menu-settings = Ρυθμίσεις…
menu-services = Υπηρεσίες
menu-hide = Απόκρυψη triib
menu-hide-others = Απόκρυψη άλλων
menu-show-all = Εμφάνιση όλων
menu-quit = Τερματισμός triib
menu-file = Αρχείο
menu-presets = Προρυθμίσεις…
menu-close-window = Κλείσιμο παραθύρου
menu-edit = Επεξεργασία
menu-cut = Αποκοπή
menu-copy = Αντιγραφή
menu-paste = Επικόλληση
menu-select-all = Επιλογή όλων
menu-view = Προβολή
menu-full-screen = Είσοδος σε πλήρη οθόνη
menu-window = Παράθυρο
menu-minimize = Ελαχιστοποίηση
menu-zoom = Ζουμ
menu-bring-all-to-front = Μεταφορά όλων μπροστά
menu-help = Βοήθεια
menu-website = Ιστότοπος του triib
menu-release-notes = Σημειώσεις έκδοσης

## The network's state, in place of a view

state-no-interface = Καμία διεπαφή
state-no-interface-note = Επιλέξτε τη διεπαφή του δικτύου AVB για την ανακάλυψη οντοτήτων.
state-starting = Εκκίνηση
state-starting-note = Άνοιγμα της διεπαφής { $interface }.
state-listening = Ακρόαση
state-listening-note = Οι οντότητες στη διεπαφή { $interface } εμφανίζονται εδώ μόλις ανακοινώσουν την παρουσία τους.
state-permission-needed = Απαιτείται άδεια
state-npcap-needed = Απαιτείται το Npcap
state-get-npcap = Λήψη του Npcap
state-copy-command = Αντιγραφή της εντολής
state-permission-watch = Το triib συνεχίζει μόνο του μόλις εκτελεστεί η εντολή.
state-cannot-use = Δεν είναι δυνατή η χρήση της διεπαφής { $interface }
state-try-again = Νέα προσπάθεια

## Entity list

entities-none-yet = Καμία οντότητα ακόμη
entities-none-yet-note = Κάθε οντότητα του δικτύου, με τους ρόλους, τις κλάσεις SR και το ρολόι της.
entities-filter = Φιλτράρισμα οντοτήτων
entities-hide-others = Απόκρυψη οντοτήτων άλλων συσκευών
entities-hide-controllers = Απόκρυψη ελεγκτών
entities-hide-other-domains = Απόκρυψη οντοτήτων εκτός του τομέα ρολογιού αυτού του υπολογιστή
entities-hidden = { $count ->
    [one] { $count } κρυφή οντότητα
   *[other] { $count } κρυφές οντότητες
}

## Inspector

inspector-title = Επιθεωρητής
inspector-entity = Οντότητα
inspector-streams = Ροές
inspector-controls = Χειριστήρια
inspector-diagnostics = Διαγνωστικά
inspector-descriptors = Περιγραφείς
inspector-select = Επιλέξτε μια οντότητα για να δείτε τις λεπτομέρειές της.
inspector-offline = Η οντότητα { $entity } είναι εκτός σύνδεσης.
inspector-heard = Δεν ανακοινώνεται: το triib τον άκουσε στις εντολές που έστειλε και τον κρατά στη λίστα μέχρι να περάσουν 5 λεπτά χωρίς εντολή.
inspector-rename = Μετονομασία
inspector-name = Όνομα
inspector-identify = Αναγνώριση
inspector-model-not-read = Το μοντέλο οντότητας δεν έχει διαβαστεί.
inspector-no-streams = Καμία ροή.
inspector-no-controls = Κανένα χειριστήριο για εμφάνιση.
inspector-no-diagnostics = Δεν αναφέρθηκαν διεπαφές ή μετρητές.
inspector-reading = Ανάγνωση περιγραφέων, { $count } μέχρι στιγμής.
inspector-read-failed = Δεν ήταν δυνατή η ανάγνωση του μοντέλου οντότητας: { $reason }.

entity-section = Οντότητα
entity-name = Όνομα
entity-triib-controller = Ελεγκτής triib
entity-triib-on = triib στο { $host }
entity-triib-here = triib σε αυτόν τον υπολογιστή
entity-controller-on = Ελεγκτής στο { $host }
entity-controller-here = Ελεγκτής σε αυτόν τον υπολογιστή
entity-controller-at = Ελεγκτής στη { $mac }
entity-group = Ομάδα
entity-product = Προϊόν
entity-firmware = Υλικολογισμικό
entity-serial-number = Σειριακός αριθμός
entity-configuration = Διαμόρφωση
entity-configuration-of = { $name } ({ $number } από { $count })
entity-configuration-holds = { $inputs } είσοδοι, { $outputs } έξοδοι, ροές { $channels } καναλιών
entity-configuration-holds-mixed = { $inputs } είσοδοι, { $outputs } έξοδοι
entity-configuration-numbered = Διαμόρφωση { $number }
entity-milan = Milan
entity-media-clock = Ρολόι πολυμέσων
entity-clock-domain = Πεδίο ρολογιού
entity-sampling-rate = Συχνότητα δειγματοληψίας
clock-source-numbered = Πηγή { $index }
rate-pull = pull { $pull }

stream-format = Μορφή
stream-id = ID ροής
stream-destination = Προορισμός
stream-max-transit = Μέγ. χρόνος διέλευσης
stream-listeners = Ακροατές
stream-listener-gone = { $entity } είσοδος { $input }, εκτός σύνδεσης
stream-disconnect-listener = Αποσύνδεση

avb-interfaces = Διεπαφές AVB
avb-interface = Διεπαφή
avb-interface-clock-identity = Ταυτότητα ρολογιού
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, τομέας { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Σε λειτουργία
avb-interface-none-reported = Δεν αναφέρθηκε κανένα
avb-interface-path = Διαδρομή
avb-interface-own-grandmaster = Είναι η ίδια Grandmaster
avb-interface-hops = { $count ->
    [one] { $count } άλμα από τον Grandmaster
   *[other] { $count } άλματα από τον Grandmaster
}
avb-interface-link-up = Ζεύξη ενεργή
avb-interface-link-down = Ζεύξη ανενεργή
avb-interface-grandmaster-changes = Αλλαγές Grandmaster
avb-interface-frames-sent = Πλαίσια που εστάλησαν
avb-interface-frames-received = Πλαίσια που ελήφθησαν
avb-interface-crc-errors = Σφάλματα CRC

tree-firmware = Υλικολογισμικό { $version }
tree-descriptor-types = { $count ->
    [one] { $count } τύπος περιγραφέα
   *[other] { $count } τύποι περιγραφέων
}
tree-clock = Ρολόι
tree-clock-source-from = { $kind }, από { $location } { $index }
tree-clock-domain-using = Χρησιμοποιεί { $source }
tree-clusters = { $count ->
    [one] { $count } συστάδα
   *[other] { $count } συστάδες
}
tree-maps = { $count ->
    [one] { $count } αντιστοίχιση
   *[other] { $count } αντιστοιχίσεις
}

advert-not-advertised = Δεν ανακοινώνεται
advert-identity = Ταυτότητα
advert-entity-id = ID οντότητας
advert-entity-model = Μοντέλο οντότητας
advert-roles = Ρόλοι
advert-talker = Talker
advert-listener = Listener
advert-clock = Ρολόι
advert-gptp-domain = Τομέας gPTP
advert-sr-classes = Κλάσεις SR
advert-indexes = Δείκτες μοντέλου οντότητας
advert-identify-control = Χειριστήριο αναγνώρισης
advert-avb-interface = Διεπαφή AVB
advert-advertising = Ανακοίνωση
advert-valid-time = Χρόνος ισχύος
advert-available-index = Δείκτης διαθεσιμότητας
advert-association = Συσχέτιση
advert-capabilities = Δυνατότητες

## Status bar

status-entities = { $count ->
    [one] { $count } οντότητα
   *[other] { $count } οντότητες
}
status-not-discovering = Ανακάλυψη ανενεργή
status-discovering = Ανακάλυψη
status-discovering-as = Ανακάλυψη ως { $controller }
status-stopped = Διακόπηκε λόγω σφάλματος
status-alarm = Συναγερμός
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } και { $count } ακόμη

## Descriptions of entities, shared by the views

role-talker = Talker { $count }
role-listener = Listener { $count }
role-controller = ελεγκτής
role-none = κανένας ρόλος
classes-a-and-b = A και B
clock-no-gptp = Χωρίς gPTP

read-not-read = Δεν διαβάστηκε
read-reading = Ανάγνωση, { $count } μέχρι στιγμής
read-ready-unreadable = { $count ->
    [one] Έτοιμο, { $count } μη αναγνώσιμος
   *[other] Έτοιμο, { $count } μη αναγνώσιμοι
}
read-ready-cached = Έτοιμο, από την κρυφή μνήμη
read-ready = Έτοιμο
read-failed = Απέτυχε: { $reason }

milan-no = Όχι
milan-before-1-3 = πριν από την 1.3
milan-certified = { $version }, πιστοποίηση { $certification }
milan-not-certified = { $version }, χωρίς πιστοποίηση

outcome-status = κατάσταση { $status }
outcome-no-response = καμία απόκριση
outcome-not-possible = δεν είναι δυνατό
outcome-connect = Δεν ήταν δυνατή η σύνδεση της ροής { $talker } με τη ροή { $listener }: { $reason }.
outcome-disconnect = Δεν ήταν δυνατή η αποσύνδεση της ροής { $listener }: { $reason }.
outcome-disconnect-listener = Δεν ήταν δυνατή η αποσύνδεση του { $listener } από το { $talker }: { $reason }.
outcome-identify = Δεν ήταν δυνατή η αναγνώριση της οντότητας { $entity }: { $reason }.
outcome-rename = Δεν ήταν δυνατή η μετονομασία του { $what } σε «{ $name }»: { $reason }.
outcome-rename-group = Δεν ήταν δυνατή η μετονομασία της ομάδας της οντότητας { $entity } σε «{ $name }»: { $reason }.
outcome-format-streaming = Δεν ήταν δυνατή η αλλαγή μορφής της ροής { $stream }: βρίσκεται σε μετάδοση. Αποσυνδέστε την πρώτα.
outcome-format = Δεν ήταν δυνατή η αλλαγή μορφής της ροής { $stream }: { $reason }.
outcome-max-transit-time = Δεν ήταν δυνατή η αλλαγή του μέγ. χρόνου διέλευσης της ροής { $stream }: { $reason }.
outcome-max-transit-time-streaming = Δεν ήταν δυνατή η αλλαγή του μέγ. χρόνου διέλευσης της ροής { $stream }: βρίσκεται σε μετάδοση. Αποσυνδέστε την πρώτα.
outcome-sampling-rate = Δεν ήταν δυνατή η αλλαγή της συχνότητας δειγματοληψίας της οντότητας { $entity }: { $reason }.
outcome-clock-source = Δεν ήταν δυνατή η αλλαγή της πηγής ρολογιού της οντότητας { $entity }: { $reason }.
outcome-configuration = Δεν ήταν δυνατή η αλλαγή της διαμόρφωσης της οντότητας { $entity }: { $reason }.
outcome-configuration-streaming = Η οντότητα { $entity } κρατά τη διαμόρφωσή της όσο μεταδίδονται οι ροές της: αποσυνδέστε τις πρώτα.
outcome-map = Δεν ήταν δυνατή η αντιστοίχιση του καναλιού στην οντότητα { $entity }: { $reason }.
outcome-unmap = Δεν ήταν δυνατή η κατάργηση της αντιστοίχισης του καναλιού στην οντότητα { $entity }: { $reason }.
outcome-control = Δεν ήταν δυνατή η ρύθμιση του «{ $control }» στην οντότητα { $entity }: { $reason }.
outcome-control-numbered = Δεν ήταν δυνατή η ρύθμιση του χειριστηρίου { $index } στην οντότητα { $entity }: { $reason }.

stream-not-connected = Μη συνδεδεμένη
stream-from = Από { $stream }
stream-from-receiving = Από { $stream }, λήψη
stream-from-waiting = Από { $stream }, αναμονή για τον Talker
stream-from-failed = Από { $stream }, η κράτηση του Talker απέτυχε: { $reason }
stream-sending-to = Αποστολή προς { $destination }

failure-no-response = δεν απάντησε
failure-refused = αρνήθηκε με { $status }
failure-malformed = η απόκρισή της δεν αποκωδικοποιήθηκε
failure-on-this-computer = εκτελείται σε αυτόν τον υπολογιστή· διαβάστε τη από άλλον

msrp-failure-1 = ανεπαρκές εύρος ζώνης
msrp-failure-2 = ανεπαρκείς πόροι γέφυρας
msrp-failure-3 = ανεπαρκές εύρος ζώνης για την κλάση κίνησης
msrp-failure-4 = το ID ροής χρησιμοποιείται από άλλον Talker
msrp-failure-5 = η διεύθυνση προορισμού χρησιμοποιείται ήδη
msrp-failure-6 = εκτοπίστηκε από ροή υψηλότερης κατάταξης
msrp-failure-7 = η αναφερόμενη καθυστέρηση άλλαξε
msrp-failure-8 = η εξερχόμενη θύρα δεν υποστηρίζει AVB
msrp-failure-9 = χρησιμοποιήστε άλλη διεύθυνση προορισμού
msrp-failure-10 = εξαντλήθηκαν οι πόροι MSRP
msrp-failure-11 = εξαντλήθηκαν οι πόροι MMRP
msrp-failure-12 = δεν είναι δυνατή η αποθήκευση της διεύθυνσης προορισμού
msrp-failure-13 = η προτεραιότητα δεν είναι προτεραιότητα κλάσης SR
msrp-failure-14 = πλαίσια πολύ μεγάλα για το μέσο μετάδοσης
msrp-failure-15 = συμπληρώθηκε το όριο fan-in της θύρας
msrp-failure-16 = άλλαξε η πρώτη τιμή καταχωρισμένης ροής
msrp-failure-17 = το VLAN είναι αποκλεισμένο στην εξερχόμενη θύρα
msrp-failure-18 = η επισήμανση VLAN είναι απενεργοποιημένη στην εξερχόμενη θύρα
msrp-failure-19 = αναντιστοιχία προτεραιότητας κλάσης SR
msrp-failure-unknown = άγνωστη αιτία
msrp-failure-at = { $reason }, στη γέφυρα { $bridge }

## Entity list columns

column-vendor = Κατασκευαστής
column-entity-image = Εικόνα οντότητας
column-vendor-image = Εικόνα κατασκευαστή
column-model = Μοντέλο
column-state = Κατάσταση
column-entity-model-id = ID μοντέλου οντότητας
column-talker-streams = Ροές Talker
column-listener-streams = Ροές Listener
column-avb-lite = AVB Lite
column-egress = Εξερχόμενη κίνηση
column-wireless = Ασύρματο

## Settings file

settings-no-place = Δεν υπάρχει πού να αποθηκευτούν οι ρυθμίσεις: ο αρχικός φάκελος δεν είναι γνωστός.
settings-unusable = Δεν ήταν δυνατή η χρήση του { $path }: { $error }.
settings-unsaved = Δεν ήταν δυνατή η αποθήκευση του { $path }: { $error }.

column-remove = Αφαίρεση στήλης
column-move-left = Μετακίνηση αριστερά
column-move-right = Μετακίνηση δεξιά
column-add = Προσθήκη στήλης
common-percent = { $value }%

## Network view

netmap-empty = Δεν υπάρχει ακόμη δίκτυο για εμφάνιση
netmap-empty-note = Οι οντότητες εμφανίζονται εδώ μόλις διαβαστούν και δηλώσουν τον τομέα ρολογιού τους.
netmap-focus-clock-path = διαδρομή ρολογιού του { $name }
netmap-focus-streams = ροές του { $name }
netmap-showing = Εμφάνιση: { $what }
netmap-devices = { $count ->
    [one] { $count } συσκευή
   *[other] { $count } συσκευές
}
netmap-bridges = { $count ->
    [one] { $count } γέφυρα
   *[other] { $count } γέφυρες
}
netmap-show-map = Εμφάνιση του χάρτη
netmap-show-details = Εμφάνιση των λεπτομερειών
stream-numbered = Ροή { $index }
netmap-bridge = Γέφυρα
netmap-access-point = Σημείο πρόσβασης
netmap-plain-switch = Μεταγωγέας χωρίς AVB
netmap-device = Συσκευή
netmap-this-computer = Αυτός ο υπολογιστής
netmap-connected = Συνδεδεμένη
netmap-advertised = Ανακοινωμένη, κανένας Listener έτοιμος
netmap-advertised-unknown-domain = Ανακοινωμένη, κανένας Listener έτοιμος ({ $listener }: άγνωστος τομέας ρολογιού)
netmap-failed-at = Η κράτηση απέτυχε στη γέφυρα { $bridge }: { $reason }
netmap-failed = Η κράτηση απέτυχε: { $reason }
netmap-no-bridge-on = Δεν εντοπίστηκε γέφυρα στη διεπαφή { $interface }
netmap-cannot-listen-on = Δεν είναι δυνατή η ακρόαση gPTP στη διεπαφή { $interface }
netmap-behind-plain-switch = Πίσω από μεταγωγέα χωρίς AVB
netmap-on-this-computer = Σε αυτόν τον υπολογιστή
netmap-path-not-reported = Η διαδρομή δεν αναφέρθηκε
netmap-gptp-not-reported = Δεν αναφέρθηκε gPTP
netmap-domain = Τομέας gPTP { $domain }
netmap-ptp-domain = Τομέας PTP { $domain }
netmap-domain-unknown = Άγνωστος τομέας
netmap-synced = Συγχρονισμένο
netmap-sync = Συγχρονισμός
netmap-not-synced = Μη συγχρονισμένο
netmap-triib-on = triib σε { $interface }
netmap-through-count = { $count ->
    [one] { $count } διερχόμενη
   *[other] { $count } διερχόμενες
}
netmap-out = { $count ->
    [one] { $count } εξερχόμενη
   *[other] { $count } εξερχόμενες
}
netmap-in = { $count ->
    [one] { $count } εισερχόμενη
   *[other] { $count } εισερχόμενες
}
netmap-failed-count = { $count ->
    [one] { $count } αποτυχημένη
   *[other] { $count } αποτυχημένες
}
netmap-advertised-only = Μόνο ανακοινωμένη
netmap-failed-state = Απέτυχε
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-sending-unheard = Αποστέλλεται, χωρίς γνωστό Listener ανατεθειμένο σε αυτήν
netmap-no-listener = Κανένας γνωστός Listener
netmap-apart-no-path = Η διαδρομή της δεν αναφέρθηκε· ακολουθεί τον Grandmaster { $grandmaster }
netmap-apart-unreported = Δεν έχει αναφέρει την κατάσταση gPTP της
netmap-apart-no-neighbor = Δεν εντοπίστηκε γέφυρα στη διεπαφή αυτού του υπολογιστή
netmap-apart-cannot-listen = Αυτός ο υπολογιστής δεν μπορεί να ακούσει gPTP στη διεπαφή του
netmap-apart-plain-switch = Αυτός ο υπολογιστής βρίσκεται πίσω από μεταγωγέα που δεν συμμετέχει στο AVB, και καμία συσκευή που τον μοιράζεται δεν εκτελεί εκεί AVB Lite, οπότε δεν ανήκει σε κανέναν τομέα
netmap-apart-on-this-computer = Εκτελείται σε αυτόν τον υπολογιστή· διαβάστε τη από άλλον υπολογιστή για να δείτε την κατάσταση gPTP
netmap-switch-state = Δεν συμμετέχει στο AVB: αντιπροσωπεύει τους μεταγωγείς μπροστά από τις παρακάτω συσκευές, οι οποίες δεν βρήκαν γέφυρα AVB και μετέπεσαν σε AVB Lite, ακολουθώντας έναν κοινό Grandmaster PTP
netmap-clock-tree = Δέντρο ρολογιού
netmap-no-grandmaster = Δεν εντοπίστηκε Grandmaster
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Χρειάζεται προσοχή
netmap-nodes-below = Κόμβοι από κάτω
netmap-bridges-below = Γέφυρες από κάτω
netmap-clock-path = Διαδρομή ρολογιού
netmap-hops = Άλματα από τον Grandmaster
netmap-link-delay = Καθυστέρηση ζεύξης
netmap-bridge-port = Θύρα γέφυρας
netmap-link-drops = Πτώσεις ζεύξης
netmap-synced-to-grandmaster = Συγχρονισμένο με τον Grandmaster
netmap-host-no-gptp = Μη συγχρονισμένο: αυτός ο υπολογιστής δεν εκτελεί gPTP
netmap-host-no-ptp = Μη συγχρονισμένο: αυτός ο υπολογιστής δεν ακολουθεί τον Grandmaster αυτού του τομέα
netmap-link-no-gptp = Μη συγχρονισμένο: το gPTP δεν εκτελείται στη ζεύξη του
netmap-ptp-offset-high = Μη συγχρονισμένο: { $offset } από τον Grandmaster, πάνω από τα 50 µs που επιτρέπει το AVB Lite
netmap-ptp-no-offset = Μη συγχρονισμένο: δεν έχει μετρήσει καμία απόκλιση από τον Grandmaster
netmap-audio = Ήχος
netmap-media-clock-streams = Ροές ρολογιού πολυμέσων
netmap-audio-streams = Ροές ήχου
netmap-bound = { $count ->
    [one] { $count } ανατεθειμένη
   *[other] { $count } ανατεθειμένες
}
netmap-flowing = Ρέει
netmap-advertised-state = Ανακοινωμένη
netmap-media-clock-stream = Ροή ρολογιού πολυμέσων
netmap-audio-stream = Ροή ήχου
netmap-reaches = Φτάνει έως
netmap-passing-count = { $count ->
    [one] { $count } διερχόμενη ροή
   *[other] { $count } διερχόμενες ροές
}
netmap-through = Διερχόμενες
netmap-passing-through = Διέρχεται από
netmap-sending = Αποστολή
netmap-receiving = Λήψη
netmap-problems = Προβλήματα
netmap-help-back = Κάντε κλικ στο φόντο για επιστροφή στην επισκόπηση.
netmap-help-stream = Κάντε κλικ σε μια ροή για να την επιθεωρήσετε ή στο φόντο για επιστροφή στην επισκόπηση.
netmap-help-clock = Το ρολόι ρέει από τον Grandmaster μέσω κάθε γέφυρας προς κάθε κόμβο του δέντρου. Μια διακεκομμένη γκρι γραμμή είναι ζεύξη στην οποία δεν εκτελείται gPTP. Κάντε κλικ σε μια συσκευή ή στο καλώδιό της για να επιθεωρήσετε τη διαδρομή ρολογιού της· κάντε κλικ στο φόντο για καθαρισμό.
netmap-help-domains = Κάθε περιοχή είναι ένας τομέας ρολογιού. Σε τομέα gPTP το ρολόι περνά από τον Grandmaster προς τα κάτω μέσω των γεφυρών AVB. Σε τομέα PTP οι συσκευές εκτελούν AVB Lite και ακολουθούν τον Grandmaster τους από άκρο σε άκρο, συγχρονισμένες εντός 50 µs· μια διακεκομμένη κάρτα αντιπροσωπεύει τους μεταγωγείς χωρίς AVB πίσω από τους οποίους κάποιες μετέπεσαν σε AVB Lite. Μια συσκευή που δεν μοιράζεται τον τομέα της με καμία άλλη έχει δική της περιοχή.
netmap-help-media-clock = Μόνο ροές ρολογιού πολυμέσων (CRF), σχεδιασμένες όπως ο ήχος: ένα καλώδιο για τα πλαίσια κάθε ροής σε κάθε ζεύξη, χρωματισμένο ανά Talker. Κάντε κλικ σε ένα καλώδιο για να επιθεωρήσετε τη ροή του ή σε μια συσκευή για να δείτε τις ροές της· κάντε κλικ στο φόντο για καθαρισμό.
netmap-help-audio = Κάθε καλώδιο αντιστοιχεί στα πλαίσια μιας ροής σε μια ζεύξη και μπαίνει σε κάθε γέφυρα που διασχίζει και βγαίνει από αυτήν. Μια ροή multicast έχει ένα καλώδιο όσο οι διαδρομές προς τους Listener της συμπίπτουν και διακλαδίζεται εκεί όπου χωρίζουν· μια ροή unicast, όπως αυτές που μπορεί να στείλει το AVB Lite, έχει δικό της καλώδιο από τον Talker προς κάθε Listener. Το χρώμα δηλώνει τον Talker: κάθε Talker έχει μια απόχρωση και κάθε ροή του είναι ένας τόνος της σε κάθε διακλάδωση. Οι κινούμενες κουκκίδες σημαίνουν ότι ρέει ήχος, ακόμη και από Talker που στέλνει χωρίς γνωστό Listener· μια ακίνητη κόκκινη γραμμή είναι αποτυχημένη κράτηση και μια ακίνητη γκρι γραμμή είναι ανακοινωμένη ροή χωρίς έτοιμο Listener· και οι δύο σταματούν εκεί όπου σταματά η κράτηση. Οι συσκευές στη μεσαία στήλη συνδέονται απευθείας στη γέφυρα του Grandmaster. Κάντε κλικ σε ένα καλώδιο για να επιθεωρήσετε τη ροή του, σε ένα κοινό καλώδιο για να δείτε όλους τους Listener του ή σε μια συσκευή για να δείτε τις ροές της· κάντε κλικ στο φόντο για καθαρισμό.

## Connections

matrix-nothing-shown = Καμία ροή για εμφάνιση
matrix-nothing-shown-note = Αλλάξτε την αναζήτηση ή τα φίλτρα για να δείτε περισσότερες ροές.
matrix-empty = Καμία ροή για σύνδεση
matrix-empty-note = Οι ροές των Talker και των Listener συναντώνται εδώ μόλις διαβαστούν οντότητες που τις διαθέτουν.
matrix-all-streams = Όλες οι ροές
matrix-connectable-only = Απόκρυψη όσων δεν συνδέονται
matrix-none-hidden = Κάθε ροή που εμφανίζεται μπορεί να συνδεθεί
matrix-hidden = { $count ->
    [one] { $count } κρυφή ροή
   *[other] { $count } κρυφές ροές
}
matrix-own = Οι έξοδοι μιας οντότητας δεν συνδέονται στις δικές της εισόδους.
matrix-working = Σε εξέλιξη.
matrix-waiting-change = Αναμονή για την τελευταία αλλαγή σε αυτή την είσοδο.
matrix-connected = Συνδεδεμένη και σε λήψη. Κάντε κλικ για αποσύνδεση.
matrix-bound-waiting = Ανατεθειμένη, αναμονή για τη ροή του Talker. Κάντε κλικ για αποσύνδεση.
matrix-bound-failed = Ανατεθειμένη, αλλά η κράτηση του Talker απέτυχε: { $reason }. Κάντε κλικ για αποσύνδεση.
matrix-bound-formats-differ = Ανατεθειμένη, αλλά οι μορφές διαφέρουν: ο Talker στέλνει { $sent }, η είσοδος έχει οριστεί σε { $set }. Κάντε κλικ για αποσύνδεση.
matrix-formats-match = Οι μορφές ταιριάζουν ({ $format }). Κάντε κλικ για σύνδεση.
matrix-format-must-change = Η είσοδος δέχεται { $sent }, αλλά έχει οριστεί σε { $set }, οπότε ίσως δεν αναπαράγει μέχρι να αλλάξει η μορφή της. Κάντε κλικ για να επιλέξετε μορφή.
matrix-format-input = Αλλαγή της εισόδου σε
matrix-format-both = Αλλαγή και των δύο σε
matrix-format-talker-busy = Ο Talker μεταδίδει σε άλλους Listener, οπότε η μορφή του μένει ίδια.
matrix-format-anyway = Σύνδεση χωρίς αλλαγή μορφών
matrix-incompatible = Η είσοδος δεν δέχεται { $sent }. Έχει οριστεί σε { $set }.
matrix-other-domain = Κρατούν χρόνο σε διαφορετικούς τομείς ρολογιού, οπότε καμία ροή δεν μπορεί να περάσει μεταξύ τους.
matrix-group-none = Δεν υπάρχει σύνδεση. Αναπτύξτε για να συνδέσετε τις ροές μία προς μία.
matrix-group-connected = { $count ->
    [one] { $count } συνδεδεμένη. Αναπτύξτε για να τη δείτε.
   *[other] { $count } συνδεδεμένες. Αναπτύξτε για να δείτε την καθεμία.
}
matrix-outputs-expand = { $count ->
    [one] { $count } έξοδος ροής. Κάντε κλικ στο βέλος για ανάπτυξη, στο όνομα για επιθεώρηση.
   *[other] { $count } έξοδοι ροής. Κάντε κλικ στο βέλος για ανάπτυξη, στο όνομα για επιθεώρηση.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } έξοδος ροής. Κάντε κλικ στο βέλος για σύμπτυξη, στο όνομα για επιθεώρηση.
   *[other] { $count } έξοδοι ροής. Κάντε κλικ στο βέλος για σύμπτυξη, στο όνομα για επιθεώρηση.
}
matrix-inputs-expand = { $count ->
    [one] { $count } είσοδος ροής. Κάντε κλικ στο βέλος για ανάπτυξη, στο όνομα για επιθεώρηση.
   *[other] { $count } είσοδοι ροής. Κάντε κλικ στο βέλος για ανάπτυξη, στο όνομα για επιθεώρηση.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } είσοδος ροής. Κάντε κλικ στο βέλος για σύμπτυξη, στο όνομα για επιθεώρηση.
   *[other] { $count } είσοδοι ροής. Κάντε κλικ στο βέλος για σύμπτυξη, στο όνομα για επιθεώρηση.
}
matrix-stream-inspect = { $detail } Κάντε κλικ για επιθεώρηση της οντότητας { $entity }.
matrix-point = Δείξτε ένα κελί
matrix-point-note = για να δείτε τον Talker και τον Listener του και αν οι μορφές τους ταιριάζουν.
matrix-legend-waiting = Ανατεθειμένη, αναμονή για τη ροή
matrix-legend-trouble = Ανατεθειμένη, κάτι δεν πάει καλά
matrix-legend-open = Μπορεί να συνδεθεί
matrix-legend-change = Πρέπει πρώτα να αλλάξει η μορφή εισόδου
matrix-legend-incompatible = Οι μορφές δεν μπορούν να ταιριάξουν
matrix-legend-other-domain = Άλλος τομέας ρολογιού
matrix-talker-outputs = Έξοδοι Talker
matrix-listener-inputs = Είσοδοι Listener

common-thousands-separator = {"."}

## Diagnostics

diag-since-start = Μετρήθηκαν από την εκκίνηση της οντότητας.
diag-stream-input = Είσοδος ροής
diag-stream-output = Έξοδος ροής
diag-locked = { $count ->
    [0] δεν κλείδωσε
    [one] κλείδωσε μία φορά
   *[other] κλείδωσε { $number } φορές
}
diag-lost-lock = { $count ->
    [0] δεν έχασε το κλείδωμα
    [one] έχασε το κλείδωμα μία φορά
   *[other] έχασε το κλείδωμα { $number } φορές
}
diag-frames-in = { $count ->
    [one] { $number } εισερχόμενο πλαίσιο
   *[other] { $number } εισερχόμενα πλαίσια
}
diag-frames-out = { $count ->
    [one] { $number } εξερχόμενο πλαίσιο
   *[other] { $number } εξερχόμενα πλαίσια
}
diag-media-locked = { $count ->
    [0] δεν κλείδωσε στο ρολόι πολυμέσων
    [one] κλείδωσε στο ρολόι πολυμέσων μία φορά
   *[other] κλείδωσε στο ρολόι πολυμέσων { $number } φορές
}
diag-lost-media-lock = { $count ->
    [0] δεν έχασε το κλείδωμα πολυμέσων
    [one] έχασε το κλείδωμα πολυμέσων μία φορά
   *[other] έχασε το κλείδωμα πολυμέσων { $number } φορές
}
diag-interrupted = { $count ->
    [0] δεν διακόπηκε
    [one] διακόπηκε μία φορά
   *[other] διακόπηκε { $number } φορές
}
diag-out-of-sequence = { $count ->
    [one] { $number } πλαίσιο εκτός σειράς
   *[other] { $number } πλαίσια εκτός σειράς
}
diag-media-resets = { $count ->
    [one] { $number } επαναφορά πολυμέσων
   *[other] { $number } επαναφορές πολυμέσων
}
diag-timestamps-uncertain = { $count ->
    [0] χωρίς αβέβαιες χρονοσφραγίδες
    [one] αβέβαιες χρονοσφραγίδες μία φορά
   *[other] αβέβαιες χρονοσφραγίδες { $number } φορές
}
diag-no-timestamp = { $count ->
    [one] { $number } πλαίσιο χωρίς χρονοσφραγίδα
   *[other] { $number } πλαίσια χωρίς χρονοσφραγίδα
}
diag-unsupported-format = { $count ->
    [one] { $number } πλαίσιο σε μη υποστηριζόμενη μορφή
   *[other] { $number } πλαίσια σε μη υποστηριζόμενη μορφή
}
diag-late = { $count ->
    [one] { $number } καθυστερημένο πλαίσιο
   *[other] { $number } καθυστερημένα πλαίσια
}
diag-early = { $count ->
    [one] { $number } πρόωρο πλαίσιο
   *[other] { $number } πρόωρα πλαίσια
}
diag-started = { $count ->
    [0] δεν ξεκίνησε
    [one] ξεκίνησε μία φορά
   *[other] ξεκίνησε { $number } φορές
}
diag-stopped = { $count ->
    [0] δεν σταμάτησε
    [one] σταμάτησε μία φορά
   *[other] σταμάτησε { $number } φορές
}
diag-reservation-failed = η κράτηση του Talker απέτυχε: { $reason }
diag-latency = { $microseconds } µs συσσωρευμένη καθυστέρηση

## AVB Lite

lite-active = Ενεργό
lite-active-untagged = Ενεργό, χωρίς ετικέτα
lite-active-vlan = Ενεργό, VLAN { $vlan }
lite-capable = Υποστηρίζεται
lite-mode = Λειτουργία
lite-mode-capable = AVB (με υποστήριξη AVB Lite)
lite-because = Αιτία
lite-fallback-none = δεν δόθηκε αιτία
lite-fallback-endpoint = πέρασε η δήλωση άλλου τελικού σημείου, άρα δεν υπάρχει γέφυρα AVB ανάμεσά τους
lite-fallback-unanswered = εννέα αιτήματα peer delay έμειναν αναπάντητα
lite-fallback-responders = δύο ή περισσότεροι απάντησαν σε ένα αίτημα peer delay, άρα ο μεταγωγέας δεν είναι γέφυρα AVB
lite-fallback-configured = το όρισε ο χειριστής ή ένας ελεγκτής
lite-fallback-other = αιτία που δεν κατονομάζει το προφίλ
lite-other-profile = Άλλο προφίλ
lite-ptp-domain = { $profile }, τομέας { $domain }
lite-offset = Απόκλιση
lite-offset-from = { $offset } από { $grandmaster }
lite-media-vlan = VLAN πολυμέσων
lite-untagged = Χωρίς ετικέτα
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Έως { $count } Listener ανά ροή, μετά multicast
   *[other] Έως { $count } Listener ανά ροή, μετά multicast
}
lite-link = Ζεύξη
lite-bandwidth = Εύρος ζώνης
lite-egress-of = { $used } από { $link }, { $share }
lite-egress-of-assumed = { $used } από { $link }, { $share }, με υπόθεση ζεύξης gigabit
lite-egress-reported = Όπως μετρά η οντότητα τις ροές που έχει δεχτεί.
lite-egress-worked-out = Από τις μορφές των συνδεδεμένων εξόδων ροής της.
lite-alarm-offset = Απόκλιση PTP { $offset }, πάνω από τα 50 µs που επιτρέπει το AVB Lite
lite-alarm-egress = Εξερχόμενη κίνηση στο { $share } της ζεύξης, πάνω από το { $limit } που επιτρέπεται στις ροές

## AVB Wireless

wireless-station = Σταθμός
wireless-access-point = Σημείο πρόσβασης
wireless-role = Ρόλος
wireless-mode = Λειτουργία
wireless-time = Χρονισμός
wireless-mode-a-ftm = Mode A, 802.1AS μέσω FTM
wireless-mode-a-tm = Mode A, 802.1AS μέσω TM
wireless-mode-b = Mode B, από πλαίσια beacon
wireless-no-time = Χωρίς χρονισμό
wireless-other-mode = Λειτουργία που δεν κατονομάζει το προφίλ
wireless-locked = Κλειδωμένο
wireless-holdover = Σε holdover
wireless-not-locked = Μη κλειδωμένο
wireless-row-locked = Σταθμός, κλειδωμένος
wireless-row-holdover = Σταθμός, σε holdover
wireless-row-not-locked = Σταθμός, μη κλειδωμένος
wireless-row-access-point = { $count ->
    [one] Σημείο πρόσβασης, { $count } σταθμός
   *[other] Σημείο πρόσβασης, { $count } σταθμοί
}
wireless-link = Ζεύξη
wireless-channel = κανάλι { $channel }
wireless-not-known = Άγνωστη
wireless-signal = Ισχύς σήματος
wireless-rate = Ρυθμός μετάδοσης
wireless-ftm-valid = { $share } έγκυρες
wireless-rtt = χρόνος μετ' επιστροφής { $rtt }
wireless-bursts = { $count ->
    [one] ριπές { $count } πλαισίου
   *[other] ριπές { $count } πλαισίων
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = το σημείο πρόσβασης παραχωρεί ριπές FTM με πλήθος πλαισίων άλλο από τρία ή δύο
wireless-reason-measurement = ούτε FTM ούτε TM με το σημείο πρόσβασης
wireless-reason-signaling = κανένα μήνυμα Signaling gPTP-capable από το σημείο πρόσβασης
wireless-reason-other = αιτία που δεν κατονομάζει το προφίλ
wireless-servo = Σφάλμα servo
wireless-stations = Σταθμοί
wireless-station-count = { $count ->
    [one] { $count } σταθμός
   *[other] { $count } σταθμοί
}
wireless-no-ftm = χωρίς FTM
wireless-unserved = Μη εξυπηρετούμενοι Listener
wireless-frame-budget = Προϋπολογισμός πλαισίων
wireless-frame-use = { $used } από { $budget } πλαίσια ανά δευτερόλεπτο
wireless-stream-frames = Πλαίσια ροών
wireless-frames-of = { $readdressed } προς σταθμούς, { $unmapped } χωρίς Listener, { $dropped } απορρίφθηκαν, { $restored } από σταθμούς
wireless-class-a-allowed = Επιτρέπεται
wireless-class-a-not-allowed = Δεν επιτρέπεται
wireless-alarm-not-locked = Χρονισμός Wi-Fi μη κλειδωμένος στο σημείο πρόσβασης
wireless-alarm-holdover = Χρονισμός Wi-Fi σε holdover, χάθηκε το κλείδωμα στο σημείο πρόσβασης
wireless-alarm-unserved = { $count ->
    [one] { $count } Listener στη θύρα Wi-Fi δεν εξυπηρετείται, πάνω από το όριο unicast
   *[other] { $count } Listener στη θύρα Wi-Fi δεν εξυπηρετούνται, πάνω από το όριο unicast
}

## Log

log-all = Όλα
log-warnings = Προειδοποιήσεις
log-pause = Παύση
log-resume = Συνέχιση
log-clear = Καθαρισμός
log-empty = Κάθε πλαίσιο ATDECC που στέλνει και λαμβάνει το triib εμφανίζεται εδώ, με τα νεότερα πρώτα.
log-none-match = Κανένα αποθηκευμένο πλαίσιο δεν ταιριάζει με το φίλτρο.
log-frames = { $count ->
    [one] { $count } πλαίσιο
   *[other] { $count } πλαίσια
}
log-shown-of = { $shown } από { $all } πλαίσια
log-sent = Εστάλη
log-heard = Ελήφθη
log-not-decoded = Δεν αποκωδικοποιήθηκε
log-warning-short = Το πεδίο control_data_length του δηλώνει { $missing } οκτάδες πέρα από το τέλος του πλαισίου.
log-warning-undecodable = Δεν αποκωδικοποιείται: { $error }.
log-warning-long-acmp = Είναι στη μακριά μορφή ACMP, την οποία μια οντότητα Milan δεν επιτρέπεται να στέλνει (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Αντιστοιχίσεις καναλιών
mapping-inputs = Είσοδοι
mapping-outputs = Έξοδοι
mapping-port = θύρα { $number }
mapping-fixed = σταθερή
mapping-not-read = Δεν έχει διαβαστεί ακόμη.
mapping-no-clusters = Καμία συστάδα.
mapping-no-streams = Καμία ροή ήχου.
mapping-none = Καμία αντιστοίχιση.
mapping-not-mapped = Χωρίς αντιστοίχιση
mapping-cluster-numbered = Συστάδα { $index }

## Presets

presets-note = Μια προρύθμιση κρατά τις πηγές ρολογιού, τις συχνότητες δειγματοληψίας, τις μορφές ροών, τα χειριστήρια και τις συνδέσεις κάθε οντότητας. Η ανάκλησή της αλλάζει ό,τι διαφέρει.
presets-none = Δεν έχουν αποθηκευτεί ακόμη προρυθμίσεις.
presets-connections = { $count ->
    [one] { $count } σύνδεση
   *[other] { $count } συνδέσεις
}
presets-recall = Ανάκληση
presets-delete = Διαγραφή
presets-no-place = Δεν υπάρχει πού να αποθηκευτούν προρυθμίσεις: ο αρχικός φάκελος δεν είναι γνωστός.
presets-undeletable = Δεν ήταν δυνατή η διαγραφή του { $path }: { $error }.
presets-saved = { $count ->
    [one] Αποθηκεύτηκε η «{ $name }» με { $count } οντότητα.
   *[other] Αποθηκεύτηκε η «{ $name }» με { $count } οντότητες.
}
presets-nothing-differs = Τίποτα δεν διαφέρει από την «{ $name }».
presets-recalling = { $count ->
    [one] Ανάκληση της «{ $name }»: { $count } αλλαγή.
   *[other] Ανάκληση της «{ $name }»: { $count } αλλαγές.
}
presets-missing = { $report } Δεν υπάρχουν εδώ ή δεν έχουν διαβαστεί: { $missing }.
presets-deleted = Διαγράφηκε η «{ $name }».
presets-host-note = Κρατά επίσης τις οντότητες αυτού του υπολογιστή και τις ξεκινά ξανά κατά την ανάκληση.
presets-host-endpoints = { $count } σε αυτόν τον υπολογιστή
presets-starting-host = Εκκίνηση των οντοτήτων αυτού του υπολογιστή για την «{ $name }»· τα υπόλοιπα ακολουθούν μόλις επανέλθουν.

## Controls

control-numbered = Χειριστήριο { $index }
control-not-shown = Δεν εμφανίζεται εδώ
control-option = Επιλογή { $number }

## Network errors

network-permission = Το triib χρειάζεται άδεια για αποστολή και λήψη ακατέργαστων πλαισίων Ethernet.
network-needs-npcap = Το triib χρειάζεται το Npcap για αποστολή και λήψη ακατέργαστων πλαισίων Ethernet.
network-npcap-administrators = Το Npcap επιτρέπει μόνο σε διαχειριστές την αποστολή και λήψη ακατέργαστων πλαισίων Ethernet. Εκτελέστε το triib ως διαχειριστής ή εγκαταστήστε ξανά το Npcap χωρίς την επιλογή μόνο για διαχειριστές.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own entities

host-add-entity = Προσθήκη οντότητας
host-new-entity = Οντότητα κεντρικού υπολογιστή { $number }
host-failed = Δεν ήταν δυνατή η προσθήκη σε αυτόν τον υπολογιστή: { $reason }
host-needs-clock = Οι οντότητες αυτού του υπολογιστή χρειάζονται ενσύρματη διεπαφή με ρολόι υλικού PTP
host-no-ptp4l = Το ptp4l δεν απαντά, έτσι οι ροές αυτού του υπολογιστή δεν μπορούν να κρατήσουν χρόνο gPTP
host-starting = Εκκίνηση
host-starting-entity = Εκκίνηση { $name }
host-setting-up = Ρύθμιση της διεπαφής { $interface } για AVB
host-no-link = Δεν υπάρχει σύνδεση στη διεπαφή { $interface }
host-starting-ptp4l = Εκκίνηση του ptp4l
host-no-linuxptp = Το linuxptp δεν είναι εγκατεστημένο, έτσι οι ροές αυτού του υπολογιστή δεν μπορούν να κρατήσουν χρόνο gPTP
host-waiting-network = Αναμονή για το δίκτυο
host-elsewhere = Το triib-endpointd εκτελείται στη διεπαφή { $interface } για άλλον χρήστη ή ως root, έτσι οι οντότητες αυτού του υπολογιστή εκτελούνται εκεί, όχι εδώ
host-foreign-mrp = Ένα άλλο πρόγραμμα στέλνει δηλώσεις MSRP ή MVRP στη διεπαφή { $interface } από τη διεύθυνση αυτού του υπολογιστή και μπορεί να αποσύρει ό,τι χρειάζονται οι ροές αυτού του υπολογιστή
host-alarm-foreign-mrp = Ένα άλλο πρόγραμμα αυτού του υπολογιστή στέλνει δηλώσεις MSRP ή MVRP στη διεπαφή { $interface }
host-state = Κατάσταση
host-streaming = Μετάδοση
host-waiting = Αναμονή για Listener
host-listening = Ακρόαση
host-bound = Δεσμευμένο, αναμονή για τον Talker
host-unbound = Μη δεσμευμένο
host-audio-from = Ήχος από
host-audio-to = Ήχος προς
host-channels = Κανάλια
host-virtual-device = Εικονική συσκευή
host-add-stream = Προσθήκη ροής
host-add-audio-input = Είσοδος ήχου
host-add-audio-output = Έξοδος ήχου
host-add-clock-input = Είσοδος ρολογιού πολυμέσων
host-add-clock-output = Έξοδος ρολογιού πολυμέσων
host-remove-stream = Αφαίρεση ροής
host-silence = Σιωπή
host-tone = Ήχος δοκιμής
host-nowhere = Πουθενά
host-default-device = Προεπιλεγμένη συσκευή
host-remove = Αφαίρεση από αυτόν τον υπολογιστή

mac-audio-device = Συσκευή ήχου
mac-no-audio-device = Καμία όσο δεν είναι συνδεδεμένη καμία ροή
mac-entity-note = Το macOS εκτελεί αυτή την οντότητα. Όσο είναι συνδεδεμένη κάποια ροή της, το macOS δημιουργεί τη συσκευή ήχου μέσω της οποίας οι εφαρμογές αναπαράγουν και ηχογραφούν, με όσα κανάλια προς κάθε κατεύθυνση ορίζει η διαμόρφωση.
mac-open-audio-midi-setup = Άνοιγμα Audio MIDI Setup
mac-avb-off = Το AVB είναι απενεργοποιημένο στο { $interface }, οπότε αυτό το Mac δεν έχει δική του οντότητα AVB. Ενεργοποιήστε το στο πρόγραμμα περιήγησης συσκευών δικτύου του Audio MIDI Setup.

listen-section = Ακρόαση
listen-start = Ακρόαση
listen-stop = Διακοπή ακρόασης
listen-source = Ακούει
listen-received = Ό,τι λαμβάνει
listen-sent = Ό,τι στέλνει στη ροή { $number } ({ $channels } κανάλια)
listen-sent-needs-macos = Για να ακούσετε ό,τι στέλνει αυτός ο Mac χρειάζεται macOS 14.2 ή νεότερο.
listen-channels = Κανάλια
listen-channel = Κανάλι { $number }
listen-channel-pair = Κανάλια { $first }–{ $last }
listen-output = Έξοδος
listen-default-output = Προεπιλεγμένη έξοδος
listen-gain = Ενίσχυση
listen-gain-db = { $db } dB
listen-starting = Έναρξη…
listen-waiting-input = Αναμονή για { $device }, την οποία δημιουργεί το macOS όσο είναι συνδεδεμένη κάποια ροή της οντότητας.
listen-waiting-output = Αναμονή για { $device }.
listen-playing = Αναπαραγωγή, κορυφή στα { $db } dB
listen-playing-silent = Αναπαραγωγή, σιωπή
