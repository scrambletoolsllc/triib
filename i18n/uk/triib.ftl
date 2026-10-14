## Language

language-name = Українська

## Common

common-close = Закрити
common-more = Ще
common-keep-toolbar-shown = Не приховувати панель інструментів
common-auto-hide-toolbar = Автоматично приховувати панель інструментів

## Settings

settings-title = Налаштування
settings-general = Загальні
settings-appearance = Вигляд
settings-language = Мова
settings-language-system = Мова системи: { $language }
settings-language-note = У текстових полях використовується системна мова введення.
settings-appearance-system = Як у системі
settings-appearance-light = Світла
settings-appearance-dark = Темна
settings-colors = Кольори
settings-system-accent = Використовувати системний акцентний колір
settings-accent-picked = Кольори triib будуються на основі кольору нижче.
settings-accent-omarchy = З теми Omarchy: { $theme }.
settings-accent-desktop = З акцентного кольору робочого столу.
settings-accent-none = Робочий стіл не має акцентного кольору, тому використовується колір нижче.
settings-motion = Рух
settings-animations = Анімації
settings-animations-note = Пружні й плавні переходи під час змін.
settings-animations-reduced = Робочий стіл просить зменшити рух, тому triib обходиться без анімації.
settings-outside-control = Зовнішнє керування
settings-allow-outside-control = Дозволити зовнішнє керування
settings-allow-outside-control-note = Агенти, як-от Claude Code, можуть бачити й змінювати вашу мережу AVB через triib --mcp. triib питає перед кожним новим агентом.
settings-allowed-agents = Дозволено: { $agents }
settings-forget-agents = Забути
settings-storage = Зберігання
settings-storage-presets = Папка пресетів
settings-storage-entities = Файл сутностей
settings-storage-apply = Застосувати
settings-storage-choose = Вибрати…
settings-storage-note = Пресети, що вже зберігаються в старому місці, залишаються там; перенесіть їх, щоб і далі ними користуватися. Параметри triib зберігаються в { $file }.
settings-storage-entities-note = Файл сутностей, якого ще немає, починається як копія використовуваного, і triib-endpointd перезапускається з ним.
dialog-choose-presets = Виберіть папку для пресетів
dialog-choose-entities = Виберіть папку для файлу сутностей
settings-full-path = Вкажіть повний шлях, наприклад ~/Documents/triib.
settings-path-is-folder = { $path } — це папка, а не файл.
settings-folder-missing = Папки { $path } немає. Спершу створіть її або виберіть іншу.
settings-path-is-file = { $path } — це файл, а не папка.
settings-cannot-write = triib не може записувати в { $path }: { $error }.
app-file-dialog-failed = Не вдалося показати вікно вибору файлу: { $error }
agent-prompt-title = Дозволити агентові { $agent } керувати triib?
agent-prompt-body = { $agent } просить доступ до зовнішнього керування triib: бачити сутності у вашій мережі та змінювати їх — підключати потоки, задавати формати, джерела синхронізації та назви. Зовнішнє керування можна вимкнути в налаштуваннях.
agent-prompt-allow = Дозволити
agent-prompt-deny = Не дозволяти

common-cancel = Скасувати
common-save = Зберегти
common-not-set = Не задано
common-unnamed = Без назви
common-none = Немає
common-mac-address = MAC-адреса
common-list-separator = {", "}

## Network interfaces

interface-up = активний
interface-link-down = немає лінку
interface-wireless = бездротовий
interface-hardware-clock = апаратний годинник
interface-hardware-clock-named = апаратний годинник { $clock }
interface-virtual = віртуальний

## Toolbar

toolbar-choose-interface = Виберіть інтерфейс
toolbar-interface = Мережевий інтерфейс
toolbar-filter-interfaces = Фільтр інтерфейсів
toolbar-hide-virtual = Приховати віртуальні інтерфейси
toolbar-hide-wireless = Приховати бездротові інтерфейси
toolbar-hide-unclocked = Приховати інтерфейси без апаратного годинника
toolbar-connections = Підключення
toolbar-network = Мережа
toolbar-entities = Сутності
toolbar-rediscover = Попросити всі сутності оголосити про себе
toolbar-rescan = Очистити й повторно просканувати всі сутності
toolbar-search = Пошук сутностей і потоків
toolbar-presets = Пресети
toolbar-log = Журнал
toolbar-inspector = Інспектор
toolbar-settings = Налаштування

## The network's state, in place of a view

state-no-interface = Інтерфейс не вибрано
state-no-interface-note = Виберіть інтерфейс у мережі AVB, щоб виявити сутності.
state-starting = Запуск
state-starting-note = Відкривається { $interface }.
state-listening = Прослуховування
state-listening-note = Сутності на { $interface } з'являються тут, щойно оголошують про себе.
state-permission-needed = Потрібен дозвіл
state-npcap-needed = Потрібен Npcap
state-get-npcap = Завантажити Npcap
state-copy-command = Скопіювати команду
state-cannot-use = Не вдається використати { $interface }
state-try-again = Повторити

## Entity list

entities-none-yet = Сутностей ще немає
entities-none-yet-note = Усі сутності в мережі з їхніми ролями, класами SR і синхронізацією.

## Inspector

