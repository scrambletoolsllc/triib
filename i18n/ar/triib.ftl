## Language

language-name = العربية

## Common

common-close = إغلاق
common-more = المزيد
common-keep-toolbar-shown = إبقاء شريط الأدوات ظاهرًا
common-auto-hide-toolbar = إخفاء شريط الأدوات تلقائيًا

## Settings

settings-title = الإعدادات
settings-general = عام
settings-appearance = المظهر
settings-language = اللغة
settings-language-system = افتراضي النظام: { $language }
settings-language-note = تُكتب حقول النص بلغة الإدخال في النظام.
settings-appearance-system = النظام
settings-appearance-light = فاتح
settings-appearance-dark = داكن
settings-colors = الألوان
settings-system-accent = استخدام لون التمييز في النظام
settings-accent-picked = تُشتق ألوان triib من اللون أدناه.
settings-accent-omarchy = من سمة Omarchy، { $theme }.
settings-accent-desktop = من لون التمييز في سطح المكتب.
settings-accent-none = ليس لسطح المكتب لون تمييز، لذا يُستخدم اللون أدناه.
settings-motion = الحركة
settings-animations = الرسوم المتحركة
settings-animations-note = حركات نابضة وانزلاقية مع تغيّر العناصر.
settings-animations-reduced = يطلب سطح المكتب تقليل الحركة، لذا يبقى triib ساكنًا.
settings-outside-control = التحكم الخارجي
settings-allow-outside-control = السماح بالتحكم الخارجي
settings-allow-outside-control-note = يمكن لوكلاء مثل Claude Code رؤية شبكة AVB لديك وتغييرها عبر triib --mcp. يسأل triib قبل كل وكيل جديد.
settings-allowed-agents = المسموح لهم: { $agents }
settings-forget-agents = نسيان
settings-storage = التخزين
settings-storage-presets = مجلد الإعدادات المسبقة
settings-storage-entities = ملف الكيانات
settings-storage-apply = تطبيق
settings-storage-choose = اختيار…
settings-storage-note = تبقى الإعدادات المسبقة المحفوظة في المكان القديم هناك؛ انقلها لمواصلة استخدامها. تُحفظ إعدادات triib في { $file }.
settings-storage-entities-note = ملف الكيانات غير الموجود بعد يبدأ نسخةً من الملف المستخدم، ويبدأ triib-endpointd من جديد عليه.
dialog-choose-presets = اختر مجلد الإعدادات المسبقة
dialog-choose-entities = اختر مجلد ملف الكيانات
settings-full-path = استخدم مسارًا كاملًا، مثل ~/Documents/triib.
settings-path-is-folder = { $path } مجلد وليس ملفًا.
settings-folder-missing = لا يوجد مجلد باسم { $path }. أنشئه أولًا، أو اختر مجلدًا آخر.
settings-path-is-file = { $path } ملف وليس مجلدًا.
settings-cannot-write = لا يمكن لـ triib الكتابة في { $path }: { $error }.
app-file-dialog-failed = تعذّر عرض مربع حوار الملفات: { $error }
agent-prompt-title = هل تسمح لـ { $agent } بالتحكم في triib؟
agent-prompt-body = يطلب { $agent } استخدام التحكم الخارجي في triib: لرؤية الكيانات على شبكتك وتغييرها، بتوصيل التدفقات وضبط التنسيقات والساعات والأسماء. يمكنك إيقاف التحكم الخارجي من الإعدادات.
agent-prompt-allow = سماح
agent-prompt-deny = عدم السماح

common-cancel = إلغاء
common-save = حفظ
common-not-set = غير محدد
common-unnamed = بلا اسم
common-none = لا شيء
common-mac-address = عنوان MAC
common-list-separator = {"، "}

## Network interfaces

interface-up = نشطة
interface-link-down = الوصلة مقطوعة
interface-wireless = لاسلكية
interface-hardware-clock = ساعة عتادية
interface-hardware-clock-named = ساعة عتادية { $clock }
interface-virtual = افتراضية

## Toolbar

toolbar-choose-interface = اختر واجهة
toolbar-interface = واجهة الشبكة
toolbar-filter-interfaces = تصفية الواجهات
toolbar-hide-virtual = إخفاء الواجهات الافتراضية
toolbar-hide-wireless = إخفاء الواجهات اللاسلكية
toolbar-hide-unclocked = إخفاء الواجهات التي بلا ساعة عتادية
toolbar-connections = الاتصالات
toolbar-network = الشبكة
toolbar-entities = الكيانات
toolbar-rediscover = اطلب من كل كيان أن يعلن عن نفسه
toolbar-rescan = امسح كل الكيانات وأعد فحصها
toolbar-search = البحث في الكيانات والتدفقات
toolbar-presets = الإعدادات المسبقة
toolbar-log = السجل
toolbar-inspector = الفاحص
toolbar-settings = الإعدادات

## The network's state, in place of a view

state-no-interface = لا توجد واجهة
state-no-interface-note = اختر الواجهة المتصلة بشبكة AVB لاكتشاف الكيانات.
state-starting = جارٍ البدء
state-starting-note = جارٍ فتح { $interface }.
state-listening = جارٍ الاستماع
state-listening-note = تظهر هنا الكيانات الموجودة على { $interface } عندما تعلن عن نفسها.
state-permission-needed = يلزم إذن
state-npcap-needed = يلزم Npcap
state-get-npcap = تنزيل Npcap
state-copy-command = نسخ الأمر
state-cannot-use = تعذّر استخدام { $interface }
state-try-again = إعادة المحاولة

## Entity list

entities-none-yet = لا كيانات بعد
entities-none-yet-note = كل كيان على الشبكة، مع أدواره وفئات SR وساعته.

## Inspector

