# Văn bản giao diện của triib bằng tiếng Việt.
# Thuật ngữ theo docs/GLOSSARY.md và docs/glossary/vi.md.

## Language

language-name = Tiếng Việt

## Common

common-close = Đóng
common-more = Thêm
common-keep-toolbar-shown = Luôn hiện thanh công cụ
common-auto-hide-toolbar = Tự động ẩn thanh công cụ

## Settings

settings-title = Cài đặt
settings-general = Chung
settings-appearance = Diện mạo
settings-language = Ngôn ngữ
settings-language-system = Mặc định hệ thống: { $language }
settings-language-note = Ô văn bản nhập theo ngôn ngữ nhập liệu của hệ thống.
settings-appearance-system = Theo hệ thống
settings-appearance-light = Sáng
settings-appearance-dark = Tối
settings-colors = Màu sắc
settings-system-accent = Dùng màu nhấn của hệ thống
settings-accent-picked = Bảng màu của triib được tạo từ màu bên dưới.
settings-accent-omarchy = Từ chủ đề Omarchy, { $theme }.
settings-accent-desktop = Từ màu nhấn của màn hình nền.
settings-accent-none = Màn hình nền không có màu nhấn, nên dùng màu bên dưới.
settings-motion = Chuyển động
settings-animations = Hiệu ứng động
settings-animations-note = Hiệu ứng nảy và trượt khi nội dung thay đổi.
settings-animations-reduced = Màn hình nền yêu cầu giảm chuyển động, nên triib không chạy hiệu ứng.
settings-outside-control = Điều khiển từ bên ngoài
settings-allow-outside-control = Cho phép điều khiển từ bên ngoài
settings-allow-outside-control-note = Các tác nhân như Claude Code có thể xem và thay đổi mạng AVB của bạn, thông qua triib --mcp. triib sẽ hỏi trước mỗi tác nhân mới.
settings-allowed-agents = Đã cho phép: { $agents }
settings-forget-agents = Quên
settings-storage = Lưu trữ
settings-storage-presets = Thư mục preset
settings-storage-entities = Tệp thực thể
settings-storage-apply = Áp dụng
settings-storage-choose = Chọn…
settings-storage-note = Các preset đã lưu ở vị trí cũ vẫn ở đó; hãy di chuyển chúng sang để tiếp tục sử dụng. Cài đặt của triib được lưu trong { $file }.
settings-storage-entities-note = Tệp thực thể chưa có sẽ bắt đầu là bản sao của tệp đang dùng, và triib-endpointd khởi động lại với tệp đó.
dialog-choose-presets = Chọn thư mục preset
dialog-choose-entities = Chọn thư mục cho tệp thực thể
settings-full-path = Hãy dùng đường dẫn đầy đủ, chẳng hạn ~/Documents/triib.
settings-path-is-folder = { $path } là thư mục, không phải tệp.
settings-folder-missing = Không có thư mục { $path }. Hãy tạo thư mục trước hoặc chọn một thư mục khác.
settings-path-is-file = { $path } là tệp, không phải thư mục.
settings-cannot-write = triib không thể ghi vào { $path }: { $error }.
app-file-dialog-failed = Không thể hiển thị hộp thoại tệp: { $error }
agent-prompt-title = Cho phép { $agent } điều khiển triib?
agent-prompt-body = { $agent } yêu cầu dùng tính năng điều khiển từ bên ngoài của triib: xem các thực thể trên mạng của bạn và thay đổi chúng, kết nối luồng và đặt định dạng, đồng hồ và tên. Bạn có thể tắt điều khiển từ bên ngoài trong Cài đặt.
agent-prompt-allow = Cho phép
agent-prompt-deny = Không cho phép

common-cancel = Hủy
common-save = Lưu
common-not-set = Chưa đặt
common-unnamed = Chưa đặt tên
common-none = Không có
common-mac-address = Địa chỉ MAC
common-list-separator = {", "}

## Network interfaces

interface-up = hoạt động
interface-link-down = mất liên kết
interface-wireless = không dây
interface-hardware-clock = đồng hồ phần cứng
interface-hardware-clock-named = đồng hồ phần cứng { $clock }
interface-virtual = ảo

## Toolbar

toolbar-choose-interface = Chọn giao diện mạng
toolbar-interface = Giao diện mạng
toolbar-filter-interfaces = Lọc giao diện
toolbar-hide-virtual = Ẩn giao diện ảo
toolbar-hide-wireless = Ẩn giao diện không dây
toolbar-hide-unclocked = Ẩn giao diện không có đồng hồ phần cứng
toolbar-connections = Kết nối
toolbar-network = Mạng
toolbar-entities = Thực thể
toolbar-rediscover = Yêu cầu mọi thực thể quảng bá lại
toolbar-rescan = Xóa và quét lại mọi thực thể
toolbar-search = Tìm thực thể và luồng
toolbar-presets = Preset
toolbar-log = Nhật ký
toolbar-inspector = Trình kiểm tra
toolbar-settings = Cài đặt
toolbar-close-window = Đóng cửa sổ

## The macOS menu bar, named as in macOS's own apps

