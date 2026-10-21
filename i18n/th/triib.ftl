## Language

language-name = ไทย

## Common

common-close = ปิด
common-more = เพิ่มเติม
common-keep-toolbar-shown = แสดงแถบเครื่องมือตลอด
common-auto-hide-toolbar = ซ่อนแถบเครื่องมืออัตโนมัติ

## Settings

settings-title = การตั้งค่า
settings-general = ทั่วไป
settings-appearance = รูปลักษณ์
settings-language = ภาษา
settings-language-system = ค่าเริ่มต้นของระบบ: { $language }
settings-language-note = ช่องข้อความใช้ภาษาป้อนข้อมูลของระบบ
settings-appearance-system = ระบบ
settings-appearance-light = สว่าง
settings-appearance-dark = มืด
settings-colors = สี
settings-system-accent = ใช้สีเน้นของระบบ
settings-accent-picked = สีด้านล่างเป็นต้นแบบของสีใน triib
settings-accent-omarchy = จากธีม Omarchy: { $theme }
settings-accent-desktop = จากสีเน้นของเดสก์ท็อป
settings-accent-none = เดสก์ท็อปไม่มีสีเน้น จึงใช้สีด้านล่าง
settings-motion = การเคลื่อนไหว
settings-animations = ภาพเคลื่อนไหว
settings-animations-note = เด้งและเลื่อนเมื่อมีการเปลี่ยนแปลง
settings-animations-reduced = เดสก์ท็อปขอให้ลดการเคลื่อนไหว triib จึงอยู่นิ่ง
settings-outside-control = การควบคุมจากภายนอก
settings-allow-outside-control = อนุญาตการควบคุมจากภายนอก
settings-allow-outside-control-note = เอเจนต์ เช่น Claude Code สามารถดูและแก้ไขเครือข่าย AVB ของคุณได้ผ่าน triib --mcp โดย triib จะถามก่อนทุกครั้งที่มีเอเจนต์ใหม่
settings-allowed-agents = อนุญาตแล้ว: { $agents }
settings-forget-agents = ลืม
settings-storage = ที่จัดเก็บ
settings-storage-presets = โฟลเดอร์พรีเซ็ต
settings-storage-entities = ไฟล์เอนทิตี
settings-storage-apply = นำไปใช้
settings-storage-choose = เลือก…
settings-storage-note = พรีเซ็ตที่เก็บไว้ในตำแหน่งเดิมจะยังอยู่ที่นั่น ย้ายมาหากต้องการใช้ต่อ การตั้งค่าของ triib บันทึกไว้ใน { $file }
settings-storage-entities-note = ไฟล์เอนทิตีที่ยังไม่มีจะเริ่มเป็นสำเนาของไฟล์ที่ใช้อยู่ และ triib-endpointd จะเริ่มใหม่ด้วยไฟล์นั้น
dialog-choose-presets = เลือกโฟลเดอร์พรีเซ็ต
dialog-choose-entities = เลือกโฟลเดอร์ของไฟล์เอนทิตี
settings-full-path = ใช้พาธแบบเต็ม เช่น ~/Documents/triib
settings-path-is-folder = { $path } เป็นโฟลเดอร์ ไม่ใช่ไฟล์
settings-folder-missing = ไม่มีโฟลเดอร์ { $path } สร้างโฟลเดอร์นี้ก่อน หรือเลือกโฟลเดอร์อื่น
settings-path-is-file = { $path } เป็นไฟล์ ไม่ใช่โฟลเดอร์
settings-cannot-write = triib เขียนลงใน { $path } ไม่ได้: { $error }
app-file-dialog-failed = ไม่สามารถแสดงกล่องโต้ตอบไฟล์ได้: { $error }
agent-prompt-title = อนุญาตให้ { $agent } ควบคุม triib หรือไม่
agent-prompt-body = { $agent } ขอใช้การควบคุมจากภายนอกของ triib เพื่อดูและแก้ไขเอนทิตีบนเครือข่ายของคุณ เชื่อมต่อสตรีม และตั้งค่ารูปแบบ คล็อก และชื่อ คุณปิดการควบคุมจากภายนอกได้ในการตั้งค่า
agent-prompt-allow = อนุญาต
agent-prompt-deny = ไม่อนุญาต

common-cancel = ยกเลิก
common-save = บันทึก
common-not-set = ไม่ได้ตั้งค่า
common-unnamed = ไม่มีชื่อ
common-none = ไม่มี
common-mac-address = ที่อยู่ MAC
common-list-separator = {", "}

## Network interfaces

interface-up = ทำงาน
interface-link-down = ลิงก์ขาด
interface-wireless = ไร้สาย
interface-hardware-clock = คล็อกฮาร์ดแวร์
interface-hardware-clock-named = คล็อกฮาร์ดแวร์ { $clock }
interface-virtual = เสมือน

## Toolbar

toolbar-choose-interface = เลือกอินเทอร์เฟซ
toolbar-interface = อินเทอร์เฟซเครือข่าย
toolbar-filter-interfaces = กรองอินเทอร์เฟซ
toolbar-hide-virtual = ซ่อนอินเทอร์เฟซเสมือน
toolbar-hide-wireless = ซ่อนอินเทอร์เฟซไร้สาย
toolbar-hide-unclocked = ซ่อนอินเทอร์เฟซที่ไม่มีคล็อกฮาร์ดแวร์
toolbar-connections = การเชื่อมต่อ
toolbar-network = เครือข่าย
toolbar-entities = เอนทิตี
toolbar-rediscover = ขอให้ทุกเอนทิตีประกาศตัว
toolbar-rescan = ล้างและสแกนเอนทิตีทั้งหมดใหม่
toolbar-search = ค้นหาเอนทิตีและสตรีม
toolbar-presets = พรีเซ็ต
toolbar-log = บันทึกเหตุการณ์
toolbar-inspector = ตัวตรวจสอบ
toolbar-settings = การตั้งค่า
toolbar-close-window = ปิดหน้าต่าง

## The macOS menu bar, named as in macOS's own apps

menu-about = เกี่ยวกับ triib
menu-settings = การตั้งค่า…
menu-services = บริการ
menu-hide = ซ่อน triib
menu-hide-others = ซ่อนแอปอื่น
menu-show-all = แสดงทั้งหมด
menu-quit = ออกจาก triib
menu-file = ไฟล์
menu-presets = พรีเซ็ต…
menu-close-window = ปิดหน้าต่าง
menu-edit = แก้ไข
menu-cut = ตัด
menu-copy = คัดลอก
menu-paste = วาง
menu-select-all = เลือกทั้งหมด
menu-view = มุมมอง
menu-full-screen = เข้าสู่โหมดเต็มหน้าจอ
menu-window = หน้าต่าง
menu-minimize = ย่อ
menu-zoom = ซูม
menu-bring-all-to-front = นำทั้งหมดมาไว้ด้านหน้า
menu-help = วิธีใช้
menu-website = เว็บไซต์ triib
menu-release-notes = บันทึกประจำรุ่น

