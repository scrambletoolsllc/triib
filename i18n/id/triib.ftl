## Language

language-name = Bahasa Indonesia

## Common

common-close = Tutup
common-more = Lainnya
common-keep-toolbar-shown = Tetap tampilkan bilah alat
common-auto-hide-toolbar = Sembunyikan bilah alat otomatis

## Settings

settings-title = Pengaturan
settings-general = Umum
settings-appearance = Tampilan
settings-language = Bahasa
settings-language-system = Bawaan sistem: { $language }
settings-language-note = Kolom teks memakai bahasa input sistem.
settings-appearance-system = Sistem
settings-appearance-light = Terang
settings-appearance-dark = Gelap
settings-colors = Warna
settings-system-accent = Gunakan warna aksen sistem
settings-accent-picked = Warna di bawah menjadi dasar warna triib.
settings-accent-omarchy = Dari tema Omarchy, { $theme }.
settings-accent-desktop = Dari warna aksen desktop.
settings-accent-none = Desktop tidak punya warna aksen, jadi warna di bawah yang dipakai.
settings-motion = Gerak
settings-animations = Animasi
settings-animations-note = Efek pegas dan geser saat sesuatu berubah.
settings-animations-reduced = Desktop meminta gerakan dikurangi, jadi triib tetap diam.
settings-outside-control = Kendali luar
settings-allow-outside-control = Izinkan kendali luar
settings-allow-outside-control-note = Agen seperti Claude Code dapat melihat dan mengubah jaringan AVB Anda melalui triib --mcp. triib akan bertanya sebelum setiap agen baru.
settings-allowed-agents = Diizinkan: { $agents }
settings-forget-agents = Lupakan
settings-storage = Penyimpanan
settings-storage-presets = Folder preset
settings-storage-entities = Berkas entitas
settings-storage-apply = Terapkan
settings-storage-choose = Pilih…
settings-storage-note = Preset yang sudah disimpan di tempat lama tetap berada di sana; pindahkan agar tetap dapat digunakan. Pengaturan triib disimpan di { $file }.
settings-storage-entities-note = Berkas entitas yang belum ada dimulai sebagai salinan berkas yang dipakai, dan triib-endpointd dijalankan ulang dengannya.
dialog-choose-presets = Pilih folder preset
dialog-choose-entities = Pilih folder untuk berkas entitas
settings-full-path = Gunakan jalur lengkap, misalnya ~/Documents/triib.
settings-path-is-folder = { $path } adalah folder, bukan berkas.
settings-folder-missing = Folder { $path } tidak ada. Buat terlebih dahulu, atau pilih folder lain.
settings-path-is-file = { $path } adalah berkas, bukan folder.
settings-cannot-write = triib tidak dapat menulis di { $path }: { $error }.
app-file-dialog-failed = Tidak dapat menampilkan dialog berkas: { $error }
agent-prompt-title = Izinkan { $agent } mengendalikan triib?
agent-prompt-body = { $agent } meminta untuk memakai kendali luar triib: melihat entitas di jaringan Anda dan mengubahnya, menghubungkan stream serta mengatur format, clock, dan nama. Anda dapat menonaktifkan kendali luar di Pengaturan.
agent-prompt-allow = Izinkan
agent-prompt-deny = Jangan izinkan

common-cancel = Batal
common-save = Simpan
common-not-set = Belum diatur
common-unnamed = Tanpa nama
common-none = Tidak ada
common-mac-address = Alamat MAC
common-list-separator = {", "}

## Network interfaces

interface-up = aktif
interface-link-down = link mati
interface-wireless = nirkabel
interface-hardware-clock = clock hardware
interface-hardware-clock-named = clock hardware { $clock }
interface-virtual = virtual

## Toolbar

toolbar-choose-interface = Pilih antarmuka
toolbar-interface = Antarmuka jaringan
toolbar-filter-interfaces = Saring antarmuka
toolbar-hide-virtual = Sembunyikan antarmuka virtual
toolbar-hide-wireless = Sembunyikan antarmuka nirkabel
toolbar-hide-unclocked = Sembunyikan antarmuka tanpa clock hardware
toolbar-connections = Koneksi
toolbar-network = Jaringan
toolbar-entities = Entitas
toolbar-rediscover = Minta setiap entitas mengumumkan diri
toolbar-rescan = Bersihkan dan pindai ulang semua entitas
toolbar-search = Cari entitas dan stream
toolbar-presets = Preset
toolbar-log = Log
toolbar-inspector = Inspektor
toolbar-settings = Pengaturan