menu-about = Giới thiệu về triib
menu-settings = Cài đặt…
menu-services = Dịch vụ
menu-hide = Ẩn triib
menu-hide-others = Ẩn các ứng dụng khác
menu-show-all = Hiển thị tất cả
menu-quit = Thoát triib
menu-file = Tệp
menu-presets = Preset…
menu-close-window = Đóng cửa sổ
menu-edit = Sửa
menu-cut = Cắt
menu-copy = Sao chép
menu-paste = Dán
menu-select-all = Chọn tất cả
menu-view = Xem
menu-full-screen = Vào chế độ toàn màn hình
menu-window = Cửa sổ
menu-minimize = Thu nhỏ cửa sổ
menu-zoom = Thu phóng
menu-bring-all-to-front = Đưa tất cả ra trước
menu-help = Trợ giúp
menu-website = Trang web triib
menu-release-notes = Ghi chú phát hành

## The network's state, in place of a view

state-no-interface = Chưa chọn giao diện
state-no-interface-note = Chọn giao diện nối với mạng AVB để phát hiện thực thể.
state-starting = Đang khởi động
state-starting-note = Đang mở { $interface }.
state-listening = Đang lắng nghe
state-listening-note = Thực thể trên { $interface } sẽ hiện ở đây khi chúng quảng bá.
state-permission-needed = Cần cấp quyền
state-npcap-needed = Cần Npcap
state-get-npcap = Tải Npcap
state-copy-command = Sao chép lệnh
state-permission-watch = triib sẽ tự tiếp tục sau khi lệnh được chạy.
state-cannot-use = Không thể dùng { $interface }
state-try-again = Thử lại

## Entity list

entities-none-yet = Chưa có thực thể nào
entities-none-yet-note = Mọi thực thể trên mạng, cùng vai trò, lớp SR và đồng hồ của chúng.
entities-filter = Lọc thực thể
entities-hide-others = Ẩn thực thể trên thiết bị khác
entities-hide-controllers = Ẩn bộ điều khiển
entities-hide-other-domains = Ẩn thực thể nằm ngoài miền đồng hồ của máy tính này
entities-hidden = Đã ẩn { $count } thực thể

## Inspector

inspector-title = Trình kiểm tra
inspector-entity = Thực thể
inspector-streams = Luồng
inspector-controls = Điều khiển
inspector-diagnostics = Chẩn đoán
inspector-descriptors = Bộ mô tả
inspector-select = Chọn một thực thể để xem chi tiết.
inspector-offline = { $entity } đang ngoại tuyến.
inspector-heard = Nó không tự quảng bá: triib nghe thấy nó qua các lệnh nó gửi, và giữ trong danh sách cho đến khi 5 phút trôi qua không có lệnh nào.
inspector-rename = Đổi tên
inspector-name = Tên
inspector-identify = Nhận dạng
inspector-model-not-read = Chưa đọc mô hình thực thể.
inspector-no-streams = Không có luồng.
inspector-no-controls = Không có mục điều khiển nào để hiển thị.
inspector-no-diagnostics = Không có giao diện hay bộ đếm nào được báo cáo.
inspector-reading = Đang đọc bộ mô tả, đã đọc { $count }.
inspector-read-failed = Không thể đọc mô hình thực thể: { $reason }.

entity-section = Thực thể
entity-name = Tên
entity-triib-controller = Bộ điều khiển triib
entity-triib-on = triib trên { $host }
entity-triib-here = triib trên máy tính này
entity-controller-on = Bộ điều khiển trên { $host }
entity-controller-here = Bộ điều khiển trên máy tính này
entity-controller-at = Bộ điều khiển tại { $mac }
entity-group = Nhóm
entity-product = Sản phẩm
entity-firmware = Firmware
entity-serial-number = Số sê-ri
entity-configuration = Cấu hình
entity-configuration-of = { $name } ({ $number }/{ $count })
entity-configuration-holds = { $inputs } vào, { $outputs } ra, luồng { $channels } kênh
entity-configuration-holds-mixed = { $inputs } vào, { $outputs } ra
entity-configuration-numbered = Cấu hình { $number }
entity-milan = Milan
entity-media-clock = Đồng hồ media
entity-clock-domain = Miền đồng hồ
entity-sampling-rate = Tần số lấy mẫu
clock-source-numbered = Nguồn { $index }
rate-pull = hệ số pull { $pull }

stream-format = Định dạng
stream-id = ID luồng
stream-destination = Đích
stream-max-transit = Thời gian truyền tối đa
stream-listeners = Bên nghe
stream-listener-gone = { $entity } đầu vào { $input }, ngoại tuyến
stream-disconnect-listener = Ngắt kết nối

avb-interfaces = Giao diện AVB
avb-interface = Giao diện
avb-interface-clock-identity = Định danh đồng hồ
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, miền { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Đang chạy
avb-interface-none-reported = Không báo cáo
avb-interface-path = Đường đi
avb-interface-own-grandmaster = Tự là Grandmaster
avb-interface-hops = Cách Grandmaster { $count } bước nhảy
avb-interface-link-up = Liên kết hoạt động
avb-interface-link-down = Mất liên kết
avb-interface-grandmaster-changes = Số lần đổi Grandmaster
avb-interface-frames-sent = Khung đã gửi
avb-interface-frames-received = Khung đã nhận
avb-interface-crc-errors = Lỗi CRC

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count } loại bộ mô tả
tree-clock = Đồng hồ
tree-clock-source-from = { $kind }, từ { $location } { $index }
tree-clock-domain-using = Dùng { $source }
tree-clusters = { $count } cụm
tree-maps = { $count } ánh xạ