## The network's state, in place of a view

state-no-interface = ไม่มีอินเทอร์เฟซ
state-no-interface-note = เลือกอินเทอร์เฟซที่อยู่บนเครือข่าย AVB เพื่อค้นหาเอนทิตี
state-starting = กำลังเริ่ม
state-starting-note = กำลังเปิด { $interface }
state-listening = กำลังรับฟัง
state-listening-note = เอนทิตีบน { $interface } จะปรากฏที่นี่เมื่อประกาศตัว
state-permission-needed = ต้องได้รับสิทธิ์
state-npcap-needed = ต้องใช้ Npcap
state-get-npcap = รับ Npcap
state-copy-command = คัดลอกคำสั่ง
state-cannot-use = ใช้ { $interface } ไม่ได้
state-try-again = ลองอีกครั้ง

## Entity list

entities-none-yet = ยังไม่มีเอนทิตี
entities-none-yet-note = ทุกเอนทิตีบนเครือข่าย พร้อมบทบาท คลาส SR และคล็อก

## Inspector

inspector-title = ตัวตรวจสอบ
inspector-entity = เอนทิตี
inspector-streams = สตรีม
inspector-controls = ตัวควบคุม
inspector-diagnostics = การวินิจฉัย
inspector-descriptors = ดีสคริปเตอร์
inspector-select = เลือกเอนทิตีเพื่อดูรายละเอียด
inspector-offline = { $entity } ออฟไลน์อยู่
inspector-heard = ไม่ได้ประกาศตัว triib ได้ยินจากคำสั่งที่ส่งมา และแสดงไว้ในรายการจนกว่าจะผ่านไป 5 นาทีโดยไม่มีคำสั่ง
inspector-rename = เปลี่ยนชื่อ
inspector-name = ชื่อ
inspector-identify = ระบุตัว
inspector-model-not-read = ยังไม่ได้อ่านโมเดลเอนทิตี
inspector-no-streams = ไม่มีสตรีม
inspector-no-controls = ไม่มีตัวควบคุมที่จะแสดง
inspector-no-diagnostics = ไม่มีการรายงานอินเทอร์เฟซหรือตัวนับ
inspector-reading = กำลังอ่านดีสคริปเตอร์ ได้ { $count } รายการแล้ว
inspector-read-failed = อ่านโมเดลเอนทิตีไม่ได้: { $reason }

entity-section = เอนทิตี
entity-name = ชื่อ
entity-triib-controller = คอนโทรลเลอร์ triib
entity-triib-on = triib บน { $host }
entity-triib-here = triib บนคอมพิวเตอร์เครื่องนี้
entity-controller-on = คอนโทรลเลอร์บน { $host }
entity-controller-here = คอนโทรลเลอร์บนคอมพิวเตอร์เครื่องนี้
entity-controller-at = คอนโทรลเลอร์ที่ { $mac }
entity-group = กลุ่ม
entity-product = ผลิตภัณฑ์
entity-firmware = เฟิร์มแวร์
entity-serial-number = หมายเลขซีเรียล
entity-configuration = การกำหนดค่า
entity-configuration-of = { $name } ({ $number } จาก { $count })
entity-configuration-holds = เข้า { $inputs } ออก { $outputs } สตรีม { $channels } ช่อง
entity-configuration-holds-mixed = เข้า { $inputs } ออก { $outputs }
entity-configuration-numbered = การกำหนดค่า { $number }
entity-milan = Milan
entity-media-clock = มีเดียคล็อก
entity-clock-domain = โดเมนคล็อก
entity-sampling-rate = อัตราสุ่มตัวอย่าง
clock-source-numbered = แหล่ง { $index }
rate-pull = pull { $pull }

stream-format = รูปแบบ
stream-id = ID สตรีม
stream-destination = ปลายทาง
stream-max-transit = เวลาส่งผ่านสูงสุด
stream-listeners = ผู้รับฟัง
stream-listener-gone = { $entity } อินพุต { $input } ออฟไลน์
stream-disconnect-listener = ตัดการเชื่อมต่อ

avb-interfaces = อินเทอร์เฟซ AVB
avb-interface = อินเทอร์เฟซ
avb-interface-clock-identity = รหัสประจำตัวคล็อก
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, โดเมน { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = ที่ทำงานอยู่
avb-interface-none-reported = ไม่มีการรายงาน
avb-interface-path = เส้นทาง
avb-interface-own-grandmaster = เป็น Grandmaster เอง
avb-interface-hops = ห่างจาก Grandmaster { $count } ฮอป
avb-interface-link-up = ลิงก์ทำงาน
avb-interface-link-down = ลิงก์ขาด
avb-interface-grandmaster-changes = การเปลี่ยน Grandmaster
avb-interface-frames-sent = เฟรมที่ส่ง
avb-interface-frames-received = เฟรมที่รับ
avb-interface-crc-errors = ข้อผิดพลาด CRC

tree-firmware = เฟิร์มแวร์ { $version }
tree-descriptor-types = ดีสคริปเตอร์ { $count } ประเภท
tree-clock = คล็อก
tree-clock-source-from = { $kind } จาก { $location } { $index }
tree-clock-domain-using = ใช้ { $source }
tree-clusters = คลัสเตอร์ { $count } ชุด
tree-maps = แมป { $count } รายการ

advert-not-advertised = ไม่ได้ประกาศ
advert-identity = ข้อมูลระบุตัว
advert-entity-id = ID เอนทิตี
advert-entity-model = โมเดลเอนทิตี
advert-roles = บทบาท
advert-talker = Talker
advert-listener = Listener
advert-clock = คล็อก
advert-gptp-domain = โดเมน gPTP
advert-sr-classes = คลาส SR
advert-indexes = ดัชนีในโมเดลเอนทิตี
advert-identify-control = ตัวควบคุมการระบุตัว
advert-avb-interface = อินเทอร์เฟซ AVB
advert-advertising = การประกาศ
advert-valid-time = ระยะเวลาที่มีผล
advert-available-index = ดัชนีความพร้อมใช้งาน
advert-association = การเชื่อมโยง
advert-capabilities = ความสามารถ

## Status bar

status-entities = { $count } เอนทิตี
status-not-discovering = ไม่ได้ค้นหา
status-discovering = กำลังค้นหา
status-discovering-as = กำลังค้นหาในนาม { $controller }
status-stopped = หยุดเพราะเกิดข้อผิดพลาด
status-alarm = สัญญาณเตือน
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } และอีก { $count } รายการ