## The network's state, in place of a view

state-no-interface = Tidak ada antarmuka
state-no-interface-note = Pilih antarmuka di jaringan AVB untuk mendeteksi entitas.
state-starting = Memulai
state-starting-note = Membuka { $interface }.
state-listening = Mendengarkan
state-listening-note = Entitas di { $interface } muncul di sini saat mengumumkan diri.
state-permission-needed = Perlu izin
state-npcap-needed = Perlu Npcap
state-get-npcap = Dapatkan Npcap
state-copy-command = Salin perintah
state-cannot-use = Tidak dapat memakai { $interface }
state-try-again = Coba lagi

## Entity list

entities-none-yet = Belum ada entitas
entities-none-yet-note = Setiap entitas di jaringan, dengan peran, kelas SR, dan clock-nya.

## Inspector

inspector-title = Inspektor
inspector-entity = Entitas
inspector-streams = Stream
inspector-controls = Kontrol
inspector-diagnostics = Diagnostik
inspector-descriptors = Deskriptor
inspector-select = Pilih entitas untuk melihat detailnya.
inspector-offline = { $entity } sedang offline.
inspector-heard = Ia tidak mengumumkan diri: triib mendengarnya dari perintah yang dikirimnya, dan mencantumkannya hingga 5 menit berlalu tanpa perintah.
inspector-rename = Ganti nama
inspector-name = Nama
inspector-identify = Identifikasi
inspector-model-not-read = Model entitasnya belum dibaca.
inspector-no-streams = Tidak ada stream.
inspector-no-controls = Tidak ada kontrol untuk ditampilkan.
inspector-no-diagnostics = Tidak ada antarmuka atau penghitung yang dilaporkan.
inspector-reading = Membaca deskriptor, sudah { $count }.
inspector-read-failed = Gagal membaca model entitas: { $reason }.

entity-section = Entitas
entity-name = Nama
entity-triib-controller = Controller triib
entity-triib-on = triib di { $host }
entity-triib-here = triib di komputer ini
entity-controller-on = Controller di { $host }
entity-controller-here = Controller di komputer ini
entity-controller-at = Controller di { $mac }
entity-group = Grup
entity-product = Produk
entity-firmware = Firmware
entity-serial-number = Nomor seri
entity-configuration = Konfigurasi
entity-configuration-of = { $name } ({ $number } dari { $count })
entity-configuration-holds = { $inputs } masuk, { $outputs } keluar, stream { $channels } kanal
entity-configuration-holds-mixed = { $inputs } masuk, { $outputs } keluar
entity-configuration-numbered = Konfigurasi { $number }
entity-milan = Milan
entity-media-clock = Media clock
entity-clock-domain = Domain clock
entity-sampling-rate = Sampling rate
clock-source-numbered = Sumber { $index }
rate-pull = pull { $pull }

stream-format = Format
stream-id = ID stream
stream-destination = Tujuan
stream-max-transit = Waktu transit maks.
stream-listeners = Pendengar
stream-listener-gone = { $entity } input { $input }, tidak online
stream-disconnect-listener = Putuskan

avb-interfaces = Antarmuka AVB
avb-interface = Antarmuka
avb-interface-clock-identity = Identitas clock
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domain { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Berjalan
avb-interface-none-reported = Tidak ada yang dilaporkan
avb-interface-path = Jalur
avb-interface-own-grandmaster = Menjadi grandmaster sendiri
avb-interface-hops = { $count } hop dari grandmaster
avb-interface-link-up = Link aktif
avb-interface-link-down = Link mati
avb-interface-grandmaster-changes = Pergantian grandmaster
avb-interface-frames-sent = Frame terkirim
avb-interface-frames-received = Frame diterima
avb-interface-crc-errors = Kesalahan CRC

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count } tipe deskriptor
tree-clock = Clock
tree-clock-source-from = { $kind }, dari { $location } { $index }
tree-clock-domain-using = Memakai { $source }
tree-clusters = { $count } klaster
tree-maps = { $count } peta

advert-not-advertised = Tidak diumumkan
advert-identity = Identitas
advert-entity-id = ID entitas
advert-entity-model = Model entitas
advert-roles = Peran
advert-talker = Talker
advert-listener = Listener
advert-clock = Clock
advert-gptp-domain = Domain gPTP
advert-sr-classes = Kelas SR
advert-indexes = Indeks model entitas
advert-identify-control = Kontrol identifikasi
advert-avb-interface = Antarmuka AVB
advert-advertising = Pengumuman
advert-valid-time = Masa berlaku
advert-available-index = Indeks ketersediaan
advert-association = Asosiasi
advert-capabilities = Kapabilitas

