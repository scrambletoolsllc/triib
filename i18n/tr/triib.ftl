# triib's interface text in Turkish. Term choices: docs/glossary/tr.md.

## Language

language-name = Türkçe

## Common

common-close = Kapat
common-more = Daha fazla
common-keep-toolbar-shown = Araç çubuğunu sürekli göster
common-auto-hide-toolbar = Araç çubuğunu otomatik gizle

## Settings

settings-title = Ayarlar
settings-general = Genel
settings-appearance = Görünüm
settings-language = Dil
settings-language-system = Sistem varsayılanı: { $language }
settings-language-note = Metin alanlarına sistemin giriş dilinde yazılır.
settings-appearance-system = Sistem
settings-appearance-light = Açık
settings-appearance-dark = Koyu
settings-colors = Renkler
settings-system-accent = Sistemin vurgu rengini kullan
settings-accent-picked = Aşağıdaki renk, triib'in renklerinin temelini oluşturur.
settings-accent-omarchy = Omarchy temasından: { $theme }.
settings-accent-desktop = Masaüstünün vurgu renginden.
settings-accent-none = Masaüstünün vurgu rengi yok, bu yüzden aşağıdaki renk kullanılıyor.
settings-motion = Hareket
settings-animations = Animasyonlar
settings-animations-note = Bir şey değiştiğinde yaylanma ve kayma efektleri.
settings-animations-reduced = Masaüstü azaltılmış hareket istiyor, bu yüzden triib hareketsiz kalıyor.
settings-outside-control = Dışarıdan denetim
settings-allow-outside-control = Dışarıdan denetime izin ver
settings-allow-outside-control-note = Claude Code gibi ajanlar, triib --mcp aracılığıyla AVB ağınızı görüp değiştirebilir. triib her yeni ajandan önce sorar.
settings-allowed-agents = İzin verilenler: { $agents }
settings-forget-agents = Unut
settings-storage = Depolama
settings-storage-presets = Ön ayarlar klasörü
settings-storage-entities = Varlık dosyası
settings-storage-apply = Uygula
settings-storage-choose = Seç…
settings-storage-note = Eski bir konumda saklanan ön ayarlar orada kalır; kullanmaya devam etmek için onları yeni konuma taşıyın. triib ayarları { $file } dosyasına kaydedilir.
settings-storage-entities-note = Henüz olmayan bir varlık dosyası kullanılanın kopyası olarak başlar ve triib-endpointd onunla yeniden başlar.
dialog-choose-presets = Ön ayarlar klasörünü seçin
dialog-choose-entities = Varlık dosyasının klasörünü seçin
settings-full-path = ~/Documents/triib gibi tam bir yol kullanın.
settings-path-is-folder = { $path } bir klasör, dosya değil.
settings-folder-missing = { $path } adlı bir klasör yok. Önce oluşturun ya da bir klasör seçin.
settings-path-is-file = { $path } bir dosya, klasör değil.
settings-cannot-write = triib, { $path } konumuna yazamıyor: { $error }.
app-file-dialog-failed = Dosya iletişim kutusu gösterilemedi: { $error }
agent-prompt-title = { $agent } triib'i denetleyebilsin mi?
agent-prompt-body = { $agent }, triib'in dışarıdan denetimini kullanmak istiyor: ağınızdaki varlıkları görmek ve değiştirmek, akışları bağlamak, biçimleri, saatleri ve adları ayarlamak için. Dışarıdan denetimi Ayarlar'dan kapatabilirsiniz.
agent-prompt-allow = İzin ver
agent-prompt-deny = İzin verme

common-cancel = İptal
common-save = Kaydet
common-not-set = Ayarlanmadı
common-unnamed = Adsız
common-none = Yok
common-mac-address = MAC adresi
common-list-separator = {", "}

## Network interfaces

interface-up = etkin
interface-link-down = link yok
interface-wireless = kablosuz
interface-hardware-clock = donanım saati
interface-hardware-clock-named = donanım saati { $clock }
interface-virtual = sanal

## Toolbar

toolbar-choose-interface = Bir arayüz seçin
toolbar-interface = Ağ arayüzü
toolbar-filter-interfaces = Arayüzleri filtrele
toolbar-hide-virtual = Sanal arayüzleri gizle
toolbar-hide-wireless = Kablosuz arayüzleri gizle
toolbar-hide-unclocked = Donanım saati olmayan arayüzleri gizle
toolbar-connections = Bağlantılar
toolbar-network = Ağ
toolbar-entities = Varlıklar
toolbar-rediscover = Her varlıktan kendini duyurmasını iste
toolbar-rescan = Tüm varlıkları temizle ve yeniden tara
toolbar-search = Varlıklarda ve akışlarda ara
toolbar-presets = Ön ayarlar
toolbar-log = Günlük
toolbar-inspector = İnceleyici
toolbar-settings = Ayarlar

