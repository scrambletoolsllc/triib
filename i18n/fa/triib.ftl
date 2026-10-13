## Language

language-name = فارسی

## Common

common-close = بستن
common-more = بیشتر
common-keep-toolbar-shown = نمایش دائمی نوار ابزار
common-auto-hide-toolbar = پنهان‌کردن خودکار نوار ابزار

## Settings

settings-title = تنظیمات
settings-general = عمومی
settings-appearance = ظاهر
settings-language = زبان
settings-language-system = پیش‌فرض سیستم: { $language }
settings-language-note = فیلدهای متنی با زبان ورودی سیستم تایپ می‌شوند.
settings-appearance-system = سیستم
settings-appearance-light = روشن
settings-appearance-dark = تیره
settings-colors = رنگ‌ها
settings-system-accent = استفاده از رنگ تأکیدی سیستم
settings-accent-picked = رنگ‌های triib از رنگ زیر ساخته می‌شوند.
settings-accent-omarchy = از پوسته Omarchy، { $theme }.
settings-accent-desktop = از رنگ تأکیدی میزکار.
settings-accent-none = میزکار رنگ تأکیدی ندارد، پس رنگ زیر به کار می‌رود.
settings-motion = حرکت
settings-animations = پویانمایی‌ها
settings-animations-note = حرکت‌های فنری و لغزشی هنگام تغییرات.
settings-animations-reduced = میزکار حرکت کمتر خواسته است، پس triib ثابت می‌ماند.
settings-outside-control = کنترل از بیرون
settings-allow-outside-control = اجازهٔ کنترل از بیرون
settings-allow-outside-control-note = عامل‌هایی مانند Claude Code می‌توانند از راه triib --mcp شبکه AVB شما را ببینند و تغییر دهند. triib پیش از هر عامل تازه می‌پرسد.
settings-allowed-agents = مجاز: { $agents }
settings-forget-agents = فراموش کردن
settings-entities-file = فایل موجودیت‌ها
settings-entities-file-note = جایی که موجودیت‌های خود این رایانه فهرست شده‌اند. برای استفاده از فایل دیگری Enter را بزنید: فایلی که هنوز وجود ندارد به‌صورت رونوشتی از این فایل آغاز می‌شود و triib-endpointd دوباره با آن راه‌اندازی می‌شود.
settings-entities-file-default = پیش‌فرض
settings-entities-file-full = یک مسیر کامل بدهید، مانند ~/entities.toml.
agent-prompt-title = به { $agent } اجازهٔ کنترل triib داده شود؟
agent-prompt-body = { $agent } می‌خواهد از کنترل از بیرون triib استفاده کند: موجودیت‌های شبکه شما را ببیند و تغییر دهد، جریان‌ها را وصل کند و قالب‌ها، ساعت‌ها و نام‌ها را تنظیم کند. می‌توانید کنترل از بیرون را در تنظیمات خاموش کنید.
agent-prompt-allow = اجازه دادن
agent-prompt-deny = اجازه ندادن

common-cancel = لغو
common-save = ذخیره
common-not-set = تنظیم‌نشده
common-unnamed = بی‌نام
common-none = هیچ
common-mac-address = آدرس MAC
common-list-separator = {"، "}

## Network interfaces

interface-up = فعال
interface-link-down = لینک قطع
interface-wireless = بی‌سیم
interface-hardware-clock = ساعت سخت‌افزاری
interface-hardware-clock-named = ساعت سخت‌افزاری { $clock }
interface-virtual = مجازی

## Toolbar

toolbar-choose-interface = یک واسط انتخاب کنید
toolbar-interface = واسط شبکه
toolbar-filter-interfaces = پالایش واسط‌ها
toolbar-hide-virtual = پنهان‌کردن واسط‌های مجازی
toolbar-hide-wireless = پنهان‌کردن واسط‌های بی‌سیم
toolbar-hide-unclocked = پنهان‌کردن واسط‌های بدون ساعت سخت‌افزاری
toolbar-connections = اتصال‌ها
toolbar-network = شبکه
toolbar-entities = موجودیت‌ها
toolbar-rediscover = درخواست از همه موجودیت‌ها برای اعلام خود
toolbar-rescan = پاک کردن و اسکن دوباره همه موجودیت‌ها
toolbar-search = جستجوی موجودیت‌ها و جریان‌ها
toolbar-presets = پیش‌تنظیم‌ها
toolbar-log = لاگ
toolbar-inspector = بازرس
toolbar-settings = تنظیمات

## The network's state, in place of a view

state-no-interface = بدون واسط
state-no-interface-note = برای کشف موجودیت‌ها، واسط متصل به شبکه AVB را انتخاب کنید.
state-starting = در حال شروع
state-starting-note = در حال باز کردن { $interface }.
state-listening = در حال گوش دادن
state-listening-note = موجودیت‌های روی { $interface } هنگام اعلام خود اینجا نمایش داده می‌شوند.
state-permission-needed = مجوز لازم است
state-npcap-needed = Npcap لازم است
state-get-npcap = دریافت Npcap
state-copy-command = کپی فرمان
state-cannot-use = استفاده از { $interface } ممکن نیست
state-try-again = تلاش دوباره