## Status bar

status-entities = { $count } entitas
status-not-discovering = Tidak mendeteksi
status-discovering = Mendeteksi
status-discovering-as = Mendeteksi sebagai { $controller }
status-stopped = Berhenti karena kesalahan
status-alarm = Alarm
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } dan { $count } lainnya

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = controller
role-none = tanpa peran
classes-a-and-b = A dan B
clock-no-gptp = Tanpa gPTP

read-not-read = Belum dibaca
read-reading = Membaca, sudah { $count }
read-ready-unreadable = Siap, { $count } tak terbaca
read-ready-cached = Siap, dari cache
read-ready = Siap
read-failed = Gagal: { $reason }

milan-no = Tidak
milan-before-1-3 = sebelum 1.3
milan-certified = { $version }, tersertifikasi { $certification }
milan-not-certified = { $version }, tidak tersertifikasi

outcome-status = status { $status }
outcome-no-response = tidak ada respons
outcome-not-possible = tidak memungkinkan
outcome-connect = Gagal menghubungkan { $talker } ke { $listener }: { $reason }.
outcome-disconnect = Gagal memutus { $listener }: { $reason }.
outcome-disconnect-listener = Gagal memutuskan { $listener } dari { $talker }: { $reason }.
outcome-identify = Gagal mengidentifikasi { $entity }: { $reason }.
outcome-rename = Gagal mengganti nama { $what } menjadi "{ $name }": { $reason }.
outcome-rename-group = Gagal mengganti nama grup { $entity } menjadi "{ $name }": { $reason }.
outcome-format-streaming = Gagal mengubah format { $stream }: stream sedang berjalan. Putuskan dulu.
outcome-format = Gagal mengubah format { $stream }: { $reason }.
outcome-max-transit-time = Gagal mengubah waktu transit maks. { $stream }: { $reason }.
outcome-max-transit-time-streaming = Gagal mengubah waktu transit maks. { $stream }: stream sedang berjalan. Putuskan dulu.
outcome-sampling-rate = Gagal mengubah sampling rate { $entity }: { $reason }.
outcome-clock-source = Gagal mengubah sumber clock { $entity }: { $reason }.
outcome-configuration = Gagal mengubah konfigurasi { $entity }: { $reason }.
outcome-configuration-streaming = { $entity } mempertahankan konfigurasinya selama stream-nya berjalan: putuskan dulu.
outcome-map = Gagal memetakan kanal di { $entity }: { $reason }.
outcome-unmap = Gagal melepas pemetaan kanal di { $entity }: { $reason }.
outcome-control = Gagal mengatur "{ $control }" di { $entity }: { $reason }.
outcome-control-numbered = Gagal mengatur kontrol { $index } di { $entity }: { $reason }.

stream-not-connected = Tidak terhubung
stream-from = Dari { $stream }
stream-from-receiving = Dari { $stream }, menerima
stream-from-waiting = Dari { $stream }, menunggu talker
stream-from-failed = Dari { $stream }, reservasi talker gagal: { $reason }
stream-sending-to = Mengirim ke { $destination }

failure-no-response = tidak merespons
failure-refused = menolak dengan { $status }
failure-malformed = responsnya tidak dapat didekode
failure-on-this-computer = berjalan di komputer ini; baca dari komputer lain

msrp-failure-1 = bandwidth tidak cukup
msrp-failure-2 = sumber daya switch tidak cukup
msrp-failure-3 = bandwidth tidak cukup untuk kelas trafik ini
msrp-failure-4 = ID stream dipakai talker lain
msrp-failure-5 = alamat tujuan sudah dipakai
msrp-failure-6 = didahului stream berperingkat lebih tinggi
msrp-failure-7 = latensi yang dilaporkan berubah
msrp-failure-8 = port egress tidak mendukung AVB
msrp-failure-9 = gunakan alamat tujuan lain
msrp-failure-10 = sumber daya MSRP habis
msrp-failure-11 = sumber daya MMRP habis
msrp-failure-12 = tidak dapat menyimpan alamat tujuan
msrp-failure-13 = prioritas bukan prioritas kelas SR
msrp-failure-14 = frame terlalu besar untuk medium
msrp-failure-15 = batas port fan-in tercapai
msrp-failure-16 = nilai pertama berubah untuk stream terdaftar
msrp-failure-17 = VLAN diblokir di port egress
msrp-failure-18 = tagging VLAN dinonaktifkan di port egress
msrp-failure-19 = prioritas kelas SR tidak cocok
msrp-failure-unknown = alasan tidak diketahui
msrp-failure-at = { $reason }, di switch { $bridge }