inspector-title = Інспектор
inspector-entity = Сутність
inspector-streams = Потоки
inspector-controls = Регулятори
inspector-diagnostics = Діагностика
inspector-descriptors = Дескриптори
inspector-select = Виберіть сутність, щоб побачити подробиці.
inspector-offline = { $entity } не в мережі.
inspector-heard = Він не оголошує себе: triib почув його в надісланих ним командах і показує його, доки не мине 5 хвилин без команд.
inspector-rename = Перейменувати
inspector-name = Назва
inspector-identify = Ідентифікувати
inspector-model-not-read = Модель сутності не прочитано.
inspector-no-streams = Немає потоків.
inspector-no-controls = Немає регуляторів для показу.
inspector-no-diagnostics = Немає даних про інтерфейси та лічильники.
inspector-reading = Читання дескрипторів, прочитано { $count }.
inspector-read-failed = Не вдалося прочитати модель сутності: { $reason }.

entity-section = Сутність
entity-name = Назва
entity-triib-controller = Контролер triib
entity-triib-on = triib на { $host }
entity-triib-here = triib на цьому комп'ютері
entity-controller-on = Контролер на { $host }
entity-controller-here = Контролер на цьому комп'ютері
entity-controller-at = Контролер за адресою { $mac }
entity-group = Група
entity-product = Продукт
entity-firmware = Прошивка
entity-serial-number = Серійний номер
entity-configuration = Конфігурація
entity-configuration-of = { $name } ({ $number } з { $count })
entity-configuration-holds = { $inputs } вх., { $outputs } вих., потоки по { $channels } кан.
entity-configuration-holds-mixed = { $inputs } вх., { $outputs } вих.
entity-configuration-numbered = Конфігурація { $number }
entity-milan = Milan
entity-media-clock = Медіасинхронізація
entity-clock-domain = Домен синхронізації
entity-sampling-rate = Частота дискретизації
clock-source-numbered = Джерело { $index }
rate-pull = pull { $pull }

stream-format = Формат
stream-id = ID потоку
stream-destination = Призначення
stream-max-transit = Макс. час проходження

avb-interfaces = Інтерфейси AVB
avb-interface = Інтерфейс
avb-interface-clock-identity = Ідентифікатор годинника
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, домен { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Працює
avb-interface-none-reported = Немає даних
avb-interface-path = Шлях
avb-interface-own-grandmaster = Сам є Grandmaster
avb-interface-hops = { $count ->
    [one] { $count } перехід від Grandmaster
    [few] { $count } переходи від Grandmaster
    [many] { $count } переходів від Grandmaster
   *[other] { $count } переходу від Grandmaster
}
avb-interface-link-up = Лінк є
avb-interface-link-down = Немає лінку
avb-interface-grandmaster-changes = Зміни Grandmaster
avb-interface-frames-sent = Надіслано кадрів
avb-interface-frames-received = Отримано кадрів
avb-interface-crc-errors = Помилки CRC

tree-firmware = Прошивка { $version }
tree-descriptor-types = { $count ->
    [one] { $count } тип дескрипторів
    [few] { $count } типи дескрипторів
    [many] { $count } типів дескрипторів
   *[other] { $count } типу дескрипторів
}
tree-clock = Синхронізація
tree-clock-source-from = { $kind }, з { $location } { $index }
tree-clock-domain-using = Використовує { $source }
tree-clusters = { $count ->
    [one] { $count } кластер
    [few] { $count } кластери
    [many] { $count } кластерів
   *[other] { $count } кластера
}
tree-maps = { $count ->
    [one] { $count } карта
    [few] { $count } карти
    [many] { $count } карт
   *[other] { $count } карти
}

advert-not-advertised = Не оголошено
advert-identity = Ідентифікація
advert-entity-id = ID сутності
advert-entity-model = Модель сутності
advert-roles = Ролі
advert-talker = Talker
advert-listener = Listener
advert-clock = Синхронізація
advert-btc = BTC
advert-gptp-domain = Домен gPTP
advert-sr-classes = Класи SR
advert-indexes = Індекси в моделі сутності
advert-identify-control = Регулятор ідентифікації
advert-avb-interface = Інтерфейс AVB
advert-advertising = Оголошення
advert-valid-time = Час дії
advert-available-index = Індекс доступності
advert-association = Асоціація
advert-capabilities = Можливості

## Status bar

status-entities = { $count ->
    [one] { $count } сутність
    [few] { $count } сутності
    [many] { $count } сутностей
   *[other] { $count } сутності
}
status-not-discovering = Виявлення не триває
status-discovering = Виявлення
status-discovering-as = Виявлення від імені { $controller }
status-stopped = Зупинено через помилку
status-alarm = Тривога
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } і ще { $count }

## Descriptions of entities, shared by the views

role-talker = Talker { $count }
role-listener = Listener { $count }
role-controller = контролер
role-none = немає ролей
classes-a-and-b = A і B
clock-no-gptp = Немає gPTP

read-not-read = Не прочитано
read-reading = Читання, прочитано { $count }
read-ready-unreadable = Готово, не читається: { $count }
read-ready-cached = Готово, з кешу
read-ready = Готово
read-failed = Помилка: { $reason }