## Entity list

entities-none-yet = هنوز موجودیتی نیست
entities-none-yet-note = همه موجودیت‌های شبکه، با نقش‌ها، کلاس‌های SR و ساعتشان.

## Inspector

inspector-title = بازرس
inspector-entity = موجودیت
inspector-streams = جریان‌ها
inspector-controls = کنترل‌ها
inspector-diagnostics = عیب‌یابی
inspector-descriptors = توصیفگرها
inspector-select = یک موجودیت را برای دیدن جزئیاتش انتخاب کنید.
inspector-offline = { $entity } آفلاین است.
inspector-rename = تغییر نام
inspector-name = نام
inspector-identify = شناسایی
inspector-model-not-read = مدل موجودیت آن خوانده نشده است.
inspector-no-streams = جریانی نیست.
inspector-no-controls = کنترلی برای نمایش نیست.
inspector-no-diagnostics = هیچ واسط یا شمارنده‌ای گزارش نشده است.
inspector-reading = در حال خواندن توصیفگرها، تاکنون { $count }.
inspector-read-failed = خواندن مدل موجودیت ممکن نشد: { $reason }.

entity-section = موجودیت
entity-name = نام
entity-group = گروه
entity-product = محصول
entity-firmware = میان‌افزار
entity-serial-number = شماره سریال
entity-configuration = پیکربندی
entity-configuration-of = { $name } ({ $number } از { $count })
entity-milan = Milan
entity-media-clock = ساعت رسانه
entity-clock-domain = دامنه ساعت
entity-sampling-rate = نرخ نمونه‌برداری
clock-source-numbered = منبع { $index }
rate-pull = کشش { $pull }

stream-format = قالب
stream-id = ID جریان
stream-destination = مقصد
stream-max-transit = بیشینه زمان عبور

avb-interfaces = واسط‌های AVB
avb-interface = واسط
avb-interface-clock-identity = هویت ساعت
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }، دامنه { $domain }
avb-interface-peer-delay = تأخیر همتا
avb-interface-running = در حال اجرا
avb-interface-none-reported = چیزی گزارش نشده
avb-interface-path = مسیر
avb-interface-own-grandmaster = خودش Grandmaster است
avb-interface-hops = { $count ->
    [one] { $count } گام تا Grandmaster
   *[other] { $count } گام تا Grandmaster
}
avb-interface-link-up = لینک برقرار
avb-interface-link-down = لینک قطع
avb-interface-grandmaster-changes = تغییرات Grandmaster
avb-interface-frames-sent = فریم‌های ارسالی
avb-interface-frames-received = فریم‌های دریافتی
avb-interface-crc-errors = خطاهای CRC

tree-firmware = میان‌افزار { $version }
tree-descriptor-types = { $count ->
    [one] { $count } نوع توصیفگر
   *[other] { $count } نوع توصیفگر
}
tree-clock = ساعت
tree-clock-source-from = { $kind }، از { $location } { $index }
tree-clock-domain-using = با استفاده از { $source }
tree-clusters = { $count ->
    [one] { $count } خوشه
   *[other] { $count } خوشه
}
tree-maps = { $count ->
    [one] { $count } نگاشت
   *[other] { $count } نگاشت
}

advert-not-advertised = اعلام‌نشده
advert-identity = هویت
advert-entity-id = ID موجودیت
advert-entity-model = مدل موجودیت
advert-roles = نقش‌ها
advert-talker = Talker
advert-listener = Listener
advert-clock = ساعت
advert-btc = BTC
advert-gptp-domain = دامنه gPTP
advert-sr-classes = کلاس‌های SR
advert-indexes = اندیس‌های مدل موجودیت
advert-identify-control = کنترل شناسایی
advert-avb-interface = واسط AVB
advert-advertising = اعلام
advert-valid-time = مدت اعتبار
advert-available-index = اندیس دسترس‌پذیری
advert-association = وابستگی
advert-capabilities = قابلیت‌ها

## Status bar

status-entities = { $count ->
    [one] { $count } موجودیت
   *[other] { $count } موجودیت
}
status-not-discovering = کشف متوقف است
status-discovering = در حال کشف
status-discovering-as = در حال کشف به‌عنوان { $controller }
status-stopped = با خطا متوقف شد
status-alarm = آلارم
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } و { $count } مورد دیگر

## Descriptions of entities, shared by the views

role-talker = Talker { $count }
role-listener = Listener { $count }
role-controller = کنترلر
role-none = بدون نقش
classes-a-and-b = A و B
clock-no-gptp = بدون gPTP

read-not-read = خوانده‌نشده
read-reading = در حال خواندن، تاکنون { $count }
read-ready-unreadable = آماده، { $count } ناخوانا
read-ready-cached = آماده، از حافظه نهان
read-ready = آماده
read-failed = ناموفق: { $reason }

milan-no = خیر
milan-before-1-3 = پیش از 1.3
milan-certified = { $version }، دارای گواهی { $certification }
milan-not-certified = { $version }، بدون گواهی