## Entity list columns

column-vendor = Vendor
column-model = Model
column-state = Status
column-entity-model-id = ID model entitas
column-talker-streams = Stream talker
column-listener-streams = Stream listener
column-avb-lite = AVB Lite
column-egress = Egress
column-wireless = Wireless

## Settings file

settings-no-place = Pengaturan tidak dapat disimpan: folder home tidak diketahui.
settings-unusable = Tidak dapat memakai { $path }: { $error }.
settings-unsaved = Tidak dapat menyimpan { $path }: { $error }.

column-remove = Hapus kolom
column-move-left = Pindah ke kiri
column-move-right = Pindah ke kanan
column-add = Tambah kolom
common-percent = { $value }%

## Network view

netmap-empty = Belum ada jaringan untuk ditampilkan
netmap-empty-note = Entitas muncul di sini setelah dibaca dan melaporkan posisinya di pohon gPTP.
netmap-focus-clock-path = jalur clock { $name }
netmap-focus-streams = stream { $name }
netmap-showing = Menampilkan { $what }
netmap-devices = { $count } perangkat
netmap-bridges = { $count } switch
netmap-show-map = Tampilkan peta
netmap-show-details = Tampilkan detail
stream-numbered = Stream { $index }
netmap-bridge = Switch
netmap-access-point = Access point
netmap-device = Perangkat
netmap-this-computer = Komputer ini
netmap-connected = Terhubung
netmap-advertised = Diumumkan, belum ada listener siap
netmap-advertised-off-tree = Diumumkan, belum ada listener siap ({ $listener } tidak ada di pohon gPTP)
netmap-failed-at = Reservasi gagal di { $bridge }: { $reason }
netmap-failed = Reservasi gagal: { $reason }
netmap-no-bridge-on = Tidak ada switch terdeteksi di { $interface }
netmap-cannot-listen-on = Tidak dapat mendengarkan gPTP di { $interface }
netmap-on-this-computer = Di komputer ini
netmap-path-not-reported = Jalur tidak dilaporkan
netmap-gptp-not-reported = gPTP tidak dilaporkan
netmap-off-tree = Tidak ada di pohon gPTP
netmap-off-ptp = Tidak ada di pohon PTP
netmap-not-lite = Di luar AVB Lite
netmap-lite-not-reported = AVB Lite tidak dilaporkan
netmap-synced = Sinkron
netmap-not-synced = Tidak sinkron
netmap-triib-on = triib di { $interface }
netmap-through-count = { $count } melintas
netmap-out = { $count } keluar
netmap-in = { $count } masuk
netmap-failed-count = { $count } gagal
netmap-advertised-only = Hanya diumumkan
netmap-failed-state = Gagal
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Tidak ada di pohon gPTP: menjadi grandmaster sendiri
netmap-apart-no-path = Jalurnya tidak dilaporkan; mengikuti grandmaster { $grandmaster }
netmap-apart-unreported = Belum melaporkan status gPTP-nya
netmap-apart-no-neighbor = Tidak ada switch terdeteksi di antarmuka komputer ini
netmap-apart-cannot-listen = Komputer ini tidak dapat mendengarkan gPTP di antarmukanya
netmap-apart-on-this-computer = Berjalan di komputer ini; baca dari komputer lain untuk melihat status gPTP-nya
netmap-apart-not-lite = Tidak menjalankan AVB Lite, jadi tidak mengikuti grandmaster
netmap-apart-lite-unreported = Tidak melaporkan apa pun tentang AVB Lite, jadi tidak diketahui apa yang diikutinya
netmap-clock-tree = Pohon clock
netmap-no-grandmaster = Tidak ada grandmaster terdeteksi
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Perlu perhatian
netmap-nodes-below = Node di bawah
netmap-bridges-below = Switch di bawah
netmap-clock-path = Jalur clock
netmap-hops = Hop dari grandmaster
netmap-link-delay = Delay link
netmap-bridge-port = Port switch
netmap-link-drops = Link terputus
netmap-synced-to-grandmaster = Sinkron ke grandmaster
netmap-host-no-gptp = Tidak sinkron: komputer ini tidak menjalankan gPTP
netmap-link-no-gptp = Tidak sinkron: gPTP tidak berjalan di link-nya
netmap-ptp-offset-high = Tidak sinkron: { $offset } dari grandmaster, melewati 50 µs yang diizinkan AVB Lite
netmap-ptp-no-offset = Tidak sinkron: belum mengukur offset apa pun dari grandmaster
netmap-audio = Audio
netmap-media-clock-streams = Stream media clock
netmap-audio-streams = Stream audio
netmap-bound = { $count } terikat
netmap-flowing = Mengalir
netmap-advertised-state = Diumumkan
netmap-media-clock-stream = Stream media clock
netmap-audio-stream = Stream audio
netmap-reaches = Mencapai
netmap-passing-count = { $count } stream melintas
netmap-through = Melalui
netmap-passing-through = Stream yang melintas
netmap-sending = Mengirim
netmap-receiving = Menerima
netmap-problems = Masalah
netmap-help-back = Klik latar belakang untuk kembali ke ikhtisar.
netmap-help-stream = Klik stream untuk memeriksanya, atau latar belakang untuk kembali ke ikhtisar.
netmap-help-ptp = Di AVB Lite, clock mengalir end-to-end dari grandmaster ke setiap perangkat, melalui switch yang tidak ikut berperan, jadi switch tersebut tidak ditampilkan. Perangkat sinkron selama mengikuti grandmaster dalam batas 50 µs. Klik perangkat atau kabelnya untuk memeriksa clock-nya; klik latar belakang untuk membatalkan pilihan.
netmap-help-clock = Clock mengalir dari grandmaster melalui setiap switch ke setiap node di pohon. Garis abu-abu putus-putus adalah link yang tidak menjalankan gPTP. Klik perangkat atau kabelnya untuk memeriksa jalur clock-nya; klik latar belakang untuk membatalkan pilihan.
netmap-help-media-clock = Hanya stream media clock (CRF), digambar seperti audio: satu kabel per stream, diwarnai menurut talker. Klik kabel untuk memeriksa stream-nya, atau perangkat untuk melihat stream-nya; klik latar belakang untuk membatalkan pilihan.
netmap-help-audio = Setiap stream punya kabel sendiri, yang masuk dan keluar dari setiap switch yang dilintasinya. Warna menurut talker: setiap talker punya satu rona, dan stream-nya adalah gradasi rona itu. Titik bergerak berarti audio mengalir; garis merah diam adalah reservasi yang gagal dan garis abu-abu diam adalah stream yang diumumkan tanpa listener siap; keduanya berhenti di tempat reservasi berhenti. Perangkat di kolom tengah terhubung langsung ke switch grandmaster. Klik kabel untuk memeriksa stream-nya, atau perangkat untuk melihat stream-nya; klik latar belakang untuk membatalkan pilihan.

