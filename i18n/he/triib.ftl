## Language

language-name = עברית

## Common

common-close = סגירה
common-more = עוד
common-keep-toolbar-shown = השארת סרגל הכלים גלוי
common-auto-hide-toolbar = הסתרה אוטומטית של סרגל הכלים

## Settings

settings-title = הגדרות
settings-general = כללי
settings-appearance = מראה
settings-language = שפה
settings-language-system = ברירת המחדל של המערכת: { $language }
settings-language-note = בשדות טקסט מקלידים בשפת הקלט של המערכת.
settings-appearance-system = מערכת
settings-appearance-light = בהיר
settings-appearance-dark = כהה
settings-colors = צבעים
settings-system-accent = שימוש בצבע ההדגשה של המערכת
settings-accent-picked = הצבע שלמטה הוא הבסיס לצבעים של triib.
settings-accent-omarchy = מתוך ערכת הנושא של Omarchy, { $theme }.
settings-accent-desktop = מתוך צבע ההדגשה של שולחן העבודה.
settings-accent-none = לשולחן העבודה אין צבע הדגשה, ולכן נעשה שימוש בצבע שלמטה.
settings-motion = תנועה
settings-animations = אנימציות
settings-animations-note = קפיצים והחלקות כשדברים משתנים.
settings-animations-reduced = שולחן העבודה מבקש תנועה מופחתת, ולכן triib נשאר דומם.
settings-outside-control = שליטה מבחוץ
settings-allow-outside-control = התרת שליטה מבחוץ
settings-allow-outside-control-note = סוכנים כמו Claude Code יכולים לראות ולשנות את רשת ה-AVB שלכם, דרך triib --mcp. triib שואל לפני כל סוכן חדש.
settings-allowed-agents = מורשים: { $agents }
settings-forget-agents = הסרה
settings-storage = אחסון
settings-storage-presets = תיקיית הפריסטים
settings-storage-entities = קובץ הישויות
settings-storage-apply = החלה
settings-storage-choose = בחירה…
settings-storage-note = פריסטים שכבר נשמרו במיקום הישן נשארים שם; כדי להמשיך להשתמש בהם צריך להעביר אותם. ההגדרות של triib נשמרות ב-{ $file }.
settings-storage-entities-note = קובץ ישויות שעוד אינו קיים מתחיל כעותק של הקובץ שבשימוש, ו-triib-endpointd מופעל מחדש איתו.
dialog-choose-presets = בחירת תיקיית הפריסטים
dialog-choose-entities = בחירת התיקייה של קובץ הישויות
settings-full-path = יש להשתמש בנתיב מלא, למשל ~/Documents/triib.
settings-path-is-folder = { $path } הוא תיקייה, לא קובץ.
settings-folder-missing = התיקייה { $path } לא קיימת. אפשר ליצור אותה קודם, או לבחור תיקייה אחרת.
settings-path-is-file = { $path } הוא קובץ, לא תיקייה.
settings-cannot-write = triib לא יכול לכתוב ב-{ $path }: { $error }.
app-file-dialog-failed = לא ניתן להציג את חלון בחירת הקבצים: { $error }
agent-prompt-title = לאפשר ל-{ $agent } לשלוט ב-triib?
agent-prompt-body = { $agent } מבקש להשתמש בשליטה מבחוץ של triib: לראות את הישויות ברשת שלכם ולשנות אותן, לחבר זרמים ולהגדיר פורמטים, שעונים ושמות. אפשר לכבות את השליטה מבחוץ בהגדרות.
agent-prompt-allow = אישור
agent-prompt-deny = דחייה

common-cancel = ביטול
common-save = שמירה
common-not-set = לא מוגדר
common-unnamed = ללא שם
common-none = אין
common-mac-address = כתובת MAC
common-list-separator = {", "}

## Network interfaces

interface-up = פעיל
interface-link-down = קישור מנותק
interface-wireless = אלחוטי
interface-hardware-clock = שעון חומרה
interface-hardware-clock-named = שעון חומרה { $clock }
interface-virtual = וירטואלי

## Toolbar

toolbar-choose-interface = בחירת ממשק
toolbar-interface = ממשק רשת
toolbar-filter-interfaces = סינון ממשקים
toolbar-hide-virtual = הסתרת ממשקים וירטואליים
toolbar-hide-wireless = הסתרת ממשקים אלחוטיים
toolbar-hide-unclocked = הסתרת ממשקים ללא שעון חומרה
toolbar-connections = חיבורים
toolbar-network = רשת
toolbar-entities = ישויות
toolbar-rediscover = בקשה מכל ישות להכריז על עצמה
toolbar-rescan = ניקוי וסריקה מחדש של כל הישויות
toolbar-search = חיפוש ישויות וזרמים
toolbar-presets = פריסטים
toolbar-log = יומן
toolbar-inspector = מפקח
toolbar-settings = הגדרות
toolbar-close-window = סגירת החלון

## The macOS menu bar, named as in macOS's own apps

menu-about = אודות triib
menu-settings = הגדרות…
menu-services = שירותים
menu-hide = הסתרת triib
menu-hide-others = הסתרת האחרים
menu-show-all = הצגת הכול
menu-quit = יציאה מ-triib
menu-file = קובץ
menu-presets = פריסטים…
menu-close-window = סגירת החלון
menu-edit = עריכה
menu-cut = גזירה
menu-copy = העתקה
menu-paste = הדבקה
menu-select-all = בחירת הכול
menu-view = תצוגה
menu-full-screen = כניסה למסך מלא
menu-window = חלון
menu-minimize = מזעור
menu-zoom = הגדלת החלון
menu-bring-all-to-front = הבאת הכול לחזית
menu-help = עזרה
menu-website = האתר של triib
menu-release-notes = הערות גרסה

## The network's state, in place of a view