## The network's state, in place of a view

state-no-interface = Arayüz yok
state-no-interface-note = Varlıkları keşfetmek için AVB ağındaki arayüzü seçin.
state-starting = Başlatılıyor
state-starting-note = { $interface } açılıyor.
state-listening = Dinleniyor
state-listening-note = { $interface } üzerindeki varlıklar kendilerini duyurdukça burada görünür.
state-permission-needed = İzin gerekli
state-npcap-needed = Npcap gerekli
state-get-npcap = Npcap'i indir
state-copy-command = Komutu kopyala
state-cannot-use = { $interface } kullanılamıyor
state-try-again = Yeniden dene

## Entity list

entities-none-yet = Henüz varlık yok
entities-none-yet-note = Ağdaki her varlık; rolleri, SR sınıfları ve saatiyle birlikte.

## Inspector

inspector-title = İnceleyici
inspector-entity = Varlık
inspector-streams = Akışlar
inspector-controls = Kontroller
inspector-diagnostics = Tanılama
inspector-descriptors = Tanımlayıcılar
inspector-select = Ayrıntılarını görmek için bir varlık seçin.
inspector-offline = { $entity } çevrim dışı.
inspector-heard = Kendini duyurmuyor: triib onu gönderdiği komutlardan duydu ve komutsuz 5 dakika geçene kadar listede tutar.
inspector-rename = Yeniden adlandır
inspector-name = Ad
inspector-identify = Tanımla
inspector-model-not-read = Varlık modeli okunmadı.
inspector-no-streams = Akış yok.
inspector-no-controls = Gösterilecek kontrol yok.
inspector-no-diagnostics = Arayüz veya sayaç bildirilmedi.
inspector-reading = Tanımlayıcılar okunuyor, şu ana kadar { $count }.
inspector-read-failed = Varlık modeli okunamadı: { $reason }.

entity-section = Varlık
entity-name = Ad
entity-triib-controller = triib denetleyicisi
entity-triib-on = { $host } üzerinde triib
entity-triib-here = Bu bilgisayarda triib
entity-controller-on = { $host } üzerinde denetleyici
entity-controller-here = Bu bilgisayarda denetleyici
entity-controller-at = { $mac } adresinde denetleyici
entity-group = Grup
entity-product = Ürün
entity-firmware = Ürün yazılımı
entity-serial-number = Seri numarası
entity-configuration = Yapılandırma
entity-configuration-of = { $name } ({ $number }/{ $count })
entity-milan = Milan
entity-media-clock = Medya saati
entity-clock-domain = Saat alanı
entity-sampling-rate = Örnekleme hızı
clock-source-numbered = Kaynak { $index }
rate-pull = pull { $pull }

stream-format = Biçim
stream-id = Akış ID'si
stream-destination = Hedef
stream-max-transit = Azami geçiş süresi

avb-interfaces = AVB arayüzleri
avb-interface = Arayüz
avb-interface-clock-identity = Saat kimliği
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, alan { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Çalışan
avb-interface-none-reported = Bildirilmedi
avb-interface-path = Yol
avb-interface-own-grandmaster = Kendisi grandmaster
avb-interface-hops = { $count ->
    [one] Grandmaster'dan { $count } atlama uzakta
   *[other] Grandmaster'dan { $count } atlama uzakta
}
avb-interface-link-up = Link kuruldu
avb-interface-link-down = Link koptu
avb-interface-grandmaster-changes = Grandmaster değişiklikleri
avb-interface-frames-sent = Gönderilen çerçeveler
avb-interface-frames-received = Alınan çerçeveler
avb-interface-crc-errors = CRC hataları

tree-firmware = Ürün yazılımı { $version }
tree-descriptor-types = { $count ->
    [one] { $count } tanımlayıcı türü
   *[other] { $count } tanımlayıcı türü
}
tree-clock = Saat
tree-clock-source-from = { $kind }, kaynak: { $location } { $index }
tree-clock-domain-using = Kullanılan: { $source }
tree-clusters = { $count ->
    [one] { $count } küme
   *[other] { $count } küme
}
tree-maps = { $count ->
    [one] { $count } eşleme
   *[other] { $count } eşleme
}

advert-not-advertised = Duyurulmadı
advert-identity = Kimlik
advert-entity-id = Varlık ID'si
advert-entity-model = Varlık modeli
advert-roles = Roller
advert-talker = Talker
advert-listener = Listener
advert-clock = Saat
advert-btc = BTC
advert-gptp-domain = gPTP alanı
advert-sr-classes = SR sınıfları
advert-indexes = Varlık modeli dizinleri
advert-identify-control = Tanımlama kontrolü
advert-avb-interface = AVB arayüzü
advert-advertising = Duyuru
advert-valid-time = Geçerlilik süresi
advert-available-index = Kullanılabilirlik dizini
advert-association = İlişkilendirme
advert-capabilities = Yetenekler

## Status bar

status-entities = { $count ->
    [one] { $count } varlık
   *[other] { $count } varlık
}
status-not-discovering = Keşif yapılmıyor
status-discovering = Keşif yapılıyor
status-discovering-as = { $controller } olarak keşif yapılıyor
status-stopped = Bir hata nedeniyle durdu
status-alarm = Alarm
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } ve { $count } tane daha

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = denetleyici
role-none = rol yok
classes-a-and-b = A ve B
clock-no-gptp = gPTP yok

