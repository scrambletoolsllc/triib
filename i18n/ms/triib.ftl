## Language

language-name = Bahasa Melayu

## Common

common-close = Tutup
common-more = Lagi
common-keep-toolbar-shown = Sentiasa tunjukkan bar alat
common-auto-hide-toolbar = Sembunyikan bar alat secara automatik

## Settings

settings-title = Tetapan
settings-general = Umum
settings-appearance = Penampilan
settings-language = Bahasa
settings-language-system = Lalai sistem: { $language }
settings-language-note = Medan teks menggunakan bahasa input sistem.
settings-appearance-system = Sistem
settings-appearance-light = Cerah
settings-appearance-dark = Gelap
settings-colors = Warna
settings-system-accent = Gunakan warna aksen sistem
settings-accent-picked = Warna di bawah menjadi asas warna triib.
settings-accent-omarchy = Daripada tema Omarchy, { $theme }.
settings-accent-desktop = Daripada warna aksen desktop.
settings-accent-none = Desktop tiada warna aksen, jadi warna di bawah digunakan.
settings-motion = Gerakan
settings-animations = Animasi
settings-animations-note = Kesan pegas dan gelongsor apabila sesuatu berubah.
settings-animations-reduced = Desktop meminta gerakan dikurangkan, jadi triib kekal pegun.
settings-outside-control = Kawalan luar
settings-allow-outside-control = Benarkan kawalan luar
settings-allow-outside-control-note = Ejen seperti Claude Code boleh melihat dan mengubah rangkaian AVB anda melalui triib --mcp. triib akan bertanya sebelum setiap ejen baharu.
settings-allowed-agents = Dibenarkan: { $agents }
settings-forget-agents = Lupakan
settings-storage = Storan
settings-storage-presets = Folder praset
settings-storage-entities = Fail entiti
settings-storage-apply = Guna
settings-storage-choose = Pilih…
settings-storage-note = Praset yang sudah disimpan di tempat lama kekal di sana; alihkannya untuk terus menggunakannya. Tetapan triib disimpan dalam { $file }.
settings-storage-entities-note = Fail entiti yang belum wujud bermula sebagai salinan fail yang digunakan, dan triib-endpointd dimulakan semula dengannya.
settings-storage-entities-cut-off = Menggunakan ini memulakan semula triib-endpointd, dan plaa versi lama atau atur cara lain yang berkongsinya akan kehilangan strimnya selama kira-kira 15 saat.
dialog-choose-presets = Pilih folder praset
dialog-choose-entities = Pilih folder untuk fail entiti
settings-full-path = Gunakan laluan penuh, contohnya ~/Documents/triib.
settings-path-is-folder = { $path } ialah folder, bukan fail.
settings-folder-missing = Folder { $path } tidak wujud. Ciptanya dahulu, atau pilih folder lain.
settings-path-is-file = { $path } ialah fail, bukan folder.
settings-cannot-write = triib tidak dapat menulis dalam { $path }: { $error }.
app-file-dialog-failed = Tidak dapat menunjukkan dialog fail: { $error }
agent-prompt-title = Benarkan { $agent } mengawal triib?
agent-prompt-body = { $agent } meminta untuk menggunakan kawalan luar triib: melihat entiti pada rangkaian anda dan mengubahnya, menyambung strim serta menetapkan format, jam dan nama. Anda boleh mematikan kawalan luar dalam Tetapan.
agent-prompt-allow = Benarkan
agent-prompt-deny = Jangan benarkan

common-cancel = Batal
common-save = Simpan
common-not-set = Tidak ditetapkan
common-unnamed = Tanpa nama
common-none = Tiada
common-mac-address = Alamat MAC
common-list-separator = {", "}

## Network interfaces

interface-up = aktif
interface-link-down = pautan terputus
interface-wireless = wayarles
interface-hardware-clock = jam perkakasan
interface-hardware-clock-named = jam perkakasan { $clock }
interface-virtual = maya

## Toolbar

toolbar-choose-interface = Pilih antara muka
toolbar-interface = Antara muka rangkaian
toolbar-filter-interfaces = Tapis antara muka
toolbar-hide-virtual = Sembunyikan antara muka maya
toolbar-hide-wireless = Sembunyikan antara muka wayarles
toolbar-hide-unclocked = Sembunyikan antara muka tanpa jam perkakasan
toolbar-connections = Sambungan
toolbar-network = Rangkaian
toolbar-entities = Entiti
toolbar-rediscover = Minta setiap entiti mengumumkan dirinya
toolbar-rescan = Kosongkan dan imbas semula semua entiti
toolbar-search = Cari entiti dan strim
toolbar-presets = Praset
toolbar-log = Log
toolbar-inspector = Pemeriksa
toolbar-settings = Tetapan
toolbar-close-window = Tutup tetingkap

## The macOS menu bar, named as in macOS's own apps