outcome-status = وضعیت { $status }
outcome-no-response = بدون پاسخ
outcome-not-possible = ممکن نیست
outcome-connect = وصل کردن { $talker } به { $listener } ممکن نشد: { $reason }.
outcome-disconnect = قطع اتصال { $listener } ممکن نشد: { $reason }.
outcome-identify = شناسایی { $entity } ممکن نشد: { $reason }.
outcome-rename = تغییر نام { $what } به «{ $name }» ممکن نشد: { $reason }.
outcome-rename-group = تغییر نام گروه { $entity } به «{ $name }» ممکن نشد: { $reason }.
outcome-format-streaming = تغییر قالب { $stream } ممکن نشد: در حال ارسال جریان است. ابتدا اتصال آن را قطع کنید.
outcome-format = تغییر قالب { $stream } ممکن نشد: { $reason }.
outcome-max-transit-time = تغییر بیشینه زمان عبور { $stream } ممکن نشد: { $reason }.
outcome-max-transit-time-streaming = تغییر بیشینه زمان عبور { $stream } ممکن نشد: در حال ارسال جریان است. ابتدا اتصال آن را قطع کنید.
outcome-sampling-rate = تغییر نرخ نمونه‌برداری { $entity } ممکن نشد: { $reason }.
outcome-clock-source = تغییر منبع ساعت { $entity } ممکن نشد: { $reason }.
outcome-map = نگاشت کانال روی { $entity } ممکن نشد: { $reason }.
outcome-unmap = حذف نگاشت کانال روی { $entity } ممکن نشد: { $reason }.
outcome-control = تنظیم «{ $control }» روی { $entity } ممکن نشد: { $reason }.
outcome-control-numbered = تنظیم کنترل { $index } روی { $entity } ممکن نشد: { $reason }.

stream-not-connected = متصل نیست
stream-from = از { $stream }
stream-from-receiving = از { $stream }، در حال دریافت
stream-from-waiting = از { $stream }، در انتظار Talker
stream-from-failed = از { $stream }، رزرو Talker ناموفق بود: { $reason }
stream-sending-to = در حال ارسال به { $destination }

failure-no-response = پاسخ نداد
failure-refused = با { $status } رد کرد
failure-malformed = پاسخش رمزگشایی نشد
failure-on-this-computer = روی همین رایانه اجرا می‌شود؛ آن را از رایانهٔ دیگری بخوانید

msrp-failure-1 = پهنای باند کافی نیست
msrp-failure-2 = منابع پل کافی نیست
msrp-failure-3 = پهنای باند برای کلاس ترافیک کافی نیست
msrp-failure-4 = ID جریان را Talker دیگری استفاده می‌کند
msrp-failure-5 = آدرس مقصد از قبل در حال استفاده است
msrp-failure-6 = جریانی با رتبه بالاتر جای آن را گرفت
msrp-failure-7 = تأخیر گزارش‌شده تغییر کرده است
msrp-failure-8 = پورت خروجی از AVB پشتیبانی نمی‌کند
msrp-failure-9 = از آدرس مقصد دیگری استفاده کنید
msrp-failure-10 = منابع MSRP تمام شده است
msrp-failure-11 = منابع MMRP تمام شده است
msrp-failure-12 = ذخیره آدرس مقصد ممکن نیست
msrp-failure-13 = اولویت، اولویت کلاس SR نیست
msrp-failure-14 = فریم‌ها برای رسانه انتقال بیش از حد بزرگ‌اند
msrp-failure-15 = پورت به سقف جریان‌های ورودی رسیده است
msrp-failure-16 = مقدار نخست یک جریان ثبت‌شده تغییر کرد
msrp-failure-17 = VLAN روی پورت خروجی مسدود است
msrp-failure-18 = برچسب‌گذاری VLAN روی پورت خروجی غیرفعال است
msrp-failure-19 = ناهمخوانی اولویت کلاس SR
msrp-failure-unknown = علت نامشخص
msrp-failure-at = { $reason }، در پل { $bridge }

## Entity list columns

column-vendor = سازنده
column-model = مدل
column-state = وضعیت
column-entity-model-id = ID مدل موجودیت
column-talker-streams = جریان‌های Talker
column-listener-streams = جریان‌های Listener
column-avb-lite = AVB Lite
column-egress = ترافیک خروجی
column-wireless = بی‌سیم

## Settings file

settings-no-place = جایی برای نگهداری تنظیمات نیست: پوشه خانگی مشخص نیست.
settings-unusable = استفاده از { $path } ممکن نشد: { $error }.
settings-unsaved = ذخیره { $path } ممکن نشد: { $error }.

column-remove = حذف ستون
column-move-left = انتقال به چپ
column-move-right = انتقال به راست
column-add = افزودن ستون
common-percent = { $value }٪

## Network view