inspector-title = الفاحص
inspector-entity = الكيان
inspector-streams = التدفقات
inspector-controls = عناصر التحكم
inspector-diagnostics = التشخيص
inspector-descriptors = الواصفات
inspector-select = اختر كيانًا لعرض تفاصيله.
inspector-offline = { $entity } خارج الشبكة.
inspector-heard = لا يعلن عن نفسه: سمعه triib في الأوامر التي أرسلها، ويُبقيه في القائمة حتى تمر 5 دقائق دون أمر منه.
inspector-rename = إعادة تسمية
inspector-name = الاسم
inspector-identify = تعريف
inspector-model-not-read = لم يُقرأ نموذج الكيان الخاص به.
inspector-no-streams = لا تدفقات.
inspector-no-controls = لا عناصر تحكم لعرضها.
inspector-no-diagnostics = لم يُبلَّغ عن واجهات أو عدادات.
inspector-reading = جارٍ قراءة الواصفات، { $count } حتى الآن.
inspector-read-failed = تعذّرت قراءة نموذج الكيان: { $reason }.

entity-section = الكيان
entity-name = الاسم
entity-triib-controller = وحدة تحكم triib
entity-triib-on = triib على { $host }
entity-triib-here = triib على هذا الحاسوب
entity-controller-on = وحدة تحكم على { $host }
entity-controller-here = وحدة تحكم على هذا الحاسوب
entity-controller-at = وحدة تحكم في { $mac }
entity-group = المجموعة
entity-product = المنتج
entity-firmware = البرنامج الثابت
entity-serial-number = الرقم التسلسلي
entity-configuration = التكوين
entity-configuration-of = { $name } ({ $number } من { $count })
entity-configuration-holds = { $inputs } دخل، { $outputs } خرج، تدفقات من { $channels } قنوات
entity-configuration-holds-mixed = { $inputs } دخل، { $outputs } خرج
entity-configuration-numbered = التكوين { $number }
entity-milan = Milan
entity-media-clock = ساعة الوسائط
entity-clock-domain = نطاق الساعة
entity-sampling-rate = معدل أخذ العينات
clock-source-numbered = المصدر { $index }
rate-pull = سحب { $pull }

stream-format = التنسيق
stream-id = ID التدفق
stream-destination = الوجهة
stream-max-transit = أقصى زمن عبور
stream-listeners = المستمعون
stream-listener-gone = { $entity } الدخل { $input }، غير متصل
stream-disconnect-listener = فصل

avb-interfaces = واجهات AVB
avb-interface = الواجهة
avb-interface-clock-identity = هوية الساعة
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }، النطاق { $domain }
avb-interface-peer-delay = تأخير النظير
avb-interface-running = يعمل
avb-interface-none-reported = لم يُبلَّغ عن شيء
avb-interface-path = المسار
avb-interface-own-grandmaster = هو Grandmaster لنفسه
avb-interface-hops = { $count ->
    [zero] على بُعد { $count } قفزة من Grandmaster
    [one] على بُعد قفزة واحدة من Grandmaster
    [two] على بُعد قفزتين من Grandmaster
    [few] على بُعد { $count } قفزات من Grandmaster
    [many] على بُعد { $count } قفزةً من Grandmaster
   *[other] على بُعد { $count } قفزة من Grandmaster
}
avb-interface-link-up = الوصلة تعمل
avb-interface-link-down = الوصلة مقطوعة
avb-interface-grandmaster-changes = تغييرات Grandmaster
avb-interface-frames-sent = الإطارات المرسلة
avb-interface-frames-received = الإطارات المستقبلة
avb-interface-crc-errors = أخطاء CRC

tree-firmware = البرنامج الثابت { $version }
tree-descriptor-types = { $count ->
    [zero] { $count } نوع من الواصفات
    [one] نوع واحد من الواصفات
    [two] نوعان من الواصفات
    [few] { $count } أنواع من الواصفات
    [many] { $count } نوعًا من الواصفات
   *[other] { $count } نوع من الواصفات
}
tree-clock = الساعة
tree-clock-source-from = { $kind }، من { $location } { $index }
tree-clock-domain-using = يستخدم { $source }
tree-clusters = { $count ->
    [zero] { $count } عنقود
    [one] عنقود واحد
    [two] عنقودان
    [few] { $count } عناقيد
    [many] { $count } عنقودًا
   *[other] { $count } عنقود
}
tree-maps = { $count ->
    [zero] { $count } تعيين
    [one] تعيين واحد
    [two] تعيينان
    [few] { $count } تعيينات
    [many] { $count } تعيينًا
   *[other] { $count } تعيين
}

advert-not-advertised = غير مُعلَن
advert-identity = الهوية
advert-entity-id = ID الكيان
advert-entity-model = نموذج الكيان
advert-roles = الأدوار
advert-talker = Talker
advert-listener = Listener
advert-clock = الساعة
advert-btc = BTC
advert-gptp-domain = نطاق gPTP
advert-sr-classes = فئات SR
advert-indexes = فهارس نموذج الكيان
advert-identify-control = عنصر تحكم التعريف
advert-avb-interface = واجهة AVB
advert-advertising = الإعلان
advert-valid-time = مدة الصلاحية
advert-available-index = فهرس التوفر
advert-association = الارتباط
advert-capabilities = القدرات

## Status bar

status-entities = { $count ->
    [zero] { $count } كيان
    [one] كيان واحد
    [two] كيانان
    [few] { $count } كيانات
    [many] { $count } كيانًا
   *[other] { $count } كيان
}
status-not-discovering = الاكتشاف متوقف
status-discovering = جارٍ الاكتشاف
status-discovering-as = جارٍ الاكتشاف بصفة { $controller }
status-stopped = توقف بسبب خطأ
status-alarm = إنذار
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } و{ $count } أخرى

## Descriptions of entities, shared by the views

role-talker = Talker { $count }
role-listener = Listener { $count }
role-controller = وحدة تحكم
role-none = بلا أدوار
classes-a-and-b = A و B
clock-no-gptp = بدون gPTP

read-not-read = لم يُقرأ
read-reading = جارٍ القراءة، { $count } حتى الآن
read-ready-unreadable = جاهز، تعذّرت قراءة { $count }
read-ready-cached = جاهز، من الذاكرة المؤقتة
read-ready = جاهز
read-failed = فشل: { $reason }

milan-no = لا
milan-before-1-3 = قبل 1.3
milan-certified = { $version }، معتمد { $certification }
milan-not-certified = { $version }، غير معتمد