menu-about = Perihal triib
menu-settings = Tetapan…
menu-services = Perkhidmatan
menu-hide = Sembunyikan triib
menu-hide-others = Sembunyikan Yang Lain
menu-show-all = Tunjukkan Semua
menu-quit = Keluar triib
menu-file = Fail
menu-presets = Praset…
menu-close-window = Tutup Tetingkap
menu-edit = Edit
menu-cut = Potong
menu-copy = Salin
menu-paste = Tampal
menu-select-all = Pilih Semua
menu-view = Paparan
menu-full-screen = Masuk Skrin Penuh
menu-window = Tetingkap
menu-minimize = Minimumkan
menu-zoom = Zum
menu-bring-all-to-front = Bawa Semua ke Hadapan
menu-help = Bantuan
menu-website = Laman Web triib
menu-release-notes = Nota Keluaran

## The network's state, in place of a view

state-no-interface = Tiada antara muka
state-no-interface-note = Pilih antara muka pada rangkaian AVB untuk mengesan entiti.
state-starting = Bermula
state-starting-note = Membuka { $interface }.
state-listening = Mendengar
state-listening-note = Entiti pada { $interface } muncul di sini apabila mengumumkan diri.
state-permission-needed = Kebenaran diperlukan
state-npcap-needed = Npcap diperlukan
state-get-npcap = Dapatkan Npcap
state-copy-command = Salin arahan
state-permission-watch = triib akan bersambung dengan sendirinya sebaik sahaja arahan dijalankan.
state-cannot-use = Tidak dapat menggunakan { $interface }
state-try-again = Cuba lagi

## Entity list

entities-none-yet = Belum ada entiti
entities-none-yet-note = Setiap entiti pada rangkaian, dengan peranan, kelas SR dan jamnya.
entities-filter = Tapis entiti
entities-hide-others = Sembunyikan entiti pada peranti lain
entities-hide-controllers = Sembunyikan pengawal
entities-hide-other-domains = Sembunyikan entiti di luar domain jam komputer ini
entities-hidden = { $count } entiti disembunyikan

## Inspector

inspector-title = Pemeriksa
inspector-entity = Entiti
inspector-streams = Strim
inspector-controls = Kawalan
inspector-diagnostics = Diagnostik
inspector-descriptors = Deskriptor
inspector-select = Pilih entiti untuk melihat butirannya.
inspector-offline = { $entity } di luar talian.
inspector-heard = Ia tidak mengumumkan dirinya: triib mendengarnya dalam arahan yang dihantarnya, dan menyenaraikannya sehingga 5 minit berlalu tanpa arahan.
inspector-rename = Namakan semula
inspector-name = Nama
inspector-identify = Kenal pasti
inspector-model-not-read = Model entitinya belum dibaca.
inspector-no-streams = Tiada strim.
inspector-no-controls = Tiada kawalan untuk ditunjukkan.
inspector-no-diagnostics = Tiada antara muka atau pembilang dilaporkan.
inspector-reading = Membaca deskriptor, { $count } setakat ini.
inspector-read-failed = Gagal membaca model entiti: { $reason }.

entity-section = Entiti
entity-name = Nama
entity-triib-controller = Pengawal triib
entity-triib-on = triib pada { $host }
entity-triib-here = triib pada komputer ini
entity-controller-on = Pengawal pada { $host }
entity-controller-here = Pengawal pada komputer ini
entity-controller-at = Pengawal di { $mac }
entity-group = Kumpulan
entity-product = Produk
entity-firmware = Perisian tegar
entity-serial-number = Nombor siri
entity-configuration = Konfigurasi
entity-configuration-of = { $name } ({ $number } daripada { $count })
entity-configuration-holds = { $inputs } masuk, { $outputs } keluar, strim { $channels } saluran
entity-configuration-holds-mixed = { $inputs } masuk, { $outputs } keluar
entity-configuration-numbered = Konfigurasi { $number }
entity-milan = Milan
entity-media-clock = Jam media
entity-clock-domain = Domain jam
entity-sampling-rate = Kadar pensampelan
clock-source-numbered = Sumber { $index }
rate-pull = pull { $pull }

stream-format = Format
stream-id = ID strim
stream-destination = Destinasi
stream-max-transit = Masa transit maks.
stream-listeners = Pendengar
stream-listener-gone = { $entity } input { $input }, tidak dalam talian
stream-disconnect-listener = Putuskan