## Connections

matrix-nothing-shown = Tidak ada stream untuk ditampilkan
matrix-nothing-shown-note = Ubah pencarian atau filter untuk melihat lebih banyak stream.
matrix-empty = Tidak ada stream untuk dihubungkan
matrix-empty-note = Stream talker dan stream listener bertemu di sini setelah entitas yang memilikinya dibaca.
matrix-all-streams = Semua stream
matrix-connectable-only = Sembunyikan yang tak bisa terhubung
matrix-none-hidden = Setiap stream yang tampil bisa terhubung
matrix-hidden = { $count } stream disembunyikan
matrix-own = Output entitas tidak terhubung ke input entitas itu sendiri.
matrix-working = Sedang diproses.
matrix-waiting-change = Menunggu perubahan terakhir pada input ini.
matrix-connected = Terhubung dan menerima. Klik untuk memutus.
matrix-bound-waiting = Terikat, menunggu stream talker. Klik untuk memutus.
matrix-bound-failed = Terikat, tetapi reservasi talker gagal: { $reason }. Klik untuk memutus.
matrix-bound-formats-differ = Terikat, tetapi formatnya berbeda: talker mengirim { $sent }, input diatur ke { $set }. Klik untuk memutus.
matrix-formats-match = Format cocok ({ $format }). Klik untuk menghubungkan.
matrix-format-must-change = Input menerima { $sent } tetapi diatur ke { $set }, jadi mungkin tidak berbunyi sampai formatnya diubah. Klik untuk memilih format.
matrix-format-input = Ubah input ke
matrix-format-both = Ubah keduanya ke
matrix-format-talker-busy = Talker sedang mengalirkan ke listener lain, jadi formatnya tetap.
matrix-format-anyway = Hubungkan tanpa mengubah format
matrix-incompatible = Input tidak menerima { $sent }. Input diatur ke { $set }.
matrix-group-none = Tidak terhubung. Bentangkan untuk menghubungkan stream satu per satu.
matrix-group-connected = { $count } terhubung. Bentangkan untuk melihat masing-masing.
matrix-outputs-expand = { $count } output stream. Klik panah untuk membentangkan, nama untuk memeriksanya.
matrix-outputs-collapse = { $count } output stream. Klik panah untuk menciutkan, nama untuk memeriksanya.
matrix-inputs-expand = { $count } input stream. Klik panah untuk membentangkan, nama untuk memeriksanya.
matrix-inputs-collapse = { $count } input stream. Klik panah untuk menciutkan, nama untuk memeriksanya.
matrix-stream-inspect = { $detail } Klik untuk memeriksa { $entity }.
matrix-point = Arahkan ke sel
matrix-point-note = untuk melihat talker dan listener-nya serta apakah formatnya cocok.
matrix-legend-waiting = Terikat, menunggu stream
matrix-legend-trouble = Terikat, ada masalah
matrix-legend-open = Bisa terhubung
matrix-legend-change = Format input harus diubah dulu
matrix-legend-incompatible = Format tidak bisa cocok
matrix-talker-outputs = Output talker
matrix-listener-inputs = Input listener