read-not-read = Okunmadı
read-reading = Okunuyor, şu ana kadar { $count }
read-ready-unreadable = Hazır, { $count } okunamayan
read-ready-cached = Hazır, önbellekten
read-ready = Hazır
read-failed = Başarısız: { $reason }

milan-no = Hayır
milan-before-1-3 = 1.3 öncesi
milan-certified = { $version }, sertifikalı { $certification }
milan-not-certified = { $version }, sertifikasız

outcome-status = durum { $status }
outcome-no-response = yanıt yok
outcome-not-possible = mümkün değil
outcome-connect = { $talker } ile { $listener } bağlanamadı: { $reason }.
outcome-disconnect = { $listener } bağlantısı kesilemedi: { $reason }.
outcome-identify = { $entity } tanımlanamadı: { $reason }.
outcome-rename = { $what } “{ $name }” olarak yeniden adlandırılamadı: { $reason }.
outcome-rename-group = { $entity } varlığının grubu “{ $name }” olarak yeniden adlandırılamadı: { $reason }.
outcome-format-streaming = { $stream } akışının biçimi değiştirilemedi: akış sürüyor. Önce bağlantısını kesin.
outcome-format = { $stream } akışının biçimi değiştirilemedi: { $reason }.
outcome-max-transit-time = { $stream } akışının azami geçiş süresi değiştirilemedi: { $reason }.
outcome-max-transit-time-streaming = { $stream } akışının azami geçiş süresi değiştirilemedi: akış sürüyor. Önce bağlantısını kesin.
outcome-sampling-rate = { $entity } varlığının örnekleme hızı değiştirilemedi: { $reason }.
outcome-clock-source = { $entity } varlığının saat kaynağı değiştirilemedi: { $reason }.
outcome-map = { $entity } varlığında kanal eşlenemedi: { $reason }.
outcome-unmap = { $entity } varlığında kanal eşlemesi kaldırılamadı: { $reason }.
outcome-control = { $entity } varlığında “{ $control }” ayarlanamadı: { $reason }.
outcome-control-numbered = { $entity } varlığında { $index } numaralı kontrol ayarlanamadı: { $reason }.

stream-not-connected = Bağlı değil
stream-from = Kaynak: { $stream }
stream-from-receiving = Kaynak: { $stream }, alınıyor
stream-from-waiting = Kaynak: { $stream }, talker bekleniyor
stream-from-failed = Kaynak: { $stream }, talker'ın rezervasyonu başarısız: { $reason }
stream-sending-to = { $destination } adresine gönderiliyor

failure-no-response = yanıt vermedi
failure-refused = { $status } ile reddetti
failure-malformed = yanıtı çözümlenemedi
failure-on-this-computer = bu bilgisayarda çalışıyor; başka bir bilgisayardan okuyun

msrp-failure-1 = yetersiz bant genişliği
msrp-failure-2 = yetersiz köprü kaynakları
msrp-failure-3 = trafik sınıfı için yetersiz bant genişliği
msrp-failure-4 = akış ID'si başka bir talker tarafından kullanılıyor
msrp-failure-5 = hedef adres zaten kullanılıyor
msrp-failure-6 = daha yüksek dereceli bir akış tarafından yerinden edildi
msrp-failure-7 = bildirilen gecikme değişti
msrp-failure-8 = çıkış portu AVB uyumlu değil
msrp-failure-9 = farklı bir hedef adres kullanın
msrp-failure-10 = MSRP kaynakları tükendi
msrp-failure-11 = MMRP kaynakları tükendi
msrp-failure-12 = hedef adres saklanamıyor
msrp-failure-13 = öncelik bir SR sınıfı önceliği değil
msrp-failure-14 = çerçeveler ortam için çok büyük
msrp-failure-15 = fan-in port sınırına ulaşıldı
msrp-failure-16 = kayıtlı bir akışın ilk değeri değişti
msrp-failure-17 = VLAN çıkış portunda engellendi
msrp-failure-18 = VLAN etiketleme çıkış portunda devre dışı
msrp-failure-19 = SR sınıfı önceliği uyuşmuyor
msrp-failure-unknown = bilinmeyen neden
msrp-failure-at = { $reason }, { $bridge } köprüsünde

## Entity list columns

column-vendor = Üretici
column-model = Model
column-state = Durum
column-entity-model-id = Varlık modeli ID'si
column-talker-streams = Talker akışları
column-listener-streams = Listener akışları
column-avb-lite = AVB Lite
column-egress = Çıkış trafiği
column-wireless = Kablosuz