netmap-empty = هنوز شبکه‌ای برای نمایش نیست
netmap-empty-note = موجودیت‌ها پس از خوانده شدن و اعلام جایگاهشان در درخت gPTP اینجا نمایش داده می‌شوند.
netmap-focus-clock-path = مسیر ساعت { $name }
netmap-focus-streams = جریان‌های { $name }
netmap-showing = در حال نمایش: { $what }
netmap-devices = { $count ->
    [one] { $count } دستگاه
   *[other] { $count } دستگاه
}
netmap-bridges = { $count ->
    [one] { $count } پل
   *[other] { $count } پل
}
netmap-show-map = نمایش نقشه
netmap-show-details = نمایش جزئیات
stream-numbered = جریان { $index }
netmap-bridge = پل
netmap-access-point = نقطه دسترسی
netmap-device = دستگاه
netmap-this-computer = این رایانه
netmap-connected = متصل
netmap-advertised = اعلام‌شده، بدون Listener آماده
netmap-advertised-off-tree = اعلام‌شده، بدون Listener آماده ({ $listener } روی درخت gPTP نیست)
netmap-failed-at = رزرو در { $bridge } ناموفق بود: { $reason }
netmap-failed = رزرو ناموفق بود: { $reason }
netmap-no-bridge-on = هیچ پلی روی { $interface } شنیده نشد
netmap-cannot-listen-on = گوش دادن به gPTP روی { $interface } ممکن نیست
netmap-on-this-computer = روی این رایانه
netmap-path-not-reported = مسیر گزارش نشده
netmap-gptp-not-reported = gPTP گزارش نشده
netmap-off-tree = خارج از درخت gPTP
netmap-off-ptp = خارج از درخت PTP
netmap-not-lite = خارج از AVB Lite
netmap-lite-not-reported = AVB Lite گزارش نشده
netmap-synced = همگام
netmap-not-synced = ناهمگام
netmap-triib-on = triib روی { $interface }
netmap-through-count = { $count } عبوری
netmap-out = { $count } خروجی
netmap-in = { $count } ورودی
netmap-failed-count = { $count } ناموفق
netmap-advertised-only = فقط اعلام‌شده
netmap-failed-state = ناموفق
netmap-stream-item = { $talker } ← { $listener } · { $state }
netmap-apart-own-grandmaster = روی درخت gPTP نیست: خودش Grandmaster است
netmap-apart-no-path = مسیرش گزارش نشده؛ از Grandmaster { $grandmaster } پیروی می‌کند
netmap-apart-unreported = وضعیت gPTP خود را گزارش نکرده است
netmap-apart-no-neighbor = هیچ پلی روی واسط این رایانه شنیده نشد
netmap-apart-cannot-listen = این رایانه نمی‌تواند روی واسط خود به gPTP گوش دهد
netmap-apart-on-this-computer = روی همین رایانه اجرا می‌شود؛ برای دیدن وضعیت gPTP آن را از رایانهٔ دیگری بخوانید
netmap-apart-not-lite = AVB Lite اجرا نمی‌کند، پس از Grandmaster پیروی نمی‌کند
netmap-apart-lite-unreported = چیزی از AVB Lite گزارش نمی‌کند، پس معلوم نیست پیرو کدام ساعت است
netmap-clock-tree = درخت ساعت
netmap-no-grandmaster = هیچ Grandmaster شنیده نشد
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = نیاز به بررسی
netmap-nodes-below = گره‌های پایین‌دست
netmap-bridges-below = پل‌های پایین‌دست
netmap-clock-path = مسیر ساعت
netmap-hops = گام تا Grandmaster
netmap-link-delay = تأخیر لینک
netmap-bridge-port = پورت پل
netmap-link-drops = قطعی‌های لینک
netmap-synced-to-grandmaster = همگام با Grandmaster
netmap-host-no-gptp = ناهمگام: این رایانه gPTP اجرا نمی‌کند
netmap-link-no-gptp = ناهمگام: gPTP روی لینک آن اجرا نمی‌شود
netmap-ptp-offset-high = ناهمگام: { $offset } از Grandmaster، فراتر از 50 µs مجاز در AVB Lite
netmap-ptp-no-offset = ناهمگام: هیچ آفستی از Grandmaster اندازه نگرفته است
netmap-audio = صدا
netmap-media-clock-streams = جریان‌های ساعت رسانه
netmap-audio-streams = جریان‌های صوتی
netmap-bound = { $count } مقید
netmap-flowing = جاری
netmap-advertised-state = اعلام‌شده
netmap-media-clock-stream = جریان ساعت رسانه
netmap-audio-stream = جریان صوتی
netmap-reaches = می‌رسد تا
netmap-passing-count = { $count ->
    [one] { $count } جریان عبوری
   *[other] { $count } جریان عبوری
}
netmap-through = از طریق
netmap-passing-through = در حال عبور
netmap-sending = در حال ارسال
netmap-receiving = در حال دریافت
netmap-problems = مشکلات
netmap-help-back = برای بازگشت به نمای کلی، روی پس‌زمینه کلیک کنید.
netmap-help-stream = روی یک جریان کلیک کنید تا بازرسی شود، یا برای بازگشت به نمای کلی روی پس‌زمینه کلیک کنید.
netmap-help-ptp = در AVB Lite ساعت به‌صورت سرتاسری از Grandmaster به هر دستگاه می‌رسد، از میان سوئیچ‌هایی که در آن نقشی ندارند، پس هیچ‌کدام نمایش داده نمی‌شوند. یک دستگاه تا زمانی همگام است که در محدودهٔ 50 µs از Grandmaster پیروی کند. روی یک دستگاه یا سیم آن کلیک کنید تا ساعتش بازرسی شود؛ برای پاک کردن انتخاب روی پس‌زمینه کلیک کنید.
netmap-help-clock = ساعت از Grandmaster از طریق هر پل به همه گره‌های درخت می‌رسد. خط خاکستری بریده‌بریده لینکی است که gPTP روی آن اجرا نمی‌شود. روی یک دستگاه یا سیم آن کلیک کنید تا مسیر ساعتش بازرسی شود؛ برای پاک کردن انتخاب روی پس‌زمینه کلیک کنید.
netmap-help-media-clock = فقط جریان‌های ساعت رسانه (CRF)، به همان شیوه صدا رسم شده‌اند: یک سیم برای هر جریان، رنگ‌شده بر اساس Talker. روی یک سیم کلیک کنید تا جریانش بازرسی شود، یا روی یک دستگاه تا جریان‌هایش را ببینید؛ برای پاک کردن انتخاب روی پس‌زمینه کلیک کنید.
netmap-help-audio = هر جریان سیم خودش را دارد که به هر پلی که از آن می‌گذرد وارد و از آن خارج می‌شود. رنگ بر اساس Talker است: هر Talker یک فام دارد و جریان‌هایش سایه‌هایی از آن‌اند. نقطه‌های متحرک یعنی صدا جاری است؛ خط قرمز ثابت یعنی رزرو ناموفق و خط خاکستری ثابت یعنی جریان اعلام‌شده بدون Listener آماده؛ هر دو جایی متوقف می‌شوند که رزرو متوقف می‌شود. دستگاه‌های ستون میانی مستقیم به پل Grandmaster وصل‌اند. روی یک سیم کلیک کنید تا جریانش بازرسی شود، یا روی یک دستگاه تا جریان‌هایش را ببینید؛ برای پاک کردن انتخاب روی پس‌زمینه کلیک کنید.