outcome-status = الحالة { $status }
outcome-no-response = لا استجابة
outcome-not-possible = غير ممكن
outcome-connect = تعذّر توصيل { $talker } بـ{ $listener }: { $reason }.
outcome-disconnect = تعذّر فصل { $listener }: { $reason }.
outcome-disconnect-listener = تعذّر فصل { $listener } عن { $talker }: { $reason }.
outcome-identify = تعذّر تعريف { $entity }: { $reason }.
outcome-rename = تعذّرت إعادة تسمية { $what } إلى «{ $name }»: { $reason }.
outcome-rename-group = تعذّرت إعادة تسمية مجموعة { $entity } إلى «{ $name }»: { $reason }.
outcome-format-streaming = تعذّر تغيير تنسيق { $stream }: إنه قيد البث. افصله أولًا.
outcome-format = تعذّر تغيير تنسيق { $stream }: { $reason }.
outcome-max-transit-time = تعذّر تغيير أقصى زمن عبور { $stream }: { $reason }.
outcome-max-transit-time-streaming = تعذّر تغيير أقصى زمن عبور { $stream }: إنه قيد البث. افصله أولًا.
outcome-sampling-rate = تعذّر تغيير معدل أخذ العينات في { $entity }: { $reason }.
outcome-clock-source = تعذّر تغيير مصدر الساعة في { $entity }: { $reason }.
outcome-configuration = تعذّر تغيير تكوين { $entity }: { $reason }.
outcome-configuration-streaming = يحتفظ { $entity } بتكوينه ما دامت تدفقاته قيد البث: افصلها أولًا.
outcome-map = تعذّر تعيين القناة على { $entity }: { $reason }.
outcome-unmap = تعذّر إلغاء تعيين القناة على { $entity }: { $reason }.
outcome-control = تعذّر ضبط «{ $control }» على { $entity }: { $reason }.
outcome-control-numbered = تعذّر ضبط عنصر التحكم { $index } على { $entity }: { $reason }.

stream-not-connected = غير متصل
stream-from = من { $stream }
stream-from-receiving = من { $stream }، قيد الاستقبال
stream-from-waiting = من { $stream }، بانتظار Talker
stream-from-failed = من { $stream }، فشل حجز Talker: { $reason }
stream-sending-to = يُرسل إلى { $destination }

failure-no-response = لم يستجب
failure-refused = رفض بالحالة { $status }
failure-malformed = تعذّر فك ترميز استجابته
failure-on-this-computer = يعمل على هذا الحاسوب؛ اقرأه من حاسوب آخر

msrp-failure-1 = عرض النطاق غير كافٍ
msrp-failure-2 = موارد الجسر غير كافية
msrp-failure-3 = عرض النطاق غير كافٍ لفئة حركة المرور
msrp-failure-4 = ID التدفق يستخدمه Talker آخر
msrp-failure-5 = عنوان الوجهة مستخدم بالفعل
msrp-failure-6 = أُزيح لصالح تدفق ذي رتبة أعلى
msrp-failure-7 = تغيّر زمن الانتقال المُبلَّغ عنه
msrp-failure-8 = منفذ الخروج لا يدعم AVB
msrp-failure-9 = استخدم عنوان وجهة مختلفًا
msrp-failure-10 = نفدت موارد MSRP
msrp-failure-11 = نفدت موارد MMRP
msrp-failure-12 = تعذّر تخزين عنوان الوجهة
msrp-failure-13 = الأولوية ليست أولوية فئة SR
msrp-failure-14 = الإطارات أكبر مما يحتمله الوسط
msrp-failure-15 = بلغ المنفذ حد التدفقات الواردة إليه
msrp-failure-16 = تغيّرت القيمة الأولى لتدفق مسجَّل
msrp-failure-17 = VLAN محظورة على منفذ الخروج
msrp-failure-18 = وسم VLAN معطَّل على منفذ الخروج
msrp-failure-19 = عدم تطابق أولوية فئة SR
msrp-failure-unknown = سبب غير معروف
msrp-failure-at = { $reason }، عند الجسر { $bridge }

## Entity list columns

column-vendor = المورّد
column-model = الطراز
column-state = الحالة
column-entity-model-id = ID نموذج الكيان
column-talker-streams = تدفقات Talker
column-listener-streams = تدفقات Listener
column-avb-lite = AVB Lite
column-egress = الخروج
column-wireless = لاسلكي

## Settings file

settings-no-place = لا يوجد مكان لحفظ الإعدادات: المجلد الرئيسي غير معروف.
settings-unusable = تعذّر استخدام { $path }: { $error }.
settings-unsaved = تعذّر حفظ { $path }: { $error }.

column-remove = إزالة العمود
column-move-left = نقل إلى اليسار
column-move-right = نقل إلى اليمين
column-add = إضافة عمود
common-percent = { $value }%

## Network view