## Descriptions of entities, shared by the views

role-talker = Talker { $count }
role-listener = Listener { $count }
role-controller = คอนโทรลเลอร์
role-none = ไม่มีบทบาท
classes-a-and-b = A และ B
clock-no-gptp = ไม่มี gPTP

read-not-read = ยังไม่ได้อ่าน
read-reading = กำลังอ่าน ได้ { $count } แล้ว
read-ready-unreadable = พร้อม อ่านไม่ได้ { $count } รายการ
read-ready-cached = พร้อม จากแคช
read-ready = พร้อม
read-failed = ล้มเหลว: { $reason }

milan-no = ไม่
milan-before-1-3 = ก่อน 1.3
milan-certified = { $version }, ได้รับการรับรอง { $certification }
milan-not-certified = { $version }, ไม่ได้รับการรับรอง

outcome-status = สถานะ { $status }
outcome-no-response = ไม่มีการตอบกลับ
outcome-not-possible = ทำไม่ได้
outcome-connect = เชื่อมต่อ { $talker } กับ { $listener } ไม่ได้: { $reason }
outcome-disconnect = ตัดการเชื่อมต่อ { $listener } ไม่ได้: { $reason }
outcome-disconnect-listener = ตัดการเชื่อมต่อ { $listener } จาก { $talker } ไม่ได้: { $reason }
outcome-identify = ระบุตัว { $entity } ไม่ได้: { $reason }
outcome-rename = เปลี่ยนชื่อ { $what } เป็น “{ $name }” ไม่ได้: { $reason }
outcome-rename-group = เปลี่ยนชื่อกลุ่มของ { $entity } เป็น “{ $name }” ไม่ได้: { $reason }
outcome-format-streaming = เปลี่ยนรูปแบบของ { $stream } ไม่ได้: สตรีมกำลังส่งอยู่ ให้ตัดการเชื่อมต่อก่อน
outcome-format = เปลี่ยนรูปแบบของ { $stream } ไม่ได้: { $reason }
outcome-max-transit-time = เปลี่ยนเวลาส่งผ่านสูงสุดของ { $stream } ไม่ได้: { $reason }
outcome-max-transit-time-streaming = เปลี่ยนเวลาส่งผ่านสูงสุดของ { $stream } ไม่ได้: สตรีมกำลังส่งอยู่ ให้ตัดการเชื่อมต่อก่อน
outcome-sampling-rate = เปลี่ยนอัตราสุ่มตัวอย่างของ { $entity } ไม่ได้: { $reason }
outcome-clock-source = เปลี่ยนแหล่งคล็อกของ { $entity } ไม่ได้: { $reason }
outcome-configuration = เปลี่ยนการกำหนดค่าของ { $entity } ไม่ได้: { $reason }
outcome-configuration-streaming = { $entity } คงการกำหนดค่าไว้ขณะที่สตรีมของมันทำงานอยู่ ให้ตัดการเชื่อมต่อก่อน
outcome-map = แมปช่องสัญญาณบน { $entity } ไม่ได้: { $reason }
outcome-unmap = ยกเลิกการแมปช่องสัญญาณบน { $entity } ไม่ได้: { $reason }
outcome-control = ตั้งค่า “{ $control }” บน { $entity } ไม่ได้: { $reason }
outcome-control-numbered = ตั้งค่าตัวควบคุม { $index } บน { $entity } ไม่ได้: { $reason }

stream-not-connected = ไม่ได้เชื่อมต่อ
stream-from = จาก { $stream }
stream-from-receiving = จาก { $stream } กำลังรับ
stream-from-waiting = จาก { $stream } กำลังรอ Talker
stream-from-failed = จาก { $stream } การจองของ Talker ล้มเหลว: { $reason }
stream-sending-to = กำลังส่งไปยัง { $destination }

failure-no-response = ไม่ตอบสนอง
failure-refused = ปฏิเสธด้วย { $status }
failure-malformed = ถอดรหัสการตอบกลับไม่ได้
failure-on-this-computer = ทำงานบนคอมพิวเตอร์เครื่องนี้ ให้อ่านจากเครื่องอื่น

msrp-failure-1 = แบนด์วิดท์ไม่พอ
msrp-failure-2 = ทรัพยากรของสวิตช์ไม่พอ
msrp-failure-3 = แบนด์วิดท์ไม่พอสำหรับคลาสทราฟฟิกนี้
msrp-failure-4 = ID สตรีมถูก Talker อื่นใช้อยู่
msrp-failure-5 = ที่อยู่ปลายทางถูกใช้อยู่แล้ว
msrp-failure-6 = ถูกสตรีมที่มีลำดับสูงกว่าแย่งสิทธิ์
msrp-failure-7 = ความหน่วงที่รายงานเปลี่ยนไป
msrp-failure-8 = พอร์ตขาออกไม่รองรับ AVB
msrp-failure-9 = ใช้ที่อยู่ปลายทางอื่น
msrp-failure-10 = ทรัพยากร MSRP หมด
msrp-failure-11 = ทรัพยากร MMRP หมด
msrp-failure-12 = จัดเก็บที่อยู่ปลายทางไม่ได้
msrp-failure-13 = ลำดับความสำคัญไม่ใช่ลำดับความสำคัญของคลาส SR
msrp-failure-14 = เฟรมใหญ่เกินไปสำหรับสื่อกลาง
msrp-failure-15 = ถึงขีดจำกัดพอร์ต fan-in แล้ว
msrp-failure-16 = ค่าแรกเปลี่ยนไปสำหรับสตรีมที่ลงทะเบียนแล้ว
msrp-failure-17 = VLAN ถูกบล็อกที่พอร์ตขาออก
msrp-failure-18 = การแท็ก VLAN ถูกปิดที่พอร์ตขาออก
msrp-failure-19 = ลำดับความสำคัญของคลาส SR ไม่ตรงกัน
msrp-failure-unknown = ไม่ทราบสาเหตุ
msrp-failure-at = { $reason } ที่สวิตช์ { $bridge }

## Entity list columns

column-vendor = ผู้ผลิต
column-model = รุ่น
column-state = สถานะ
column-entity-model-id = ID โมเดลเอนทิตี
column-talker-streams = สตรีม Talker
column-listener-streams = สตรีม Listener
column-avb-lite = AVB Lite
column-egress = ขาออก
column-wireless = ไร้สาย