## Connections

matrix-nothing-shown = جریانی برای نمایش نیست
matrix-nothing-shown-note = برای دیدن جریان‌های بیشتر، جستجو یا فیلترها را تغییر دهید.
matrix-empty = جریانی برای اتصال نیست
matrix-empty-note = جریان‌های Talker و Listener پس از خوانده شدن موجودیت‌هایشان اینجا به هم می‌رسند.
matrix-all-streams = همه جریان‌ها
matrix-connectable-only = پنهان‌کردن موارد غیرقابل اتصال
matrix-none-hidden = همه جریان‌های نمایش‌داده‌شده قابل اتصال‌اند
matrix-hidden = { $count ->
    [one] { $count } جریان پنهان
   *[other] { $count } جریان پنهان
}
matrix-own = خروجی‌های یک موجودیت به ورودی‌های خودش وصل نمی‌شوند.
matrix-working = در حال انجام.
matrix-waiting-change = در انتظار آخرین تغییر این ورودی.
matrix-connected = متصل و در حال دریافت. برای قطع اتصال کلیک کنید.
matrix-bound-waiting = مقید، در انتظار جریان Talker. برای قطع اتصال کلیک کنید.
matrix-bound-failed = مقید، اما رزرو Talker ناموفق بود: { $reason }. برای قطع اتصال کلیک کنید.
matrix-bound-formats-differ = مقید، اما قالب‌ها متفاوت‌اند: Talker { $sent } می‌فرستد و ورودی روی { $set } تنظیم شده است. برای قطع اتصال کلیک کنید.
matrix-formats-match = قالب‌ها یکسان‌اند ({ $format }). برای اتصال کلیک کنید.
matrix-format-must-change = ورودی { $sent } را می‌پذیرد اما روی { $set } تنظیم شده است، پس ممکن است تا تغییر قالبش پخش نکند. برای انتخاب یک قالب کلیک کنید.
matrix-format-input = تغییر ورودی به
matrix-format-both = تغییر هر دو به
matrix-format-talker-busy = Talker برای Listenerهای دیگر پخش می‌کند، پس قالبش تغییر نمی‌کند.
matrix-format-anyway = اتصال بدون تغییر قالب‌ها
matrix-incompatible = ورودی { $sent } را نمی‌پذیرد. روی { $set } تنظیم شده است.
matrix-group-none = متصل نیست. برای وصل کردن تک‌تک جریان‌ها باز کنید.
matrix-group-connected = { $count } متصل. برای دیدن هرکدام باز کنید.
matrix-outputs-expand = { $count ->
    [one] { $count } خروجی جریان. برای باز کردن روی پیکان کلیک کنید و برای بازرسی روی نام.
   *[other] { $count } خروجی جریان. برای باز کردن روی پیکان کلیک کنید و برای بازرسی روی نام.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } خروجی جریان. برای جمع کردن روی پیکان کلیک کنید و برای بازرسی روی نام.
   *[other] { $count } خروجی جریان. برای جمع کردن روی پیکان کلیک کنید و برای بازرسی روی نام.
}
matrix-inputs-expand = { $count ->
    [one] { $count } ورودی جریان. برای باز کردن روی پیکان کلیک کنید و برای بازرسی روی نام.
   *[other] { $count } ورودی جریان. برای باز کردن روی پیکان کلیک کنید و برای بازرسی روی نام.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } ورودی جریان. برای جمع کردن روی پیکان کلیک کنید و برای بازرسی روی نام.
   *[other] { $count } ورودی جریان. برای جمع کردن روی پیکان کلیک کنید و برای بازرسی روی نام.
}
matrix-stream-inspect = { $detail } برای بازرسی { $entity } کلیک کنید.
matrix-point = به یک خانه اشاره کنید
matrix-point-note = تا Talker و Listener آن و همخوانی قالب‌هایشان را ببینید.
matrix-legend-waiting = مقید، در انتظار جریان
matrix-legend-trouble = مقید، مشکلی هست
matrix-legend-open = قابل اتصال
matrix-legend-change = ابتدا قالب ورودی باید تغییر کند
matrix-legend-incompatible = قالب‌ها ناسازگارند
matrix-talker-outputs = خروجی‌های Talker
matrix-listener-inputs = ورودی‌های Listener