netmap-empty = لا توجد شبكة لعرضها بعد
netmap-empty-note = تظهر الكيانات هنا بعد قراءتها وإبلاغها عن موقعها في شجرة gPTP.
netmap-focus-clock-path = مسار ساعة { $name }
netmap-focus-streams = تدفقات { $name }
netmap-showing = معروض: { $what }
netmap-devices = { $count ->
    [zero] { $count } جهاز
    [one] جهاز واحد
    [two] جهازان
    [few] { $count } أجهزة
    [many] { $count } جهازًا
   *[other] { $count } جهاز
}
netmap-bridges = { $count ->
    [zero] { $count } جسر
    [one] جسر واحد
    [two] جسران
    [few] { $count } جسور
    [many] { $count } جسرًا
   *[other] { $count } جسر
}
netmap-show-map = إظهار الخريطة
netmap-show-details = إظهار التفاصيل
stream-numbered = التدفق { $index }
netmap-bridge = جسر
netmap-access-point = نقطة وصول
netmap-device = جهاز
netmap-this-computer = هذا الحاسوب
netmap-connected = متصل
netmap-advertised = مُعلَن، لا Listener جاهز
netmap-advertised-off-tree = مُعلَن، لا Listener جاهز ({ $listener } ليس على شجرة gPTP)
netmap-failed-at = فشل الحجز عند { $bridge }: { $reason }
netmap-failed = فشل الحجز: { $reason }
netmap-no-bridge-on = لم يُسمع أي جسر على { $interface }
netmap-cannot-listen-on = تعذّر الاستماع إلى gPTP على { $interface }
netmap-on-this-computer = على هذا الحاسوب
netmap-path-not-reported = لم يُبلَّغ عن المسار
netmap-gptp-not-reported = لم يُبلَّغ عن gPTP
netmap-off-tree = خارج شجرة gPTP
netmap-off-ptp = خارج شجرة PTP
netmap-not-lite = خارج AVB Lite
netmap-lite-not-reported = لم يُبلَّغ عن AVB Lite
netmap-synced = متزامن
netmap-not-synced = غير متزامن
netmap-triib-on = triib على { $interface }
netmap-through-count = { $count } عابرة
netmap-out = { $count } صادرة
netmap-in = { $count } واردة
netmap-failed-count = { $count } فاشلة
netmap-advertised-only = مُعلَن فقط
netmap-failed-state = فشل
netmap-stream-item = { $talker } ← { $listener } · { $state }
netmap-apart-own-grandmaster = ليس على شجرة gPTP: إنه Grandmaster لنفسه
netmap-apart-no-path = لم يُبلَّغ عن مساره؛ إنه يتبع Grandmaster { $grandmaster }
netmap-apart-unreported = لم يُبلِّغ عن حالة gPTP الخاصة به
netmap-apart-no-neighbor = لم يُسمع أي جسر على واجهة هذا الحاسوب
netmap-apart-cannot-listen = لا يستطيع هذا الحاسوب الاستماع إلى gPTP على واجهته
netmap-apart-on-this-computer = يعمل على هذا الحاسوب؛ اقرأه من حاسوب آخر لترى حالة gPTP فيه
netmap-apart-not-lite = لا يشغّل AVB Lite، فلا يتبع Grandmaster
netmap-apart-lite-unreported = لا يُبلِّغ عن أي شيء بشأن AVB Lite، فلا يُعرف مَن يتبع
netmap-clock-tree = شجرة الساعة
netmap-no-grandmaster = لم يُسمع أي Grandmaster
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = يحتاج إلى انتباه
netmap-nodes-below = العُقد التابعة
netmap-bridges-below = الجسور التابعة
netmap-clock-path = مسار الساعة
netmap-hops = القفزات من Grandmaster
netmap-link-delay = تأخير الوصلة
netmap-bridge-port = منفذ الجسر
netmap-link-drops = انقطاعات الوصلة
netmap-synced-to-grandmaster = متزامن مع Grandmaster
netmap-host-no-gptp = غير متزامن: هذا الحاسوب لا يشغّل gPTP
netmap-link-no-gptp = غير متزامن: لا يعمل gPTP على وصلته
netmap-ptp-offset-high = غير متزامن: إزاحته { $offset } عن Grandmaster، متجاوزةً حد 50 µs الذي يسمح به AVB Lite
netmap-ptp-no-offset = غير متزامن: لم يقِس أي إزاحة عن Grandmaster
netmap-audio = الصوت
netmap-media-clock-streams = تدفقات ساعة الوسائط
netmap-audio-streams = التدفقات الصوتية
netmap-bound = { $count } مربوطة
netmap-flowing = يتدفق
netmap-advertised-state = مُعلَن
netmap-media-clock-stream = تدفق ساعة الوسائط
netmap-audio-stream = تدفق صوتي
netmap-reaches = يصل حتى
netmap-passing-count = { $count ->
    [zero] { $count } تدفق عابر
    [one] تدفق عابر واحد
    [two] تدفقان عابران
    [few] { $count } تدفقات عابرة
    [many] { $count } تدفقًا عابرًا
   *[other] { $count } تدفق عابر
}
netmap-through = عبر
netmap-passing-through = عابر
netmap-sending = يُرسل
netmap-receiving = يستقبل
netmap-problems = مشكلات
netmap-help-back = انقر على الخلفية للعودة إلى النظرة العامة.
netmap-help-stream = انقر على تدفق لفحصه، أو على الخلفية للعودة إلى النظرة العامة.
netmap-help-ptp = في AVB Lite تتدفق الساعة من طرف إلى طرف، من Grandmaster إلى كل جهاز، عبر محوّلات لا تشارك فيها، فلا يُعرض أيٌّ منها. يكون الجهاز متزامنًا ما دام يتبع Grandmaster في حدود 50 µs. انقر على جهاز أو على سلكه لفحص ساعته؛ وانقر على الخلفية لإلغاء التحديد.
netmap-help-clock = تتدفق الساعة من Grandmaster عبر كل جسر إلى كل عقدة في الشجرة. الخط الرمادي المتقطع وصلة لا يعمل عليها gPTP. انقر على جهاز أو على سلكه لفحص مسار ساعته؛ وانقر على الخلفية لإلغاء التحديد.
netmap-help-media-clock = تدفقات ساعة الوسائط (CRF) فقط، مرسومة بالطريقة نفسها كالصوت: سلك لكل تدفق، ملوّن حسب Talker. انقر على سلك لفحص تدفقه، أو على جهاز لرؤية تدفقاته؛ وانقر على الخلفية لإلغاء التحديد.
netmap-help-audio = لكل تدفق سلكه الخاص، يدخل كل جسر يعبره ويخرج منه. اللون حسب Talker: لكل Talker لون، وتدفقاته درجات منه. النقاط المتحركة تعني أن الصوت يتدفق؛ والخط الأحمر الثابت حجز فاشل، والخط الرمادي الثابت تدفق مُعلَن بلا Listener جاهز؛ وكلاهما يتوقف حيث يتوقف الحجز. الأجهزة في العمود الأوسط متصلة مباشرة بجسر Grandmaster. انقر على سلك لفحص تدفقه، أو على جهاز لرؤية تدفقاته؛ وانقر على الخلفية لإلغاء التحديد.

## Connections