avb-interfaces = Antara muka AVB
avb-interface = Antara muka
avb-interface-clock-identity = Identiti jam
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domain { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Berjalan
avb-interface-none-reported = Tiada dilaporkan
avb-interface-path = Laluan
avb-interface-own-grandmaster = Grandmaster bagi dirinya sendiri
avb-interface-hops = { $count } hop dari grandmaster
avb-interface-link-up = Pautan aktif
avb-interface-link-down = Pautan terputus
avb-interface-grandmaster-changes = Pertukaran grandmaster
avb-interface-frames-sent = Bingkai dihantar
avb-interface-frames-received = Bingkai diterima
avb-interface-crc-errors = Ralat CRC

tree-firmware = Perisian tegar { $version }
tree-descriptor-types = { $count } jenis deskriptor
tree-clock = Jam
tree-clock-source-from = { $kind }, daripada { $location } { $index }
tree-clock-domain-using = Menggunakan { $source }
tree-clusters = { $count } kluster
tree-maps = { $count } peta

advert-not-advertised = Tidak diumumkan
advert-identity = Identiti
advert-entity-id = ID entiti
advert-entity-model = Model entiti
advert-roles = Peranan
advert-talker = Talker
advert-listener = Listener
advert-clock = Jam
advert-gptp-domain = Domain gPTP
advert-sr-classes = Kelas SR
advert-indexes = Indeks model entiti
advert-identify-control = Kawalan kenal pasti
advert-avb-interface = Antara muka AVB
advert-advertising = Pengumuman
advert-valid-time = Tempoh sah
advert-available-index = Indeks ketersediaan
advert-association = Perkaitan
advert-capabilities = Keupayaan

## Status bar

status-entities = { $count } entiti
status-not-discovering = Tidak mengesan
status-discovering = Mengesan
status-discovering-as = Mengesan sebagai { $controller }
status-stopped = Dihentikan oleh ralat
status-alarm = Penggera
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } dan { $count } lagi

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = pengawal
role-none = tiada peranan
classes-a-and-b = A dan B
clock-no-gptp = Tiada gPTP

read-not-read = Belum dibaca
read-reading = Membaca, { $count } setakat ini
read-ready-unreadable = Sedia, { $count } tidak dapat dibaca
read-ready-cached = Sedia, daripada cache
read-ready = Sedia
read-failed = Gagal: { $reason }

milan-no = Tidak
milan-before-1-3 = sebelum 1.3
milan-certified = { $version }, diperakui { $certification }
milan-not-certified = { $version }, tidak diperakui

outcome-status = status { $status }
outcome-no-response = tiada tindak balas
outcome-not-possible = tidak mungkin
outcome-connect = Gagal menyambungkan { $talker } ke { $listener }: { $reason }.
outcome-disconnect = Gagal memutuskan sambungan { $listener }: { $reason }.
outcome-disconnect-listener = Gagal memutuskan { $listener } daripada { $talker }: { $reason }.
outcome-identify = Gagal mengenal pasti { $entity }: { $reason }.
outcome-rename = Gagal menamakan semula { $what } kepada "{ $name }": { $reason }.
outcome-rename-group = Gagal menamakan semula kumpulan { $entity } kepada "{ $name }": { $reason }.
outcome-format-streaming = Gagal menukar format { $stream }: strim sedang berjalan. Putuskan sambungannya dahulu.
outcome-format = Gagal menukar format { $stream }: { $reason }.
outcome-max-transit-time = Gagal menukar masa transit maks. { $stream }: { $reason }.
outcome-max-transit-time-streaming = Gagal menukar masa transit maks. { $stream }: strim sedang berjalan. Putuskan sambungannya dahulu.
outcome-sampling-rate = Gagal menukar kadar pensampelan { $entity }: { $reason }.
outcome-clock-source = Gagal menukar sumber jam { $entity }: { $reason }.
outcome-configuration = Gagal menukar konfigurasi { $entity }: { $reason }.
outcome-configuration-streaming = { $entity } mengekalkan konfigurasinya selagi strimnya berjalan: putuskan sambungannya dahulu.
outcome-map = Gagal memetakan saluran pada { $entity }: { $reason }.
outcome-unmap = Gagal menyahpetakan saluran pada { $entity }: { $reason }.
outcome-control = Gagal menetapkan "{ $control }" pada { $entity }: { $reason }.
outcome-control-numbered = Gagal menetapkan kawalan { $index } pada { $entity }: { $reason }.

stream-not-connected = Tidak disambungkan
stream-from = Daripada { $stream }
stream-from-receiving = Daripada { $stream }, menerima
stream-from-waiting = Daripada { $stream }, menunggu talker
stream-from-failed = Daripada { $stream }, tempahan talker gagal: { $reason }
stream-sending-to = Menghantar ke { $destination }

failure-no-response = tiada tindak balas
failure-refused = ditolak dengan { $status }
failure-malformed = tindak balasnya tidak dapat dinyahkod
failure-on-this-computer = berjalan pada komputer ini; baca dari komputer lain

msrp-failure-1 = lebar jalur tidak mencukupi
msrp-failure-2 = sumber suis tidak mencukupi
msrp-failure-3 = lebar jalur tidak mencukupi untuk kelas trafik ini
msrp-failure-4 = ID strim digunakan oleh talker lain
msrp-failure-5 = alamat destinasi sudah digunakan
msrp-failure-6 = didahului oleh strim yang lebih tinggi kedudukannya
msrp-failure-7 = kependaman yang dilaporkan telah berubah
msrp-failure-8 = port egress tidak menyokong AVB
msrp-failure-9 = gunakan alamat destinasi lain
msrp-failure-10 = kehabisan sumber MSRP
msrp-failure-11 = kehabisan sumber MMRP
msrp-failure-12 = tidak dapat menyimpan alamat destinasi
msrp-failure-13 = keutamaan bukan keutamaan kelas SR
msrp-failure-14 = bingkai terlalu besar untuk medium
msrp-failure-15 = had port fan-in dicapai
msrp-failure-16 = nilai pertama berubah bagi strim berdaftar
msrp-failure-17 = VLAN disekat pada port egress
msrp-failure-18 = pengetegan VLAN dinyahdayakan pada port egress
msrp-failure-19 = keutamaan kelas SR tidak sepadan
msrp-failure-unknown = sebab tidak diketahui
msrp-failure-at = { $reason }, pada suis { $bridge }