## Settings file

settings-no-place = ไม่มีที่สำหรับเก็บการตั้งค่า: ไม่ทราบโฟลเดอร์โฮม
settings-unusable = ใช้ { $path } ไม่ได้: { $error }
settings-unsaved = บันทึก { $path } ไม่ได้: { $error }

column-remove = ลบคอลัมน์
column-move-left = ย้ายไปทางซ้าย
column-move-right = ย้ายไปทางขวา
column-add = เพิ่มคอลัมน์
common-percent = { $value }%

## Network view

netmap-empty = ยังไม่มีเครือข่ายที่จะแสดง
netmap-empty-note = เอนทิตีจะปรากฏที่นี่เมื่ออ่านแล้วและได้รายงานตำแหน่งในทรี gPTP แล้ว
netmap-focus-clock-path = เส้นทางคล็อกของ { $name }
netmap-focus-streams = สตรีมของ { $name }
netmap-showing = กำลังแสดง{ $what }
netmap-devices = { $count } อุปกรณ์
netmap-bridges = { $count } สวิตช์
netmap-show-map = แสดงแผนที่
netmap-show-details = แสดงรายละเอียด
stream-numbered = สตรีม { $index }
netmap-bridge = สวิตช์
netmap-access-point = แอคเซสพอยต์
netmap-plain-switch = สวิตช์ที่ไม่ใช่ AVB
netmap-device = อุปกรณ์
netmap-this-computer = คอมพิวเตอร์เครื่องนี้
netmap-connected = เชื่อมต่อแล้ว
netmap-advertised = ประกาศแล้ว ยังไม่มี Listener ที่พร้อม
netmap-advertised-off-tree = ประกาศแล้ว ยังไม่มี Listener ที่พร้อม ({ $listener } ไม่อยู่ในทรี gPTP)
netmap-failed-at = การจองล้มเหลวที่ { $bridge }: { $reason }
netmap-failed = การจองล้มเหลว: { $reason }
netmap-no-bridge-on = ไม่พบสวิตช์บน { $interface }
netmap-cannot-listen-on = ไม่สามารถรับฟัง gPTP บน { $interface }
netmap-on-this-computer = บนคอมพิวเตอร์เครื่องนี้
netmap-path-not-reported = ไม่มีการรายงานเส้นทาง
netmap-gptp-not-reported = ไม่มีการรายงาน gPTP
netmap-off-tree = ไม่อยู่ในทรี gPTP
netmap-off-ptp = ไม่อยู่ในทรี PTP
netmap-not-lite = ไม่อยู่ใน AVB Lite
netmap-lite-not-reported = ไม่มีการรายงาน AVB Lite
netmap-domain = โดเมน gPTP { $domain }
netmap-lite-only = AVB Lite เท่านั้น
netmap-lite-set = AVB Lite, ตั้งค่าด้วยตนเอง
netmap-synced = ซิงค์แล้ว
netmap-not-synced = ไม่ได้ซิงค์
netmap-triib-on = triib บน { $interface }
netmap-through-count = ผ่าน { $count }
netmap-out = ออก { $count }
netmap-in = เข้า { $count }
netmap-failed-count = ล้มเหลว { $count }
netmap-advertised-only = ประกาศเท่านั้น
netmap-failed-state = ล้มเหลว
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-sending-unheard = กำลังส่ง ไม่มี Listener ที่รู้จักผูกอยู่
netmap-no-listener = ไม่มี Listener ที่รู้จัก
netmap-apart-own-grandmaster = ไม่อยู่ในทรี gPTP: เป็น Grandmaster เอง
netmap-apart-no-path = ไม่ได้รายงานเส้นทาง แต่ซิงค์ตาม Grandmaster { $grandmaster }
netmap-apart-unreported = ยังไม่ได้รายงานสถานะ gPTP
netmap-apart-no-neighbor = ไม่พบสวิตช์บนอินเทอร์เฟซของคอมพิวเตอร์เครื่องนี้
netmap-apart-cannot-listen = คอมพิวเตอร์เครื่องนี้ไม่สามารถรับฟัง gPTP บนอินเทอร์เฟซของตน
netmap-apart-on-this-computer = ทำงานบนคอมพิวเตอร์เครื่องนี้ ให้อ่านจากเครื่องอื่นเพื่อดูสถานะ gPTP
netmap-apart-not-lite = ไม่ได้รัน AVB Lite จึงไม่ได้ซิงค์ตาม Grandmaster
netmap-apart-lite-unreported = ไม่ได้รายงานข้อมูล AVB Lite เลย จึงไม่ทราบว่าซิงค์ตามอะไร
netmap-apart-other-domain = รัน gPTP บนโดเมน { $domain } แต่ทรีอยู่บนโดเมน { $network } จึงไม่ได้ซิงค์ตาม Grandmaster ของทรี
netmap-apart-lite-only = รันเฉพาะ AVB Lite โดยไม่มีส่วนร่วมใน gPTP หรือ SRP สวิตช์ AVB จึงไม่จองสิ่งใดไว้ให้สตรีมของอุปกรณ์นี้
netmap-apart-lite-set = รัน AVB Lite เพราะผู้ควบคุมระบบหรือคอนโทรลเลอร์ตั้งค่าไว้ โดยไม่มีส่วนร่วมใน gPTP หรือ SRP
netmap-switch-state = ไม่มีส่วนร่วมใน AVB: แทนสวิตช์ที่อยู่หน้าอุปกรณ์ด้านล่าง ซึ่งไม่พบสวิตช์ AVB จึงเปลี่ยนไปใช้ AVB Lite และซิงค์ตาม Grandmaster PTP ตัวเดียวกัน
netmap-clock-tree = ทรีคล็อก
netmap-no-grandmaster = ไม่พบ Grandmaster
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = ต้องตรวจสอบ
netmap-nodes-below = โหนดด้านล่าง
netmap-bridges-below = สวิตช์ด้านล่าง
netmap-clock-path = เส้นทางคล็อก
netmap-hops = ฮอปจาก Grandmaster
netmap-link-delay = ความหน่วงของลิงก์
netmap-bridge-port = พอร์ตสวิตช์
netmap-link-drops = ลิงก์หลุด
netmap-synced-to-grandmaster = ซิงค์กับ Grandmaster แล้ว
netmap-host-no-gptp = ไม่ได้ซิงค์: คอมพิวเตอร์เครื่องนี้ไม่ได้รัน gPTP
netmap-link-no-gptp = ไม่ได้ซิงค์: ลิงก์นี้ไม่ได้รัน gPTP
netmap-ptp-offset-high = ไม่ได้ซิงค์: ออฟเซ็ต { $offset } จาก Grandmaster เกิน 50 µs ที่ AVB Lite อนุญาต
netmap-ptp-no-offset = ไม่ได้ซิงค์: ไม่มีออฟเซ็ตที่วัดได้จาก Grandmaster
netmap-audio = เสียง
netmap-media-clock-streams = สตรีมมีเดียคล็อก
netmap-audio-streams = สตรีมเสียง
netmap-bound = ผูกแล้ว { $count }
netmap-flowing = กำลังไหล
netmap-advertised-state = ประกาศแล้ว
netmap-media-clock-stream = สตรีมมีเดียคล็อก
netmap-audio-stream = สตรีมเสียง
netmap-reaches = ไปถึง
netmap-passing-count = สตรีมที่ผ่าน { $count } รายการ
netmap-through = ผ่าน
netmap-passing-through = สตรีมที่ผ่าน
netmap-sending = กำลังส่ง
netmap-receiving = กำลังรับ
netmap-problems = ปัญหา
netmap-help-back = คลิกพื้นหลังเพื่อกลับไปยังภาพรวม
netmap-help-stream = คลิกสตรีมเพื่อตรวจสอบ หรือคลิกพื้นหลังเพื่อกลับไปยังภาพรวม
netmap-help-ptp = ใน AVB Lite คล็อกไหลแบบต้นทางถึงปลายทางจาก Grandmaster ไปยังทุกอุปกรณ์ ผ่านสวิตช์ที่ไม่ได้มีส่วนร่วม จึงไม่แสดงสวิตช์เหล่านั้น อุปกรณ์จะถือว่าซิงค์แล้วตราบที่ตาม Grandmaster ได้ภายใน 50 µs คลิกอุปกรณ์หรือสายของอุปกรณ์เพื่อตรวจสอบคล็อก คลิกพื้นหลังเพื่อล้างการเลือก
netmap-help-clock = คล็อกไหลจาก Grandmaster ผ่านสวิตช์แต่ละตัวไปยังทุกโหนดในทรี เส้นสีเทาแบบประคือลิงก์ที่ไม่ได้รัน gPTP คลิกอุปกรณ์หรือสายของอุปกรณ์เพื่อตรวจสอบเส้นทางคล็อก คลิกพื้นหลังเพื่อล้างการเลือก
netmap-help-lite = อุปกรณ์ที่ไม่พบสวิตช์ AVB จึงเปลี่ยนไปใช้ AVB Lite จะห้อยอยู่ใต้การ์ดเส้นประที่แทนสวิตช์ที่อยู่หน้าอุปกรณ์เหล่านั้น หนึ่งใบต่อ Grandmaster PTP แต่ละตัวที่ซิงค์ตาม ส่วนอุปกรณ์ที่รันเฉพาะ AVB Lite หรือถูกตั้งค่าให้ใช้ และอุปกรณ์ในโดเมน gPTP อื่น จะอยู่ในพื้นที่ของตนเอง
netmap-help-media-clock = เฉพาะสตรีมมีเดียคล็อก (CRF) วาดแบบเดียวกับเสียง: หนึ่งสายต่อเฟรมของแต่ละสตรีมบนแต่ละลิงก์ แยกสีตาม Talker คลิกสายเพื่อตรวจสอบสตรีม หรือคลิกอุปกรณ์เพื่อดูสตรีมของอุปกรณ์ คลิกพื้นหลังเพื่อล้างการเลือก
netmap-help-audio = แต่ละสายคือเฟรมของสตรีมหนึ่งบนลิงก์หนึ่ง ซึ่งเข้าและออกจากทุกสวิตช์ที่ผ่าน สตรีมมัลติคาสต์มีสายเดียวตราบที่เส้นทางไปยัง Listener ของสตรีมนั้นยังไปด้วยกัน และแตกกิ่งตรงจุดที่เส้นทางแยกจากกัน ส่วนสตรีมยูนิคาสต์ซึ่ง AVB Lite ส่งได้ มีสายของตัวเองจาก Talker ไปยัง Listener แต่ละตัว สีแยกตาม Talker: Talker แต่ละตัวมีสีหลักหนึ่งสี และแต่ละสตรีมของ Talker นั้นเป็นเฉดหนึ่งของสีนั้นในทุกกิ่ง จุดที่เคลื่อนที่หมายถึงมีเสียงไหลอยู่ แม้จาก Talker ที่ส่งโดยไม่มี Listener ที่รู้จัก เส้นสีแดงนิ่งคือการจองที่ล้มเหลว และเส้นสีเทานิ่งคือสตรีมที่ประกาศแล้วแต่ยังไม่มี Listener ที่พร้อม ทั้งสองแบบหยุดตรงจุดที่การจองหยุด อุปกรณ์ในคอลัมน์กลางเชื่อมต่อตรงกับสวิตช์ของ Grandmaster คลิกสายเพื่อตรวจสอบสตรีม คลิกสายที่ใช้ร่วมกันเพื่อดู Listener ทั้งหมดของสาย หรือคลิกอุปกรณ์เพื่อดูสตรีมของอุปกรณ์ คลิกพื้นหลังเพื่อล้างการเลือก