advert-not-advertised = Không quảng bá
advert-identity = Định danh
advert-entity-id = ID thực thể
advert-entity-model = Mô hình thực thể
advert-roles = Vai trò
advert-talker = Talker
advert-listener = Listener
advert-clock = Đồng hồ
advert-gptp-domain = Miền gPTP
advert-sr-classes = Lớp SR
advert-indexes = Chỉ mục mô hình thực thể
advert-identify-control = Điều khiển nhận dạng
advert-avb-interface = Giao diện AVB
advert-advertising = Quảng bá
advert-valid-time = Thời gian hiệu lực
advert-available-index = Chỉ mục khả dụng
advert-association = Kết hợp
advert-capabilities = Khả năng

## Status bar

status-entities = { $count } thực thể
status-not-discovering = Không phát hiện
status-discovering = Đang phát hiện
status-discovering-as = Đang phát hiện với tư cách { $controller }
status-stopped = Đã dừng do lỗi
status-alarm = Báo động
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } và { $count } báo động khác

## Descriptions of entities, shared by the views

role-talker = Talker { $count }
role-listener = Listener { $count }
role-controller = bộ điều khiển
role-none = không có vai trò
classes-a-and-b = A và B
clock-no-gptp = Không có gPTP

read-not-read = Chưa đọc
read-reading = Đang đọc, đã đọc { $count }
read-ready-unreadable = Sẵn sàng, { $count } không đọc được
read-ready-cached = Sẵn sàng, từ bộ nhớ đệm
read-ready = Sẵn sàng
read-failed = Thất bại: { $reason }

milan-no = Không
milan-before-1-3 = trước 1.3
milan-certified = { $version }, chứng nhận { $certification }
milan-not-certified = { $version }, chưa chứng nhận

outcome-status = trạng thái { $status }
outcome-no-response = không có phản hồi
outcome-not-possible = không thể thực hiện
outcome-connect = Không thể kết nối { $talker } tới { $listener }: { $reason }.
outcome-disconnect = Không thể ngắt kết nối { $listener }: { $reason }.
outcome-disconnect-listener = Không thể ngắt kết nối { $listener } khỏi { $talker }: { $reason }.
outcome-identify = Không thể gửi lệnh nhận dạng tới { $entity }: { $reason }.
outcome-rename = Không thể đổi tên { $what } thành “{ $name }”: { $reason }.
outcome-rename-group = Không thể đổi tên nhóm của { $entity } thành “{ $name }”: { $reason }.
outcome-format-streaming = Không thể đổi định dạng của { $stream }: luồng đang truyền. Hãy ngắt kết nối trước.
outcome-format = Không thể đổi định dạng của { $stream }: { $reason }.
outcome-max-transit-time = Không thể đổi thời gian truyền tối đa của { $stream }: { $reason }.
outcome-max-transit-time-streaming = Không thể đổi thời gian truyền tối đa của { $stream }: luồng đang truyền. Hãy ngắt kết nối trước.
outcome-sampling-rate = Không thể đổi tần số lấy mẫu của { $entity }: { $reason }.
outcome-clock-source = Không thể đổi nguồn đồng hồ của { $entity }: { $reason }.
outcome-configuration = Không thể đổi cấu hình của { $entity }: { $reason }.
outcome-configuration-streaming = { $entity } giữ nguyên cấu hình khi các luồng của nó đang chạy: hãy ngắt kết nối chúng trước.
outcome-map = Không thể ánh xạ kênh trên { $entity }: { $reason }.
outcome-unmap = Không thể bỏ ánh xạ kênh trên { $entity }: { $reason }.
outcome-control = Không thể đặt “{ $control }” trên { $entity }: { $reason }.
outcome-control-numbered = Không thể đặt điều khiển { $index } trên { $entity }: { $reason }.

stream-not-connected = Chưa kết nối
stream-from = Từ { $stream }
stream-from-receiving = Từ { $stream }, đang nhận
stream-from-waiting = Từ { $stream }, đang chờ Talker
stream-from-failed = Từ { $stream }, đặt trước của Talker thất bại: { $reason }
stream-sending-to = Đang gửi tới { $destination }

failure-no-response = không phản hồi
failure-refused = bị từ chối với { $status }
failure-malformed = không giải mã được phản hồi
failure-on-this-computer = chạy trên chính máy tính này; hãy đọc từ máy khác