common-thousands-separator = {"٬"}

## Diagnostics

diag-since-start = شمارش از زمان راه‌اندازی موجودیت.
diag-stream-input = ورودی جریان
diag-stream-output = خروجی جریان
diag-locked = { $count ->
    [0] قفل برقرار نشده
    [one] یک بار قفل برقرار شده
   *[other] { $number } بار قفل برقرار شده
}
diag-lost-lock = { $count ->
    [0] قفل از دست نرفته
    [one] یک بار قفل از دست رفته
   *[other] { $number } بار قفل از دست رفته
}
diag-frames-in = { $count ->
    [one] { $number } فریم ورودی
   *[other] { $number } فریم ورودی
}
diag-frames-out = { $count ->
    [one] { $number } فریم خروجی
   *[other] { $number } فریم خروجی
}
diag-media-locked = { $count ->
    [0] قفل رسانه برقرار نشده
    [one] یک بار قفل رسانه برقرار شده
   *[other] { $number } بار قفل رسانه برقرار شده
}
diag-lost-media-lock = { $count ->
    [0] قفل رسانه از دست نرفته
    [one] یک بار قفل رسانه از دست رفته
   *[other] { $number } بار قفل رسانه از دست رفته
}
diag-interrupted = { $count ->
    [0] بدون وقفه
    [one] یک بار وقفه
   *[other] { $number } بار وقفه
}
diag-out-of-sequence = { $count ->
    [one] { $number } فریم خارج از ترتیب
   *[other] { $number } فریم خارج از ترتیب
}
diag-media-resets = { $count ->
    [one] { $number } بازنشانی رسانه
   *[other] { $number } بازنشانی رسانه
}
diag-timestamps-uncertain = { $count ->
    [0] بدون مهر زمانی نامطمئن
    [one] یک بار مهر زمانی نامطمئن
   *[other] { $number } بار مهر زمانی نامطمئن
}
diag-no-timestamp = { $count ->
    [one] { $number } فریم بدون مهر زمانی
   *[other] { $number } فریم بدون مهر زمانی
}
diag-unsupported-format = { $count ->
    [one] { $number } فریم با قالب پشتیبانی‌نشده
   *[other] { $number } فریم با قالب پشتیبانی‌نشده
}
diag-late = { $count ->
    [one] { $number } فریم دیررس
   *[other] { $number } فریم دیررس
}
diag-early = { $count ->
    [one] { $number } فریم زودرس
   *[other] { $number } فریم زودرس
}
diag-started = { $count ->
    [0] شروع نشده
    [one] یک بار شروع شده
   *[other] { $number } بار شروع شده
}
diag-stopped = { $count ->
    [0] متوقف نشده
    [one] یک بار متوقف شده
   *[other] { $number } بار متوقف شده
}
diag-reservation-failed = رزرو Talker ناموفق بود: { $reason }
diag-latency = تأخیر انباشته { $microseconds } µs

## AVB Lite

lite-active = فعال
lite-active-untagged = فعال، بدون برچسب
lite-active-vlan = فعال، VLAN { $vlan }
lite-capable = پشتیبانی‌شده
lite-mode = حالت
lite-mode-capable = AVB (با پشتیبانی از AVB Lite)
lite-because = علت
lite-fallback-none = علتی ذکر نشده
lite-fallback-endpoint = اعلان نقطه پایانی دیگری رسید، پس هیچ پل AVB بین آن‌ها نیست
lite-fallback-unanswered = نُه درخواست تأخیر همتا بی‌پاسخ ماند
lite-fallback-responders = دو یا چند دستگاه به یک درخواست تأخیر همتا پاسخ دادند، پس سوئیچ پل AVB نیست
lite-fallback-configured = اپراتور یا یک کنترلر آن را تنظیم کرد
lite-fallback-other = علتی که پروفایل نام نمی‌برد
lite-other-profile = پروفایل دیگر
lite-ptp-domain = { $profile }، دامنه { $domain }
lite-offset = آفست
lite-offset-from = { $offset } از { $grandmaster }
lite-media-vlan = VLAN رسانه
lite-untagged = بدون برچسب
lite-unicast = تک‌پخشی
lite-fanout = { $count ->
    [one] تا { $count } Listener برای هر جریان، سپس چندپخشی
   *[other] تا { $count } Listener برای هر جریان، سپس چندپخشی
}
lite-link = لینک
lite-bandwidth = پهنای باند
lite-egress-of = { $used } از { $link }، { $share }
lite-egress-of-assumed = { $used } از { $link }، { $share }، با فرض لینک گیگابیتی
lite-egress-reported = بر اساس شمارش خود موجودیت از جریان‌های پذیرفته‌شده‌اش.
lite-egress-worked-out = محاسبه‌شده از قالب‌های خروجی‌های جریان متصل آن.
lite-alarm-offset = آفست PTP { $offset }، فراتر از 50 µs مجاز در AVB Lite
lite-alarm-egress = ترافیک خروجی در { $share } از لینک، فراتر از { $limit } مجاز برای جریان‌ها