state-no-interface = אין ממשק
state-no-interface-note = בחרו את הממשק שמחובר לרשת AVB כדי לגלות ישויות.
state-starting = מתחיל
state-starting-note = פותח את { $interface }.
state-listening = מאזין
state-listening-note = ישויות ב-{ $interface } יופיעו כאן כשהן מכריזות על עצמן.
state-permission-needed = נדרשת הרשאה
state-npcap-needed = נדרש Npcap
state-get-npcap = הורדת Npcap
state-copy-command = העתקת הפקודה
state-cannot-use = לא ניתן להשתמש ב-{ $interface }
state-try-again = ניסיון חוזר

## Entity list

entities-none-yet = עדיין אין ישויות
entities-none-yet-note = כל ישות ברשת, עם התפקידים, מחלקות ה-SR והשעון שלה.

## Inspector

inspector-title = מפקח
inspector-entity = ישות
inspector-streams = זרמים
inspector-controls = פקדים
inspector-diagnostics = אבחון
inspector-descriptors = מתארים
inspector-select = בחרו ישות כדי לראות את פרטיה.
inspector-offline = { $entity } במצב לא מקוון.
inspector-heard = הוא לא מכריז על עצמו: triib שמע אותו בפקודות ששלח, ומציג אותו ברשימה עד שעוברות 5 דקות בלי פקודה.
inspector-rename = שינוי שם
inspector-name = שם
inspector-identify = זיהוי
inspector-model-not-read = מודל הישות שלה לא נקרא.
inspector-no-streams = אין זרמים.
inspector-no-controls = אין פקדים להצגה.
inspector-no-diagnostics = לא דווחו ממשקים או מונים.
inspector-reading = קורא מתארים, { $count } עד כה.
inspector-read-failed = לא ניתן היה לקרוא את מודל הישות: { $reason }.

entity-section = ישות
entity-name = שם
entity-triib-controller = בקר triib
entity-triib-on = triib ב-{ $host }
entity-triib-here = triib במחשב הזה
entity-controller-on = בקר ב-{ $host }
entity-controller-here = בקר במחשב הזה
entity-controller-at = בקר בכתובת { $mac }
entity-group = קבוצה
entity-product = מוצר
entity-firmware = קושחה
entity-serial-number = מספר סידורי
entity-configuration = תצורה
entity-configuration-of = { $name } ({ $number } מתוך { $count })
entity-configuration-holds = { $inputs } כניסות, { $outputs } יציאות, זרמים של { $channels } ערוצים
entity-configuration-holds-mixed = { $inputs } כניסות, { $outputs } יציאות
entity-configuration-numbered = תצורה { $number }
entity-milan = Milan
entity-media-clock = שעון מדיה
entity-clock-domain = תחום שעון
entity-sampling-rate = קצב דגימה
clock-source-numbered = מקור { $index }
rate-pull = משיכה { $pull }

stream-format = פורמט
stream-id = ID של הזרם
stream-destination = יעד
stream-max-transit = זמן מעבר מרבי
stream-listeners = מאזינים
stream-listener-gone = { $entity } כניסה { $input }, לא מחובר
stream-disconnect-listener = ניתוק

avb-interfaces = ממשקי AVB
avb-interface = ממשק
avb-interface-clock-identity = זהות שעון
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, תחום { $domain }
avb-interface-peer-delay = השהיית עמית
avb-interface-running = פועל
avb-interface-none-reported = לא דווח
avb-interface-path = נתיב
avb-interface-own-grandmaster = ה-Grandmaster של עצמו
avb-interface-hops = { $count ->
    [one] קפיצה אחת מה-Grandmaster
    [two] { $count } קפיצות מה-Grandmaster
   *[other] { $count } קפיצות מה-Grandmaster
}
avb-interface-link-up = קישור פעיל
avb-interface-link-down = קישור מנותק
avb-interface-grandmaster-changes = החלפות Grandmaster
avb-interface-frames-sent = מסגרות שנשלחו
avb-interface-frames-received = מסגרות שהתקבלו
avb-interface-crc-errors = שגיאות CRC

tree-firmware = קושחה { $version }
tree-descriptor-types = { $count ->
    [one] סוג מתאר אחד
    [two] { $count } סוגי מתארים
   *[other] { $count } סוגי מתארים
}
tree-clock = שעון
tree-clock-source-from = { $kind }, מתוך { $location } { $index }
tree-clock-domain-using = משתמש ב-{ $source }
tree-clusters = { $count ->
    [one] אשכול אחד
    [two] { $count } אשכולות
   *[other] { $count } אשכולות
}
tree-maps = { $count ->
    [one] מיפוי אחד
    [two] { $count } מיפויים
   *[other] { $count } מיפויים
}

advert-not-advertised = לא מוכרז
advert-identity = זהות
advert-entity-id = ID ישות
advert-entity-model = מודל ישות
advert-roles = תפקידים
advert-talker = Talker
advert-listener = Listener
advert-clock = שעון
advert-gptp-domain = תחום gPTP
advert-sr-classes = מחלקות SR
advert-indexes = אינדקסים במודל הישות
advert-identify-control = פקד זיהוי
advert-avb-interface = ממשק AVB
advert-advertising = מכריז
advert-valid-time = זמן תוקף
advert-available-index = אינדקס זמינות
advert-association = אסוציאציה
advert-capabilities = יכולות

## Status bar

status-entities = { $count ->
    [one] ישות אחת
    [two] { $count } ישויות
   *[other] { $count } ישויות
}
status-not-discovering = לא מתבצע גילוי
status-discovering = מתבצע גילוי
status-discovering-as = מתבצע גילוי בתור { $controller }
status-stopped = נעצר בגלל שגיאה
status-alarm = התראה
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } ועוד { $count }

## Descriptions of entities, shared by the views

role-talker = Talker { $count }
role-listener = Listener { $count }
role-controller = בקר
role-none = ללא תפקידים
classes-a-and-b = A ו-B
clock-no-gptp = ללא gPTP