common-thousands-separator = {"."}
common-decimal-separator = {","}

## Diagnostics

diag-since-start = Dihitung sejak entitas dimulai.
diag-stream-input = Input stream
diag-stream-output = Output stream
diag-locked = { $count ->
    [0] tidak terkunci
    [1] terkunci sekali
   *[other] terkunci { $number } kali
}
diag-lost-lock = { $count ->
    [0] tidak lepas kunci
    [1] lepas kunci sekali
   *[other] lepas kunci { $number } kali
}
diag-frames-in = { $number } frame masuk
diag-frames-out = { $number } frame keluar
diag-media-locked = { $count ->
    [0] media tidak terkunci
    [1] media terkunci sekali
   *[other] media terkunci { $number } kali
}
diag-lost-media-lock = { $count ->
    [0] tidak lepas kunci media
    [1] lepas kunci media sekali
   *[other] lepas kunci media { $number } kali
}
diag-interrupted = { $count ->
    [0] tidak terputus
    [1] terputus sekali
   *[other] terputus { $number } kali
}
diag-out-of-sequence = { $number } frame tidak berurutan
diag-media-resets = { $number } reset media
diag-timestamps-uncertain = { $count ->
    [0] timestamp selalu pasti
    [1] timestamp tidak pasti sekali
   *[other] timestamp tidak pasti { $number } kali
}
diag-no-timestamp = { $number } frame tanpa timestamp
diag-unsupported-format = { $number } frame dalam format yang tidak didukung
diag-late = { $number } frame terlambat
diag-early = { $number } frame terlalu awal
diag-started = { $count ->
    [0] tidak dimulai
    [1] dimulai sekali
   *[other] dimulai { $number } kali
}
diag-stopped = { $count ->
    [0] tidak berhenti
    [1] berhenti sekali
   *[other] berhenti { $number } kali
}
diag-reservation-failed = reservasi talker gagal: { $reason }
diag-latency = latensi terakumulasi { $microseconds } µs

## AVB Lite

lite-active = Aktif
lite-active-untagged = Aktif, tanpa tag
lite-active-vlan = Aktif, VLAN { $vlan }
lite-capable = Mendukung
lite-mode = Mode
lite-mode-capable = AVB (mendukung AVB Lite)
lite-because = Karena
lite-fallback-none = tidak ada alasan yang diberikan
lite-fallback-endpoint = deklarasi endpoint lain sampai, jadi tidak ada switch AVB di antara keduanya
lite-fallback-unanswered = sembilan permintaan peer delay tidak dijawab
lite-fallback-responders = dua atau lebih menjawab satu permintaan peer delay, jadi switch itu bukan switch AVB
lite-fallback-configured = diatur oleh operator atau controller
lite-fallback-other = alasan yang tidak disebutkan profil
lite-other-profile = Profil lain
lite-ptp-domain = { $profile }, domain { $domain }
lite-offset = Offset
lite-offset-from = { $offset } dari { $grandmaster }
lite-media-vlan = VLAN media
lite-untagged = Tanpa tag
lite-unicast = Unicast
lite-fanout = Hingga { $count } listener per stream, lalu multicast
lite-link = Link
lite-bandwidth = Bandwidth
lite-egress-of = { $used } dari { $link }, { $share }
lite-egress-of-assumed = { $used } dari { $link }, { $share }, diasumsikan link gigabit
lite-egress-reported = Menurut hitungan entitas atas stream yang diterimanya.
lite-egress-worked-out = Dihitung dari format output stream yang terhubung.
lite-alarm-offset = Offset PTP { $offset }, melewati 50 µs yang diizinkan AVB Lite
lite-alarm-egress = Egress { $share } dari link, melewati batas { $limit } untuk stream