## AVB Wireless

wireless-station = ایستگاه
wireless-access-point = نقطه دسترسی
wireless-role = نقش
wireless-mode = حالت
wireless-time = زمان
wireless-mode-a-ftm = Mode A، 802.1AS روی FTM
wireless-mode-a-tm = Mode A، 802.1AS روی TM
wireless-mode-b = Mode B، از فریم‌های بیکن
wireless-no-time = بدون زمان
wireless-other-mode = حالتی که پروفایل نام نمی‌برد
wireless-locked = قفل‌شده
wireless-holdover = در حالت هولداور
wireless-not-locked = قفل‌نشده
wireless-row-locked = ایستگاه، قفل‌شده
wireless-row-holdover = ایستگاه، در حالت هولداور
wireless-row-not-locked = ایستگاه، قفل‌نشده
wireless-row-access-point = { $count ->
    [one] نقطه دسترسی، { $count } ایستگاه
   *[other] نقطه دسترسی، { $count } ایستگاه
}
wireless-link = لینک
wireless-channel = کانال { $channel }
wireless-not-known = نامشخص
wireless-signal = قدرت سیگنال
wireless-rate = نرخ ارسال
wireless-ftm-valid = { $share } معتبر
wireless-rtt = زمان رفت‌وبرگشت { $rtt }
wireless-bursts = { $count ->
    [one] برست‌های { $count } فریمی
   *[other] برست‌های { $count } فریمی
}
wireless-not-as-capable = FALSE، { $reason }
wireless-reason-bursts = نقطه دسترسی برست‌های FTM با تعداد فریمی غیر از سه یا دو تخصیص می‌دهد
wireless-reason-measurement = نه FTM و نه TM با نقطه دسترسی
wireless-reason-signaling = پیام Signaling از نوع gPTP-capable از نقطه دسترسی نمی‌رسد
wireless-reason-other = علتی که پروفایل نام نمی‌برد
wireless-servo = خطای سروو
wireless-stations = ایستگاه‌ها
wireless-station-count = { $count ->
    [one] { $count } ایستگاه
   *[other] { $count } ایستگاه
}
wireless-no-ftm = بدون FTM
wireless-unserved = Listenerهای بدون سرویس
wireless-frame-budget = بودجه فریم
wireless-frame-use = { $used } از { $budget } فریم در ثانیه
wireless-stream-frames = فریم‌های جریان
wireless-frames-of = { $readdressed } به ایستگاه‌ها، { $unmapped } بدون Listener، { $dropped } دورریخته‌شده، { $restored } از ایستگاه‌ها
wireless-class-a-allowed = مجاز
wireless-class-a-not-allowed = غیرمجاز
wireless-alarm-not-locked = زمان Wi-Fi روی نقطه دسترسی قفل نشده
wireless-alarm-holdover = زمان Wi-Fi در حالت هولداور، قفل روی نقطه دسترسی از دست رفته
wireless-alarm-unserved = { $count ->
    [one] { $count } Listener روی پورت Wi-Fi بدون سرویس، فراتر از حد تک‌پخشی
   *[other] { $count } Listener روی پورت Wi-Fi بدون سرویس، فراتر از حد تک‌پخشی
}

## Log

log-all = همه
log-warnings = هشدارها
log-pause = مکث
log-resume = ادامه
log-clear = پاک کردن
log-empty = هر فریم ATDECC که triib می‌فرستد یا می‌شنود، از جدیدترین، اینجا نمایش داده می‌شود.
log-none-match = هیچ فریم نگهداری‌شده‌ای با فیلتر مطابقت ندارد.
log-frames = { $count ->
    [one] { $count } فریم
   *[other] { $count } فریم
}
log-shown-of = { $shown } از { $all } فریم
log-sent = ارسالی
log-heard = شنیده‌شده
log-not-decoded = رمزگشایی‌نشده
log-warning-short = control_data_length آن { $missing } بایت فراتر از انتهای فریم را ادعا می‌کند.
log-warning-undecodable = رمزگشایی نمی‌شود: { $error }.
log-warning-long-acmp = در قالب بلند ACMP است که موجودیت Milan مجاز به ارسال آن نیست (Milan 1.3، 5.5.2.2).

