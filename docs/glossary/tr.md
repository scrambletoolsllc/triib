# Turkish (tr) term choices

How `i18n/tr/triib.ftl` renders the terms in `docs/GLOSSARY.md`. The text
addresses the user formally (seçin, tıklayın) and uses “ ” for quotation
marks. Numbers use a full stop for thousands and a comma for decimals,
and the percent sign comes before the number (%80). Nouns stay singular
after a number, so most plural variants read the same.

Names that triib fills in are kept free of suffixes by letting a head noun
take them («X varlığının», «X akışının») or by
postpositions that need none («ile», «olarak», «üzerinde»). Suffixes on
the loanwords below follow their pronunciation, after an apostrophe.

The code uppercases the map's area labels and capitalizes the start of
role and diagnostics lists as Turkish writes them, *i* as *İ*
(«Alan bilinmiyor» shows as ALAN BİLİNMİYOR).

| English | Rendering | Note |
|---|---|---|
| talker | talker | Loanword, lower case mid-sentence: «talker'ın rezervasyonu», «Talker akışları». |
| listener | listener | As for talker: «hazır listener yok», «Listener girişleri». |
| grandmaster | grandmaster | Kept; never «ana saat» or a word built on «master». «Grandmaster'dan», «grandmaster'ını». |
| entity | varlık | The standard technical word; «cihaz» is kept for *device* in the network view. |
| entity model | varlık modeli | |
| controller | denetleyici | |
| stream | akış | «ses akışı», «medya saati akışı». |
| stream input, stream output | akış girişi, akış çıkışı | Matrix corner: «Talker çıkışları», «Listener girişleri». |
| connection, connect, bind | bağlantı, bağla; atanmış | Disconnect: «bağlantıyı kes». A bound input is «atanmış», kept apart from «bağlı» (connected and receiving). |
| media clock | medya saati | Also «Medya VLAN'ı», «medya sıfırlaması». |
| clock domain | saat alanı | The gPTP domain is «gPTP alanı». |
| clock source | saat kaynağı | |
| sampling rate | örnekleme hızı | |
| bridge | köprü | «AVB köprüsü», «Köprü portu»; the one mention of the switch itself is «switch». |
| reservation | rezervasyon | As in «kaynak rezervasyonu». |
| egress | çıkış (trafiği, portu) | Column and alarm: «Çıkış trafiği»; egress port: «çıkış portu». |
| link | link | The word network engineers use; «bağlantı» stays for *connection*. «Link kuruldu», «Link koptu», «Link kopmaları». |
| peer delay | peer delay | Kept, as PTP writing does: «peer delay isteği». |
| offset | sapma | «PTP sapması». |
| hop | atlama | «Grandmaster'dan atlama sayısı». |
| unicast, multicast | unicast, multicast | Kept, as practitioners say them. |
| fan-out | (phrase) | Not shown as a word; the inspector says «Akış başına en fazla N listener, sonra multicast». |
| descriptor | tanımlayıcı | As in «dosya tanımlayıcısı». |
| cluster | küme | A technical grouping; «grup» stays for entity groups. |
| stream port | akış portu | |
| channel mapping | kanal eşlemesi | Verbs: «eşle», «eşlemeyi kaldır»; «Eşlenmedi». |
| control | kontrol | «Kontroller» for the tab; *controller* is «denetleyici». |
| preset | ön ayar | «Geri çağır» for recall. |
| identify | tanımla | The button is «Tanımla». |
| counter | sayaç | |
| locked, lost lock | kilitlendi, kilit kaybedildi | Media lock: «medya saatine kilitlendi», «medya kilidi». |
| holding over | holdover modunda | Kept, as telecom engineers say it: «Holdover modunda», «İstasyon, holdover modunda». |
| station | istasyon | The IEEE 802.11 term; router pages may say «istemci». |
| access point | erişim noktası | As on a router's settings page. |
| beacon | beacon | Kept, with a head noun taking the suffix: «beacon çerçevelerinden». |
| signal | sinyal | The label is «Sinyal gücü» (signal strength). |
| interrupted | kesildi | |
| timestamp | zaman damgası | |
| advertise, advertised | duyur, duyuruldu | Also for an entity announcing itself: «kendini duyurmak». The ADP section is «Duyuru». |
| interface | arayüz | «Ağ arayüzü», «AVB arayüzü». |
| hardware clock | donanım saati | |
| virtual (interface) | sanal | |

Other recurring words: frame «çerçeve», discovery «keşif», heard
«algılandı» (in the map) and «Alındı» (in the log), format «biçim»,
inspector «İnceleyici», log «Günlük».

## Choices a native speaker should check

- «Tanımla» for *identify* next to «tanımlayıcı» for *descriptor*: the
  two share a root; «Belirle» or «Konumu göster» are alternatives for the
  button.
- «atanmış» for *bound*.
- «link» kept for *link*, since «bağlantı» is already *connection*.
- «rezervasyon» rather than «ayırma» for *reservation*.
- «İnceleyici» for the Inspector, chosen to keep apart from
  «denetleyici» (controller).
- «istasyon» for *station* rather than «istemci», and «holdover modunda»
  kept in English.