msrp-failure-1 = không đủ băng thông
msrp-failure-2 = không đủ tài nguyên switch
msrp-failure-3 = không đủ băng thông cho lớp lưu lượng
msrp-failure-4 = ID luồng đang được một Talker khác dùng
msrp-failure-5 = địa chỉ đích đã được dùng
msrp-failure-6 = bị một luồng có thứ hạng cao hơn giành quyền
msrp-failure-7 = độ trễ được báo cáo đã thay đổi
msrp-failure-8 = cổng ra không hỗ trợ AVB
msrp-failure-9 = hãy dùng địa chỉ đích khác
msrp-failure-10 = hết tài nguyên MSRP
msrp-failure-11 = hết tài nguyên MMRP
msrp-failure-12 = không thể lưu địa chỉ đích
msrp-failure-13 = mức ưu tiên không phải mức ưu tiên của lớp SR
msrp-failure-14 = khung quá lớn so với môi trường truyền
msrp-failure-15 = đã đạt giới hạn cổng fan-in
msrp-failure-16 = giá trị đầu tiên của một luồng đã đăng ký bị thay đổi
msrp-failure-17 = VLAN bị chặn trên cổng ra
msrp-failure-18 = gắn thẻ VLAN bị tắt trên cổng ra
msrp-failure-19 = mức ưu tiên lớp SR không khớp
msrp-failure-unknown = lý do không xác định
msrp-failure-at = { $reason }, tại switch { $bridge }

## Entity list columns

column-vendor = Nhà sản xuất
column-entity-image = Hình ảnh thực thể
column-vendor-image = Hình ảnh nhà sản xuất
column-model = Model
column-state = Trạng thái
column-entity-model-id = ID mô hình thực thể
column-talker-streams = Luồng Talker
column-listener-streams = Luồng Listener
column-avb-lite = AVB Lite
column-egress = Lưu lượng ra
column-wireless = Không dây

## Settings file

settings-no-place = Không có nơi lưu cài đặt: không xác định được thư mục nhà.
settings-unusable = Không thể dùng { $path }: { $error }.
settings-unsaved = Không thể lưu { $path }: { $error }.

column-remove = Xóa cột
column-move-left = Chuyển sang trái
column-move-right = Chuyển sang phải
column-add = Thêm cột
common-percent = { $value }%

## Network view