## Entity list columns

column-vendor = Pengeluar
column-entity-image = Imej entiti
column-vendor-image = Imej pengeluar
column-model = Model
column-state = Keadaan
column-entity-model-id = ID model entiti
column-talker-streams = Strim talker
column-listener-streams = Strim listener
column-avb-lite = AVB Lite
column-egress = Egress
column-wireless = Wayarles

## Settings file

settings-no-place = Tiada tempat untuk menyimpan tetapan: folder rumah tidak diketahui.
settings-unusable = Tidak dapat menggunakan { $path }: { $error }.
settings-unsaved = Tidak dapat menyimpan { $path }: { $error }.

column-remove = Buang lajur
column-move-left = Alih ke kiri
column-move-right = Alih ke kanan
column-add = Tambah lajur
common-percent = { $value }%

## Network view

netmap-empty = Belum ada rangkaian untuk ditunjukkan
netmap-empty-note = Entiti muncul di sini setelah dibaca dan melaporkan domain jamnya.
netmap-focus-clock-path = laluan jam { $name }
netmap-focus-streams = strim { $name }
netmap-showing = Menunjukkan { $what }
netmap-devices = { $count } peranti
netmap-bridges = { $count } suis
netmap-show-map = Tunjukkan peta
netmap-show-details = Tunjukkan butiran
stream-numbered = Strim { $index }
netmap-bridge = Suis
netmap-access-point = Titik capaian
netmap-plain-switch = Suis bukan AVB
netmap-device = Peranti
netmap-this-computer = Komputer ini
netmap-connected = Disambungkan
netmap-advertised = Diumumkan, tiada listener sedia
netmap-advertised-unknown-domain = Diumumkan, tiada listener sedia (domain jam { $listener } tidak diketahui)
netmap-failed-at = Tempahan gagal di { $bridge }: { $reason }
netmap-failed = Tempahan gagal: { $reason }
netmap-no-bridge-on = Tiada suis dikesan pada { $interface }
netmap-cannot-listen-on = Tidak dapat mendengar gPTP pada { $interface }
netmap-behind-plain-switch = Di belakang suis bukan AVB
netmap-on-this-computer = Pada komputer ini
netmap-path-not-reported = Laluan tidak dilaporkan
netmap-gptp-not-reported = gPTP tidak dilaporkan
netmap-domain = Domain gPTP { $domain }
netmap-ptp-domain = Domain PTP { $domain }
netmap-domain-unknown = Domain tidak diketahui
netmap-synced = Disegerakkan
netmap-sync = Penyegerakan
netmap-not-synced = Tidak disegerakkan
netmap-triib-on = triib pada { $interface }
netmap-through-count = { $count } melalui
netmap-out = { $count } keluar
netmap-in = { $count } masuk
netmap-failed-count = { $count } gagal
netmap-advertised-only = Diumumkan sahaja
netmap-failed-state = Gagal
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-sending-unheard = Menghantar, tiada listener yang diketahui terikat padanya
netmap-no-listener = Tiada listener yang diketahui
netmap-apart-no-path = Laluannya tidak dilaporkan; ia mengikut grandmaster { $grandmaster }
netmap-apart-unreported = Ia belum melaporkan keadaan gPTP-nya
netmap-apart-no-neighbor = Tiada suis dikesan pada antara muka komputer ini
netmap-apart-cannot-listen = Komputer ini tidak dapat mendengar gPTP pada antara mukanya
netmap-apart-plain-switch = Komputer ini berada di belakang suis yang tidak mengambil bahagian dalam AVB, dan tiada peranti yang berkongsi suis itu menjalankan AVB Lite di sana, jadi komputer ini tiada dalam sebarang domain
netmap-apart-on-this-computer = Ia berjalan pada komputer ini; baca dari komputer lain untuk melihat keadaan gPTP
netmap-switch-state = Tidak mengambil bahagian dalam AVB: ia mewakili suis di hadapan peranti di bawah, yang tidak menemui suis AVB lalu beralih kepada AVB Lite, dan mengikut grandmaster PTP yang sama
netmap-clock-tree = Pepohon jam
netmap-no-grandmaster = Tiada grandmaster dikesan
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Perlu perhatian
netmap-nodes-below = Nod di bawah
netmap-bridges-below = Suis di bawah
netmap-clock-path = Laluan jam
netmap-hops = Hop dari grandmaster
netmap-link-delay = Lengahan pautan
netmap-bridge-port = Port suis
netmap-link-drops = Pautan putus
netmap-synced-to-grandmaster = Disegerakkan dengan grandmaster
netmap-host-no-gptp = Tidak disegerakkan: komputer ini tidak menjalankan gPTP
netmap-host-no-ptp = Tidak disegerakkan: komputer ini tidak mengikuti grandmaster domain ini
netmap-link-no-gptp = Tidak disegerakkan: gPTP tidak berjalan pada pautannya
netmap-ptp-offset-high = Tidak disegerakkan: { $offset } dari grandmaster, melebihi 50 µs yang dibenarkan AVB Lite
netmap-ptp-no-offset = Tidak disegerakkan: ia belum mengukur sebarang ofset dari grandmaster
netmap-audio = Audio
netmap-media-clock-streams = Strim jam media
netmap-audio-streams = Strim audio
netmap-bound = { $count } terikat
netmap-flowing = Mengalir
netmap-advertised-state = Diumumkan
netmap-media-clock-stream = Strim jam media
netmap-audio-stream = Strim audio
netmap-reaches = Sampai ke
netmap-passing-count = { $count } strim melalui
netmap-through = Melalui
netmap-passing-through = Sedang melalui
netmap-sending = Menghantar
netmap-receiving = Menerima
netmap-problems = Masalah
netmap-help-back = Klik latar belakang untuk kembali ke gambaran keseluruhan.
netmap-help-stream = Klik strim untuk memeriksanya, atau latar belakang untuk kembali ke gambaran keseluruhan.
netmap-help-clock = Jam mengalir dari grandmaster melalui setiap suis ke setiap nod dalam pepohon. Garis kelabu putus-putus ialah pautan yang tidak menjalankan gPTP. Klik peranti atau wayarnya untuk memeriksa laluan jamnya; klik latar belakang untuk mengosongkan pilihan.
netmap-help-domains = Setiap kawasan ialah satu domain jam. Dalam domain gPTP, jam turun dari grandmaster melalui suis AVB. Dalam domain PTP, peranti menjalankan AVB Lite dan mengikuti grandmaster-nya dari hujung ke hujung, segerak dalam 50 µs; kad bergaris putus-putus mewakili suis yang tidak mengambil bahagian dalam AVB, yang di belakangnya sebahagian peranti beralih kepada AVB Lite. Peranti yang tidak berkongsi domainnya dengan peranti lain mempunyai kawasan tersendiri.
netmap-help-media-clock = Strim jam media (CRF) sahaja, dilukis sama seperti audio: satu wayar bagi bingkai setiap strim pada setiap pautan, diwarnakan mengikut talker. Klik wayar untuk memeriksa strimnya, atau peranti untuk melihat strimnya; klik latar belakang untuk mengosongkan pilihan.
netmap-help-audio = Setiap wayar ialah bingkai satu strim pada satu pautan, yang masuk dan keluar dari setiap suis yang dilaluinya. Strim multicast mempunyai satu wayar selagi laluan ke listenernya bergerak bersama, dan bercabang di tempat laluan itu berpisah; strim unicast, seperti yang boleh dihantar oleh AVB Lite, mempunyai wayarnya sendiri dari talker ke setiap listener. Warna mengikut talker: setiap talker mempunyai satu rona, dan setiap strimnya ialah ton rona itu pada setiap cabang. Titik bergerak bermaksud audio sedang mengalir, walaupun daripada talker yang menghantar tanpa listener yang diketahui; garis merah pegun ialah tempahan yang gagal dan garis kelabu pegun ialah strim yang diumumkan tanpa listener sedia; kedua-duanya berhenti di tempat tempahan berhenti. Peranti di lajur tengah bersambung terus ke suis grandmaster. Klik wayar untuk memeriksa strimnya, wayar kongsi untuk melihat semua listenernya, atau peranti untuk melihat strimnya; klik latar belakang untuk mengosongkan pilihan.