## Connections

matrix-nothing-shown = ไม่มีสตรีมที่จะแสดง
matrix-nothing-shown-note = เปลี่ยนคำค้นหาหรือตัวกรองเพื่อดูสตรีมเพิ่มเติม
matrix-empty = ไม่มีสตรีมที่จะเชื่อมต่อ
matrix-empty-note = สตรีม Talker และสตรีม Listener จะมาพบกันที่นี่เมื่ออ่านเอนทิตีที่มีสตรีมเหล่านั้นแล้ว
matrix-all-streams = สตรีมทั้งหมด
matrix-connectable-only = ซ่อนสิ่งที่เชื่อมต่อไม่ได้
matrix-none-hidden = ทุกสตรีมที่แสดงเชื่อมต่อได้
matrix-hidden = ซ่อนสตรีม { $count } รายการ
matrix-own = เอาต์พุตของเอนทิตีเชื่อมต่อกับอินพุตของตัวเองไม่ได้
matrix-working = กำลังดำเนินการ
matrix-waiting-change = กำลังรอการเปลี่ยนแปลงล่าสุดของอินพุตนี้
matrix-connected = เชื่อมต่อแล้วและกำลังรับ คลิกเพื่อตัดการเชื่อมต่อ
matrix-bound-waiting = ผูกแล้ว กำลังรอสตรีมของ Talker คลิกเพื่อตัดการเชื่อมต่อ
matrix-bound-failed = ผูกแล้ว แต่การจองของ Talker ล้มเหลว: { $reason } คลิกเพื่อตัดการเชื่อมต่อ
matrix-bound-formats-differ = ผูกแล้ว แต่รูปแบบไม่ตรงกัน: Talker ส่ง { $sent } แต่อินพุตตั้งไว้ที่ { $set } คลิกเพื่อตัดการเชื่อมต่อ
matrix-formats-match = รูปแบบตรงกัน ({ $format }) คลิกเพื่อเชื่อมต่อ
matrix-format-must-change = อินพุตรับ { $sent } ได้ แต่ตั้งไว้ที่ { $set } จึงอาจไม่มีเสียงจนกว่าจะเปลี่ยนรูปแบบ คลิกเพื่อเลือกรูปแบบ
matrix-format-input = เปลี่ยนอินพุตเป็น
matrix-format-both = เปลี่ยนทั้งสองเป็น
matrix-format-talker-busy = Talker กำลังสตรีมไปยัง Listener อื่น จึงคงรูปแบบเดิมไว้
matrix-format-anyway = เชื่อมต่อโดยไม่เปลี่ยนรูปแบบ
matrix-incompatible = อินพุตไม่รับ { $sent } ขณะนี้ตั้งไว้ที่ { $set }
matrix-group-none = ไม่ได้เชื่อมต่อ ขยายเพื่อเชื่อมต่อสตรีมทีละรายการ
matrix-group-connected = เชื่อมต่อแล้ว { $count } รายการ ขยายเพื่อดูแต่ละรายการ
matrix-outputs-expand = เอาต์พุตสตรีม { $count } รายการ คลิกลูกศรเพื่อขยาย คลิกชื่อเพื่อตรวจสอบ
matrix-outputs-collapse = เอาต์พุตสตรีม { $count } รายการ คลิกลูกศรเพื่อยุบ คลิกชื่อเพื่อตรวจสอบ
matrix-inputs-expand = อินพุตสตรีม { $count } รายการ คลิกลูกศรเพื่อขยาย คลิกชื่อเพื่อตรวจสอบ
matrix-inputs-collapse = อินพุตสตรีม { $count } รายการ คลิกลูกศรเพื่อยุบ คลิกชื่อเพื่อตรวจสอบ
matrix-stream-inspect = { $detail } คลิกเพื่อตรวจสอบ { $entity }
matrix-point = ชี้ที่เซลล์
matrix-point-note = เพื่อดู Talker และ Listener ของเซลล์นั้น และดูว่ารูปแบบตรงกันหรือไม่
matrix-legend-waiting = ผูกแล้ว กำลังรอสตรีม
matrix-legend-trouble = ผูกแล้ว มีบางอย่างผิดปกติ
matrix-legend-open = เชื่อมต่อได้
matrix-legend-change = ต้องเปลี่ยนรูปแบบอินพุตก่อน
matrix-legend-incompatible = รูปแบบเข้ากันไม่ได้
matrix-talker-outputs = เอาต์พุต Talker
matrix-listener-inputs = อินพุต Listener