read-not-read = לא נקרא
read-reading = בקריאה, { $count } עד כה
read-ready-unreadable = מוכן, { $count } לא קריאים
read-ready-cached = מוכן, מהמטמון
read-ready = מוכן
read-failed = נכשל: { $reason }

milan-no = לא
milan-before-1-3 = לפני 1.3
milan-certified = { $version }, מוסמך { $certification }
milan-not-certified = { $version }, לא מוסמך

outcome-status = סטטוס { $status }
outcome-no-response = אין תגובה
outcome-not-possible = לא אפשרי
outcome-connect = לא ניתן היה לחבר את { $talker } אל { $listener }: { $reason }.
outcome-disconnect = לא ניתן היה לנתק את { $listener }: { $reason }.
outcome-disconnect-listener = לא ניתן היה לנתק את { $listener } מ-{ $talker }: { $reason }.
outcome-identify = לא ניתן היה לזהות את { $entity }: { $reason }.
outcome-rename = לא ניתן היה לשנות את השם של { $what } ל-"{ $name }": { $reason }.
outcome-rename-group = לא ניתן היה לשנות את שם הקבוצה של { $entity } ל-"{ $name }": { $reason }.
outcome-format-streaming = לא ניתן היה לשנות את הפורמט של { $stream }: הוא משדר. נתקו אותו קודם.
outcome-format = לא ניתן היה לשנות את הפורמט של { $stream }: { $reason }.
outcome-max-transit-time = לא ניתן היה לשנות את זמן המעבר המרבי של { $stream }: { $reason }.
outcome-max-transit-time-streaming = לא ניתן היה לשנות את זמן המעבר המרבי של { $stream }: הוא משדר. נתקו אותו קודם.
outcome-sampling-rate = לא ניתן היה לשנות את קצב הדגימה של { $entity }: { $reason }.
outcome-clock-source = לא ניתן היה לשנות את מקור השעון של { $entity }: { $reason }.
outcome-configuration = לא ניתן היה לשנות את התצורה של { $entity }: { $reason }.
outcome-configuration-streaming = { $entity } שומר על התצורה שלו כל עוד הזרמים שלו פעילים: נתקו אותם קודם.
outcome-map = לא ניתן היה למפות את הערוץ ב-{ $entity }: { $reason }.
outcome-unmap = לא ניתן היה לבטל את מיפוי הערוץ ב-{ $entity }: { $reason }.
outcome-control = לא ניתן היה להגדיר את "{ $control }" ב-{ $entity }: { $reason }.
outcome-control-numbered = לא ניתן היה להגדיר את פקד { $index } ב-{ $entity }: { $reason }.

stream-not-connected = לא מחובר
stream-from = מ-{ $stream }
stream-from-receiving = מ-{ $stream }, בקליטה
stream-from-waiting = מ-{ $stream }, ממתין ל-Talker
stream-from-failed = מ-{ $stream }, השריון של ה-Talker נכשל: { $reason }
stream-sending-to = שולח אל { $destination }

failure-no-response = היא לא הגיבה
failure-refused = היא סירבה עם { $status }
failure-malformed = לא ניתן היה לפענח את התגובה שלה
failure-on-this-computer = היא פועלת במחשב הזה; יש לקרוא אותה ממחשב אחר

msrp-failure-1 = אין מספיק רוחב פס
msrp-failure-2 = אין מספיק משאבים בגשר
msrp-failure-3 = אין מספיק רוחב פס למחלקת התעבורה
msrp-failure-4 = ה-ID של הזרם בשימוש של Talker אחר
msrp-failure-5 = כתובת היעד כבר בשימוש
msrp-failure-6 = נדחק על ידי זרם בדרגה גבוהה יותר
msrp-failure-7 = ההשהיה המדווחת השתנתה
msrp-failure-8 = פורט היציאה אינו תומך ב-AVB
msrp-failure-9 = יש להשתמש בכתובת יעד אחרת
msrp-failure-10 = משאבי MSRP אזלו
msrp-failure-11 = משאבי MMRP אזלו
msrp-failure-12 = לא ניתן לאחסן את כתובת היעד
msrp-failure-13 = העדיפות אינה עדיפות של מחלקת SR
msrp-failure-14 = המסגרות גדולות מדי עבור התווך
msrp-failure-15 = הפורט הגיע למגבלת הזרמים הנכנסים
msrp-failure-16 = הערך הראשון השתנה עבור זרם רשום
msrp-failure-17 = ה-VLAN חסום בפורט היציאה
msrp-failure-18 = תיוג VLAN מושבת בפורט היציאה
msrp-failure-19 = אי-התאמה בעדיפות מחלקת SR
msrp-failure-unknown = סיבה לא ידועה
msrp-failure-at = { $reason }, בגשר { $bridge }

## Entity list columns

column-vendor = יצרן
column-model = דגם
column-state = מצב
column-entity-model-id = ID מודל ישות
column-talker-streams = זרמי Talker
column-listener-streams = זרמי Listener
column-avb-lite = AVB Lite
column-egress = תעבורה יוצאת
column-wireless = אלחוטי

## Settings file

settings-no-place = אין היכן לשמור את ההגדרות: תיקיית הבית אינה ידועה.
settings-unusable = לא ניתן היה להשתמש ב-{ $path }: { $error }.
settings-unsaved = לא ניתן היה לשמור את { $path }: { $error }.

column-remove = הסרת העמודה
column-move-left = הזזה שמאלה
column-move-right = הזזה ימינה
column-add = הוספת עמודה
common-percent = { $value }%

## Network view