## Settings file

settings-no-place = Ayarları saklayacak bir yer yok: ana klasör bilinmiyor.
settings-unusable = { $path } kullanılamadı: { $error }.
settings-unsaved = { $path } kaydedilemedi: { $error }.

column-remove = Sütunu kaldır
column-move-left = Sola taşı
column-move-right = Sağa taşı
column-add = Sütun ekle
common-percent = %{ $value }

## Network view

netmap-empty = Henüz gösterilecek ağ yok
netmap-empty-note = Varlıklar okunup gPTP ağacındaki yerlerini bildirdikten sonra burada görünür.
netmap-focus-clock-path = { $name } saat yolu
netmap-focus-streams = { $name } akışları
netmap-showing = { $what } gösteriliyor
netmap-devices = { $count ->
    [one] { $count } cihaz
   *[other] { $count } cihaz
}
netmap-bridges = { $count ->
    [one] { $count } köprü
   *[other] { $count } köprü
}
netmap-show-map = Haritayı göster
netmap-show-details = Ayrıntıları göster
stream-numbered = Akış { $index }
netmap-bridge = Köprü
netmap-access-point = Erişim noktası
netmap-device = Cihaz
netmap-this-computer = Bu bilgisayar
netmap-connected = Bağlı
netmap-advertised = Duyuruldu, hazır listener yok
netmap-advertised-off-tree = Duyuruldu, hazır listener yok ({ $listener } gPTP ağacı dışında)
netmap-failed-at = { $bridge } köprüsünde rezervasyon başarısız: { $reason }
netmap-failed = Rezervasyon başarısız: { $reason }
netmap-no-bridge-on = { $interface } üzerinde köprü algılanmadı
netmap-cannot-listen-on = { $interface } üzerinde gPTP dinlenemiyor
netmap-on-this-computer = Bu bilgisayarda
netmap-path-not-reported = Yol bildirilmedi
netmap-gptp-not-reported = gPTP bildirilmedi
netmap-off-tree = gPTP ağacı dışında
netmap-off-ptp = PTP ağacı dışında
netmap-not-lite = AVB Lite dışında
netmap-lite-not-reported = AVB Lite bildirilmedi
netmap-synced = Senkronize
netmap-not-synced = Senkronize değil
netmap-triib-on = { $interface } üzerinde triib
netmap-through-count = { $count } geçen
netmap-out = { $count } giden
netmap-in = { $count } gelen
netmap-failed-count = { $count } başarısız
netmap-advertised-only = Yalnızca duyuruldu
netmap-failed-state = Başarısız
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = gPTP ağacı dışında: kendisi grandmaster
netmap-apart-no-path = Yolu bildirilmedi; { $grandmaster } grandmaster'ını izliyor
netmap-apart-unreported = gPTP durumunu bildirmedi
netmap-apart-no-neighbor = Bu bilgisayarın arayüzünde köprü algılanmadı
netmap-apart-cannot-listen = Bu bilgisayar kendi arayüzünde gPTP dinleyemiyor
netmap-apart-on-this-computer = Bu bilgisayarda çalışıyor; gPTP durumunu görmek için başka bir bilgisayardan okuyun
netmap-apart-not-lite = AVB Lite çalıştırmıyor, bu yüzden grandmaster'ı izlemiyor
netmap-apart-lite-unreported = AVB Lite hakkında hiçbir şey bildirmiyor, bu yüzden neyi izlediği bilinmiyor
netmap-clock-tree = Saat ağacı
netmap-no-grandmaster = Grandmaster algılanmadı
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Dikkat gerektiriyor
netmap-nodes-below = Alttaki düğümler
netmap-bridges-below = Alttaki köprüler
netmap-clock-path = Saat yolu
netmap-hops = Grandmaster'dan atlama sayısı
netmap-link-delay = Link gecikmesi
netmap-bridge-port = Köprü portu
netmap-link-drops = Link kopmaları
netmap-synced-to-grandmaster = Grandmaster ile senkronize
netmap-host-no-gptp = Senkronize değil: bu bilgisayar gPTP çalıştırmıyor
netmap-link-no-gptp = Senkronize değil: linkinde gPTP çalışmıyor
netmap-ptp-offset-high = Senkronize değil: grandmaster'dan { $offset } sapma, AVB Lite için izin verilen 50 µs sınırının üzerinde
netmap-ptp-no-offset = Senkronize değil: grandmaster'dan hiçbir sapma ölçmedi
netmap-audio = Ses
netmap-media-clock-streams = Medya saati akışları
netmap-audio-streams = Ses akışları
netmap-bound = { $count } atanmış
netmap-flowing = Akıyor
netmap-advertised-state = Duyuruldu
netmap-media-clock-stream = Medya saati akışı
netmap-audio-stream = Ses akışı
netmap-reaches = Ulaştığı yer
netmap-passing-count = { $count ->
    [one] { $count } geçen akış
   *[other] { $count } geçen akış
}
netmap-through = Geçen
netmap-passing-through = Geçtiği köprüler
netmap-sending = Gönderilen
netmap-receiving = Alınan
netmap-problems = Sorunlar
netmap-help-back = Genel görünüme dönmek için arka plana tıklayın.
netmap-help-stream = İncelemek için bir akışa, genel görünüme dönmek için arka plana tıklayın.
netmap-help-ptp = AVB Lite'ta saat, uçtan uca grandmaster'dan her cihaza akar; aradaki switch'ler buna katılmaz, bu yüzden hiçbiri gösterilmez. Bir cihaz, grandmaster'ı 50 µs içinde izlediği sürece senkronizedir. Saatini incelemek için bir cihaza veya kablosuna tıklayın; seçimi temizlemek için arka plana tıklayın.
netmap-help-clock = Saat, grandmaster'dan her köprü üzerinden ağaçtaki her düğüme akar. Kesik gri çizgi, gPTP çalışmayan bir linktir. Saat yolunu incelemek için bir cihaza veya kablosuna tıklayın; seçimi temizlemek için arka plana tıklayın.
netmap-help-media-clock = Yalnızca medya saati (CRF) akışları, ses gibi çizilir: akış başına bir kablo, talker'a göre renklendirilmiş. Akışını incelemek için bir kabloya, akışlarını görmek için bir cihaza tıklayın; seçimi temizlemek için arka plana tıklayın.
netmap-help-audio = Her akışın kendi kablosu vardır; geçtiği her köprüye girer ve oradan çıkar. Renk talker'a göredir: her talker'ın bir tonu vardır ve akışları bu tonun açık ve koyu halleridir. Hareket eden noktalar sesin aktığını gösterir; duran kırmızı çizgi başarısız bir rezervasyon, duran gri çizgi ise hazır listener'ı olmayan duyurulmuş bir akıştır; ikisi de rezervasyonun durduğu yerde biter. Ortadaki sütundaki cihazlar doğrudan grandmaster'ın köprüsüne bağlanır. Akışını incelemek için bir kabloya, akışlarını görmek için bir cihaza tıklayın; seçimi temizlemek için arka plana tıklayın.