common-thousands-separator = {","}
common-decimal-separator = {"."}

## Diagnostics

diag-since-start = นับตั้งแต่เอนทิตีเริ่มทำงาน
diag-stream-input = อินพุตสตรีม
diag-stream-output = เอาต์พุตสตรีม
diag-locked = { $count ->
    [0] ไม่เคยล็อก
   *[other] ล็อก { $number } ครั้ง
}
diag-lost-lock = { $count ->
    [0] ไม่เคยหลุดล็อก
   *[other] หลุดล็อก { $number } ครั้ง
}
diag-frames-in = รับ { $number } เฟรม
diag-frames-out = ส่ง { $number } เฟรม
diag-media-locked = { $count ->
    [0] ไม่เคยล็อกมีเดีย
   *[other] ล็อกมีเดีย { $number } ครั้ง
}
diag-lost-media-lock = { $count ->
    [0] ไม่เคยหลุดล็อกมีเดีย
   *[other] หลุดล็อกมีเดีย { $number } ครั้ง
}
diag-interrupted = { $count ->
    [0] ไม่เคยหยุดชะงัก
   *[other] หยุดชะงัก { $number } ครั้ง
}
diag-out-of-sequence = เฟรมผิดลำดับ { $number } เฟรม
diag-media-resets = รีเซ็ตมีเดีย { $number } ครั้ง
diag-timestamps-uncertain = { $count ->
    [0] ไทม์สแตมป์แน่นอนตลอด
   *[other] ไทม์สแตมป์ไม่แน่นอน { $number } ครั้ง
}
diag-no-timestamp = เฟรมไม่มีไทม์สแตมป์ { $number } เฟรม
diag-unsupported-format = เฟรมในรูปแบบที่ไม่รองรับ { $number } เฟรม
diag-late = เฟรมมาช้า { $number } เฟรม
diag-early = เฟรมมาเร็ว { $number } เฟรม
diag-started = { $count ->
    [0] ไม่เคยเริ่ม
   *[other] เริ่ม { $number } ครั้ง
}
diag-stopped = { $count ->
    [0] ไม่เคยหยุด
   *[other] หยุด { $number } ครั้ง
}
diag-reservation-failed = การจองของ Talker ล้มเหลว: { $reason }
diag-latency = ความหน่วงสะสม { $microseconds } µs

## AVB Lite