netmap-empty = עדיין אין רשת להצגה
netmap-empty-note = ישויות יופיעו כאן לאחר שנקראו ודיווחו על מיקומן בעץ ה-gPTP.
netmap-focus-clock-path = נתיב השעון של { $name }
netmap-focus-streams = הזרמים של { $name }
netmap-showing = מוצג: { $what }
netmap-devices = { $count ->
    [one] התקן אחד
    [two] { $count } התקנים
   *[other] { $count } התקנים
}
netmap-bridges = { $count ->
    [one] גשר אחד
    [two] { $count } גשרים
   *[other] { $count } גשרים
}
netmap-show-map = הצגת המפה
netmap-show-details = הצגת הפרטים
stream-numbered = זרם { $index }
netmap-bridge = גשר
netmap-access-point = נקודת גישה
netmap-plain-switch = מתג ללא AVB
netmap-device = התקן
netmap-this-computer = המחשב הזה
netmap-connected = מחובר
netmap-advertised = מוכרז, אין Listener מוכן
netmap-advertised-off-tree = מוכרז, אין Listener מוכן ({ $listener } לא בעץ ה-gPTP)
netmap-failed-at = השריון נכשל ב-{ $bridge }: { $reason }
netmap-failed = השריון נכשל: { $reason }
netmap-no-bridge-on = לא נשמע גשר ב-{ $interface }
netmap-cannot-listen-on = אי אפשר להאזין ל-gPTP ב-{ $interface }
netmap-on-this-computer = במחשב הזה
netmap-path-not-reported = הנתיב לא דווח
netmap-gptp-not-reported = gPTP לא דווח
netmap-off-tree = מחוץ לעץ ה-gPTP
netmap-off-ptp = מחוץ לעץ ה-PTP
netmap-not-lite = מחוץ ל-AVB Lite
netmap-lite-not-reported = AVB Lite לא דווח
netmap-domain = תחום gPTP { $domain }
netmap-lite-only = AVB Lite בלבד
netmap-lite-set = AVB Lite, הוגדר ידנית
netmap-synced = מסונכרן
netmap-not-synced = לא מסונכרן
netmap-triib-on = triib ב-{ $interface }
netmap-through-count = { $count } עוברים
netmap-out = { $count } יוצאים
netmap-in = { $count } נכנסים
netmap-failed-count = { $count } נכשלו
netmap-advertised-only = מוכרז בלבד
netmap-failed-state = נכשל
netmap-stream-item = { $talker } ← { $listener } · { $state }
netmap-sending-unheard = שולח, ללא Listener ידוע שמשויך אליו
netmap-no-listener = אין Listener ידוע
netmap-apart-own-grandmaster = לא בעץ ה-gPTP: הוא ה-Grandmaster של עצמו
netmap-apart-no-path = הנתיב שלו לא דווח; הוא עוקב אחר Grandmaster { $grandmaster }
netmap-apart-unreported = הוא לא דיווח על מצב ה-gPTP שלו
netmap-apart-no-neighbor = לא נשמע גשר בממשק של המחשב הזה
netmap-apart-cannot-listen = המחשב הזה לא יכול להאזין ל-gPTP בממשק שלו
netmap-apart-on-this-computer = היא פועלת במחשב הזה; כדי לראות את מצב ה-gPTP שלה יש לקרוא אותה ממחשב אחר
netmap-apart-not-lite = הוא לא מריץ AVB Lite, ולכן הוא לא עוקב אחר ה-Grandmaster
netmap-apart-lite-unreported = הוא לא מדווח דבר על AVB Lite, ולכן לא ידוע אחר מי הוא עוקב
netmap-apart-other-domain = הוא מריץ gPTP בתחום { $domain } והעץ בתחום { $network }, ולכן הוא לא עוקב אחר ה-Grandmaster של העץ
netmap-apart-lite-only = הוא מריץ AVB Lite בלבד, בלי לקחת חלק ב-gPTP או ב-SRP, ולכן גשרי AVB לא משריינים דבר עבור הזרמים שלו
netmap-apart-lite-set = הוא מריץ AVB Lite כי המפעיל או בקר הגדירו אותו כך, בלי לקחת חלק ב-gPTP או ב-SRP
netmap-switch-state = לא לוקח חלק ב-AVB: מייצג את המתגים שלפני ההתקנים שמתחת, שלא מצאו גשר AVB ועברו ל-AVB Lite, ועוקבים אחר אותו Grandmaster של PTP
netmap-clock-tree = עץ השעון
netmap-no-grandmaster = לא נשמע Grandmaster
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = דורש תשומת לב
netmap-nodes-below = צמתים מתחתיו
netmap-bridges-below = גשרים מתחתיו
netmap-clock-path = נתיב שעון
netmap-hops = קפיצות מה-Grandmaster
netmap-link-delay = השהיית קישור
netmap-bridge-port = פורט גשר
netmap-link-drops = נפילות קישור
netmap-synced-to-grandmaster = מסונכרן ל-Grandmaster
netmap-host-no-gptp = לא מסונכרן: המחשב הזה לא מריץ gPTP
netmap-link-no-gptp = לא מסונכרן: gPTP לא פועל בקישור שלו
netmap-ptp-offset-high = לא מסונכרן: { $offset } מה-Grandmaster, מעבר ל-50 µs ש-AVB Lite מתיר
netmap-ptp-no-offset = לא מסונכרן: הוא לא מדד שום היסט מה-Grandmaster
netmap-audio = אודיו
netmap-media-clock-streams = זרמי שעון מדיה
netmap-audio-streams = זרמי אודיו
netmap-bound = { $count } משויכים
netmap-flowing = זורם
netmap-advertised-state = מוכרז
netmap-media-clock-stream = זרם שעון מדיה
netmap-audio-stream = זרם אודיו
netmap-reaches = מגיע עד
netmap-passing-count = { $count ->
    [one] זרם אחד עובר
    [two] { $count } זרמים עוברים
   *[other] { $count } זרמים עוברים
}
netmap-through = דרך
netmap-passing-through = עובר דרך
netmap-sending = שולח
netmap-receiving = קולט
netmap-problems = בעיות
netmap-help-back = לחצו על הרקע כדי לחזור לסקירה הכללית.
netmap-help-stream = לחצו על זרם כדי לבדוק אותו, או על הרקע כדי לחזור לסקירה הכללית.
netmap-help-ptp = ב-AVB Lite השעון זורם מקצה לקצה, מה-Grandmaster לכל התקן, דרך מתגים שאינם לוקחים בו חלק, ולכן אף אחד מהם לא מוצג. התקן מסונכרן כל עוד הוא עוקב אחר ה-Grandmaster בטווח של 50 µs. לחצו על התקן או על החוט שלו כדי לבדוק את השעון שלו; לחצו על הרקע כדי לנקות.
netmap-help-clock = השעון זורם מה-Grandmaster דרך כל גשר לכל צומת בעץ. קו אפור מקווקו הוא קישור שבו gPTP לא פועל. לחצו על התקן או על החוט שלו כדי לבדוק את נתיב השעון שלו; לחצו על הרקע כדי לנקות.
netmap-help-lite = התקנים שלא מצאו גשר AVB ועברו ל-AVB Lite תלויים מתחת לכרטיס מקווקו שמייצג את המתגים שלפניהם, אחד לכל Grandmaster של PTP שהם עוקבים אחריו. התקנים שמריצים AVB Lite בלבד או שהוגדרו לכך, והתקנים בתחום gPTP אחר, נמצאים באזורים משלהם.
netmap-help-media-clock = זרמי שעון מדיה (CRF) בלבד, מצוירים כמו אודיו: חוט אחד למסגרות של כל זרם בכל קישור, בצבע לפי ה-Talker. לחצו על חוט כדי לבדוק את הזרם שלו, או על התקן כדי לראות את הזרמים שלו; לחצו על הרקע כדי לנקות.
netmap-help-audio = כל חוט מייצג את המסגרות של זרם אחד בקישור אחד, והוא נכנס לכל גשר שהוא חוצה ויוצא ממנו. לזרם מולטיקאסט יש חוט אחד כל עוד הנתיבים אל ה-Listeners שלו עוברים יחד, והוא מתפצל היכן שהם נפרדים; לזרם יוניקאסט, כפי ש-AVB Lite יכול לשלוח, יש חוט משלו מה-Talker לכל Listener. הצבע הוא לפי ה-Talker: לכל Talker יש גוון משלו, וכל אחד מהזרמים שלו הוא דרגה של הגוון הזה בכל ענף. נקודות נעות פירושן שאודיו זורם, גם מ-Talker ששולח בלי Listener ידוע; קו אדום דומם הוא שריון שנכשל, וקו אפור דומם הוא זרם מוכרז שאין לו Listener מוכן; שניהם נעצרים היכן שהשריון נעצר. התקנים בעמודה האמצעית מחוברים ישירות לגשר של ה-Grandmaster. לחצו על חוט כדי לבדוק את הזרם שלו, על חוט משותף כדי לראות את כל ה-Listeners שלו, או על התקן כדי לראות את הזרמים שלו; לחצו על הרקע כדי לנקות.