## Connections

matrix-nothing-shown = Tiada strim untuk ditunjukkan
matrix-nothing-shown-note = Tukar carian atau penapis untuk melihat lebih banyak strim.
matrix-empty = Tiada strim untuk disambungkan
matrix-empty-note = Strim talker dan strim listener bertemu di sini setelah entiti yang memilikinya dibaca.
matrix-all-streams = Semua strim
matrix-filter = Tapis strim
matrix-hide-unconnectable = Sembunyikan strim yang tidak boleh bersambung
matrix-hide-lite = Sembunyikan entiti yang menjalankan AVB Lite
matrix-hide-wireless = Sembunyikan entiti wayarles
matrix-hidden = { $count } strim disembunyikan
matrix-own = Output sesuatu entiti tidak bersambung ke inputnya sendiri.
matrix-working = Sedang diproses.
matrix-waiting-change = Menunggu perubahan terakhir pada input ini.
matrix-connected = Disambungkan dan menerima. Klik untuk memutuskan sambungan.
matrix-bound-waiting = Terikat, menunggu strim talker. Klik untuk memutuskan sambungan.
matrix-bound-failed = Terikat, tetapi tempahan talker gagal: { $reason }. Klik untuk memutuskan sambungan.
matrix-bound-formats-differ = Terikat, tetapi formatnya berbeza: talker menghantar { $sent }, input ditetapkan kepada { $set }. Klik untuk memutuskan sambungan.
matrix-formats-match = Format sepadan ({ $format }). Klik untuk menyambung.
matrix-format-must-change = Input menerima { $sent } tetapi ditetapkan kepada { $set }, jadi ia mungkin tidak berbunyi sehingga formatnya ditukar. Klik untuk memilih format.
matrix-format-input = Tukar input kepada
matrix-format-both = Tukar kedua-duanya kepada
matrix-format-talker-busy = Talker sedang menstrim kepada listener lain, jadi formatnya kekal.
matrix-format-anyway = Sambung tanpa menukar format
matrix-incompatible = Input tidak menerima { $sent }. Ia ditetapkan kepada { $set }.
matrix-other-domain = Kedua-duanya menjaga masa dalam domain jam yang berbeza, jadi tiada strim boleh melalui antara keduanya.
matrix-group-none = Tidak disambungkan. Kembangkan untuk menyambung strim satu demi satu.
matrix-group-connected = { $count } disambungkan. Kembangkan untuk melihat setiap satu.
matrix-outputs-expand = { $count } output strim. Klik anak panah untuk mengembangkan, nama untuk memeriksanya.
matrix-outputs-collapse = { $count } output strim. Klik anak panah untuk menguncupkan, nama untuk memeriksanya.
matrix-inputs-expand = { $count } input strim. Klik anak panah untuk mengembangkan, nama untuk memeriksanya.
matrix-inputs-collapse = { $count } input strim. Klik anak panah untuk menguncupkan, nama untuk memeriksanya.
matrix-stream-inspect = { $detail } Klik untuk memeriksa { $entity }.
matrix-point = Halakan ke sel
matrix-point-note = untuk melihat talker dan listener-nya serta sama ada format mereka sepadan.
matrix-legend-waiting = Terikat, menunggu strim
matrix-legend-trouble = Terikat, ada masalah
matrix-legend-open = Boleh bersambung
matrix-legend-change = Format input perlu ditukar dahulu
matrix-legend-incompatible = Format tidak boleh sepadan
matrix-legend-other-domain = Domain jam lain
matrix-talker-outputs = Output talker
matrix-listener-inputs = Input listener