lite-active = ใช้งานอยู่
lite-active-untagged = ใช้งานอยู่ ไม่ติดแท็ก
lite-active-vlan = ใช้งานอยู่ บน VLAN { $vlan }
lite-capable = รองรับ
lite-mode = โหมด
lite-mode-capable = AVB (รองรับ AVB Lite)
lite-because = สาเหตุ
lite-fallback-none = ไม่ระบุสาเหตุ
lite-fallback-endpoint = การประกาศของเอนด์พอยต์อื่นผ่านมาถึงได้ จึงไม่มีสวิตช์ AVB คั่นอยู่ระหว่างกัน
lite-fallback-unanswered = คำขอ peer delay เก้าครั้งไม่ได้รับคำตอบ
lite-fallback-responders = มีสองรายขึ้นไปตอบคำขอ peer delay เดียวกัน สวิตช์นั้นจึงไม่ใช่สวิตช์ AVB
lite-fallback-configured = ผู้ควบคุมระบบหรือคอนโทรลเลอร์ตั้งค่าไว้
lite-fallback-other = สาเหตุที่โปรไฟล์ไม่ได้ระบุไว้
lite-other-profile = โปรไฟล์อื่น
lite-ptp-domain = { $profile }, โดเมน { $domain }
lite-offset = ออฟเซ็ต
lite-offset-from = { $offset } จาก { $grandmaster }
lite-media-vlan = VLAN มีเดีย
lite-untagged = ไม่ติดแท็ก
lite-unicast = ยูนิคาสต์
lite-fanout = สูงสุด { $count } Listener ต่อสตรีม จากนั้นเป็นมัลติคาสต์
lite-link = ลิงก์
lite-bandwidth = แบนด์วิดท์
lite-egress-of = { $used } จาก { $link }, { $share }
lite-egress-of-assumed = { $used } จาก { $link }, { $share }, โดยสมมติว่าเป็นลิงก์กิกะบิต
lite-egress-reported = ตามที่เอนทิตีนับสตรีมที่รับเข้าแล้ว
lite-egress-worked-out = คำนวณจากรูปแบบของเอาต์พุตสตรีมที่เชื่อมต่ออยู่
lite-alarm-offset = ออฟเซ็ต PTP { $offset } เกิน 50 µs ที่ AVB Lite อนุญาต
lite-alarm-egress = ขาออก { $share } ของลิงก์ เกิน { $limit } ที่สตรีมใช้ได้

## AVB Wireless

wireless-station = สถานี
wireless-access-point = แอคเซสพอยต์
wireless-role = บทบาท
wireless-mode = โหมด
wireless-time = เวลา
wireless-mode-a-ftm = Mode A, 802.1AS ผ่าน FTM
wireless-mode-a-tm = Mode A, 802.1AS ผ่าน TM
wireless-mode-b = Mode B, จากบีคอน
wireless-no-time = ไม่ได้รับเวลา
wireless-other-mode = โหมดที่โปรไฟล์ไม่ได้ระบุไว้
wireless-locked = ล็อกแล้ว
wireless-holdover = อยู่ใน holdover
wireless-not-locked = ไม่ได้ล็อก
wireless-row-locked = สถานี ล็อกแล้ว
wireless-row-holdover = สถานี อยู่ใน holdover
wireless-row-not-locked = สถานี ไม่ได้ล็อก
wireless-row-access-point = แอคเซสพอยต์ มี { $count } สถานี
wireless-link = ลิงก์
wireless-channel = ช่องสัญญาณ { $channel }
wireless-not-known = ไม่ทราบ
wireless-signal = สัญญาณ
wireless-rate = อัตราส่ง
wireless-ftm-valid = ใช้ได้ { $share }
wireless-rtt = เวลาไป-กลับ { $rtt }
wireless-bursts = เบิร์สต์ละ { $count } เฟรม
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = แอคเซสพอยต์ให้เบิร์สต์ FTM ที่มีจำนวนเฟรมอื่นนอกจากสามหรือสอง
wireless-reason-measurement = ไม่มีทั้ง FTM และ TM กับแอคเซสพอยต์
wireless-reason-signaling = ไม่ได้รับ Signaling แบบ gPTP-capable จากแอคเซสพอยต์
wireless-reason-other = สาเหตุที่โปรไฟล์ไม่ได้ระบุไว้
wireless-servo = ค่าผิดพลาดเซอร์โว
wireless-stations = สถานี
wireless-station-count = { $count } สถานี
wireless-no-ftm = ไม่มี FTM
wireless-unserved = Listener ที่ไม่ได้รับบริการ
wireless-frame-budget = งบประมาณเฟรม
wireless-frame-use = { $used } จาก { $budget } เฟรมต่อวินาที
wireless-stream-frames = เฟรมสตรีม
wireless-frames-of = ส่งถึงสถานี { $readdressed }, ไม่มี Listener { $unmapped }, ถูกทิ้ง { $dropped }, จากสถานี { $restored }
wireless-class-a-allowed = อนุญาต
wireless-class-a-not-allowed = ไม่อนุญาต
wireless-alarm-not-locked = เวลา Wi-Fi ไม่ได้ล็อกกับแอคเซสพอยต์
wireless-alarm-holdover = เวลา Wi-Fi อยู่ใน holdover หลุดล็อกจากแอคเซสพอยต์
wireless-alarm-unserved = Listener { $count } ตัวบนพอร์ต Wi-Fi ไม่ได้รับบริการ เพราะเกินขีดจำกัดยูนิคาสต์

## Log

log-all = ทั้งหมด
log-warnings = คำเตือน
log-pause = หยุดชั่วคราว
log-resume = ทำต่อ
log-clear = ล้าง
log-empty = ทุกเฟรม ATDECC ที่ triib ส่งและได้ยินจะปรากฏที่นี่ โดยเรียงใหม่สุดไว้ก่อน
log-none-match = ไม่มีเฟรมที่เก็บไว้ตรงกับตัวกรอง
log-frames = { $count } เฟรม
log-shown-of = { $shown } จาก { $all } เฟรม
log-sent = ส่ง
log-heard = ได้ยิน
log-not-decoded = ไม่ได้ถอดรหัส
log-warning-short = control_data_length ของเฟรมระบุความยาวเกินท้ายเฟรม { $missing } ออกเต็ต
log-warning-undecodable = ถอดรหัสไม่ได้: { $error }
log-warning-long-acmp = เฟรมนี้อยู่ในรูปแบบ ACMP แบบยาว ซึ่งเอนทิตี Milan ห้ามส่ง (Milan 1.3, 5.5.2.2)

## Channel mappings

mapping-section = การแมปช่องสัญญาณ
mapping-inputs = อินพุต
mapping-outputs = เอาต์พุต
mapping-port = พอร์ต { $number }
mapping-fixed = คงที่
mapping-not-read = ยังไม่ได้อ่าน
mapping-no-clusters = ไม่มีคลัสเตอร์
mapping-no-streams = ไม่มีสตรีมเสียง
mapping-none = ไม่มีการแมป
mapping-not-mapped = ไม่ได้แมป
mapping-cluster-numbered = คลัสเตอร์ { $index }

## Presets