## Connections

matrix-nothing-shown = אין זרמים להצגה
matrix-nothing-shown-note = שנו את החיפוש או את המסננים כדי לראות זרמים נוספים.
matrix-empty = אין זרמים לחיבור
matrix-empty-note = זרמי Talker וזרמי Listener נפגשים כאן לאחר שהישויות שלהם נקראו.
matrix-all-streams = כל הזרמים
matrix-connectable-only = הסתרת מה שלא ניתן לחבר
matrix-none-hidden = כל זרם מוצג ניתן לחיבור
matrix-hidden = { $count ->
    [one] זרם אחד מוסתר
    [two] { $count } זרמים מוסתרים
   *[other] { $count } זרמים מוסתרים
}
matrix-own = היציאות של ישות לא מתחברות לכניסות שלה עצמה.
matrix-working = בטיפול.
matrix-waiting-change = ממתין לשינוי האחרון בכניסה הזו.
matrix-connected = מחובר וקולט. לחצו כדי לנתק.
matrix-bound-waiting = משויך, ממתין לזרם של ה-Talker. לחצו כדי לנתק.
matrix-bound-failed = משויך, אבל השריון של ה-Talker נכשל: { $reason }. לחצו כדי לנתק.
matrix-bound-formats-differ = משויך, אבל הפורמטים שונים: ה-Talker שולח { $sent }, והכניסה מוגדרת ל-{ $set }. לחצו כדי לנתק.
matrix-formats-match = הפורמטים תואמים ({ $format }). לחצו כדי לחבר.
matrix-format-must-change = הכניסה מקבלת { $sent } אבל מוגדרת ל-{ $set }, ולכן ייתכן שלא תנגן עד שהפורמט שלה ישתנה. לחצו כדי לבחור פורמט.
matrix-format-input = שינוי הכניסה אל
matrix-format-both = שינוי שניהם אל
matrix-format-talker-busy = ה-Talker משדר ל-Listeners אחרים, ולכן הפורמט שלו נשאר.
matrix-format-anyway = חיבור בלי לשנות פורמטים
matrix-incompatible = הכניסה לא מקבלת { $sent }. היא מוגדרת ל-{ $set }.
matrix-group-none = לא מחובר. הרחיבו כדי לחבר זרמים אחד אחד.
matrix-group-connected = { $count } מחוברים. הרחיבו כדי לראות כל אחד.
matrix-outputs-expand = { $count ->
    [one] יציאת זרם אחת. לחצו על החץ כדי להרחיב, ועל השם כדי לבדוק אותה.
    [two] { $count } יציאות זרם. לחצו על החץ כדי להרחיב, ועל השם כדי לבדוק אותה.
   *[other] { $count } יציאות זרם. לחצו על החץ כדי להרחיב, ועל השם כדי לבדוק אותה.
}
matrix-outputs-collapse = { $count ->
    [one] יציאת זרם אחת. לחצו על החץ כדי לכווץ, ועל השם כדי לבדוק אותה.
    [two] { $count } יציאות זרם. לחצו על החץ כדי לכווץ, ועל השם כדי לבדוק אותה.
   *[other] { $count } יציאות זרם. לחצו על החץ כדי לכווץ, ועל השם כדי לבדוק אותה.
}
matrix-inputs-expand = { $count ->
    [one] כניסת זרם אחת. לחצו על החץ כדי להרחיב, ועל השם כדי לבדוק אותה.
    [two] { $count } כניסות זרם. לחצו על החץ כדי להרחיב, ועל השם כדי לבדוק אותה.
   *[other] { $count } כניסות זרם. לחצו על החץ כדי להרחיב, ועל השם כדי לבדוק אותה.
}
matrix-inputs-collapse = { $count ->
    [one] כניסת זרם אחת. לחצו על החץ כדי לכווץ, ועל השם כדי לבדוק אותה.
    [two] { $count } כניסות זרם. לחצו על החץ כדי לכווץ, ועל השם כדי לבדוק אותה.
   *[other] { $count } כניסות זרם. לחצו על החץ כדי לכווץ, ועל השם כדי לבדוק אותה.
}
matrix-stream-inspect = { $detail } לחצו כדי לבדוק את { $entity }.
matrix-point = הצביעו על תא
matrix-point-note = כדי לראות את ה-Talker וה-Listener שלו והאם הפורמטים שלהם תואמים.
matrix-legend-waiting = משויך, ממתין לזרם
matrix-legend-trouble = משויך, משהו לא תקין
matrix-legend-open = ניתן לחבר
matrix-legend-change = יש לשנות קודם את פורמט הכניסה
matrix-legend-incompatible = הפורמטים לא יכולים להתאים
matrix-talker-outputs = יציאות Talker
matrix-listener-inputs = כניסות Listener