matrix-nothing-shown = لا تدفقات لعرضها
matrix-nothing-shown-note = غيّر البحث أو عوامل التصفية لرؤية مزيد من التدفقات.
matrix-empty = لا تدفقات لتوصيلها
matrix-empty-note = تلتقي هنا تدفقات Talker وتدفقات Listener بعد قراءة الكيانات التي تملكها.
matrix-all-streams = كل التدفقات
matrix-connectable-only = إخفاء ما لا يمكن توصيله
matrix-none-hidden = كل التدفقات المعروضة قابلة للتوصيل
matrix-hidden = { $count ->
    [zero] { $count } تدفق مخفي
    [one] تدفق مخفي واحد
    [two] تدفقان مخفيان
    [few] { $count } تدفقات مخفية
    [many] { $count } تدفقًا مخفيًا
   *[other] { $count } تدفق مخفي
}
matrix-own = لا تتصل مخارج أي كيان بمداخله.
matrix-working = جارٍ التنفيذ.
matrix-waiting-change = بانتظار آخر تغيير على هذا المدخل.
matrix-connected = متصل ويستقبل. انقر للفصل.
matrix-bound-waiting = مربوط، بانتظار تدفق Talker. انقر للفصل.
matrix-bound-failed = مربوط، لكن حجز Talker فشل: { $reason }. انقر للفصل.
matrix-bound-formats-differ = مربوط، لكن التنسيقات مختلفة: يُرسل Talker { $sent }، والمدخل مضبوط على { $set }. انقر للفصل.
matrix-formats-match = التنسيقات متطابقة ({ $format }). انقر للتوصيل.
matrix-format-must-change = يقبل المدخل { $sent } لكنه مضبوط على { $set }، لذا قد لا يُشغّل الصوت حتى يتغير تنسيقه. انقر لاختيار تنسيق.
matrix-format-input = تغيير المدخل إلى
matrix-format-both = تغيير كليهما إلى
matrix-format-talker-busy = يبث الـ Talker إلى Listener آخرين، لذا يبقى تنسيقه كما هو.
matrix-format-anyway = التوصيل دون تغيير التنسيقات
matrix-incompatible = لا يقبل المدخل { $sent }. إنه مضبوط على { $set }.
matrix-group-none = غير متصل. وسّع لتوصيل التدفقات واحدًا تلو الآخر.
matrix-group-connected = المتصلة: { $count }. وسّع لرؤية كل منها.
matrix-outputs-expand = { $count ->
    [zero] { $count } مخرج تدفق. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
    [one] مخرج تدفق واحد. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
    [two] مخرجا تدفق. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
    [few] { $count } مخارج تدفق. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
    [many] { $count } مخرج تدفق. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
   *[other] { $count } مخرج تدفق. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
}
matrix-outputs-collapse = { $count ->
    [zero] { $count } مخرج تدفق. انقر على السهم للطي، وعلى الاسم لفحصه.
    [one] مخرج تدفق واحد. انقر على السهم للطي، وعلى الاسم لفحصه.
    [two] مخرجا تدفق. انقر على السهم للطي، وعلى الاسم لفحصه.
    [few] { $count } مخارج تدفق. انقر على السهم للطي، وعلى الاسم لفحصه.
    [many] { $count } مخرج تدفق. انقر على السهم للطي، وعلى الاسم لفحصه.
   *[other] { $count } مخرج تدفق. انقر على السهم للطي، وعلى الاسم لفحصه.
}
matrix-inputs-expand = { $count ->
    [zero] { $count } مدخل تدفق. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
    [one] مدخل تدفق واحد. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
    [two] مدخلا تدفق. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
    [few] { $count } مداخل تدفق. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
    [many] { $count } مدخل تدفق. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
   *[other] { $count } مدخل تدفق. انقر على السهم للتوسيع، وعلى الاسم لفحصه.
}
matrix-inputs-collapse = { $count ->
    [zero] { $count } مدخل تدفق. انقر على السهم للطي، وعلى الاسم لفحصه.
    [one] مدخل تدفق واحد. انقر على السهم للطي، وعلى الاسم لفحصه.
    [two] مدخلا تدفق. انقر على السهم للطي، وعلى الاسم لفحصه.
    [few] { $count } مداخل تدفق. انقر على السهم للطي، وعلى الاسم لفحصه.
    [many] { $count } مدخل تدفق. انقر على السهم للطي، وعلى الاسم لفحصه.
   *[other] { $count } مدخل تدفق. انقر على السهم للطي، وعلى الاسم لفحصه.
}
matrix-stream-inspect = { $detail } انقر لفحص { $entity }.
matrix-point = أشر إلى خلية
matrix-point-note = لرؤية Talker و Listener الخاصين بها ومعرفة ما إذا كانت تنسيقاتهما متوافقة.
matrix-legend-waiting = مربوط، بانتظار التدفق
matrix-legend-trouble = مربوط، ثمة خلل
matrix-legend-open = يمكن التوصيل
matrix-legend-change = يجب تغيير تنسيق المدخل أولًا
matrix-legend-incompatible = التنسيقات لا تتوافق
matrix-talker-outputs = مخارج Talker
matrix-listener-inputs = مداخل Listener

common-thousands-separator = {","}

## Diagnostics