## AVB Wireless

wireless-station = Station
wireless-access-point = Access point
wireless-role = Peran
wireless-mode = Mode
wireless-time = Waktu
wireless-mode-a-ftm = Mode A, 802.1AS melalui FTM
wireless-mode-a-tm = Mode A, 802.1AS melalui TM
wireless-mode-b = Mode B, dari beacon
wireless-no-time = Tanpa waktu
wireless-other-mode = Mode yang tidak disebutkan profil
wireless-locked = Terkunci
wireless-holdover = Holdover
wireless-not-locked = Tidak terkunci
wireless-row-locked = Station, terkunci
wireless-row-holdover = Station, holdover
wireless-row-not-locked = Station, tidak terkunci
wireless-row-access-point = Access point, { $count } station
wireless-link = Link
wireless-channel = kanal { $channel }
wireless-not-known = Tidak diketahui
wireless-signal = Sinyal
wireless-rate = Kecepatan
wireless-ftm-valid = { $share } valid
wireless-rtt = round trip { $rtt }
wireless-bursts = burst { $count } frame
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = access point memberikan burst FTM dengan jumlah frame selain tiga atau dua
wireless-reason-measurement = tidak ada FTM maupun TM dengan access point
wireless-reason-signaling = tidak ada Signaling gPTP-capable dari access point
wireless-reason-other = alasan yang tidak disebutkan profil
wireless-servo = Galat servo
wireless-stations = Station
wireless-station-count = { $count } station
wireless-no-ftm = tanpa FTM
wireless-unserved = Listener tidak dilayani
wireless-frame-budget = Anggaran frame
wireless-frame-use = { $used } dari { $budget } frame per detik
wireless-stream-frames = Frame stream
wireless-frames-of = { $readdressed } ke station, { $unmapped } tanpa listener, { $dropped } dibuang, { $restored } dari station
wireless-class-a-allowed = Diizinkan
wireless-class-a-not-allowed = Tidak diizinkan
wireless-alarm-not-locked = Waktu Wi-Fi tidak terkunci ke access point
wireless-alarm-holdover = Waktu Wi-Fi dalam holdover, lepas kunci dari access point
wireless-alarm-unserved = { $count } listener di port Wi-Fi tidak dilayani, melewati batas unicast

## Log