netmap-empty = Chưa có mạng để hiển thị
netmap-empty-note = Thực thể sẽ hiện ở đây sau khi đã được đọc và cho biết miền đồng hồ của chúng.
netmap-focus-clock-path = đường đi đồng hồ của { $name }
netmap-focus-streams = các luồng của { $name }
netmap-showing = Đang hiển thị { $what }
netmap-devices = { $count } thiết bị
netmap-bridges = { $count } switch
netmap-show-map = Hiện sơ đồ
netmap-show-details = Hiện chi tiết
stream-numbered = Luồng { $index }
netmap-bridge = Switch
netmap-access-point = Điểm truy cập
netmap-plain-switch = Switch không AVB
netmap-device = Thiết bị
netmap-this-computer = Máy tính này
netmap-connected = Đã kết nối
netmap-advertised = Đã quảng bá, chưa có Listener sẵn sàng
netmap-advertised-unknown-domain = Đã quảng bá, chưa có Listener sẵn sàng (không rõ miền đồng hồ của { $listener })
netmap-failed-at = Đặt trước thất bại tại { $bridge }: { $reason }
netmap-failed = Đặt trước thất bại: { $reason }
netmap-no-bridge-on = Không thấy switch nào trên { $interface }
netmap-cannot-listen-on = Không thể nghe gPTP trên { $interface }
netmap-behind-plain-switch = Phía sau switch không AVB
netmap-on-this-computer = Trên máy tính này
netmap-path-not-reported = Chưa báo cáo đường đi
netmap-gptp-not-reported = Chưa báo cáo gPTP
netmap-domain = Miền gPTP { $domain }
netmap-ptp-domain = Miền PTP { $domain }
netmap-domain-unknown = Không rõ miền
netmap-synced = Đã đồng bộ
netmap-sync = Đồng bộ
netmap-not-synced = Chưa đồng bộ
netmap-triib-on = triib trên { $interface }
netmap-through-count = { $count } đi qua
netmap-out = { $count } ra
netmap-in = { $count } vào
netmap-failed-count = { $count } thất bại
netmap-advertised-only = Chỉ quảng bá
netmap-failed-state = Thất bại
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-sending-unheard = Đang gửi, không có Listener đã biết nào được gán vào
netmap-no-listener = Không có Listener đã biết
netmap-apart-no-path = Chưa báo cáo đường đi; nó theo Grandmaster { $grandmaster }
netmap-apart-unreported = Chưa báo cáo trạng thái gPTP
netmap-apart-no-neighbor = Không thấy switch nào trên giao diện của máy tính này
netmap-apart-cannot-listen = Máy tính này không thể nghe gPTP trên giao diện của mình
netmap-apart-plain-switch = Máy tính này ở phía sau một switch không tham gia AVB, và không thiết bị nào dùng chung switch đó chạy AVB Lite ở đấy, nên nó không thuộc miền nào
netmap-apart-on-this-computer = Chạy trên chính máy tính này; hãy đọc từ máy khác để xem trạng thái gPTP
netmap-switch-state = Không tham gia AVB: đại diện cho các switch phía trước các thiết bị bên dưới, vốn không tìm thấy switch AVB nên chuyển sang AVB Lite và cùng theo một Grandmaster PTP
netmap-clock-tree = Cây đồng hồ
netmap-no-grandmaster = Không thấy Grandmaster nào
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Cần chú ý
netmap-nodes-below = Nút phía dưới
netmap-bridges-below = Switch phía dưới
netmap-clock-path = Đường đi đồng hồ
netmap-hops = Số bước nhảy từ Grandmaster
netmap-link-delay = Độ trễ liên kết
netmap-bridge-port = Cổng switch
netmap-link-drops = Số lần mất liên kết
netmap-synced-to-grandmaster = Đã đồng bộ với Grandmaster
netmap-host-no-gptp = Chưa đồng bộ: máy tính này không chạy gPTP
netmap-host-no-ptp = Chưa đồng bộ: máy tính này không theo Grandmaster của miền này
netmap-link-no-gptp = Chưa đồng bộ: gPTP không chạy trên liên kết của nó
netmap-ptp-offset-high = Chưa đồng bộ: lệch { $offset } so với Grandmaster, vượt mức 50 µs mà AVB Lite cho phép
netmap-ptp-no-offset = Chưa đồng bộ: nó chưa đo được độ lệch nào so với Grandmaster
netmap-audio = Âm thanh
netmap-media-clock-streams = Luồng đồng hồ media
netmap-audio-streams = Luồng âm thanh
netmap-bound = { $count } đã gán
netmap-flowing = Đang truyền
netmap-advertised-state = Đã quảng bá
netmap-media-clock-stream = Luồng đồng hồ media
netmap-audio-stream = Luồng âm thanh
netmap-reaches = Đi tới
netmap-passing-count = { $count } luồng đi qua
netmap-through = Đi qua
netmap-passing-through = Các luồng đi qua
netmap-sending = Đang gửi
netmap-receiving = Đang nhận
netmap-problems = Sự cố
netmap-help-back = Nhấp vào nền để quay lại tổng quan.
netmap-help-stream = Nhấp vào một luồng để kiểm tra, hoặc nhấp vào nền để quay lại tổng quan.
netmap-help-clock = Đồng hồ đi từ Grandmaster qua từng switch tới mọi nút trên cây. Đường xám đứt nét là liên kết không chạy gPTP. Nhấp vào một thiết bị hoặc dây của nó để kiểm tra đường đi đồng hồ; nhấp vào nền để bỏ chọn.
netmap-help-domains = Mỗi khu vực là một miền đồng hồ. Trong miền gPTP, đồng hồ đi từ Grandmaster xuống qua các switch AVB. Trong miền PTP, các thiết bị chạy AVB Lite và theo Grandmaster của chúng từ đầu đến cuối, đồng bộ trong phạm vi 50 µs; thẻ viền đứt nét đại diện cho các switch không tham gia AVB mà phía sau đó một số thiết bị đã chuyển sang AVB Lite. Thiết bị không dùng chung miền với thiết bị nào khác có khu vực riêng.
netmap-help-media-clock = Chỉ các luồng đồng hồ media (CRF), vẽ giống như âm thanh: một dây cho các khung của mỗi luồng trên mỗi liên kết, tô màu theo Talker. Nhấp vào một dây để kiểm tra luồng của nó, hoặc vào một thiết bị để xem các luồng của thiết bị đó; nhấp vào nền để bỏ chọn.
netmap-help-audio = Mỗi dây là các khung của một luồng trên một liên kết, đi vào và ra khỏi mọi switch mà nó đi qua. Một luồng multicast chỉ có một dây chừng nào đường đi tới các Listener của nó còn chung nhau, và rẽ nhánh ở nơi chúng tách ra; một luồng unicast, loại mà AVB Lite có thể gửi, có dây riêng từ Talker tới từng Listener. Màu theo Talker: mỗi Talker có một sắc màu, và mỗi luồng của nó là một sắc độ của màu đó trên mọi nhánh. Chấm chuyển động nghĩa là âm thanh đang truyền, kể cả từ một Talker đang gửi mà không có Listener nào đã biết; đường đỏ đứng yên là đặt trước thất bại, còn đường xám đứng yên là đã quảng bá nhưng chưa có Listener sẵn sàng; cả hai dừng ở nơi đặt trước dừng lại. Thiết bị ở cột giữa nối thẳng vào switch của Grandmaster. Nhấp vào một dây để kiểm tra luồng của nó, vào một dây dùng chung để xem tất cả Listener của nó, hoặc vào một thiết bị để xem các luồng của thiết bị đó; nhấp vào nền để bỏ chọn.

## Connections