## Connections

matrix-nothing-shown = Gösterilecek akış yok
matrix-nothing-shown-note = Daha fazla akış görmek için aramayı veya filtreleri değiştirin.
matrix-empty = Bağlanacak akış yok
matrix-empty-note = Talker ve listener akışları, bu akışlara sahip varlıklar okunduğunda burada buluşur.
matrix-all-streams = Tüm akışlar
matrix-connectable-only = Bağlanamayanları gizle
matrix-none-hidden = Gösterilen her akış bağlanabilir
matrix-hidden = { $count ->
    [one] { $count } akış gizli
   *[other] { $count } akış gizli
}
matrix-own = Bir varlığın çıkışları kendi girişlerine bağlanmaz.
matrix-working = Üzerinde çalışılıyor.
matrix-waiting-change = Bu girişteki son değişiklik bekleniyor.
matrix-connected = Bağlı ve alıyor. Bağlantıyı kesmek için tıklayın.
matrix-bound-waiting = Atanmış, talker'ın akışı bekleniyor. Bağlantıyı kesmek için tıklayın.
matrix-bound-failed = Atanmış, ancak talker'ın rezervasyonu başarısız: { $reason }. Bağlantıyı kesmek için tıklayın.
matrix-bound-formats-differ = Atanmış, ancak biçimler farklı: talker'ın gönderdiği { $sent }, girişin ayarı { $set }. Bağlantıyı kesmek için tıklayın.
matrix-formats-match = Biçimler uyuşuyor ({ $format }). Bağlamak için tıklayın.
matrix-format-must-change = Giriş { $sent } biçimini kabul ediyor ancak { $set } olarak ayarlı, bu yüzden biçimi değişene kadar çalmayabilir. Bir biçim seçmek için tıklayın.
matrix-format-input = Girişi şuna değiştir
matrix-format-both = İkisini de şuna değiştir
matrix-format-talker-busy = Talker başka listener'lara yayın yapıyor, bu yüzden biçimi değişmez.
matrix-format-anyway = Biçimleri değiştirmeden bağla
matrix-incompatible = Giriş { $sent } biçimini kabul etmiyor. Ayarı: { $set }.
matrix-group-none = Bağlı değil. Akışları tek tek bağlamak için genişletin.
matrix-group-connected = { $count } bağlı. Her birini görmek için genişletin.
matrix-outputs-expand = { $count ->
    [one] { $count } akış çıkışı. Genişletmek için oka, incelemek için ada tıklayın.
   *[other] { $count } akış çıkışı. Genişletmek için oka, incelemek için ada tıklayın.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } akış çıkışı. Daraltmak için oka, incelemek için ada tıklayın.
   *[other] { $count } akış çıkışı. Daraltmak için oka, incelemek için ada tıklayın.
}
matrix-inputs-expand = { $count ->
    [one] { $count } akış girişi. Genişletmek için oka, incelemek için ada tıklayın.
   *[other] { $count } akış girişi. Genişletmek için oka, incelemek için ada tıklayın.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } akış girişi. Daraltmak için oka, incelemek için ada tıklayın.
   *[other] { $count } akış girişi. Daraltmak için oka, incelemek için ada tıklayın.
}
matrix-stream-inspect = { $detail } { $entity } varlığını incelemek için tıklayın.
matrix-point = Bir hücrenin üzerine gelin
matrix-point-note = ve talker'ını, listener'ını ve biçimlerinin uyuşup uyuşmadığını görün.
matrix-legend-waiting = Atanmış, akış bekleniyor
matrix-legend-trouble = Atanmış, bir sorun var
matrix-legend-open = Bağlanabilir
matrix-legend-change = Önce giriş biçimi değişmeli
matrix-legend-incompatible = Biçimler uyuşamaz
matrix-talker-outputs = Talker çıkışları
matrix-listener-inputs = Listener girişleri