diag-since-start = محسوبة منذ بدء تشغيل الكيان.
diag-stream-input = مدخل التدفق
diag-stream-output = مخرج التدفق
diag-locked = { $count ->
    [0] لم يتم القفل
    [one] تم القفل مرة واحدة
    [two] تم القفل مرتين
    [few] تم القفل { $number } مرات
    [many] تم القفل { $number } مرة
   *[other] تم القفل { $number } مرة
}
diag-lost-lock = { $count ->
    [0] لم يُفقد القفل
    [one] فُقد القفل مرة واحدة
    [two] فُقد القفل مرتين
    [few] فُقد القفل { $number } مرات
    [many] فُقد القفل { $number } مرة
   *[other] فُقد القفل { $number } مرة
}
diag-frames-in = { $count ->
    [zero] { $number } إطار وارد
    [one] إطار وارد واحد
    [two] إطاران واردان
    [few] { $number } إطارات واردة
    [many] { $number } إطارًا واردًا
   *[other] { $number } إطار وارد
}
diag-frames-out = { $count ->
    [zero] { $number } إطار صادر
    [one] إطار صادر واحد
    [two] إطاران صادران
    [few] { $number } إطارات صادرة
    [many] { $number } إطارًا صادرًا
   *[other] { $number } إطار صادر
}
diag-media-locked = { $count ->
    [0] لم يتم قفل الوسائط
    [one] تم قفل الوسائط مرة واحدة
    [two] تم قفل الوسائط مرتين
    [few] تم قفل الوسائط { $number } مرات
    [many] تم قفل الوسائط { $number } مرة
   *[other] تم قفل الوسائط { $number } مرة
}
diag-lost-media-lock = { $count ->
    [0] لم يُفقد قفل الوسائط
    [one] فُقد قفل الوسائط مرة واحدة
    [two] فُقد قفل الوسائط مرتين
    [few] فُقد قفل الوسائط { $number } مرات
    [many] فُقد قفل الوسائط { $number } مرة
   *[other] فُقد قفل الوسائط { $number } مرة
}
diag-interrupted = { $count ->
    [0] لم ينقطع
    [one] انقطع مرة واحدة
    [two] انقطع مرتين
    [few] انقطع { $number } مرات
    [many] انقطع { $number } مرة
   *[other] انقطع { $number } مرة
}
diag-out-of-sequence = { $count ->
    [zero] { $number } إطار خارج التسلسل
    [one] إطار واحد خارج التسلسل
    [two] إطاران خارج التسلسل
    [few] { $number } إطارات خارج التسلسل
    [many] { $number } إطارًا خارج التسلسل
   *[other] { $number } إطار خارج التسلسل
}
diag-media-resets = { $count ->
    [zero] { $number } إعادة ضبط للوسائط
    [one] { $number } إعادة ضبط للوسائط
    [two] { $number } إعادة ضبط للوسائط
    [few] { $number } عمليات إعادة ضبط للوسائط
    [many] { $number } عملية إعادة ضبط للوسائط
   *[other] { $number } إعادة ضبط للوسائط
}
diag-timestamps-uncertain = { $count ->
    [0] لا طوابع زمنية غير مؤكدة
    [one] طوابع زمنية غير مؤكدة مرة واحدة
    [two] طوابع زمنية غير مؤكدة مرتين
    [few] طوابع زمنية غير مؤكدة { $number } مرات
    [many] طوابع زمنية غير مؤكدة { $number } مرة
   *[other] طوابع زمنية غير مؤكدة { $number } مرة
}
diag-no-timestamp = { $count ->
    [zero] { $number } إطار بلا طابع زمني
    [one] إطار واحد بلا طابع زمني
    [two] إطاران بلا طابع زمني
    [few] { $number } إطارات بلا طابع زمني
    [many] { $number } إطارًا بلا طابع زمني
   *[other] { $number } إطار بلا طابع زمني
}
diag-unsupported-format = { $count ->
    [zero] { $number } إطار بتنسيق غير مدعوم
    [one] إطار واحد بتنسيق غير مدعوم
    [two] إطاران بتنسيق غير مدعوم
    [few] { $number } إطارات بتنسيق غير مدعوم
    [many] { $number } إطارًا بتنسيق غير مدعوم
   *[other] { $number } إطار بتنسيق غير مدعوم
}
diag-late = { $count ->
    [zero] { $number } إطار متأخر
    [one] إطار متأخر واحد
    [two] إطاران متأخران
    [few] { $number } إطارات متأخرة
    [many] { $number } إطارًا متأخرًا
   *[other] { $number } إطار متأخر
}
diag-early = { $count ->
    [zero] { $number } إطار مبكر
    [one] إطار مبكر واحد
    [two] إطاران مبكران
    [few] { $number } إطارات مبكرة
    [many] { $number } إطارًا مبكرًا
   *[other] { $number } إطار مبكر
}
diag-started = { $count ->
    [0] لم يبدأ
    [one] بدأ مرة واحدة
    [two] بدأ مرتين
    [few] بدأ { $number } مرات
    [many] بدأ { $number } مرة
   *[other] بدأ { $number } مرة
}
diag-stopped = { $count ->
    [0] لم يتوقف
    [one] توقف مرة واحدة
    [two] توقف مرتين
    [few] توقف { $number } مرات
    [many] توقف { $number } مرة
   *[other] توقف { $number } مرة
}
diag-reservation-failed = فشل حجز Talker: { $reason }
diag-latency = زمن انتقال متراكم قدره { $microseconds } µs

## AVB Lite

lite-active = نشط
lite-active-untagged = نشط، بلا وسم
lite-active-vlan = نشط، VLAN { $vlan }
lite-capable = مدعوم
lite-mode = الوضع
lite-mode-capable = AVB (يدعم AVB Lite)
lite-because = السبب
lite-fallback-none = لم يُذكر سبب
lite-fallback-endpoint = وصل تصريح نقطة طرفية أخرى، فلا يوجد جسر AVB بينهما
lite-fallback-unanswered = بقيت تسعة طلبات لتأخير النظير بلا رد
lite-fallback-responders = أجاب اثنان أو أكثر عن طلب واحد لتأخير النظير، فالمحوّل ليس جسر AVB
lite-fallback-configured = ضبطه المشغّل أو وحدة تحكم
lite-fallback-other = سبب لا يسمّيه ملف التعريف
lite-other-profile = ملف تعريف آخر
lite-ptp-domain = { $profile }، النطاق { $domain }
lite-offset = الإزاحة
lite-offset-from = { $offset } عن { $grandmaster }
lite-media-vlan = VLAN الوسائط
lite-untagged = بلا وسم
lite-unicast = بث أحادي
lite-fanout = { $count ->
    [zero] حتى { $count } Listener لكل تدفق، ثم بث متعدد
    [one] حتى Listener واحد لكل تدفق، ثم بث متعدد
    [two] حتى { $count } Listener لكل تدفق، ثم بث متعدد
    [few] حتى { $count } Listener لكل تدفق، ثم بث متعدد
    [many] حتى { $count } Listener لكل تدفق، ثم بث متعدد
   *[other] حتى { $count } Listener لكل تدفق، ثم بث متعدد
}
lite-link = الوصلة
lite-bandwidth = عرض النطاق
lite-egress-of = { $used } من { $link }، { $share }
lite-egress-of-assumed = { $used } من { $link }، { $share }، بافتراض وصلة جيجابت
lite-egress-reported = كما يعدّ الكيان تدفقاته المقبولة.
lite-egress-worked-out = محسوب من تنسيقات مخارج التدفق المتصلة.
lite-alarm-offset = إزاحة PTP { $offset }، متجاوزةً حد 50 µs الذي يسمح به AVB Lite
lite-alarm-egress = الخروج عند { $share } من الوصلة، متجاوزًا { $limit } المسموح بها للتدفقات