milan-no = Ні
milan-before-1-3 = до 1.3
milan-certified = { $version }, сертифікація { $certification }
milan-not-certified = { $version }, без сертифікації

outcome-status = статус { $status }
outcome-no-response = немає відповіді
outcome-not-possible = неможливо
outcome-connect = Не вдалося підключити { $talker } до { $listener }: { $reason }.
outcome-disconnect = Не вдалося відключити { $listener }: { $reason }.
outcome-identify = Не вдалося ідентифікувати { $entity }: { $reason }.
outcome-rename = Не вдалося перейменувати { $what } на «{ $name }»: { $reason }.
outcome-rename-group = Не вдалося перейменувати групу сутності { $entity } на «{ $name }»: { $reason }.
outcome-format-streaming = Не вдалося змінити формат потоку { $stream }: триває передавання. Спершу відключіть потік.
outcome-format = Не вдалося змінити формат потоку { $stream }: { $reason }.
outcome-max-transit-time = Не вдалося змінити макс. час проходження потоку { $stream }: { $reason }.
outcome-max-transit-time-streaming = Не вдалося змінити макс. час проходження потоку { $stream }: триває передавання. Спершу відключіть потік.
outcome-sampling-rate = Не вдалося змінити частоту дискретизації сутності { $entity }: { $reason }.
outcome-clock-source = Не вдалося змінити джерело синхронізації сутності { $entity }: { $reason }.
outcome-configuration = Не вдалося змінити конфігурацію сутності { $entity }: { $reason }.
outcome-configuration-streaming = Сутність { $entity } зберігає конфігурацію, поки тривають її потоки: спершу відключіть їх.
outcome-map = Не вдалося призначити канал у сутності { $entity }: { $reason }.
outcome-unmap = Не вдалося скасувати призначення каналу в сутності { $entity }: { $reason }.
outcome-control = Не вдалося задати «{ $control }» у сутності { $entity }: { $reason }.
outcome-control-numbered = Не вдалося задати регулятор { $index } у сутності { $entity }: { $reason }.

stream-not-connected = Не підключено
stream-from = Від { $stream }
stream-from-receiving = Від { $stream }, приймання
stream-from-waiting = Від { $stream }, очікування Talker
stream-from-failed = Від { $stream }, резервування Talker не вдалося: { $reason }
stream-sending-to = Надсилання на { $destination }

failure-no-response = сутність не відповіла
failure-refused = відмова зі статусом { $status }
failure-malformed = відповідь не вдалося декодувати
failure-on-this-computer = сутність працює на цьому комп'ютері; прочитайте її з іншого

msrp-failure-1 = недостатньо смуги пропускання
msrp-failure-2 = недостатньо ресурсів комутатора
msrp-failure-3 = недостатньо смуги для класу трафіку
msrp-failure-4 = ID потоку використовує інший Talker
msrp-failure-5 = адреса призначення вже використовується
msrp-failure-6 = витіснено потоком вищого рангу
msrp-failure-7 = заявлена затримка змінилася
msrp-failure-8 = вихідний порт не підтримує AVB
msrp-failure-9 = використайте іншу адресу призначення
msrp-failure-10 = вичерпано ресурси MSRP
msrp-failure-11 = вичерпано ресурси MMRP
msrp-failure-12 = неможливо зберегти адресу призначення
msrp-failure-13 = пріоритет не є пріоритетом класу SR
msrp-failure-14 = кадри завеликі для середовища передавання
msrp-failure-15 = досягнуто межі вхідних портів (fan-in)
msrp-failure-16 = змінилося перше значення зареєстрованого потоку
msrp-failure-17 = VLAN заблоковано на вихідному порту
msrp-failure-18 = тегування VLAN вимкнено на вихідному порту
msrp-failure-19 = невідповідність пріоритету класу SR
msrp-failure-unknown = невідома причина
msrp-failure-at = { $reason }, на комутаторі { $bridge }

## Entity list columns

column-vendor = Виробник
column-model = Модель
column-state = Стан
column-entity-model-id = ID моделі сутності
column-talker-streams = Потоки Talker
column-listener-streams = Потоки Listener
column-avb-lite = AVB Lite
column-egress = Вихідний трафік
column-wireless = Бездротова мережа

## Settings file

settings-no-place = Ніде зберігати налаштування: домашня тека невідома.
settings-unusable = Не вдалося використати { $path }: { $error }.
settings-unsaved = Не вдалося зберегти { $path }: { $error }.

column-remove = Видалити стовпець
column-move-left = Перемістити ліворуч
column-move-right = Перемістити праворуч
column-add = Додати стовпець
common-percent = { $value }%

## Network view