common-thousands-separator = {"."}

## Diagnostics

diag-since-start = Varlık başladığından beri sayıldı.
diag-stream-input = Akış girişi
diag-stream-output = Akış çıkışı
diag-locked = { $count ->
    [0] kilitlenmedi
    [one] bir kez kilitlendi
   *[other] { $number } kez kilitlendi
}
diag-lost-lock = { $count ->
    [0] kilit kaybedilmedi
    [one] kilit bir kez kaybedildi
   *[other] kilit { $number } kez kaybedildi
}
diag-frames-in = { $count ->
    [one] { $number } gelen çerçeve
   *[other] { $number } gelen çerçeve
}
diag-frames-out = { $count ->
    [one] { $number } giden çerçeve
   *[other] { $number } giden çerçeve
}
diag-media-locked = { $count ->
    [0] medya saatine kilitlenmedi
    [one] medya saatine bir kez kilitlendi
   *[other] medya saatine { $number } kez kilitlendi
}
diag-lost-media-lock = { $count ->
    [0] medya kilidi kaybedilmedi
    [one] medya kilidi bir kez kaybedildi
   *[other] medya kilidi { $number } kez kaybedildi
}
diag-interrupted = { $count ->
    [0] kesilmedi
    [one] bir kez kesildi
   *[other] { $number } kez kesildi
}
diag-out-of-sequence = { $count ->
    [one] { $number } sıra dışı çerçeve
   *[other] { $number } sıra dışı çerçeve
}
diag-media-resets = { $count ->
    [one] { $number } medya sıfırlaması
   *[other] { $number } medya sıfırlaması
}
diag-timestamps-uncertain = { $count ->
    [0] belirsiz zaman damgası yok
    [one] bir kez belirsiz zaman damgası
   *[other] { $number } kez belirsiz zaman damgası
}
diag-no-timestamp = { $count ->
    [one] { $number } zaman damgasız çerçeve
   *[other] { $number } zaman damgasız çerçeve
}
diag-unsupported-format = { $count ->
    [one] { $number } desteklenmeyen biçimde çerçeve
   *[other] { $number } desteklenmeyen biçimde çerçeve
}
diag-late = { $count ->
    [one] { $number } geç kalan çerçeve
   *[other] { $number } geç kalan çerçeve
}
diag-early = { $count ->
    [one] { $number } erken gelen çerçeve
   *[other] { $number } erken gelen çerçeve
}
diag-started = { $count ->
    [0] başlamadı
    [one] bir kez başladı
   *[other] { $number } kez başladı
}
diag-stopped = { $count ->
    [0] durmadı
    [one] bir kez durdu
   *[other] { $number } kez durdu
}
diag-reservation-failed = talker'ın rezervasyonu başarısız: { $reason }
diag-latency = { $microseconds } µs birikmiş gecikme

## AVB Lite