presets-note = พรีเซ็ตเก็บแหล่งคล็อก อัตราสุ่มตัวอย่าง รูปแบบสตรีม ตัวควบคุม และการเชื่อมต่อของแต่ละเอนทิตี การเรียกคืนจะเปลี่ยนเฉพาะสิ่งที่ต่างไป
presets-none = ยังไม่มีพรีเซ็ตที่บันทึกไว้
presets-connections = { $count } การเชื่อมต่อ
presets-recall = เรียกคืน
presets-delete = ลบ
presets-no-place = ไม่มีที่สำหรับเก็บพรีเซ็ต: ไม่ทราบโฟลเดอร์โฮม
presets-undeletable = ลบ { $path } ไม่ได้: { $error }
presets-saved = บันทึก “{ $name }” พร้อม { $count } เอนทิตีแล้ว
presets-nothing-differs = ไม่มีสิ่งใดต่างจาก “{ $name }”
presets-recalling = กำลังเรียกคืน “{ $name }”: เปลี่ยน { $count } รายการ
presets-missing = { $report } ไม่อยู่ที่นี่หรือยังไม่ได้อ่าน: { $missing }
presets-deleted = ลบ “{ $name }” แล้ว
presets-host-note = และยังเก็บเอนทิตีของคอมพิวเตอร์เครื่องนี้ไว้ด้วย โดยจะเริ่มใหม่เมื่อเรียกคืน
presets-host-endpoints = { $count } รายการบนคอมพิวเตอร์เครื่องนี้
presets-starting-host = กำลังเริ่มเอนทิตีของคอมพิวเตอร์เครื่องนี้สำหรับ “{ $name }” ส่วนที่เหลือจะตามมาเมื่อกลับมาแล้ว

## Controls

control-numbered = ตัวควบคุม { $index }
control-not-shown = ไม่แสดงที่นี่
control-option = ตัวเลือก { $number }

## Network errors

network-permission = triib ต้องได้รับสิทธิ์ในการส่งและรับเฟรม Ethernet แบบดิบ
network-needs-npcap = triib ต้องใช้ Npcap ในการส่งและรับเฟรม Ethernet แบบดิบ
network-npcap-administrators = Npcap อนุญาตให้เฉพาะผู้ดูแลระบบส่งและรับเฟรม Ethernet แบบดิบ เรียกใช้ triib ในฐานะผู้ดูแลระบบ หรือติดตั้ง Npcap ใหม่โดยไม่เลือกตัวเลือกสำหรับผู้ดูแลระบบเท่านั้น

matrix-stream-format = { $format }
matrix-stream-format-state = { $format } { $state }

## This computer's own entities

host-add-entity = เพิ่มเอนทิตี
host-show-mine = แสดงเฉพาะเอนทิตีของคอมพิวเตอร์เครื่องนี้
host-show-all = แสดงเอนทิตีทั้งหมด
host-new-entity = เอนทิตีของโฮสต์ { $number }
host-failed = ไม่สามารถเพิ่มลงในคอมพิวเตอร์เครื่องนี้: { $reason }
host-needs-clock = เอนทิตีของคอมพิวเตอร์เครื่องนี้ต้องใช้อินเทอร์เฟซแบบมีสายที่มีคล็อกฮาร์ดแวร์ PTP
host-no-ptp4l = ptp4l ไม่ตอบ สตรีมของคอมพิวเตอร์เครื่องนี้จึงรักษาเวลา gPTP ไม่ได้
host-elsewhere = triib-endpointd ทำงานบน { $interface } ในนามผู้ใช้อื่นหรือ root เอนทิตีของคอมพิวเตอร์เครื่องนี้จึงทำงานที่นั่น ไม่ใช่ที่นี่
host-foreign-mrp = โปรแกรมอื่นประกาศ MSRP หรือ MVRP บน { $interface } จากที่อยู่ของคอมพิวเตอร์เครื่องนี้ ซึ่งอาจเพิกถอนสิ่งที่สตรีมของคอมพิวเตอร์เครื่องนี้ต้องใช้
host-alarm-foreign-mrp = โปรแกรมอื่นบนคอมพิวเตอร์เครื่องนี้ประกาศ MSRP หรือ MVRP บน { $interface }
host-state = สถานะ
host-streaming = กำลังสตรีม
host-waiting = รอ Listener
host-listening = กำลังรับฟัง
host-bound = ผูกแล้ว รอ Talker
host-unbound = ไม่ได้ผูก
host-audio-from = เสียงจาก
host-audio-to = เสียงไปยัง
host-channels = จำนวนช่องสัญญาณ
host-virtual-device = อุปกรณ์เสมือน
host-add-stream = เพิ่มสตรีม
host-add-audio-input = อินพุตเสียง
host-add-audio-output = เอาต์พุตเสียง
host-add-clock-input = อินพุตมีเดียคล็อก
host-add-clock-output = เอาต์พุตมีเดียคล็อก
host-remove-stream = เอาสตรีมออก
host-silence = เงียบ
host-tone = เสียงทดสอบ
host-nowhere = ไม่ส่งไปไหน
host-default-device = อุปกรณ์เริ่มต้น
host-remove = นำออกจากคอมพิวเตอร์เครื่องนี้

mac-audio-device = อุปกรณ์เสียง
mac-no-audio-device = ไม่มีขณะที่ไม่มีสตรีมเชื่อมต่อ
mac-entity-note = macOS เป็นผู้รันเอนทิตีนี้ ขณะที่สตรีมใดสตรีมหนึ่งของมันเชื่อมต่ออยู่ macOS จะสร้างอุปกรณ์เสียงที่แอปใช้เล่นและบันทึก โดยมีจำนวนช่องในแต่ละทิศทางตามที่การกำหนดค่ากำหนด
mac-open-audio-midi-setup = เปิด Audio MIDI Setup
mac-avb-off = AVB ปิดอยู่บน { $interface } Mac เครื่องนี้จึงไม่มีเอนทิตี AVB ของตัวเอง เปิดได้ใน Network Device Browser ของ Audio MIDI Setup

listen-section = ฟัง
listen-start = ฟัง
listen-stop = หยุดฟัง
listen-source = ฟัง
listen-received = สิ่งที่รับเข้ามา
listen-sent = สิ่งที่ส่งออกบนสตรีม { $number } ({ $channels } ช่อง)
listen-sent-needs-macos = การฟังสิ่งที่ Mac เครื่องนี้ส่งออกต้องใช้ macOS 14.2 ขึ้นไป
listen-channels = ช่องสัญญาณ
listen-channel = ช่อง { $number }
listen-channel-pair = ช่อง { $first }–{ $last }
listen-output = เอาต์พุต
listen-default-output = เอาต์พุตเริ่มต้น
listen-gain = เกน
listen-gain-db = { $db } dB
listen-starting = กำลังเริ่ม…
listen-waiting-input = กำลังรอ { $device } ซึ่ง macOS สร้างขึ้นขณะที่สตรีมใดสตรีมหนึ่งของเอนทิตีเชื่อมต่ออยู่
listen-waiting-output = กำลังรอ { $device }
listen-playing = กำลังเล่น จุดสูงสุด { $db } dB
listen-playing-silent = กำลังเล่น เงียบ