netmap-empty = Мережі для показу ще немає
netmap-empty-note = Сутності з'являться тут, щойно їх буде прочитано і вони повідомлять своє місце в дереві gPTP.
netmap-focus-clock-path = Шлях синхронізації { $name }
netmap-focus-streams = Потоки { $name }
netmap-showing = Показано: { $what }
netmap-devices = { $count ->
    [one] { $count } пристрій
    [few] { $count } пристрої
    [many] { $count } пристроїв
   *[other] { $count } пристрою
}
netmap-bridges = { $count ->
    [one] { $count } комутатор
    [few] { $count } комутатори
    [many] { $count } комутаторів
   *[other] { $count } комутатора
}
netmap-show-map = Показати карту
netmap-show-details = Показати подробиці
stream-numbered = Потік { $index }
netmap-bridge = Комутатор
netmap-access-point = Точка доступу
netmap-device = Пристрій
netmap-this-computer = Цей комп'ютер
netmap-connected = Підключено
netmap-advertised = Оголошено, немає готового Listener
netmap-advertised-off-tree = Оголошено, немає готового Listener ({ $listener } не в дереві gPTP)
netmap-failed-at = Резервування не вдалося на { $bridge }: { $reason }
netmap-failed = Резервування не вдалося: { $reason }
netmap-no-bridge-on = На { $interface } не чути комутатора
netmap-cannot-listen-on = Не вдається слухати gPTP на { $interface }
netmap-on-this-computer = На цьому комп'ютері
netmap-path-not-reported = Шлях не повідомлено
netmap-gptp-not-reported = Немає даних gPTP
netmap-off-tree = Не в дереві gPTP
netmap-off-ptp = Не в дереві PTP
netmap-not-lite = Не в AVB Lite
netmap-lite-not-reported = Немає даних AVB Lite
netmap-synced = Синхронізовано
netmap-not-synced = Не синхронізовано
netmap-triib-on = triib на { $interface }
netmap-through-count = транзитом: { $count }
netmap-out = вихідних: { $count }
netmap-in = вхідних: { $count }
netmap-failed-count = зі збоєм: { $count }
netmap-advertised-only = Лише оголошено
netmap-failed-state = Збій
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Не в дереві gPTP: сам є Grandmaster
netmap-apart-no-path = Шлях не повідомлено; синхронізується з Grandmaster { $grandmaster }
netmap-apart-unreported = Стан gPTP не повідомлено
netmap-apart-no-neighbor = На інтерфейсі цього комп'ютера не чути комутатора
netmap-apart-cannot-listen = Цей комп'ютер не може слухати gPTP на своєму інтерфейсі
netmap-apart-on-this-computer = Працює на цьому комп'ютері; щоб побачити стан gPTP, прочитайте її з іншого комп'ютера
netmap-apart-not-lite = Не працює в AVB Lite, тому не синхронізується з Grandmaster
netmap-apart-lite-unreported = Нічого не повідомляє про AVB Lite, тому невідомо, з чим синхронізується
netmap-clock-tree = Дерево синхронізації
netmap-no-grandmaster = Не чути Grandmaster
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Потребує уваги
netmap-nodes-below = Вузли нижче
netmap-bridges-below = Комутатори нижче
netmap-clock-path = Шлях синхронізації
netmap-hops = Переходів від Grandmaster
netmap-link-delay = Затримка лінку
netmap-bridge-port = Порт комутатора
netmap-link-drops = Обриви лінку
netmap-synced-to-grandmaster = Синхронізовано з Grandmaster
netmap-host-no-gptp = Не синхронізовано: на цьому комп'ютері не працює gPTP
netmap-link-no-gptp = Не синхронізовано: на його лінку не працює gPTP
netmap-ptp-offset-high = Не синхронізовано: { $offset } від Grandmaster, більше за 50 µs, які допускає AVB Lite
netmap-ptp-no-offset = Не синхронізовано: зсув від Grandmaster не виміряно
netmap-audio = Аудіо
netmap-media-clock-streams = Потоки медіасинхронізації
netmap-audio-streams = Аудіопотоки
netmap-bound = прив'язано: { $count }
netmap-flowing = Передається
netmap-advertised-state = Оголошено
netmap-media-clock-stream = Потік медіасинхронізації
netmap-audio-stream = Аудіопотік
netmap-reaches = Доходить до
netmap-passing-count = { $count ->
    [one] { $count } транзитний потік
    [few] { $count } транзитні потоки
    [many] { $count } транзитних потоків
   *[other] { $count } транзитного потоку
}
netmap-through = Через
netmap-passing-through = Транзитом
netmap-sending = Надсилає
netmap-receiving = Приймає
netmap-problems = Проблеми
netmap-help-back = Клацніть фон, щоб повернутися до огляду.
netmap-help-stream = Клацніть потік, щоб відкрити його в інспекторі, або фон, щоб повернутися до огляду.
netmap-help-ptp = В AVB Lite синхронізація йде наскрізно від Grandmaster до кожного пристрою через комутатори, які в ній не беруть участі, тому їх не показано. Пристрій синхронізований, доки його зсув від Grandmaster не перевищує 50 µs. Клацніть пристрій або його провід, щоб відкрити його синхронізацію в інспекторі; клацніть фон, щоб зняти виділення.
netmap-help-clock = Синхронізація йде від Grandmaster через кожен комутатор до всіх вузлів дерева. Сіра пунктирна лінія — це лінк, на якому не працює gPTP. Клацніть пристрій або його провід, щоб відкрити його шлях синхронізації в інспекторі; клацніть фон, щоб зняти виділення.
netmap-help-media-clock = Лише потоки медіасинхронізації (CRF), намальовані так само, як аудіо: один провід на потік, колір за Talker. Клацніть провід, щоб відкрити його потік в інспекторі, або пристрій, щоб побачити його потоки; клацніть фон, щоб зняти виділення.
netmap-help-audio = Кожен потік має власний провід, що входить у кожен комутатор на шляху і виходить із нього. Колір залежить від Talker: кожен Talker має свій відтінок, а його потоки — варіації цього відтінку. Рухомі точки означають, що аудіо передається; нерухома червона лінія — невдале резервування, нерухома сіра — оголошений потік без готового Listener; обидві обриваються там, де закінчується резервування. Пристрої в середньому стовпці підключені безпосередньо до комутатора Grandmaster. Клацніть провід, щоб відкрити його потік в інспекторі, або пристрій, щоб побачити його потоки; клацніть фон, щоб зняти виділення.