common-thousands-separator = {","}
common-decimal-separator = {"."}

## Diagnostics

diag-since-start = Dikira sejak entiti dimulakan.
diag-stream-input = Input strim
diag-stream-output = Output strim
diag-locked = { $count ->
    [0] tidak terkunci
    [1] terkunci sekali
   *[other] terkunci { $number } kali
}
diag-lost-lock = { $count ->
    [0] tidak terlepas kunci
    [1] terlepas kunci sekali
   *[other] terlepas kunci { $number } kali
}
diag-frames-in = { $number } bingkai masuk
diag-frames-out = { $number } bingkai keluar
diag-media-locked = { $count ->
    [0] media tidak terkunci
    [1] media terkunci sekali
   *[other] media terkunci { $number } kali
}
diag-lost-media-lock = { $count ->
    [0] tidak terlepas kunci media
    [1] terlepas kunci media sekali
   *[other] terlepas kunci media { $number } kali
}
diag-interrupted = { $count ->
    [0] tidak terganggu
    [1] terganggu sekali
   *[other] terganggu { $number } kali
}
diag-out-of-sequence = { $number } bingkai tidak mengikut urutan
diag-media-resets = { $number } tetapan semula media
diag-timestamps-uncertain = { $count ->
    [0] cap masa sentiasa pasti
    [1] cap masa tidak pasti sekali
   *[other] cap masa tidak pasti { $number } kali
}
diag-no-timestamp = { $number } bingkai tanpa cap masa
diag-unsupported-format = { $number } bingkai dalam format yang tidak disokong
diag-late = { $number } bingkai lewat
diag-early = { $number } bingkai terlalu awal
diag-started = { $count ->
    [0] tidak dimulakan
    [1] dimulakan sekali
   *[other] dimulakan { $number } kali
}
diag-stopped = { $count ->
    [0] tidak berhenti
    [1] berhenti sekali
   *[other] berhenti { $number } kali
}
diag-reservation-failed = tempahan talker gagal: { $reason }
diag-latency = kependaman terkumpul { $microseconds } µs

## AVB Lite

lite-active = Aktif
lite-active-untagged = Aktif, tanpa teg
lite-active-vlan = Aktif, VLAN { $vlan }
lite-capable = Menyokong
lite-mode = Mod
lite-mode-capable = AVB (menyokong AVB Lite)
lite-because = Sebab
lite-fallback-none = tiada sebab diberikan
lite-fallback-endpoint = pengisytiharan endpoint lain sampai, jadi tiada suis AVB di antara keduanya
lite-fallback-unanswered = sembilan permintaan peer delay tidak dijawab
lite-fallback-responders = dua atau lebih menjawab satu permintaan peer delay, jadi suis itu bukan suis AVB
lite-fallback-configured = ditetapkan oleh pengendali atau pengawal
lite-fallback-other = sebab yang tidak dinamakan oleh profil
lite-other-profile = Profil lain
lite-ptp-domain = { $profile }, domain { $domain }
lite-offset = Ofset
lite-offset-from = { $offset } dari { $grandmaster }
lite-media-vlan = VLAN media
lite-untagged = Tanpa teg
lite-unicast = Unicast
lite-fanout = Sehingga { $count } listener bagi setiap strim, kemudian multicast
lite-link = Pautan
lite-bandwidth = Lebar jalur
lite-egress-of = { $used } daripada { $link }, { $share }
lite-egress-of-assumed = { $used } daripada { $link }, { $share }, pautan gigabit diandaikan
lite-egress-reported = Mengikut kiraan entiti bagi strim yang diterimanya.
lite-egress-worked-out = Dikira daripada format output strim yang disambungkan.
lite-alarm-offset = Ofset PTP { $offset }, melebihi 50 µs yang dibenarkan AVB Lite
lite-alarm-egress = Egress pada { $share } daripada pautan, melebihi had { $limit } untuk strim