## AVB Wireless

wireless-station = محطة
wireless-access-point = نقطة وصول
wireless-role = الدور
wireless-mode = الوضع
wireless-time = التوقيت
wireless-mode-a-ftm = Mode A، 802.1AS عبر إطارات FTM
wireless-mode-a-tm = Mode A، 802.1AS عبر إطارات TM
wireless-mode-b = Mode B، من إطارات المنارة
wireless-no-time = بلا توقيت
wireless-other-mode = وضع لا يسمّيه ملف التعريف
wireless-locked = تم القفل
wireless-holdover = في وضع الاحتفاظ
wireless-not-locked = لم يتم القفل
wireless-row-locked = محطة، تم القفل
wireless-row-holdover = محطة، في وضع الاحتفاظ
wireless-row-not-locked = محطة، لم يتم القفل
wireless-row-access-point = { $count ->
    [zero] نقطة وصول، { $count } محطة
    [one] نقطة وصول، محطة واحدة
    [two] نقطة وصول، محطتان
    [few] نقطة وصول، { $count } محطات
    [many] نقطة وصول، { $count } محطةً
   *[other] نقطة وصول، { $count } محطة
}
wireless-link = الوصلة
wireless-channel = القناة { $channel }
wireless-not-known = غير معروفة
wireless-signal = قوة الإشارة
wireless-rate = معدل الإرسال
wireless-ftm-valid = { $share } صالحة
wireless-rtt = زمن الذهاب والإياب { $rtt }
wireless-bursts = { $count ->
    [zero] رشقات من { $count } إطار
    [one] رشقات من إطار واحد
    [two] رشقات من إطارين
    [few] رشقات من { $count } إطارات
    [many] رشقات من { $count } إطارًا
   *[other] رشقات من { $count } إطار
}
wireless-not-as-capable = FALSE، { $reason }
wireless-reason-bursts = تمنح نقطة الوصول رشقات FTM لا تتألف من ثلاثة إطارات أو إطارين
wireless-reason-measurement = لا FTM ولا TM مع نقطة الوصول
wireless-reason-signaling = لا رسائل Signaling من نوع gPTP-capable من نقطة الوصول
wireless-reason-other = سبب لا يسمّيه ملف التعريف
wireless-servo = خطأ المؤازر
wireless-stations = المحطات
wireless-station-count = { $count ->
    [zero] { $count } محطة
    [one] محطة واحدة
    [two] محطتان
    [few] { $count } محطات
    [many] { $count } محطةً
   *[other] { $count } محطة
}
wireless-no-ftm = بلا FTM
wireless-unserved = Listener بلا خدمة
wireless-frame-budget = ميزانية الإطارات
wireless-frame-use = الإطارات في الثانية: { $used } من { $budget }
wireless-stream-frames = إطارات التدفقات
wireless-frames-of = { $readdressed } إلى المحطات، { $unmapped } بلا Listener، { $dropped } أُسقطت، { $restored } من المحطات
wireless-class-a-allowed = مسموح
wireless-class-a-not-allowed = غير مسموح
wireless-alarm-not-locked = لم يتم قفل توقيت Wi-Fi على نقطة الوصول
wireless-alarm-holdover = توقيت Wi-Fi في وضع الاحتفاظ، فُقد القفل على نقطة الوصول
wireless-alarm-unserved = { $count ->
    [zero] { $count } Listener على منفذ Wi-Fi بلا خدمة لتجاوز حد البث الأحادي
    [one] Listener واحد على منفذ Wi-Fi بلا خدمة لتجاوز حد البث الأحادي
    [two] { $count } Listener على منفذ Wi-Fi بلا خدمة لتجاوز حد البث الأحادي
    [few] { $count } Listener على منفذ Wi-Fi بلا خدمة لتجاوز حد البث الأحادي
    [many] { $count } Listener على منفذ Wi-Fi بلا خدمة لتجاوز حد البث الأحادي
   *[other] { $count } Listener على منفذ Wi-Fi بلا خدمة لتجاوز حد البث الأحادي
}

## Log

log-all = الكل
log-warnings = التحذيرات
log-pause = إيقاف مؤقت
log-resume = استئناف
log-clear = مسح
log-empty = يظهر هنا كل إطار ATDECC يرسله triib أو يسمعه، الأحدث أولًا.
log-none-match = لا يطابق عامل التصفية أي إطار محفوظ.
log-frames = { $count ->
    [zero] { $count } إطار
    [one] إطار واحد
    [two] إطاران
    [few] { $count } إطارات
    [many] { $count } إطارًا
   *[other] { $count } إطار
}
log-shown-of = الإطارات: { $shown } من { $all }
log-sent = مُرسَل
log-heard = مسموع
log-not-decoded = لم يُفك ترميزه
log-warning-short = يشير control_data_length فيه إلى { $missing } بايت بعد نهاية الإطار.
log-warning-undecodable = تعذّر فك ترميزه: { $error }.
log-warning-long-acmp = إنه بصيغة ACMP الطويلة، التي لا يجوز لكيان Milan إرسالها (Milan 1.3، 5.5.2.2).

## Channel mappings

mapping-section = تعيين القنوات
mapping-inputs = المداخل
mapping-outputs = المخارج
mapping-port = منفذ { $number }
mapping-fixed = ثابت
mapping-not-read = لم يُقرأ بعد.
mapping-no-clusters = لا عناقيد.
mapping-no-streams = لا تدفقات صوتية.
mapping-none = لا تعيينات.
mapping-not-mapped = غير معيَّن
mapping-cluster-numbered = العنقود { $index }

## Presets

