# Greek (el) term choices

How `i18n/el/triib.ftl` renders the terms in `docs/GLOSSARY.md`. The text
addresses the user in the polite plural (Επιλέξτε, Κάντε κλικ), uses « »
for quotation marks and the ano teleia (·) where English has a semicolon.
Numbers use a full stop for thousands and a comma for decimals, and the
percent sign follows the number with no space (80%).

| English | Rendering | Note |
|---|---|---|
| talker | Talker (ο), invariable | Kept in Latin script and capitalized, like other networking terms Greek writes in Latin; the article carries the case: «του Talker», «Ροές Talker». |
| listener | Listener (ο), invariable | As for talker: «κανένας Listener έτοιμος», «Είσοδοι Listener». |
| grandmaster | Grandmaster (ο), invariable | Kept in Latin script; never a word built on «master» (κύριο, αφέντης). |
| entity | οντότητα | Not «συσκευή», which is kept for *device* in the network view. |
| entity model | μοντέλο οντότητας | |
| controller | ελεγκτής | The role is «ελεγκτής» in the role list as well. |
| stream | ροή | Feminine, so «συνδεδεμένη», «ανακοινωμένη» agree with it. |
| stream input, stream output | είσοδος ροής, έξοδος ροής | Matrix corner: «Έξοδοι Talker», «Είσοδοι Listener». |
| connection, connect, bind | σύνδεση, συνδέω; ανατεθειμένη | A bound input is «ανατεθειμένη» (assigned to the talker's stream), kept apart from «συνδεδεμένη» (connected and receiving) and from «κράτηση» (reservation). |
| media clock | ρολόι πολυμέσων | Also «ροές ρολογιού πολυμέσων», «VLAN πολυμέσων», «κλείδωμα πολυμέσων». |
| clock domain | πεδίο ρολογιού | The digital-audio term; the gPTP domain is a different «τομέας». |
| clock source | πηγή ρολογιού | |
| sampling rate | συχνότητα δειγματοληψίας | The usual Greek audio term, though long for an inspector label. |
| bridge | γέφυρα | The networking term for an IEEE 802.1 bridge; the one mention of the switch itself is «μεταγωγέας». |
| reservation | κράτηση | As in «κράτηση πόρων» (resource reservation). |
| egress | εξερχόμενη (κίνηση, θύρα) | Column and alarm: «Εξερχόμενη κίνηση»; egress port: «εξερχόμενη θύρα». |
| link | ζεύξη | «Ζεύξη ενεργή», «Ζεύξη ανενεργή», «Πτώσεις ζεύξης», «Καθυστέρηση ζεύξης». |
| peer delay | Peer delay | Kept in Latin script, as PTP writing does: «αιτήματα peer delay». |
| offset | απόκλιση | «Απόκλιση PTP». |
| hop | άλμα | «Άλματα από τον Grandmaster». |
| unicast, multicast | unicast, multicast | Kept, as practitioners say them. |
| fan-out | (phrase) | Not shown as a word; the inspector says «Έως N Listener ανά ροή, μετά multicast». |
| descriptor | περιγραφέας | As in «περιγραφέας αρχείου». |
| cluster | συστάδα | A technical grouping; «ομάδα» stays for entity groups. |
| stream port | θύρα ροής | |
| channel mapping | αντιστοίχιση καναλιών | Verbs: «αντιστοίχιση», «κατάργηση της αντιστοίχισης»; «Χωρίς αντιστοίχιση». |
| control | χειριστήριο | «Χειριστήρια» for the tab; «Ρυθμίσεις» stays for Settings. |
| preset | προρύθμιση | «Ανάκληση» for recall. |
| identify | αναγνώριση | The button is «Αναγνώριση». |
| counter | μετρητής | |
| locked, lost lock | κλείδωσε, έχασε το κλείδωμα | PLL lock: «κλείδωσε στο ρολόι πολυμέσων». |
| holding over | σε holdover | Kept in Latin script, as Greek telecom and PTP writing does, like peer delay: «Σε holdover», «Σταθμός, σε holdover». |
| station | σταθμός | The IEEE 802.11 term: «Σταθμός, κλειδωμένος», «3 σταθμοί». |
| access point | σημείο πρόσβασης | As on a router's settings page. |
| beacon | beacon | Kept in Latin script: «Mode B, από πλαίσια beacon». |
| signal | σήμα | The label is «Ισχύς σήματος» (signal strength), as phones show it. |
| interrupted | διακόπηκε | |
| timestamp | χρονοσφραγίδα | |
| advertise, advertised | ανακοινώνω, ανακοινωμένη | Also for an entity announcing itself: «ανακοινώνει την παρουσία της». The ADP section is «Ανακοίνωση». |
| interface | διεπαφή | Feminine: «ενεργή», «ασύρματη», «εικονική» agree with it. |
| hardware clock | ρολόι υλικού | |
| virtual (interface) | εικονική | |

Other recurring words: frame «πλαίσιο», discovery «ανακάλυψη», heard
«εντοπίστηκε» (in the map) and «Ελήφθη» (in the log), format «μορφή»,
inspector «Επιθεωρητής», log «Αρχείο καταγραφής».

## Choices a native speaker should check

- «ανατεθειμένη» for *bound*: «δεσμευμένη» is the literal word but reads
  as *reserved*, next to «κράτηση» for the reservation.
- «Talker», «Listener», «Grandmaster» capitalized throughout; lower case
  («ο talker») is also seen in Greek technical writing.
- «γέφυρα» for *bridge*: engineers often say «switch AVB».
- «ζεύξη» for *link*: correct, but some engineers simply say «link».
- «ρολόι πολυμέσων» for *media clock*, and «συχνότητα δειγματοληψίας»,
  which may be too long for the inspector's labels.
- The map's area labels are uppercased by the code, which drops the tonos
  as Greek capitals do («Άγνωστος τομέας» shows as ΑΓΝΩΣΤΟΣ ΤΟΜΕΑΣ).
- «σε holdover» kept in Latin script; «σε κατάσταση διατήρησης» is the
  Greek phrase. The station's *Time* label is «Χρονισμός».