## Channel mappings

mapping-section = نگاشت کانال‌ها
mapping-inputs = ورودی‌ها
mapping-outputs = خروجی‌ها
mapping-port = پورت { $number }
mapping-fixed = ثابت
mapping-not-read = هنوز خوانده نشده.
mapping-no-clusters = خوشه‌ای نیست.
mapping-no-streams = جریان صوتی‌ای نیست.
mapping-none = نگاشتی نیست.
mapping-not-mapped = نگاشت‌نشده
mapping-cluster-numbered = خوشه { $index }

## Presets

presets-note = پیش‌تنظیم، منابع ساعت، نرخ‌های نمونه‌برداری، قالب‌های جریان، کنترل‌ها و اتصال‌های هر موجودیت را نگه می‌دارد. فراخوانی آن هر آنچه را متفاوت است تغییر می‌دهد.
presets-none = هنوز پیش‌تنظیمی ذخیره نشده.
presets-connections = { $count ->
    [one] { $count } اتصال
   *[other] { $count } اتصال
}
presets-recall = فراخوانی
presets-delete = حذف
presets-no-place = جایی برای نگهداری پیش‌تنظیم‌ها نیست: پوشه خانگی مشخص نیست.
presets-undeletable = حذف { $path } ممکن نشد: { $error }.
presets-saved = { $count ->
    [one] «{ $name }» با { $count } موجودیت ذخیره شد.
   *[other] «{ $name }» با { $count } موجودیت ذخیره شد.
}
presets-nothing-differs = هیچ چیز با «{ $name }» تفاوت ندارد.
presets-recalling = { $count ->
    [one] در حال فراخوانی «{ $name }»: { $count } تغییر.
   *[other] در حال فراخوانی «{ $name }»: { $count } تغییر.
}
presets-missing = { $report } حاضر نیست یا خوانده نشده: { $missing }.
presets-deleted = «{ $name }» حذف شد.
presets-host-note = همچنین موجودیت‌های خود این رایانه را نگه می‌دارد و هنگام فراخوانی دوباره راه‌اندازی‌شان می‌کند.
presets-host-endpoints = { $count } روی این رایانه
presets-starting-host = در حال راه‌اندازی موجودیت‌های این رایانه برای «{ $name }»؛ بقیه پس از بازگشت آن‌ها انجام می‌شود.

## Controls

control-numbered = کنترل { $index }
control-not-shown = اینجا نمایش داده نمی‌شود
control-option = گزینه { $number }

## Network errors

network-permission = triib برای ارسال و دریافت فریم‌های خام اترنت به مجوز نیاز دارد.
network-needs-npcap = triib برای ارسال و دریافت فریم‌های خام اترنت به Npcap نیاز دارد.
network-npcap-administrators = Npcap فقط به مدیران اجازهٔ ارسال و دریافت فریم‌های خام اترنت را می‌دهد. triib را به‌عنوان مدیر اجرا کنید، یا Npcap را بدون گزینهٔ «فقط مدیران» دوباره نصب کنید.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {"٫"}

## This computer's own entities

host-add-entity = افزودن موجودیت
host-show-mine = فقط موجودیت‌های خود این رایانه نمایش داده شود
host-show-all = نمایش همه موجودیت‌ها
host-new-entity = موجودیت میزبان { $number }
host-failed = افزودن به این رایانه ممکن نشد: { $reason }
host-needs-clock = موجودیت‌های خود این رایانه به یک واسط سیمی با ساعت سخت‌افزاری PTP نیاز دارند
host-no-ptp4l = ptp4l پاسخ نمی‌دهد، پس جریان‌های این رایانه نمی‌توانند زمان gPTP را نگه دارند
host-elsewhere = triib-endpointd روی { $interface } برای کاربر دیگری یا به‌عنوان root اجرا می‌شود، پس موجودیت‌های این رایانه آنجا اجرا می‌شوند، نه اینجا
host-foreign-mrp = برنامهٔ دیگری از آدرس این رایانه روی { $interface } اعلان‌های MSRP یا MVRP می‌فرستد و ممکن است آنچه را که جریان‌های این رایانه لازم دارند پس بگیرد
host-alarm-foreign-mrp = برنامهٔ دیگری در این رایانه روی { $interface } اعلان‌های MSRP یا MVRP می‌فرستد
host-state = وضعیت
host-streaming = در حال پخش
host-waiting = در انتظار یک Listener
host-listening = در حال گوش دادن
host-bound = پیوند خورده، در انتظار Talker
host-unbound = بدون پیوند
host-audio-from = صدا از
host-audio-to = صدا به
host-channels = کانال‌ها
host-virtual-device = دستگاه مجازی
host-add-stream = افزودن جریان
host-add-audio-input = ورودی صدا
host-add-audio-output = خروجی صدا
host-add-clock-input = ورودی ساعت رسانه
host-add-clock-output = خروجی ساعت رسانه
host-remove-stream = حذف جریان
host-silence = سکوت
host-tone = صدای آزمایشی
host-nowhere = هیچ‌جا
host-default-device = دستگاه پیش‌فرض
host-remove = حذف از این رایانه