## Connections

matrix-nothing-shown = Немає потоків для показу
matrix-nothing-shown-note = Змініть пошук або фільтри, щоб побачити більше потоків.
matrix-empty = Немає потоків для підключення
matrix-empty-note = Потоки Talker і Listener з'являться тут, щойно буде прочитано сутності, які їх мають.
matrix-all-streams = Усі потоки
matrix-connectable-only = Приховати те, що не можна підключити
matrix-none-hidden = Усі показані потоки можна підключити
matrix-hidden = { $count ->
    [one] { $count } потік приховано
    [few] { $count } потоки приховано
    [many] { $count } потоків приховано
   *[other] { $count } потоку приховано
}
matrix-own = Виходи сутності не підключаються до її власних входів.
matrix-working = Виконується.
matrix-waiting-change = Очікування останньої зміни цього входу.
matrix-connected = Підключено, триває приймання. Клацніть, щоб відключити.
matrix-bound-waiting = Прив'язано, очікування потоку Talker. Клацніть, щоб відключити.
matrix-bound-failed = Прив'язано, але резервування Talker не вдалося: { $reason }. Клацніть, щоб відключити.
matrix-bound-formats-differ = Прив'язано, але формати різняться: Talker надсилає { $sent }, вхід налаштовано на { $set }. Клацніть, щоб відключити.
matrix-formats-match = Формати збігаються ({ $format }). Клацніть, щоб підключити.
matrix-format-must-change = Вхід приймає { $sent }, але налаштований на { $set }, тому може не відтворювати звук, доки формат не зміниться. Клацніть, щоб вибрати формат.
matrix-format-input = Змінити вхід на
matrix-format-both = Змінити обидва на
matrix-format-talker-busy = Talker передає потік іншим Listener, тому його формат не змінюється.
matrix-format-anyway = Підключити, не змінюючи формати
matrix-incompatible = Вхід не приймає { $sent }. Його налаштовано на { $set }.
matrix-group-none = Не підключено. Розгорніть, щоб підключати потоки по одному.
matrix-group-connected = Підключено: { $count }. Розгорніть, щоб побачити кожне підключення.
matrix-outputs-expand = { $count ->
    [one] { $count } вихід потоку. Клацніть стрілку, щоб розгорнути, або назву, щоб відкрити в інспекторі.
    [few] { $count } виходи потоків. Клацніть стрілку, щоб розгорнути, або назву, щоб відкрити в інспекторі.
    [many] { $count } виходів потоків. Клацніть стрілку, щоб розгорнути, або назву, щоб відкрити в інспекторі.
   *[other] { $count } виходу потоку. Клацніть стрілку, щоб розгорнути, або назву, щоб відкрити в інспекторі.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } вихід потоку. Клацніть стрілку, щоб згорнути, або назву, щоб відкрити в інспекторі.
    [few] { $count } виходи потоків. Клацніть стрілку, щоб згорнути, або назву, щоб відкрити в інспекторі.
    [many] { $count } виходів потоків. Клацніть стрілку, щоб згорнути, або назву, щоб відкрити в інспекторі.
   *[other] { $count } виходу потоку. Клацніть стрілку, щоб згорнути, або назву, щоб відкрити в інспекторі.
}
matrix-inputs-expand = { $count ->
    [one] { $count } вхід потоку. Клацніть стрілку, щоб розгорнути, або назву, щоб відкрити в інспекторі.
    [few] { $count } входи потоків. Клацніть стрілку, щоб розгорнути, або назву, щоб відкрити в інспекторі.
    [many] { $count } входів потоків. Клацніть стрілку, щоб розгорнути, або назву, щоб відкрити в інспекторі.
   *[other] { $count } входу потоку. Клацніть стрілку, щоб розгорнути, або назву, щоб відкрити в інспекторі.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } вхід потоку. Клацніть стрілку, щоб згорнути, або назву, щоб відкрити в інспекторі.
    [few] { $count } входи потоків. Клацніть стрілку, щоб згорнути, або назву, щоб відкрити в інспекторі.
    [many] { $count } входів потоків. Клацніть стрілку, щоб згорнути, або назву, щоб відкрити в інспекторі.
   *[other] { $count } входу потоку. Клацніть стрілку, щоб згорнути, або назву, щоб відкрити в інспекторі.
}
matrix-stream-inspect = { $detail } Клацніть, щоб відкрити { $entity } в інспекторі.
matrix-point = Наведіть на клітинку
matrix-point-note = — побачите її Talker і Listener та чи сумісні їхні формати.
matrix-legend-waiting = Прив'язано, очікування потоку
matrix-legend-trouble = Прив'язано, є проблема
matrix-legend-open = Можна підключити
matrix-legend-change = Спершу треба змінити формат входу
matrix-legend-incompatible = Формати несумісні
matrix-talker-outputs = Виходи Talker
matrix-listener-inputs = Входи Listener