matrix-nothing-shown = Không có luồng để hiển thị
matrix-nothing-shown-note = Đổi tìm kiếm hoặc bộ lọc để xem thêm luồng.
matrix-empty = Không có luồng để kết nối
matrix-empty-note = Luồng Talker và luồng Listener gặp nhau ở đây sau khi đã đọc các thực thể có chúng.
matrix-all-streams = Tất cả luồng
matrix-filter = Lọc luồng
matrix-hide-unconnectable = Ẩn luồng không thể kết nối
matrix-hide-lite = Ẩn thực thể đang chạy AVB Lite
matrix-hide-wireless = Ẩn thực thể không dây
matrix-hidden = Đã ẩn { $count } luồng
matrix-own = Đầu ra của một thực thể không kết nối với đầu vào của chính nó.
matrix-working = Đang xử lý.
matrix-waiting-change = Đang chờ thay đổi trước đó của đầu vào này.
matrix-connected = Đã kết nối và đang nhận. Nhấp để ngắt kết nối.
matrix-bound-waiting = Đã gán, đang chờ luồng của Talker. Nhấp để ngắt kết nối.
matrix-bound-failed = Đã gán, nhưng đặt trước của Talker thất bại: { $reason }. Nhấp để ngắt kết nối.
matrix-bound-formats-differ = Đã gán, nhưng định dạng khác nhau: Talker gửi { $sent }, đầu vào đang đặt là { $set }. Nhấp để ngắt kết nối.
matrix-formats-match = Định dạng khớp ({ $format }). Nhấp để kết nối.
matrix-format-must-change = Đầu vào nhận được { $sent } nhưng đang đặt là { $set }, nên có thể không phát cho tới khi đổi định dạng. Nhấp để chọn định dạng.
matrix-format-input = Đổi đầu vào thành
matrix-format-both = Đổi cả hai thành
matrix-format-talker-busy = Talker đang truyền tới các Listener khác nên giữ nguyên định dạng.
matrix-format-anyway = Kết nối mà không đổi định dạng
matrix-incompatible = Đầu vào không nhận { $sent }. Nó đang đặt là { $set }.
matrix-other-domain = Chúng giữ thời gian ở các miền đồng hồ khác nhau, nên không luồng nào có thể đi giữa chúng.
matrix-group-none = Chưa kết nối. Mở rộng để kết nối từng luồng.
matrix-group-connected = { $count } đã kết nối. Mở rộng để xem từng kết nối.
matrix-outputs-expand = { $count } đầu ra luồng. Nhấp mũi tên để mở rộng, nhấp tên để kiểm tra.
matrix-outputs-collapse = { $count } đầu ra luồng. Nhấp mũi tên để thu gọn, nhấp tên để kiểm tra.
matrix-inputs-expand = { $count } đầu vào luồng. Nhấp mũi tên để mở rộng, nhấp tên để kiểm tra.
matrix-inputs-collapse = { $count } đầu vào luồng. Nhấp mũi tên để thu gọn, nhấp tên để kiểm tra.
matrix-stream-inspect = { $detail } Nhấp để kiểm tra { $entity }.
matrix-point = Trỏ vào một ô
matrix-point-note = để xem Talker, Listener của ô và định dạng của chúng có khớp không.
matrix-legend-waiting = Đã gán, đang chờ luồng
matrix-legend-trouble = Đã gán, có sự cố
matrix-legend-open = Có thể kết nối
matrix-legend-change = Cần đổi định dạng đầu vào trước
matrix-legend-incompatible = Định dạng không thể khớp
matrix-legend-other-domain = Miền đồng hồ khác
matrix-talker-outputs = Đầu ra Talker
matrix-listener-inputs = Đầu vào Listener

common-thousands-separator = {"."}
common-decimal-separator = {","}

## Diagnostics

diag-since-start = Đếm từ khi thực thể khởi động.
diag-stream-input = Đầu vào luồng
diag-stream-output = Đầu ra luồng
diag-locked = { $count ->
    [0] chưa khóa
   *[other] khóa { $number } lần
}
diag-lost-lock = { $count ->
    [0] chưa mất khóa
   *[other] mất khóa { $number } lần
}
diag-frames-in = { $number } khung vào
diag-frames-out = { $number } khung ra
diag-media-locked = { $count ->
    [0] chưa khóa media
   *[other] khóa media { $number } lần
}
diag-lost-media-lock = { $count ->
    [0] chưa mất khóa media
   *[other] mất khóa media { $number } lần
}
diag-interrupted = { $count ->
    [0] chưa bị gián đoạn
   *[other] bị gián đoạn { $number } lần
}
diag-out-of-sequence = { $number } khung sai thứ tự
diag-media-resets = { $number } lần đặt lại media
diag-timestamps-uncertain = { $count ->
    [0] dấu thời gian chưa từng bất định
   *[other] dấu thời gian bất định { $number } lần
}
diag-no-timestamp = { $number } khung không có dấu thời gian
diag-unsupported-format = { $number } khung có định dạng không được hỗ trợ
diag-late = { $number } khung đến muộn
diag-early = { $number } khung đến sớm
diag-started = { $count ->
    [0] chưa khởi động
   *[other] khởi động { $number } lần
}
diag-stopped = { $count ->
    [0] chưa dừng
   *[other] dừng { $number } lần
}
diag-reservation-failed = đặt trước của Talker thất bại: { $reason }
diag-latency = độ trễ tích lũy { $microseconds } µs

## AVB Lite