## AVB Wireless

wireless-station = Stesen
wireless-access-point = Titik capaian
wireless-role = Peranan
wireless-mode = Mod
wireless-time = Masa
wireless-mode-a-ftm = Mode A, 802.1AS melalui FTM
wireless-mode-a-tm = Mode A, 802.1AS melalui TM
wireless-mode-b = Mode B, daripada beacon
wireless-no-time = Tanpa masa
wireless-other-mode = Mod yang tidak dinamakan oleh profil
wireless-locked = Terkunci
wireless-holdover = Holdover
wireless-not-locked = Tidak terkunci
wireless-row-locked = Stesen, terkunci
wireless-row-holdover = Stesen, holdover
wireless-row-not-locked = Stesen, tidak terkunci
wireless-row-access-point = Titik capaian, { $count } stesen
wireless-link = Pautan
wireless-channel = saluran { $channel }
wireless-not-known = Tidak diketahui
wireless-signal = Isyarat
wireless-rate = Kadar
wireless-ftm-valid = { $share } sah
wireless-rtt = pergi balik { $rtt }
wireless-bursts = burst { $count } bingkai
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = titik capaian memberikan burst FTM dengan bilangan bingkai selain tiga atau dua
wireless-reason-measurement = tiada FTM mahupun TM dengan titik capaian
wireless-reason-signaling = tiada Signaling gPTP-capable daripada titik capaian
wireless-reason-other = sebab yang tidak dinamakan oleh profil
wireless-servo = Ralat servo
wireless-stations = Stesen
wireless-station-count = { $count } stesen
wireless-no-ftm = tanpa FTM
wireless-unserved = Listener tidak dilayan
wireless-frame-budget = Bajet bingkai
wireless-frame-use = { $used } daripada { $budget } bingkai sesaat
wireless-stream-frames = Bingkai strim
wireless-frames-of = { $readdressed } ke stesen, { $unmapped } tanpa listener, { $dropped } digugurkan, { $restored } daripada stesen
wireless-class-a-allowed = Dibenarkan
wireless-class-a-not-allowed = Tidak dibenarkan
wireless-alarm-not-locked = Masa Wi-Fi tidak terkunci pada titik capaian
wireless-alarm-holdover = Masa Wi-Fi dalam holdover, terlepas kunci daripada titik capaian
wireless-alarm-unserved = { $count } listener pada port Wi-Fi tidak dilayan, melebihi had unicast

## Log