lite-active = Etkin
lite-active-untagged = Etkin, etiketsiz
lite-active-vlan = Etkin, VLAN { $vlan }
lite-capable = Destekleniyor
lite-mode = Mod
lite-mode-capable = AVB (AVB Lite destekli)
lite-because = Neden
lite-fallback-none = neden belirtilmedi
lite-fallback-endpoint = başka bir uç noktanın bildirimi geldi, yani aralarında AVB köprüsü yok
lite-fallback-unanswered = dokuz peer delay isteği yanıtsız kaldı
lite-fallback-responders = bir peer delay isteğine iki veya daha fazla yanıt geldi, yani switch bir AVB köprüsü değil
lite-fallback-configured = operatör veya bir denetleyici ayarladı
lite-fallback-other = profilin adlandırmadığı bir neden
lite-other-profile = Başka bir profil
lite-ptp-domain = { $profile }, alan { $domain }
lite-offset = Sapma
lite-offset-from = { $offset } (grandmaster: { $grandmaster })
lite-media-vlan = Medya VLAN'ı
lite-untagged = Etiketsiz
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Akış başına en fazla { $count } listener, sonra multicast
   *[other] Akış başına en fazla { $count } listener, sonra multicast
}
lite-link = Link
lite-bandwidth = Bant genişliği
lite-egress-of = { $used } / { $link }, { $share }
lite-egress-of-assumed = { $used } / { $link }, { $share }, gigabit link varsayıldı
lite-egress-reported = Varlığın kabul ettiği akışları kendi saydığı şekliyle.
lite-egress-worked-out = Bağlı akış çıkışlarının biçimlerinden hesaplandı.
lite-alarm-offset = PTP sapması { $offset }, AVB Lite için izin verilen 50 µs sınırının üzerinde
lite-alarm-egress = Çıkış trafiği linkin { $share } kadarı, akışlara izin verilen { $limit } sınırının üzerinde

## AVB Wireless