common-thousands-separator = {","}

## Diagnostics

diag-since-start = נספר מאז שהישות הופעלה.
diag-stream-input = כניסת זרם
diag-stream-output = יציאת זרם
diag-locked = { $count ->
    [0] לא ננעל
    [one] ננעל פעם אחת
    [two] ננעל פעמיים
   *[other] ננעל { $number } פעמים
}
diag-lost-lock = { $count ->
    [0] לא איבד נעילה
    [one] איבד נעילה פעם אחת
    [two] איבד נעילה פעמיים
   *[other] איבד נעילה { $number } פעמים
}
diag-frames-in = { $count ->
    [one] מסגרת נכנסת אחת
    [two] { $number } מסגרות נכנסות
   *[other] { $number } מסגרות נכנסות
}
diag-frames-out = { $count ->
    [one] מסגרת יוצאת אחת
    [two] { $number } מסגרות יוצאות
   *[other] { $number } מסגרות יוצאות
}
diag-media-locked = { $count ->
    [0] לא ננעל על המדיה
    [one] ננעל על המדיה פעם אחת
    [two] ננעל על המדיה פעמיים
   *[other] ננעל על המדיה { $number } פעמים
}
diag-lost-media-lock = { $count ->
    [0] לא איבד נעילת מדיה
    [one] איבד נעילת מדיה פעם אחת
    [two] איבד נעילת מדיה פעמיים
   *[other] איבד נעילת מדיה { $number } פעמים
}
diag-interrupted = { $count ->
    [0] לא נקטע
    [one] נקטע פעם אחת
    [two] נקטע פעמיים
   *[other] נקטע { $number } פעמים
}
diag-out-of-sequence = { $count ->
    [one] מסגרת אחת שלא לפי הסדר
    [two] { $number } מסגרות שלא לפי הסדר
   *[other] { $number } מסגרות שלא לפי הסדר
}
diag-media-resets = { $count ->
    [one] איפוס מדיה אחד
    [two] { $number } איפוסי מדיה
   *[other] { $number } איפוסי מדיה
}
diag-timestamps-uncertain = { $count ->
    [0] אין חותמות זמן לא ודאיות
    [one] חותמות זמן לא ודאיות פעם אחת
    [two] חותמות זמן לא ודאיות פעמיים
   *[other] חותמות זמן לא ודאיות { $number } פעמים
}
diag-no-timestamp = { $count ->
    [one] מסגרת אחת ללא חותמת זמן
    [two] { $number } מסגרות ללא חותמת זמן
   *[other] { $number } מסגרות ללא חותמת זמן
}
diag-unsupported-format = { $count ->
    [one] מסגרת אחת בפורמט לא נתמך
    [two] { $number } מסגרות בפורמט לא נתמך
   *[other] { $number } מסגרות בפורמט לא נתמך
}
diag-late = { $count ->
    [one] מסגרת מאוחרת אחת
    [two] { $number } מסגרות מאוחרות
   *[other] { $number } מסגרות מאוחרות
}
diag-early = { $count ->
    [one] מסגרת מוקדמת אחת
    [two] { $number } מסגרות מוקדמות
   *[other] { $number } מסגרות מוקדמות
}
diag-started = { $count ->
    [0] לא הופעל
    [one] הופעל פעם אחת
    [two] הופעל פעמיים
   *[other] הופעל { $number } פעמים
}
diag-stopped = { $count ->
    [0] לא נעצר
    [one] נעצר פעם אחת
    [two] נעצר פעמיים
   *[other] נעצר { $number } פעמים
}
diag-reservation-failed = השריון של ה-Talker נכשל: { $reason }
diag-latency = { $microseconds } µs השהיה מצטברת

## AVB Lite