log-all = Semua
log-warnings = Amaran
log-pause = Jeda
log-resume = Sambung semula
log-clear = Kosongkan
log-empty = Setiap bingkai ATDECC yang dihantar dan didengar triib muncul di sini, yang terbaharu dahulu.
log-none-match = Tiada bingkai simpanan yang sepadan dengan penapis.
log-frames = { $count } bingkai
log-shown-of = { $shown } daripada { $all } bingkai
log-sent = Dihantar
log-heard = Didengar
log-not-decoded = Tidak dinyahkod
log-warning-short = control_data_length-nya mendakwa { $missing } oktet melepasi hujung bingkai.
log-warning-undecodable = Ia tidak dapat dinyahkod: { $error }.
log-warning-long-acmp = Ia dalam bentuk ACMP panjang, yang tidak boleh dihantar oleh entiti Milan (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Pemetaan saluran
mapping-inputs = Input
mapping-outputs = Output
mapping-port = port { $number }
mapping-fixed = tetap
mapping-not-read = Belum dibaca.
mapping-no-clusters = Tiada kluster.
mapping-no-streams = Tiada strim audio.
mapping-none = Tiada pemetaan.
mapping-not-mapped = Tidak dipetakan
mapping-cluster-numbered = Kluster { $index }

## Presets

presets-note = Praset menyimpan sumber jam, kadar pensampelan, format strim, kawalan dan sambungan setiap entiti. Memanggilnya semula menukar apa yang berbeza.
presets-none = Belum ada praset disimpan.
presets-connections = { $count } sambungan
presets-recall = Panggil semula
presets-delete = Padam
presets-no-place = Tiada tempat untuk menyimpan praset: folder rumah tidak diketahui.
presets-undeletable = Tidak dapat memadam { $path }: { $error }.
presets-saved = "{ $name }" disimpan dengan { $count } entiti.
presets-nothing-differs = Tiada yang berbeza daripada "{ $name }".
presets-recalling = Memanggil semula "{ $name }": { $count } perubahan.
presets-missing = { $report } Tiada di sini atau belum dibaca: { $missing }.
presets-deleted = "{ $name }" dipadam.
presets-host-note = Ia juga menyimpan entiti milik komputer ini, dan memulakannya semula apabila dipanggil semula.
presets-host-endpoints = { $count } pada komputer ini
presets-starting-host = Memulakan entiti komputer ini untuk "{ $name }"; selebihnya menyusul sebaik sahaja ia kembali.

## Controls

control-numbered = Kawalan { $index }
control-not-shown = Tidak ditunjukkan di sini
control-option = Pilihan { $number }

## Network errors

network-permission = triib memerlukan kebenaran untuk menghantar dan menerima bingkai Ethernet mentah.
network-needs-npcap = triib memerlukan Npcap untuk menghantar dan menerima bingkai Ethernet mentah.
network-npcap-administrators = Npcap hanya membenarkan pentadbir menghantar dan menerima bingkai Ethernet mentah. Jalankan triib sebagai pentadbir, atau pasang semula Npcap tanpa pilihan pentadbir sahaja.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own entities

host-add-entity = Tambah entiti
host-new-entity = Entiti hos { $number }
host-failed = Tidak dapat menambahkannya pada komputer ini: { $reason }
host-needs-clock = Entiti milik komputer ini memerlukan antara muka berwayar dengan jam perkakasan PTP
host-no-ptp4l = ptp4l tidak menjawab, jadi strim komputer ini tidak dapat mengekalkan masa gPTP
host-starting = Bermula
host-starting-entity = Memulakan { $name }
host-setting-up = Menyediakan { $interface } untuk AVB
host-no-link = Tiada pautan pada { $interface }
host-starting-ptp4l = Memulakan ptp4l
host-no-linuxptp = linuxptp tidak dipasang, jadi strim komputer ini tidak dapat mengekalkan masa gPTP
host-waiting-network = Menunggu rangkaian
host-elsewhere = triib-endpointd berjalan pada { $interface } untuk pengguna lain atau sebagai root, jadi entiti milik komputer ini berjalan di sana, bukan di sini
host-foreign-mrp = Atur cara lain membuat pengisytiharan MSRP atau MVRP pada { $interface } daripada alamat komputer ini, yang boleh menarik balik apa yang diperlukan oleh strim komputer ini
host-alarm-foreign-mrp = Atur cara lain pada komputer ini membuat pengisytiharan MSRP atau MVRP pada { $interface }
host-sharers-cut-off = Menambah atau menukar entiti milik komputer ini di sini memindahkannya dari { $interface }, tempat plaa versi lama atau atur cara lain yang berkongsi triib-endpointd akan kehilangan strimnya selama kira-kira 15 saat
host-state = Keadaan
host-streaming = Menstrim
host-waiting = Menunggu listener
host-listening = Mendengar
host-bound = Terikat, menunggu talker
host-unbound = Tidak terikat
host-audio-from = Audio daripada
host-audio-to = Audio ke
host-channels = Saluran
host-virtual-device = Peranti maya
host-add-stream = Tambah strim
host-add-audio-input = Input audio
host-add-audio-output = Output audio
host-add-clock-input = Input jam media
host-add-clock-output = Output jam media
host-remove-stream = Alih keluar strim
host-silence = Senyap
host-tone = Nada ujian
host-nowhere = Tiada ke mana-mana
host-default-device = Peranti lalai
host-remove = Alih keluar daripada komputer ini

mac-audio-device = Peranti audio
mac-no-audio-device = Tiada selagi tiada strim disambungkan
mac-entity-note = macOS menjalankan entiti ini. Selagi salah satu strimnya disambungkan, macOS mencipta peranti audio tempat apl memainkan dan merakam, dengan bilangan saluran pada setiap arah seperti yang ditetapkan konfigurasi.
mac-open-audio-midi-setup = Buka Audio MIDI Setup
mac-avb-off = AVB dimatikan pada { $interface }, jadi Mac ini tiada entiti AVB sendiri. Hidupkannya dalam Network Device Browser pada Audio MIDI Setup.
mac-alarm-own-clock = Menggugurkan audio yang diterimanya selagi mengikut { $clock }
mac-follow-clock = Ikut { $source }

listen-section = Dengar
listen-start = Dengar
listen-stop = Berhenti mendengar
listen-source = Mendengar
listen-received = Yang diterimanya
listen-sent = Yang dihantarnya pada strim { $number } ({ $channels } saluran)
listen-sent-needs-macos = Mendengar apa yang dihantar oleh Mac ini memerlukan macOS 14.2 atau lebih baharu.
listen-channels = Saluran
listen-channel = Saluran { $number }
listen-channel-pair = Saluran { $first }–{ $last }
listen-output = Output
listen-default-output = Output lalai
listen-gain = Gandaan
listen-gain-db = { $db } dB
listen-starting = Bermula…
listen-waiting-input = Menunggu { $device }, yang dicipta macOS selagi salah satu strim entiti disambungkan.
listen-waiting-output = Menunggu { $device }.
listen-playing = Memainkan, puncak { $db } dB
listen-playing-silent = Memainkan, senyap