lite-active = Đang hoạt động
lite-active-untagged = Đang hoạt động, không gắn thẻ
lite-active-vlan = Đang hoạt động, VLAN { $vlan }
lite-capable = Hỗ trợ
lite-mode = Chế độ
lite-mode-capable = AVB (hỗ trợ AVB Lite)
lite-because = Lý do
lite-fallback-none = không nêu lý do
lite-fallback-endpoint = khai báo của một thiết bị đầu cuối khác đi qua được, nên giữa chúng không có switch AVB
lite-fallback-unanswered = chín yêu cầu peer delay không được trả lời
lite-fallback-responders = một yêu cầu peer delay nhận được từ hai phản hồi trở lên, nên switch không phải switch AVB
lite-fallback-configured = do người vận hành hoặc một bộ điều khiển đặt
lite-fallback-other = lý do mà profile không nêu tên
lite-other-profile = Profile khác
lite-ptp-domain = { $profile }, miền { $domain }
lite-offset = Độ lệch
lite-offset-from = { $offset } so với { $grandmaster }
lite-media-vlan = VLAN media
lite-untagged = Không gắn thẻ
lite-unicast = Unicast
lite-fanout = Mỗi luồng gửi lần lượt tới tối đa { $count } Listener, sau đó chuyển sang multicast
lite-link = Liên kết
lite-bandwidth = Băng thông
lite-egress-of = { $used } trên { $link }, { $share }
lite-egress-of-assumed = { $used } trên { $link }, { $share }, giả định liên kết gigabit
lite-egress-reported = Theo số luồng được chấp nhận mà thực thể đếm.
lite-egress-worked-out = Tính từ định dạng của các đầu ra luồng đã kết nối.
lite-alarm-offset = Độ lệch PTP { $offset }, vượt mức 50 µs mà AVB Lite cho phép
lite-alarm-egress = Lưu lượng ra chiếm { $share } liên kết, vượt mức { $limit } mà các luồng được phép dùng

## AVB Wireless

wireless-station = Trạm
wireless-access-point = Điểm truy cập
wireless-role = Vai trò
wireless-mode = Chế độ
wireless-time = Thời gian
wireless-mode-a-ftm = Mode A, 802.1AS qua FTM
wireless-mode-a-tm = Mode A, 802.1AS qua TM
wireless-mode-b = Mode B, từ beacon
wireless-no-time = Không nhận thời gian
wireless-other-mode = Chế độ mà profile không nêu tên
wireless-locked = Đã khóa
wireless-holdover = Đang holdover
wireless-not-locked = Chưa khóa
wireless-row-locked = Trạm, đã khóa
wireless-row-holdover = Trạm, đang holdover
wireless-row-not-locked = Trạm, chưa khóa
wireless-row-access-point = Điểm truy cập, { $count } trạm
wireless-link = Liên kết
wireless-channel = kênh { $channel }
wireless-not-known = Không rõ
wireless-signal = Tín hiệu
wireless-rate = Tốc độ
wireless-ftm-valid = { $share } hợp lệ
wireless-rtt = khứ hồi { $rtt }
wireless-bursts = mỗi burst { $count } khung
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = điểm truy cập cấp burst FTM có số khung khác ba hoặc hai
wireless-reason-measurement = không có cả FTM lẫn TM với điểm truy cập
wireless-reason-signaling = không có bản tin Signaling gPTP-capable từ điểm truy cập
wireless-reason-other = lý do mà profile không nêu tên
wireless-servo = Sai số servo
wireless-stations = Các trạm
wireless-station-count = { $count } trạm
wireless-no-ftm = không có FTM
wireless-unserved = Listener không được phục vụ
wireless-frame-budget = Ngân sách khung
wireless-frame-use = { $used } trên { $budget } khung mỗi giây
wireless-stream-frames = Khung luồng
wireless-frames-of = { $readdressed } tới các trạm, { $unmapped } không có Listener, { $dropped } bị hủy, { $restored } từ các trạm
wireless-class-a-allowed = Cho phép
wireless-class-a-not-allowed = Không cho phép
wireless-alarm-not-locked = Thời gian Wi-Fi chưa khóa với điểm truy cập
wireless-alarm-holdover = Thời gian Wi-Fi đang holdover, mất khóa với điểm truy cập
wireless-alarm-unserved = { $count } Listener trên cổng Wi-Fi không được phục vụ, vượt giới hạn unicast

## Log