lite-active = פעיל
lite-active-untagged = פעיל, ללא תיוג
lite-active-vlan = פעיל, VLAN { $vlan }
lite-capable = נתמך
lite-mode = מצב
lite-mode-capable = AVB (תומך ב-AVB Lite)
lite-because = סיבה
lite-fallback-none = לא צוינה סיבה
lite-fallback-endpoint = הצהרה של נקודת קצה אחרת עברה, ולכן אין גשר AVB ביניהן
lite-fallback-unanswered = תשע בקשות השהיית עמית לא נענו
lite-fallback-responders = שניים או יותר ענו לבקשת השהיית עמית אחת, ולכן המתג אינו גשר AVB
lite-fallback-configured = המפעיל או בקר הגדירו זאת
lite-fallback-other = סיבה שהפרופיל לא מגדיר
lite-other-profile = פרופיל אחר
lite-ptp-domain = { $profile }, תחום { $domain }
lite-offset = היסט
lite-offset-from = { $offset } מ-{ $grandmaster }
lite-media-vlan = VLAN מדיה
lite-untagged = ללא תיוג
lite-unicast = יוניקאסט
lite-fanout = { $count ->
    [one] עד Listener אחד לכל זרם, ואז מולטיקאסט
    [two] עד { $count } Listener לכל זרם, ואז מולטיקאסט
   *[other] עד { $count } Listener לכל זרם, ואז מולטיקאסט
}
lite-link = קישור
lite-bandwidth = רוחב פס
lite-egress-of = { $used } מתוך { $link }, { $share }
lite-egress-of-assumed = { $used } מתוך { $link }, { $share }, בהנחת קישור ג'יגביט
lite-egress-reported = כפי שהישות סופרת את הזרמים שאושרו לה.
lite-egress-worked-out = מחושב מהפורמטים של יציאות הזרם המחוברות שלה.
lite-alarm-offset = היסט PTP { $offset }, מעבר ל-50 µs ש-AVB Lite מתיר
lite-alarm-egress = תעבורה יוצאת ב-{ $share } מהקישור, מעבר ל-{ $limit } שזרמים רשאים לתפוס

## AVB Wireless