wireless-station = İstasyon
wireless-access-point = Erişim noktası
wireless-role = Rol
wireless-mode = Mod
wireless-time = Zaman
wireless-mode-a-ftm = Mode A, FTM üzerinden 802.1AS
wireless-mode-a-tm = Mode A, TM üzerinden 802.1AS
wireless-mode-b = Mode B, beacon çerçevelerinden
wireless-no-time = Zaman yok
wireless-other-mode = Profilin adlandırmadığı bir mod
wireless-locked = Kilitli
wireless-holdover = Holdover modunda
wireless-not-locked = Kilitli değil
wireless-row-locked = İstasyon, kilitli
wireless-row-holdover = İstasyon, holdover modunda
wireless-row-not-locked = İstasyon, kilitli değil
wireless-row-access-point = { $count ->
    [one] Erişim noktası, { $count } istasyon
   *[other] Erişim noktası, { $count } istasyon
}
wireless-link = Link
wireless-channel = kanal { $channel }
wireless-not-known = Bilinmiyor
wireless-signal = Sinyal gücü
wireless-rate = İletim hızı
wireless-ftm-valid = { $share } geçerli
wireless-rtt = gidiş-dönüş süresi { $rtt }
wireless-bursts = { $count ->
    [one] { $count } çerçevelik burst'ler
   *[other] { $count } çerçevelik burst'ler
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = erişim noktası FTM burst'lerini üç veya iki dışında bir çerçeve sayısıyla veriyor
wireless-reason-measurement = erişim noktasıyla FTM de TM de yok
wireless-reason-signaling = erişim noktasından gPTP-capable Signaling mesajı yok
wireless-reason-other = profilin adlandırmadığı bir neden
wireless-servo = Servo hatası
wireless-stations = İstasyonlar
wireless-station-count = { $count ->
    [one] { $count } istasyon
   *[other] { $count } istasyon
}
wireless-no-ftm = FTM desteksiz
wireless-unserved = Hizmet verilmeyen listener'lar
wireless-frame-budget = Çerçeve bütçesi
wireless-frame-use = saniyede { $used } / { $budget } çerçeve
wireless-stream-frames = Akış çerçeveleri
wireless-frames-of = { $readdressed } istasyonlara, { $unmapped } listener'sız, { $dropped } düşürüldü, { $restored } istasyonlardan
wireless-class-a-allowed = İzin veriliyor
wireless-class-a-not-allowed = İzin verilmiyor
wireless-alarm-not-locked = Wi-Fi zamanı erişim noktasına kilitli değil
wireless-alarm-holdover = Wi-Fi zamanı holdover modunda, erişim noktasına olan kilit kaybedildi
wireless-alarm-unserved = { $count ->
    [one] Wi-Fi portunda { $count } listener hizmet almıyor, unicast sınırının üzerinde
   *[other] Wi-Fi portunda { $count } listener hizmet almıyor, unicast sınırının üzerinde
}

## Log

log-all = Tümü
log-warnings = Uyarılar
log-pause = Duraklat
log-resume = Sürdür
log-clear = Temizle
log-empty = triib'in gönderdiği ve aldığı her ATDECC çerçevesi, en yenisi en üstte olmak üzere burada görünür.
log-none-match = Saklanan hiçbir çerçeve filtreyle eşleşmiyor.
log-frames = { $count ->
    [one] { $count } çerçeve
   *[other] { $count } çerçeve
}
log-shown-of = { $shown } / { $all } çerçeve
log-sent = Gönderildi
log-heard = Alındı
log-not-decoded = Çözümlenmedi
log-warning-short = control_data_length alanına göre çerçevenin sonundan { $missing } sekizli fazla veri var.
log-warning-undecodable = Çözümlenemiyor: { $error }.
log-warning-long-acmp = Bir Milan varlığının gönderemeyeceği uzun ACMP biçiminde (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Kanal eşlemeleri
mapping-inputs = Girişler
mapping-outputs = Çıkışlar
mapping-port = port { $number }
mapping-fixed = sabit
mapping-not-read = Henüz okunmadı.
mapping-no-clusters = Küme yok.
mapping-no-streams = Ses akışı yok.
mapping-none = Eşleme yok.
mapping-not-mapped = Eşlenmedi
mapping-cluster-numbered = Küme { $index }

## Presets

presets-note = Bir ön ayar, her varlığın saat kaynaklarını, örnekleme hızlarını, akış biçimlerini, kontrollerini ve bağlantılarını saklar. Geri çağrıldığında yalnızca farklı olanlar değişir.
presets-none = Henüz kayıtlı ön ayar yok.
presets-connections = { $count ->
    [one] { $count } bağlantı
   *[other] { $count } bağlantı
}
presets-recall = Geri çağır
presets-delete = Sil
presets-no-place = Ön ayarları saklayacak bir yer yok: ana klasör bilinmiyor.
presets-undeletable = { $path } silinemedi: { $error }.
presets-saved = { $count ->
    [one] “{ $name }”, { $count } varlıkla kaydedildi.
   *[other] “{ $name }”, { $count } varlıkla kaydedildi.
}
presets-nothing-differs = “{ $name }” ön ayarından farklı bir şey yok.
presets-recalling = { $count ->
    [one] “{ $name }” geri çağrılıyor: { $count } değişiklik.
   *[other] “{ $name }” geri çağrılıyor: { $count } değişiklik.
}
presets-missing = { $report } Burada olmayan veya okunmamış: { $missing }.
presets-deleted = “{ $name }” silindi.
presets-host-note = Bu bilgisayarın kendi varlıklarını da saklar ve geri çağrıldığında yeniden başlatır.
presets-host-endpoints = bu bilgisayarda { $count }
presets-starting-host = “{ $name }” için bu bilgisayarın varlıkları başlatılıyor; geri kalanı onlar dönünce gelecek.

## Controls

control-numbered = Kontrol { $index }
control-not-shown = Burada gösterilmiyor
control-option = Seçenek { $number }

## Network errors

network-permission = triib'in ham Ethernet çerçeveleri gönderip alabilmesi için izin gerekiyor.
network-needs-npcap = triib'in ham Ethernet çerçeveleri gönderip alabilmesi için Npcap gerekiyor.
network-npcap-administrators = Npcap ham Ethernet çerçevelerini yalnızca yöneticilerin gönderip almasına izin veriyor. triib'i yönetici olarak çalıştırın veya Npcap'i yalnızca yöneticiler seçeneği olmadan yeniden yükleyin.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own entities

host-add-entity = Varlık ekle
host-show-mine = Yalnızca bu bilgisayarın kendi varlıklarını göster
host-show-all = Tüm varlıkları göster
host-new-entity = Ana bilgisayar varlığı { $number }
host-failed = Bu bilgisayara eklenemedi: { $reason }
host-needs-clock = Bu bilgisayarın kendi varlıkları PTP donanım saati olan kablolu bir arayüze ihtiyaç duyar
host-no-ptp4l = ptp4l yanıt vermiyor, bu yüzden bu bilgisayarın akışları gPTP zamanını tutamıyor
host-elsewhere = triib-endpointd { $interface } üzerinde başka bir kullanıcı için ya da root olarak çalışıyor, bu yüzden bu bilgisayarın varlıkları burada değil, orada çalışıyor
host-foreign-mrp = Başka bir program { $interface } üzerinde bu bilgisayarın adresinden MSRP veya MVRP bildirimleri yapıyor ve bu bilgisayarın akışlarının ihtiyaç duyduğu şeyleri geri çekebilir
host-alarm-foreign-mrp = Bu bilgisayardaki başka bir program { $interface } üzerinde MSRP veya MVRP bildirimleri yapıyor
host-state = Durum
host-streaming = Yayın yapıyor
host-waiting = Bir listener bekleniyor
host-listening = Dinleniyor
host-bound = Bağlı, talker bekleniyor
host-unbound = Bağlı değil
host-audio-from = Ses kaynağı
host-audio-to = Ses hedefi
host-channels = Kanallar
host-virtual-device = Sanal cihaz
host-add-stream = Akış ekle
host-add-audio-input = Ses girişi
host-add-audio-output = Ses çıkışı
host-add-clock-input = Medya saati girişi
host-add-clock-output = Medya saati çıkışı
host-remove-stream = Akışı kaldır
host-silence = Sessizlik
host-tone = Test tonu
host-nowhere = Hiçbir yere
host-default-device = Varsayılan aygıt
host-remove = Bu bilgisayardan kaldır