common-thousands-separator = {" "}

## Diagnostics

diag-since-start = Підраховано від запуску сутності.
diag-stream-input = Вхід потоку
diag-stream-output = Вихід потоку
diag-locked = { $count ->
    [0] без захоплення синхронізації
    [one] { $number } захоплення синхронізації
    [few] { $number } захоплення синхронізації
    [many] { $number } захоплень синхронізації
   *[other] { $number } захоплення синхронізації
}
diag-lost-lock = { $count ->
    [0] без втрати синхронізації
    [one] { $number } втрата синхронізації
    [few] { $number } втрати синхронізації
    [many] { $number } втрат синхронізації
   *[other] { $number } втрати синхронізації
}
diag-frames-in = { $count ->
    [one] { $number } отриманий кадр
    [few] { $number } отримані кадри
    [many] { $number } отриманих кадрів
   *[other] { $number } отриманого кадру
}
diag-frames-out = { $count ->
    [one] { $number } надісланий кадр
    [few] { $number } надіслані кадри
    [many] { $number } надісланих кадрів
   *[other] { $number } надісланого кадру
}
diag-media-locked = { $count ->
    [0] без захоплення медіасинхронізації
    [one] { $number } захоплення медіасинхронізації
    [few] { $number } захоплення медіасинхронізації
    [many] { $number } захоплень медіасинхронізації
   *[other] { $number } захоплення медіасинхронізації
}
diag-lost-media-lock = { $count ->
    [0] без втрати медіасинхронізації
    [one] { $number } втрата медіасинхронізації
    [few] { $number } втрати медіасинхронізації
    [many] { $number } втрат медіасинхронізації
   *[other] { $number } втрати медіасинхронізації
}
diag-interrupted = { $count ->
    [0] без переривань
    [one] { $number } переривання
    [few] { $number } переривання
    [many] { $number } переривань
   *[other] { $number } переривання
}
diag-out-of-sequence = { $count ->
    [one] { $number } кадр з порушенням послідовності
    [few] { $number } кадри з порушенням послідовності
    [many] { $number } кадрів з порушенням послідовності
   *[other] { $number } кадру з порушенням послідовності
}
diag-media-resets = { $count ->
    [one] { $number } скидання медіа
    [few] { $number } скидання медіа
    [many] { $number } скидань медіа
   *[other] { $number } скидання медіа
}
diag-timestamps-uncertain = { $count ->
    [0] мітки часу достовірні
    [one] недостовірні мітки часу { $number } раз
    [few] недостовірні мітки часу { $number } рази
    [many] недостовірні мітки часу { $number } разів
   *[other] недостовірні мітки часу { $number } раза
}
diag-no-timestamp = { $count ->
    [one] { $number } кадр без мітки часу
    [few] { $number } кадри без мітки часу
    [many] { $number } кадрів без мітки часу
   *[other] { $number } кадру без мітки часу
}
diag-unsupported-format = { $count ->
    [one] { $number } кадр у непідтримуваному форматі
    [few] { $number } кадри в непідтримуваному форматі
    [many] { $number } кадрів у непідтримуваному форматі
   *[other] { $number } кадру в непідтримуваному форматі
}
diag-late = { $count ->
    [one] { $number } запізнілий кадр
    [few] { $number } запізнілі кадри
    [many] { $number } запізнілих кадрів
   *[other] { $number } запізнілого кадру
}
diag-early = { $count ->
    [one] { $number } передчасний кадр
    [few] { $number } передчасні кадри
    [many] { $number } передчасних кадрів
   *[other] { $number } передчасного кадру
}
diag-started = { $count ->
    [0] без запусків
    [one] { $number } запуск
    [few] { $number } запуски
    [many] { $number } запусків
   *[other] { $number } запуску
}
diag-stopped = { $count ->
    [0] без зупинок
    [one] { $number } зупинка
    [few] { $number } зупинки
    [many] { $number } зупинок
   *[other] { $number } зупинки
}
diag-reservation-failed = резервування Talker не вдалося: { $reason }
diag-latency = накопичена затримка { $microseconds } µs

## AVB Lite

lite-active = Активний
lite-active-untagged = Активний, без тегу
lite-active-vlan = Активний, VLAN { $vlan }
lite-capable = Підтримується
lite-mode = Режим
lite-mode-capable = AVB (підтримує AVB Lite)
lite-because = Причина
lite-fallback-none = причину не вказано
lite-fallback-endpoint = надійшла декларація іншого кінцевого пристрою, отже, між ними немає AVB-комутатора
lite-fallback-unanswered = дев'ять запитів peer delay лишилися без відповіді
lite-fallback-responders = на один запит peer delay відповіли двоє або більше, отже, комутатор не підтримує AVB
lite-fallback-configured = так задав оператор або контролер
lite-fallback-other = причина, якої профіль не називає
lite-other-profile = Інший профіль
lite-ptp-domain = { $profile }, домен { $domain }
lite-offset = Зсув
lite-offset-from = { $offset } від { $grandmaster }
lite-media-vlan = VLAN для медіа
lite-untagged = Без тегу
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] До { $count } Listener на потік, далі multicast
    [few] До { $count } Listener на потік, далі multicast
    [many] До { $count } Listener на потік, далі multicast
   *[other] До { $count } Listener на потік, далі multicast
}
lite-link = Лінк
lite-bandwidth = Смуга пропускання
lite-egress-of = { $used } з { $link }, { $share }
lite-egress-of-assumed = { $used } з { $link }, { $share }, припускається гігабітний лінк
lite-egress-reported = Так рахує сама сутність за своїми допущеними потоками.
lite-egress-worked-out = Розраховано за форматами її підключених виходів потоків.
lite-alarm-offset = Зсув PTP { $offset }, більше за 50 µs, які допускає AVB Lite
lite-alarm-egress = Вихідний трафік — { $share } лінку, більше за допустимі для потоків { $limit }