log-all = Semua
log-warnings = Peringatan
log-pause = Jeda
log-resume = Lanjutkan
log-clear = Bersihkan
log-empty = Setiap frame ATDECC yang dikirim dan didengar triib muncul di sini, yang terbaru di atas.
log-none-match = Tidak ada frame tersimpan yang cocok dengan filter.
log-frames = { $count } frame
log-shown-of = { $shown } dari { $all } frame
log-sent = Terkirim
log-heard = Didengar
log-not-decoded = Tidak didekode
log-warning-short = control_data_length-nya mengklaim { $missing } oktet melewati akhir frame.
log-warning-undecodable = Tidak dapat didekode: { $error }.
log-warning-long-acmp = Frame ini berbentuk ACMP panjang, yang tidak boleh dikirim entitas Milan (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Pemetaan kanal
mapping-inputs = Input
mapping-outputs = Output
mapping-port = port { $number }
mapping-fixed = tetap
mapping-not-read = Belum dibaca.
mapping-no-clusters = Tidak ada klaster.
mapping-no-streams = Tidak ada stream audio.
mapping-none = Tidak ada pemetaan.
mapping-not-mapped = Tidak dipetakan
mapping-cluster-numbered = Klaster { $index }

## Presets

presets-note = Preset menyimpan sumber clock, sampling rate, format stream, kontrol, dan koneksi setiap entitas. Memanggilnya mengubah apa yang berbeda.
presets-none = Belum ada preset tersimpan.
presets-connections = { $count } koneksi
presets-recall = Panggil
presets-delete = Hapus
presets-no-place = Preset tidak dapat disimpan: folder home tidak diketahui.
presets-undeletable = Tidak dapat menghapus { $path }: { $error }.
presets-saved = "{ $name }" disimpan dengan { $count } entitas.
presets-nothing-differs = Tidak ada yang berbeda dari "{ $name }".
presets-recalling = Memanggil "{ $name }": { $count } perubahan.
presets-missing = { $report } Tidak ada atau belum dibaca: { $missing }.
presets-deleted = "{ $name }" dihapus.
presets-host-note = Preset juga menyimpan entitas milik komputer ini, dan menjalankannya lagi saat dipanggil.
presets-host-endpoints = { $count } di komputer ini
presets-starting-host = Menjalankan entitas komputer ini untuk "{ $name }"; sisanya menyusul setelah semuanya kembali.

## Controls

control-numbered = Kontrol { $index }
control-not-shown = Tidak ditampilkan di sini
control-option = Opsi { $number }

## Network errors

network-permission = triib memerlukan izin untuk mengirim dan menerima frame Ethernet mentah.
network-needs-npcap = triib memerlukan Npcap untuk mengirim dan menerima frame Ethernet mentah.
network-npcap-administrators = Npcap hanya mengizinkan administrator mengirim dan menerima frame Ethernet mentah. Jalankan triib sebagai administrator, atau instal ulang Npcap tanpa opsi khusus administrator.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own entities

host-add-entity = Tambah entitas
host-show-mine = Tampilkan hanya entitas milik komputer ini
host-show-all = Tampilkan semua entitas
host-new-entity = Entitas host { $number }
host-failed = Tidak dapat menambahkannya ke komputer ini: { $reason }
host-needs-clock = Entitas milik komputer ini memerlukan antarmuka kabel dengan clock hardware PTP
host-no-ptp4l = ptp4l tidak menjawab, jadi stream komputer ini tidak dapat menjaga waktu gPTP
host-elsewhere = triib-endpointd berjalan di { $interface } untuk pengguna lain atau sebagai root, jadi entitas milik komputer ini berjalan di sana, bukan di sini
host-foreign-mrp = Program lain membuat deklarasi MSRP atau MVRP di { $interface } dari alamat komputer ini, yang dapat menarik kembali apa yang dibutuhkan stream komputer ini
host-alarm-foreign-mrp = Program lain di komputer ini membuat deklarasi MSRP atau MVRP di { $interface }
host-state = Status
host-streaming = Mengalirkan
host-waiting = Menunggu listener
host-listening = Mendengarkan
host-bound = Terikat, menunggu talker
host-unbound = Tidak terikat
host-audio-from = Audio dari
host-audio-to = Audio ke
host-channels = Kanal
host-virtual-device = Perangkat virtual
host-add-stream = Tambah stream
host-add-audio-input = Input audio
host-add-audio-output = Output audio
host-add-clock-input = Input media clock
host-add-clock-output = Output media clock
host-remove-stream = Hapus stream
host-silence = Senyap
host-tone = Nada uji
host-nowhere = Tidak ke mana pun
host-default-device = Perangkat bawaan
host-remove = Hapus dari komputer ini

mac-audio-device = Perangkat audio
mac-no-audio-device = Tidak ada selama tidak ada stream yang tersambung
mac-entity-note = macOS menjalankan entitas ini. Selama salah satu stream-nya tersambung, macOS membuat perangkat audio tempat aplikasi memutar dan merekam, dengan jumlah kanal di tiap arah sesuai konfigurasi.
mac-open-audio-midi-setup = Buka Audio MIDI Setup
mac-avb-off = AVB mati di { $interface }, jadi Mac ini tidak punya entitas AVB sendiri. Nyalakan di Network Device Browser pada Audio MIDI Setup.

listen-section = Dengarkan
listen-start = Dengarkan
listen-stop = Berhenti mendengarkan
listen-source = Mendengarkan
listen-received = Yang diterimanya
listen-sent = Yang dikirimnya di stream { $number } ({ $channels } kanal)
listen-sent-needs-macos = Mendengarkan apa yang dikirim Mac ini memerlukan macOS 14.2 atau lebih baru.
listen-channels = Kanal
listen-channel = Kanal { $number }
listen-channel-pair = Kanal { $first }–{ $last }
listen-output = Output
listen-default-output = Output bawaan
listen-gain = Penguatan
listen-gain-db = { $db } dB
listen-starting = Memulai…
listen-waiting-input = Menunggu { $device }, yang dibuat macOS selama salah satu stream entitas tersambung.
listen-waiting-output = Menunggu { $device }.
listen-playing = Memutar, puncak { $db } dB
listen-playing-silent = Memutar, senyap