log-all = Tất cả
log-warnings = Cảnh báo
log-pause = Tạm dừng
log-resume = Tiếp tục
log-clear = Xóa
log-empty = Mọi khung ATDECC mà triib gửi và nhận đều hiện ở đây, mới nhất ở trên cùng.
log-none-match = Không có khung đã lưu nào khớp với bộ lọc.
log-frames = { $count } khung
log-shown-of = { $shown } trên { $all } khung
log-sent = Đã gửi
log-heard = Đã nhận
log-not-decoded = Chưa giải mã
log-warning-short = Trường control_data_length của khung khai báo vượt quá cuối khung { $missing } byte.
log-warning-undecodable = Không giải mã được: { $error }.
log-warning-long-acmp = Khung ở dạng ACMP dài, dạng mà thực thể Milan không được phép gửi (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Ánh xạ kênh
mapping-inputs = Đầu vào
mapping-outputs = Đầu ra
mapping-port = cổng { $number }
mapping-fixed = cố định
mapping-not-read = Chưa đọc.
mapping-no-clusters = Không có cụm.
mapping-no-streams = Không có luồng âm thanh.
mapping-none = Không có ánh xạ.
mapping-not-mapped = Chưa ánh xạ
mapping-cluster-numbered = Cụm { $index }

## Presets

presets-note = Preset lưu nguồn đồng hồ, tần số lấy mẫu, định dạng luồng, điều khiển và kết nối của từng thực thể. Khi gọi lại, chỉ những gì khác biệt mới bị thay đổi.
presets-none = Chưa lưu preset nào.
presets-connections = { $count } kết nối
presets-recall = Gọi lại
presets-delete = Xóa
presets-no-place = Không có nơi lưu preset: không xác định được thư mục nhà.
presets-undeletable = Không thể xóa { $path }: { $error }.
presets-saved = Đã lưu “{ $name }” với { $count } thực thể.
presets-nothing-differs = Không có gì khác với “{ $name }”.
presets-recalling = Đang gọi lại “{ $name }”: { $count } thay đổi.
presets-missing = { $report } Không có ở đây hoặc chưa đọc: { $missing }.
presets-deleted = Đã xóa “{ $name }”.
presets-host-note = Preset cũng lưu các thực thể của chính máy tính này, và khởi động lại chúng khi gọi lại.
presets-host-endpoints = { $count } trên máy tính này
presets-starting-host = Đang khởi động các thực thể của máy tính này cho “{ $name }”; phần còn lại sẽ tiếp tục khi chúng hoạt động trở lại.

## Controls

control-numbered = Điều khiển { $index }
control-not-shown = Không hiển thị ở đây
control-option = Tùy chọn { $number }

## Network errors

network-permission = triib cần quyền gửi và nhận khung Ethernet thô.
network-needs-npcap = triib cần Npcap để gửi và nhận khung Ethernet thô.
network-npcap-administrators = Npcap chỉ cho phép quản trị viên gửi và nhận khung Ethernet thô. Hãy chạy triib với quyền quản trị viên, hoặc cài lại Npcap mà không chọn tùy chọn chỉ dành cho quản trị viên.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own entities

host-add-entity = Thêm thực thể
host-new-entity = Thực thể máy chủ { $number }
host-failed = Không thể thêm vào máy tính này: { $reason }
host-needs-clock = Các thực thể của chính máy tính này cần giao diện có dây với đồng hồ phần cứng PTP
host-no-ptp4l = ptp4l không phản hồi nên các luồng của máy tính này không thể giữ thời gian gPTP
host-starting = Đang khởi động
host-starting-entity = Đang khởi động { $name }
host-setting-up = Đang thiết lập { $interface } cho AVB
host-no-link = Không có liên kết trên { $interface }
host-starting-ptp4l = Đang khởi động ptp4l
host-no-linuxptp = Chưa cài linuxptp nên các luồng của máy tính này không thể giữ thời gian gPTP
host-waiting-network = Đang chờ mạng
host-elsewhere = triib-endpointd đang chạy trên { $interface } cho một người dùng khác hoặc dưới quyền root, nên các thực thể của máy tính này chạy ở đó, không phải ở đây
host-foreign-mrp = Một chương trình khác khai báo MSRP hoặc MVRP trên { $interface } từ địa chỉ của máy tính này, điều này có thể rút lại những gì các luồng của máy tính này cần
host-alarm-foreign-mrp = Một chương trình khác trên máy tính này khai báo MSRP hoặc MVRP trên { $interface }
host-state = Trạng thái
host-streaming = Đang phát
host-waiting = Đang chờ Listener
host-listening = Đang lắng nghe
host-bound = Đã gắn, đang chờ Talker
host-unbound = Chưa gắn
host-audio-from = Âm thanh từ
host-audio-to = Âm thanh tới
host-channels = Số kênh
host-virtual-device = Thiết bị ảo
host-add-stream = Thêm luồng
host-add-audio-input = Đầu vào âm thanh
host-add-audio-output = Đầu ra âm thanh
host-add-clock-input = Đầu vào đồng hồ media
host-add-clock-output = Đầu ra đồng hồ media
host-remove-stream = Gỡ luồng
host-silence = Im lặng
host-tone = Âm thử
host-nowhere = Không đi đâu
host-default-device = Thiết bị mặc định
host-remove = Gỡ khỏi máy tính này

mac-audio-device = Thiết bị âm thanh
mac-no-audio-device = Không có khi chưa luồng nào được kết nối
mac-entity-note = macOS chạy thực thể này. Khi có luồng của nó được kết nối, macOS tạo thiết bị âm thanh mà ứng dụng dùng để phát và ghi, với số kênh mỗi chiều do cấu hình quy định.
mac-open-audio-midi-setup = Mở Audio MIDI Setup
mac-avb-off = AVB đang tắt trên { $interface }, nên máy Mac này không có thực thể AVB riêng. Hãy bật nó trong Network Device Browser của Audio MIDI Setup.

listen-section = Nghe
listen-start = Nghe
listen-stop = Dừng nghe
listen-source = Nghe
listen-received = Những gì nó nhận
listen-sent = Những gì nó gửi trên luồng { $number } ({ $channels } kênh)
listen-sent-needs-macos = Để nghe những gì máy Mac này gửi cần macOS 14.2 trở lên.
listen-channels = Kênh
listen-channel = Kênh { $number }
listen-channel-pair = Kênh { $first }–{ $last }
listen-output = Đầu ra
listen-default-output = Đầu ra mặc định
listen-gain = Độ lợi
listen-gain-db = { $db } dB
listen-starting = Đang khởi động…
listen-waiting-input = Đang chờ { $device }, thiết bị macOS tạo ra khi có luồng của thực thể được kết nối.
listen-waiting-output = Đang chờ { $device }.
listen-playing = Đang phát, đỉnh { $db } dB
listen-playing-silent = Đang phát, im lặng