## AVB Wireless

wireless-station = Станція
wireless-access-point = Точка доступу
wireless-role = Роль
wireless-mode = Режим
wireless-time = Час
wireless-mode-a-ftm = Mode A, 802.1AS через FTM
wireless-mode-a-tm = Mode A, 802.1AS через TM
wireless-mode-b = Mode B, з кадрів Beacon
wireless-no-time = Без синхронізації часу
wireless-other-mode = Режим, якого профіль не називає
wireless-locked = Синхронізацію захоплено
wireless-holdover = Режим утримання
wireless-not-locked = Немає захоплення синхронізації
wireless-row-locked = Станція, захоплення
wireless-row-holdover = Станція, утримання
wireless-row-not-locked = Станція, немає захоплення
wireless-row-access-point = { $count ->
    [one] Точка доступу, { $count } станція
    [few] Точка доступу, { $count } станції
    [many] Точка доступу, { $count } станцій
   *[other] Точка доступу, { $count } станції
}
wireless-link = Лінк
wireless-channel = канал { $channel }
wireless-not-known = Невідомо
wireless-signal = Сигнал
wireless-rate = Швидкість
wireless-ftm-valid = { $share } достовірних
wireless-rtt = кругова затримка { $rtt }
wireless-bursts = { $count ->
    [one] серії з { $count } кадру
    [few] серії з { $count } кадрів
    [many] серії з { $count } кадрів
   *[other] серії з { $count } кадру
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = точка доступу призначає серії FTM не з трьох і не з двох кадрів
wireless-reason-measurement = ні FTM, ні TM з точкою доступу
wireless-reason-signaling = немає повідомлення Signaling gPTP-capable від точки доступу
wireless-reason-other = причина, якої профіль не називає
wireless-servo = Похибка підстроювання
wireless-stations = Станції
wireless-station-count = { $count ->
    [one] { $count } станція
    [few] { $count } станції
    [many] { $count } станцій
   *[other] { $count } станції
}
wireless-no-ftm = без FTM
wireless-unserved = Listener без обслуговування
wireless-frame-budget = Бюджет кадрів
wireless-frame-use = { $budget ->
    [one] { $used } з { $budget } кадру за секунду
    [few] { $used } з { $budget } кадрів за секунду
    [many] { $used } з { $budget } кадрів за секунду
   *[other] { $used } з { $budget } кадру за секунду
}
wireless-stream-frames = Кадри потоків
wireless-frames-of = станціям: { $readdressed }, без Listener: { $unmapped }, відкинуто: { $dropped }, від станцій: { $restored }
wireless-class-a-allowed = Дозволено
wireless-class-a-not-allowed = Не дозволено
wireless-alarm-not-locked = Немає захоплення синхронізації часу Wi-Fi від точки доступу
wireless-alarm-holdover = Час Wi-Fi у режимі утримання, синхронізацію від точки доступу втрачено
wireless-alarm-unserved = { $count ->
    [one] { $count } Listener на порту Wi-Fi без обслуговування, понад ліміт unicast
    [few] { $count } Listener на порту Wi-Fi без обслуговування, понад ліміт unicast
    [many] { $count } Listener на порту Wi-Fi без обслуговування, понад ліміт unicast
   *[other] { $count } Listener на порту Wi-Fi без обслуговування, понад ліміт unicast
}

## Log

log-all = Усі
log-warnings = Попередження
log-pause = Пауза
log-resume = Продовжити
log-clear = Очистити
log-empty = Тут з'являються всі кадри ATDECC, які triib надсилає і чує, найновіші вгорі.
log-none-match = Жоден збережений кадр не відповідає фільтру.
log-frames = { $count ->
    [one] { $count } кадр
    [few] { $count } кадри
    [many] { $count } кадрів
   *[other] { $count } кадру
}
log-shown-of = { $all ->
    [one] { $shown } з { $all } кадру
    [few] { $shown } з { $all } кадрів
    [many] { $shown } з { $all } кадрів
   *[other] { $shown } з { $all } кадру
}
log-sent = Надіслано
log-heard = Отримано
log-not-decoded = Не декодовано
log-warning-short = { $missing ->
    [one] Поле control_data_length виходить за кінець кадру на { $missing } октет.
    [few] Поле control_data_length виходить за кінець кадру на { $missing } октети.
    [many] Поле control_data_length виходить за кінець кадру на { $missing } октетів.
   *[other] Поле control_data_length виходить за кінець кадру на { $missing } октету.
}
log-warning-undecodable = Кадр не декодується: { $error }.
log-warning-long-acmp = Кадр у довгій формі ACMP, яку сутність Milan надсилати не повинна (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Призначення каналів
mapping-inputs = Входи
mapping-outputs = Виходи
mapping-port = порт { $number }
mapping-fixed = фіксовано
mapping-not-read = Ще не прочитано.
mapping-no-clusters = Немає кластерів.
mapping-no-streams = Немає аудіопотоків.
mapping-none = Немає призначень.
mapping-not-mapped = Не призначено
mapping-cluster-numbered = Кластер { $index }

## Presets

presets-note = Пресет зберігає джерела синхронізації, частоти дискретизації, формати потоків, регулятори й підключення кожної сутності. Під час виклику змінюється те, що відрізняється.
presets-none = Пресетів ще немає.
presets-connections = { $count ->
    [one] { $count } підключення
    [few] { $count } підключення
    [many] { $count } підключень
   *[other] { $count } підключення
}
presets-recall = Викликати
presets-delete = Видалити
presets-no-place = Ніде зберігати пресети: домашня тека невідома.
presets-undeletable = Не вдалося видалити { $path }: { $error }.
presets-saved = { $count ->
    [one] Пресет «{ $name }» збережено: { $count } сутність.
    [few] Пресет «{ $name }» збережено: { $count } сутності.
    [many] Пресет «{ $name }» збережено: { $count } сутностей.
   *[other] Пресет «{ $name }» збережено: { $count } сутності.
}
presets-nothing-differs = Ніщо не відрізняється від «{ $name }».
presets-recalling = { $count ->
    [one] Виклик «{ $name }»: { $count } зміна.
    [few] Виклик «{ $name }»: { $count } зміни.
    [many] Виклик «{ $name }»: { $count } змін.
   *[other] Виклик «{ $name }»: { $count } зміни.
}
presets-missing = { $report } Немає в мережі або не прочитано: { $missing }.
presets-deleted = Пресет «{ $name }» видалено.
presets-host-note = Він також зберігає власні сутності цього комп'ютера та перезапускає їх під час виклику.
presets-host-endpoints = { $count } на цьому комп'ютері
presets-starting-host = Запускаються сутності цього комп'ютера для «{ $name }»; решта буде, коли вони повернуться.

## Controls

control-numbered = Регулятор { $index }
control-not-shown = Тут не показується
control-option = Варіант { $number }

## Network errors

network-permission = triib потребує дозволу надсилати й отримувати сирі кадри Ethernet.
network-needs-npcap = triib потребує Npcap, щоб надсилати й отримувати сирі кадри Ethernet.
network-npcap-administrators = Npcap дозволяє надсилати й отримувати сирі кадри Ethernet лише адміністраторам. Запустіть triib від імені адміністратора або перевстановіть Npcap без параметра «лише для адміністраторів».

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own entities

host-add-entity = Додати сутність
host-show-mine = Показувати лише власні сутності цього комп'ютера
host-show-all = Показувати всі сутності
host-new-entity = Сутність хоста { $number }
host-failed = Не вдалося додати на цей комп'ютер: { $reason }
host-needs-clock = Власним сутностям цього комп'ютера потрібен дротовий інтерфейс з апаратним годинником PTP
host-no-ptp4l = ptp4l не відповідає, тому потоки цього комп'ютера не можуть тримати час gPTP
host-elsewhere = triib-endpointd працює на { $interface } від імені іншого користувача або root, тому сутності цього комп'ютера працюють там, а не тут
host-foreign-mrp = Інша програма надсилає декларації MSRP або MVRP на { $interface } з адреси цього комп'ютера і може відкликати те, що потрібно потокам цього комп'ютера
host-alarm-foreign-mrp = Інша програма на цьому комп'ютері надсилає декларації MSRP або MVRP на { $interface }
host-state = Стан
host-streaming = Передавання
host-waiting = Очікування Listener
host-listening = Прослуховування
host-bound = Прив'язано, очікування Talker
host-unbound = Не прив'язано
host-audio-from = Звук із
host-audio-to = Звук до
host-channels = Канали
host-virtual-device = Віртуальний пристрій
host-add-stream = Додати потік
host-add-audio-input = Аудіовхід
host-add-audio-output = Аудіовихід
host-add-clock-input = Вхід медіасинхронізації
host-add-clock-output = Вихід медіасинхронізації
host-remove-stream = Вилучити потік
host-silence = Тиша
host-tone = Тестовий тон
host-nowhere = Нікуди
host-default-device = Типовий пристрій
host-remove = Вилучити з цього комп'ютера

mac-audio-device = Аудіопристрій
mac-no-audio-device = Немає
mac-entity-note = Цю сутність запускає macOS. Застосунки відтворюють і записують через її аудіопристрій, а її конфігурація визначає кількість каналів у кожен бік.
mac-open-audio-midi-setup = Відкрити «Налаштування Audio-MIDI»
mac-avb-off = AVB вимкнено на { $interface }, тому цей Mac не має власної сутності AVB. Увімкніть його в оглядачі мережевих пристроїв програми «Налаштування Audio-MIDI».