wireless-station = תחנה
wireless-access-point = נקודת גישה
wireless-role = תפקיד
wireless-mode = מצב
wireless-time = זמן
wireless-mode-a-ftm = Mode A, 802.1AS על גבי FTM
wireless-mode-a-tm = Mode A, 802.1AS על גבי TM
wireless-mode-b = Mode B, ממסגרות beacon
wireless-no-time = ללא זמן
wireless-other-mode = מצב שהפרופיל לא מגדיר
wireless-locked = נעול
wireless-holdover = ב-holdover
wireless-not-locked = לא נעול
wireless-row-locked = תחנה, נעולה
wireless-row-holdover = תחנה, ב-holdover
wireless-row-not-locked = תחנה, לא נעולה
wireless-row-access-point = { $count ->
    [one] נקודת גישה, תחנה אחת
    [two] נקודת גישה, { $count } תחנות
   *[other] נקודת גישה, { $count } תחנות
}
wireless-link = קישור
wireless-channel = ערוץ { $channel }
wireless-not-known = לא ידוע
wireless-signal = עוצמת אות
wireless-rate = קצב שידור
wireless-ftm-valid = { $share } תקפות
wireless-rtt = זמן הלוך ושוב { $rtt }
wireless-bursts = { $count ->
    [one] פרצים של מסגרת אחת
    [two] פרצים של { $count } מסגרות
   *[other] פרצים של { $count } מסגרות
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = נקודת הגישה מקצה פרצי FTM שאינם של שלוש או שתי מסגרות
wireless-reason-measurement = אין FTM ואין TM מול נקודת הגישה
wireless-reason-signaling = אין הודעת Signaling מסוג gPTP-capable מנקודת הגישה
wireless-reason-other = סיבה שהפרופיל לא מגדיר
wireless-servo = שגיאת סרוו
wireless-stations = תחנות
wireless-station-count = { $count ->
    [one] תחנה אחת
    [two] { $count } תחנות
   *[other] { $count } תחנות
}
wireless-no-ftm = ללא FTM
wireless-unserved = Listener ללא שירות
wireless-frame-budget = תקציב מסגרות
wireless-frame-use = { $used } מתוך { $budget } מסגרות בשנייה
wireless-stream-frames = מסגרות זרם
wireless-frames-of = { $readdressed } לתחנות, { $unmapped } ללא Listener, { $dropped } נזרקו, { $restored } מתחנות
wireless-class-a-allowed = מותר
wireless-class-a-not-allowed = לא מותר
wireless-alarm-not-locked = זמן ה-Wi-Fi לא נעול על נקודת הגישה
wireless-alarm-holdover = זמן ה-Wi-Fi ב-holdover, איבד נעילה על נקודת הגישה
wireless-alarm-unserved = { $count ->
    [one] Listener אחד בפורט ה-Wi-Fi לא מקבל שירות, מעבר למגבלת היוניקאסט
    [two] { $count } Listener בפורט ה-Wi-Fi לא מקבלים שירות, מעבר למגבלת היוניקאסט
   *[other] { $count } Listener בפורט ה-Wi-Fi לא מקבלים שירות, מעבר למגבלת היוניקאסט
}

## Log

log-all = הכול
log-warnings = אזהרות
log-pause = עצירה
log-resume = המשך
log-clear = ניקוי
log-empty = כל מסגרת ATDECC ש-triib שולח ושומע מופיעה כאן, החדשה ביותר ראשונה.
log-none-match = אף מסגרת שמורה לא תואמת למסנן.
log-frames = { $count ->
    [one] מסגרת אחת
    [two] { $count } מסגרות
   *[other] { $count } מסגרות
}
log-shown-of = { $shown } מתוך { $all } מסגרות
log-sent = נשלחה
log-heard = נשמעה
log-not-decoded = לא פוענחה
log-warning-short = ה-control_data_length שלה מצהיר על { $missing } בתים מעבר לסוף המסגרת.
log-warning-undecodable = לא ניתן לפענח אותה: { $error }.
log-warning-long-acmp = היא בצורת ACMP הארוכה, שישות Milan אינה רשאית לשלוח (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = מיפויי ערוצים
mapping-inputs = כניסות
mapping-outputs = יציאות
mapping-port = פורט { $number }
mapping-fixed = קבוע
mapping-not-read = עדיין לא נקרא.
mapping-no-clusters = אין אשכולות.
mapping-no-streams = אין זרמי אודיו.
mapping-none = אין מיפויים.
mapping-not-mapped = לא ממופה
mapping-cluster-numbered = אשכול { $index }

## Presets

presets-note = פריסט שומר את מקורות השעון, קצבי הדגימה, פורמטי הזרמים, הפקדים והחיבורים של כל ישות. טעינתו משנה את מה ששונה ממנו.
presets-none = עדיין לא נשמרו פריסטים.
presets-connections = { $count ->
    [one] חיבור אחד
    [two] { $count } חיבורים
   *[other] { $count } חיבורים
}
presets-recall = טעינה
presets-delete = מחיקה
presets-no-place = אין היכן לשמור פריסטים: תיקיית הבית אינה ידועה.
presets-undeletable = לא ניתן היה למחוק את { $path }: { $error }.
presets-saved = { $count ->
    [one] "{ $name }" נשמר עם ישות אחת.
    [two] "{ $name }" נשמר עם { $count } ישויות.
   *[other] "{ $name }" נשמר עם { $count } ישויות.
}
presets-nothing-differs = שום דבר לא שונה מ-"{ $name }".
presets-recalling = { $count ->
    [one] טוען את "{ $name }": שינוי אחד.
    [two] טוען את "{ $name }": { $count } שינויים.
   *[other] טוען את "{ $name }": { $count } שינויים.
}
presets-missing = { $report } לא נמצאות כאן או לא נקראו: { $missing }.
presets-deleted = "{ $name }" נמחק.
presets-host-note = הוא שומר גם את הישויות של המחשב הזה, ומפעיל אותן מחדש בטעינה.
presets-host-endpoints = { $count } במחשב הזה
presets-starting-host = מפעיל את הישויות של המחשב הזה עבור "{ $name }"; השאר יבוא כשיחזרו.

## Controls

control-numbered = פקד { $index }
control-not-shown = לא מוצג כאן
control-option = אפשרות { $number }

## Network errors

network-permission = triib זקוק להרשאה כדי לשלוח ולקבל מסגרות אתרנט גולמיות.
network-needs-npcap = triib זקוק ל-Npcap כדי לשלוח ולקבל מסגרות אתרנט גולמיות.
network-npcap-administrators = Npcap מאפשר רק למנהלי מערכת לשלוח ולקבל מסגרות אתרנט גולמיות. יש להפעיל את triib כמנהל מערכת, או להתקין מחדש את Npcap בלי האפשרות של מנהלי מערכת בלבד.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {"."}

## This computer's own entities

host-add-entity = הוספת ישות
host-show-mine = הצגת הישויות של המחשב הזה בלבד
host-show-all = הצגת כל הישויות
host-new-entity = ישות מארח { $number }
host-failed = לא ניתן היה להוסיף למחשב הזה: { $reason }
host-needs-clock = הישויות של המחשב הזה צריכות ממשק קווי עם שעון חומרה PTP
host-no-ptp4l = ptp4l לא עונה, ולכן הזרמים של המחשב הזה לא יכולים לשמור על זמן gPTP
host-elsewhere = triib-endpointd פועל ב-{ $interface } עבור משתמש אחר או כ-root, ולכן הישויות של המחשב הזה פועלות שם ולא כאן
host-foreign-mrp = תוכנה אחרת שולחת הצהרות MSRP או MVRP ב-{ $interface } מהכתובת של המחשב הזה, וכך היא עלולה לבטל את מה שהזרמים של המחשב הזה צריכים
host-alarm-foreign-mrp = תוכנה אחרת במחשב הזה שולחת הצהרות MSRP או MVRP ב-{ $interface }
host-state = מצב
host-streaming = משדר
host-waiting = ממתין ל-Listener
host-listening = מאזין
host-bound = מקושר, ממתין ל-Talker
host-unbound = לא מקושר
host-audio-from = שמע מ
host-audio-to = שמע אל
host-channels = ערוצים
host-virtual-device = התקן וירטואלי
host-add-stream = הוספת זרם
host-add-audio-input = כניסת שמע
host-add-audio-output = יציאת שמע
host-add-clock-input = כניסת שעון מדיה
host-add-clock-output = יציאת שעון מדיה
host-remove-stream = הסרת הזרם
host-silence = שקט
host-tone = צליל בדיקה
host-nowhere = לשום מקום
host-default-device = התקן ברירת מחדל
host-remove = הסרה מהמחשב הזה

mac-audio-device = התקן שמע
mac-no-audio-device = אין, כל עוד אף זרם אינו מחובר
mac-entity-note = macOS מפעיל את הישות הזו. כל עוד אחד הזרמים שלה מחובר, macOS יוצר את התקן השמע שדרכו אפליקציות מנגנות ומקליטות, עם מספר הערוצים בכל כיוון שהתצורה קובעת.
mac-open-audio-midi-setup = פתיחת Audio MIDI Setup
mac-avb-off = AVB כבוי ב-{ $interface }, ולכן ל-Mac הזה אין ישות AVB משלו. הפעילו אותו בדפדפן התקני הרשת של Audio MIDI Setup.

listen-section = האזנה
listen-start = האזנה
listen-stop = הפסקת האזנה
listen-source = מאזין ל
listen-received = מה שהיא מקבלת
listen-sent = מה שהיא שולחת בזרם { $number } ({ $channels } ערוצים)
listen-sent-needs-macos = האזנה למה שה-Mac הזה שולח דורשת macOS 14.2 ומעלה.
listen-channels = ערוצים
listen-channel = ערוץ { $number }
listen-channel-pair = ערוצים { $first }–{ $last }
listen-output = פלט
listen-default-output = פלט ברירת מחדל
listen-gain = הגבר
listen-gain-db = { $db } dB
listen-starting = מתחיל…
listen-waiting-input = ממתין ל-{ $device }, ש-macOS יוצר כל עוד אחד הזרמים של הישות מחובר.
listen-waiting-output = ממתין ל-{ $device }.
listen-playing = מתנגן, בשיא של { $db } dB
listen-playing-silent = מתנגן, שקט