presets-note = يحفظ الإعداد المسبق مصادر الساعة ومعدلات أخذ العينات وتنسيقات التدفقات وعناصر التحكم والاتصالات لكل كيان. استدعاؤه يغيّر ما يختلف.
presets-none = لا إعدادات مسبقة محفوظة بعد.
presets-connections = { $count ->
    [zero] { $count } اتصال
    [one] اتصال واحد
    [two] اتصالان
    [few] { $count } اتصالات
    [many] { $count } اتصالًا
   *[other] { $count } اتصال
}
presets-recall = استدعاء
presets-delete = حذف
presets-no-place = لا يوجد مكان لحفظ الإعدادات المسبقة: المجلد الرئيسي غير معروف.
presets-undeletable = تعذّر حذف { $path }: { $error }.
presets-saved = { $count ->
    [zero] حُفظ «{ $name }» ويشمل { $count } كيان.
    [one] حُفظ «{ $name }» ويشمل كيانًا واحدًا.
    [two] حُفظ «{ $name }» ويشمل كيانين.
    [few] حُفظ «{ $name }» ويشمل { $count } كيانات.
    [many] حُفظ «{ $name }» ويشمل { $count } كيانًا.
   *[other] حُفظ «{ $name }» ويشمل { $count } كيان.
}
presets-nothing-differs = لا شيء يختلف عن «{ $name }».
presets-recalling = { $count ->
    [zero] جارٍ استدعاء «{ $name }»: { $count } تغيير.
    [one] جارٍ استدعاء «{ $name }»: تغيير واحد.
    [two] جارٍ استدعاء «{ $name }»: تغييران.
    [few] جارٍ استدعاء «{ $name }»: { $count } تغييرات.
    [many] جارٍ استدعاء «{ $name }»: { $count } تغييرًا.
   *[other] جارٍ استدعاء «{ $name }»: { $count } تغيير.
}
presets-missing = { $report } غير موجودة أو لم تُقرأ: { $missing }.
presets-deleted = حُذف «{ $name }».
presets-host-note = ويحفظ أيضًا كيانات هذا الحاسوب الخاصة، ويشغّلها من جديد عند الاستدعاء.
presets-host-endpoints = { $count } على هذا الحاسوب
presets-starting-host = جارٍ تشغيل كيانات هذا الحاسوب لـ«{ $name }»؛ ويتبعها الباقي حين تعود.

## Controls

control-numbered = عنصر التحكم { $index }
control-not-shown = غير معروض هنا
control-option = الخيار { $number }

## Network errors

network-permission = يحتاج triib إلى إذن لإرسال إطارات إيثرنت الخام واستقبالها.
network-needs-npcap = يحتاج triib إلى Npcap لإرسال إطارات إيثرنت الخام واستقبالها.
network-npcap-administrators = يسمح Npcap للمسؤولين فقط بإرسال إطارات إيثرنت الخام واستقبالها. شغّل triib كمسؤول، أو ثبّت Npcap مجددًا دون خيار المسؤولين فقط.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {"."}

## This computer's own entities

host-add-entity = إضافة كيان
host-show-mine = عرض كيانات هذا الحاسوب الخاصة فقط
host-show-all = عرض كل الكيانات
host-new-entity = كيان المضيف { $number }
host-failed = تعذّرت إضافته إلى هذا الحاسوب: { $reason }
host-needs-clock = تحتاج كيانات هذا الحاسوب الخاصة إلى واجهة سلكية بساعة PTP عتادية
host-no-ptp4l = لا يجيب ptp4l، لذا لا تستطيع تدفقات هذا الحاسوب الالتزام بتوقيت gPTP
host-elsewhere = يعمل triib-endpointd على { $interface } لمستخدم آخر أو بصفة root، لذا تعمل كيانات هذا الحاسوب هناك لا هنا
host-foreign-mrp = يرسل برنامج آخر تصريحات MSRP أو MVRP على { $interface } من عنوان هذا الحاسوب، مما قد يسحب ما تحتاج إليه تدفقات هذا الحاسوب
host-alarm-foreign-mrp = يرسل برنامج آخر على هذا الحاسوب تصريحات MSRP أو MVRP على { $interface }
host-state = الحالة
host-streaming = جارٍ البث
host-waiting = في انتظار Listener
host-listening = جارٍ الاستماع
host-bound = مرتبط، في انتظار Talker
host-unbound = غير مرتبط
host-audio-from = الصوت من
host-audio-to = الصوت إلى
host-channels = القنوات
host-virtual-device = جهاز افتراضي
host-add-stream = إضافة تدفق
host-add-audio-input = دخل صوتي
host-add-audio-output = خرج صوتي
host-add-clock-input = دخل ساعة الوسائط
host-add-clock-output = خرج ساعة الوسائط
host-remove-stream = إزالة التدفق
host-silence = صمت
host-tone = نغمة اختبار
host-nowhere = لا مكان
host-default-device = الجهاز الافتراضي
host-remove = إزالة من هذا الحاسوب

mac-audio-device = جهاز الصوت
mac-no-audio-device = لا يوجد ما دام لا تدفق متصل
mac-entity-note = يشغّل macOS هذا الكيان. ما دام أحد تدفقاته متصلًا، ينشئ macOS جهاز الصوت الذي تشغّل التطبيقات الصوت عبره وتسجّل منه، بعدد القنوات في كل اتجاه الذي يحدّده التكوين.
mac-open-audio-midi-setup = فتح Audio MIDI Setup
mac-avb-off = خاصية AVB متوقفة على { $interface }، لذا ليس لهذا الـ Mac كيان AVB خاص به. شغّلها في متصفح أجهزة الشبكة في Audio MIDI Setup.

listen-section = الاستماع
listen-start = استماع
listen-stop = إيقاف الاستماع
listen-channels = القنوات
listen-channel = القناة { $number }
listen-channel-pair = القناتان { $first }–{ $last }
listen-output = الخرج
listen-default-output = الخرج الافتراضي
listen-gain = الكسب
listen-gain-db = { $db } dB
listen-starting = جارٍ البدء…
listen-waiting-input = بانتظار { $device }، الذي ينشئه macOS ما دام أحد تدفقات الكيان متصلًا.
listen-waiting-output = بانتظار { $device }.
listen-playing = قيد التشغيل، الذروة { $db } dB
listen-playing-silent = قيد التشغيل، صامت
