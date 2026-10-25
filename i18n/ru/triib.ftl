## Language

language-name = Русский

## Common

common-close = Закрыть
common-more = Ещё
common-keep-toolbar-shown = Не скрывать панель инструментов
common-auto-hide-toolbar = Автоматически скрывать панель инструментов

## Settings

settings-title = Настройки
settings-general = Общие
settings-appearance = Внешний вид
settings-language = Язык
settings-language-system = Язык системы: { $language }
settings-language-note = В текстовых полях используется системный язык ввода.
settings-appearance-system = Как в системе
settings-appearance-light = Светлая
settings-appearance-dark = Тёмная
settings-colors = Цвета
settings-system-accent = Использовать системный акцентный цвет
settings-accent-picked = Цвета triib строятся на основе цвета ниже.
settings-accent-omarchy = Из темы Omarchy: { $theme }.
settings-accent-desktop = Из акцентного цвета рабочего стола.
settings-accent-none = У рабочего стола нет акцентного цвета, поэтому используется цвет ниже.
settings-motion = Движение
settings-animations = Анимации
settings-animations-note = Пружинящие и плавные переходы при изменениях.
settings-animations-reduced = Рабочий стол просит уменьшить движение, поэтому triib обходится без анимации.
settings-outside-control = Внешнее управление
settings-allow-outside-control = Разрешить внешнее управление
settings-allow-outside-control-note = Агенты, например Claude Code, могут видеть и изменять вашу сеть AVB через triib --mcp. triib спрашивает перед каждым новым агентом.
settings-allowed-agents = Разрешено: { $agents }
settings-forget-agents = Забыть
settings-storage = Хранение
settings-storage-presets = Папка пресетов
settings-storage-entities = Файл сущностей
settings-storage-apply = Применить
settings-storage-choose = Выбрать…
settings-storage-note = Пресеты, уже хранящиеся в старом месте, остаются там; перенесите их, чтобы и дальше ими пользоваться. Настройки triib сохраняются в { $file }.
settings-storage-entities-note = Файл сущностей, которого ещё нет, начинается как копия используемого, и triib-endpointd перезапускается с ним.
dialog-choose-presets = Выберите папку для пресетов
dialog-choose-entities = Выберите папку для файла сущностей
settings-full-path = Укажите полный путь, например ~/Documents/triib.
settings-path-is-folder = { $path } — это папка, а не файл.
settings-folder-missing = Папки { $path } нет. Сначала создайте её или выберите другую.
settings-path-is-file = { $path } — это файл, а не папка.
settings-cannot-write = triib не может записывать в { $path }: { $error }.
app-file-dialog-failed = Не удалось показать окно выбора файла: { $error }
agent-prompt-title = Разрешить агенту { $agent } управлять triib?
agent-prompt-body = { $agent } просит доступ к внешнему управлению triib: видеть сущности в вашей сети и изменять их — подключать потоки, задавать форматы, источники синхронизации и имена. Внешнее управление можно отключить в настройках.
agent-prompt-allow = Разрешить
agent-prompt-deny = Не разрешать

common-cancel = Отмена
common-save = Сохранить
common-not-set = Не задано
common-unnamed = Без имени
common-none = Нет
common-mac-address = MAC-адрес
common-list-separator = {", "}

## Network interfaces

interface-up = активен
interface-link-down = нет линка
interface-wireless = беспроводной
interface-hardware-clock = аппаратные часы
interface-hardware-clock-named = аппаратные часы { $clock }
interface-virtual = виртуальный

## Toolbar

toolbar-choose-interface = Выберите интерфейс
toolbar-interface = Сетевой интерфейс
toolbar-filter-interfaces = Фильтр интерфейсов
toolbar-hide-virtual = Скрыть виртуальные интерфейсы
toolbar-hide-wireless = Скрыть беспроводные интерфейсы
toolbar-hide-unclocked = Скрыть интерфейсы без аппаратных часов
toolbar-connections = Подключения
toolbar-network = Сеть
toolbar-entities = Сущности
toolbar-rediscover = Попросить все сущности объявить о себе
toolbar-rescan = Очистить и заново просканировать все сущности
toolbar-search = Поиск сущностей и потоков
toolbar-presets = Пресеты
toolbar-log = Журнал
toolbar-inspector = Инспектор
toolbar-settings = Настройки
toolbar-close-window = Закрыть окно

## The macOS menu bar, named as in macOS's own apps

menu-about = О программе triib
menu-settings = Настройки…
menu-services = Службы
menu-hide = Скрыть triib
menu-hide-others = Скрыть остальные
menu-show-all = Показать все
menu-quit = Завершить triib
menu-file = Файл
menu-presets = Пресеты…
menu-close-window = Закрыть окно
menu-edit = Правка
menu-cut = Вырезать
menu-copy = Скопировать
menu-paste = Вставить
menu-select-all = Выбрать все
menu-view = Вид
menu-full-screen = Перейти в полноэкранный режим
menu-window = Окно
menu-minimize = Свернуть
menu-zoom = Изменить масштаб
menu-bring-all-to-front = Все окна — на передний план
menu-help = Справка
menu-website = Сайт triib
menu-release-notes = Примечания к выпуску

## The network's state, in place of a view

state-no-interface = Интерфейс не выбран
state-no-interface-note = Выберите интерфейс в сети AVB, чтобы обнаружить сущности.
state-starting = Запуск
state-starting-note = Открывается { $interface }.
state-listening = Прослушивание
state-listening-note = Сущности на { $interface } появляются здесь по мере того, как объявляют о себе.
state-permission-needed = Нужно разрешение
state-npcap-needed = Нужен Npcap
state-get-npcap = Скачать Npcap
state-copy-command = Скопировать команду
state-permission-watch = triib продолжит сам, как только команда будет выполнена.
state-cannot-use = Не удаётся использовать { $interface }
state-try-again = Повторить

## Entity list

entities-none-yet = Сущностей пока нет
entities-none-yet-note = Все сущности в сети с их ролями, классами SR и синхронизацией.
entities-filter = Фильтр сущностей
entities-hide-others = Скрыть сущности других устройств
entities-hide-controllers = Скрыть контроллеры
entities-hide-other-domains = Скрыть сущности вне домена синхронизации этого компьютера
entities-hidden = { $count ->
    [one] { $count } сущность скрыта
    [few] { $count } сущности скрыто
    [many] { $count } сущностей скрыто
   *[other] { $count } сущности скрыто
}

## Inspector

inspector-title = Инспектор
inspector-entity = Сущность
inspector-streams = Потоки
inspector-controls = Регуляторы
inspector-diagnostics = Диагностика
inspector-descriptors = Дескрипторы
inspector-select = Выберите сущность, чтобы увидеть подробности.
inspector-offline = { $entity } не в сети.
inspector-heard = Он не объявляет о себе: triib услышал его в отправленных им командах и показывает его, пока не пройдёт 5 минут без команд.
inspector-rename = Переименовать
inspector-name = Имя
inspector-identify = Идентифицировать
inspector-model-not-read = Модель сущности не прочитана.
inspector-no-streams = Нет потоков.
inspector-no-controls = Нет регуляторов для отображения.
inspector-no-diagnostics = Нет данных об интерфейсах и счётчиках.
inspector-reading = Чтение дескрипторов, прочитано { $count }.
inspector-read-failed = Не удалось прочитать модель сущности: { $reason }.

entity-section = Сущность
entity-name = Имя
entity-triib-controller = Контроллер triib
entity-triib-on = triib на { $host }
entity-triib-here = triib на этом компьютере
entity-controller-on = Контроллер на { $host }
entity-controller-here = Контроллер на этом компьютере
entity-controller-at = Контроллер по адресу { $mac }
entity-group = Группа
entity-product = Продукт
entity-firmware = Прошивка
entity-serial-number = Серийный номер
entity-configuration = Конфигурация
entity-configuration-of = { $name } ({ $number } из { $count })
entity-configuration-holds = { $inputs } вх., { $outputs } вых., потоки по { $channels } кан.
entity-configuration-holds-mixed = { $inputs } вх., { $outputs } вых.
entity-configuration-numbered = Конфигурация { $number }
entity-milan = Milan
entity-media-clock = Медиасинхронизация
entity-clock-domain = Домен синхронизации
entity-sampling-rate = Частота дискретизации
clock-source-numbered = Источник { $index }
rate-pull = pull { $pull }

stream-format = Формат
stream-id = ID потока
stream-destination = Назначение
stream-max-transit = Макс. время прохождения
stream-listeners = Слушатели
stream-listener-gone = { $entity }, вход { $input }, не в сети
stream-disconnect-listener = Отключить

avb-interfaces = Интерфейсы AVB
avb-interface = Интерфейс
avb-interface-clock-identity = Идентификатор часов
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, домен { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Работает
avb-interface-none-reported = Нет данных
avb-interface-path = Путь
avb-interface-own-grandmaster = Сам является Grandmaster
avb-interface-hops = { $count ->
    [one] { $count } переход от Grandmaster
    [few] { $count } перехода от Grandmaster
    [many] { $count } переходов от Grandmaster
   *[other] { $count } перехода от Grandmaster
}
avb-interface-link-up = Линк есть
avb-interface-link-down = Нет линка
avb-interface-grandmaster-changes = Смены Grandmaster
avb-interface-frames-sent = Отправлено кадров
avb-interface-frames-received = Принято кадров
avb-interface-crc-errors = Ошибки CRC

tree-firmware = Прошивка { $version }
tree-descriptor-types = { $count ->
    [one] { $count } тип дескрипторов
    [few] { $count } типа дескрипторов
    [many] { $count } типов дескрипторов
   *[other] { $count } типа дескрипторов
}
tree-clock = Синхронизация
tree-clock-source-from = { $kind }, из { $location } { $index }
tree-clock-domain-using = Использует { $source }
tree-clusters = { $count ->
    [one] { $count } кластер
    [few] { $count } кластера
    [many] { $count } кластеров
   *[other] { $count } кластера
}
tree-maps = { $count ->
    [one] { $count } карта
    [few] { $count } карты
    [many] { $count } карт
   *[other] { $count } карты
}

advert-not-advertised = Не объявлено
advert-identity = Идентификация
advert-entity-id = ID сущности
advert-entity-model = Модель сущности
advert-roles = Роли
advert-talker = Talker
advert-listener = Listener
advert-clock = Синхронизация
advert-gptp-domain = Домен gPTP
advert-sr-classes = Классы SR
advert-indexes = Индексы в модели сущности
advert-identify-control = Регулятор идентификации
advert-avb-interface = Интерфейс AVB
advert-advertising = Объявление
advert-valid-time = Время действия
advert-available-index = Индекс доступности
advert-association = Ассоциация
advert-capabilities = Возможности

## Status bar

status-entities = { $count ->
    [one] { $count } сущность
    [few] { $count } сущности
    [many] { $count } сущностей
   *[other] { $count } сущности
}
status-not-discovering = Обнаружение не идёт
status-discovering = Обнаружение
status-discovering-as = Обнаружение от имени { $controller }
status-stopped = Остановлено из-за ошибки
status-alarm = Тревога
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } и ещё { $count }

## Descriptions of entities, shared by the views

role-talker = Talker { $count }
role-listener = Listener { $count }
role-controller = контроллер
role-none = нет ролей
classes-a-and-b = A и B
clock-no-gptp = Нет gPTP

read-not-read = Не прочитано
read-reading = Чтение, прочитано { $count }
read-ready-unreadable = Готово, не читается: { $count }
read-ready-cached = Готово, из кэша
read-ready = Готово
read-failed = Ошибка: { $reason }

milan-no = Нет
milan-before-1-3 = до 1.3
milan-certified = { $version }, сертификация { $certification }
milan-not-certified = { $version }, без сертификации

outcome-status = статус { $status }
outcome-no-response = нет ответа
outcome-not-possible = невозможно
outcome-connect = Не удалось подключить { $talker } к { $listener }: { $reason }.
outcome-disconnect = Не удалось отключить { $listener }: { $reason }.
outcome-disconnect-listener = Не удалось отключить { $listener } от { $talker }: { $reason }.
outcome-identify = Не удалось идентифицировать { $entity }: { $reason }.
outcome-rename = Не удалось переименовать { $what } в «{ $name }»: { $reason }.
outcome-rename-group = Не удалось переименовать группу сущности { $entity } в «{ $name }»: { $reason }.
outcome-format-streaming = Не удалось изменить формат потока { $stream }: идёт передача. Сначала отключите поток.
outcome-format = Не удалось изменить формат потока { $stream }: { $reason }.
outcome-max-transit-time = Не удалось изменить макс. время прохождения потока { $stream }: { $reason }.
outcome-max-transit-time-streaming = Не удалось изменить макс. время прохождения потока { $stream }: идёт передача. Сначала отключите поток.
outcome-sampling-rate = Не удалось изменить частоту дискретизации сущности { $entity }: { $reason }.
outcome-clock-source = Не удалось изменить источник синхронизации сущности { $entity }: { $reason }.
outcome-configuration = Не удалось изменить конфигурацию сущности { $entity }: { $reason }.
outcome-configuration-streaming = Сущность { $entity } сохраняет конфигурацию, пока идут её потоки: сначала отключите их.
outcome-map = Не удалось назначить канал в сущности { $entity }: { $reason }.
outcome-unmap = Не удалось снять назначение канала в сущности { $entity }: { $reason }.
outcome-control = Не удалось задать «{ $control }» в сущности { $entity }: { $reason }.
outcome-control-numbered = Не удалось задать регулятор { $index } в сущности { $entity }: { $reason }.

stream-not-connected = Не подключено
stream-from = От { $stream }
stream-from-receiving = От { $stream }, приём
stream-from-waiting = От { $stream }, ожидание Talker
stream-from-failed = От { $stream }, резервирование Talker не удалось: { $reason }
stream-sending-to = Отправка на { $destination }

failure-no-response = сущность не ответила
failure-refused = отказ со статусом { $status }
failure-malformed = ответ не удалось декодировать
failure-on-this-computer = сущность работает на этом компьютере; прочитайте её с другого

msrp-failure-1 = недостаточно полосы пропускания
msrp-failure-2 = недостаточно ресурсов коммутатора
msrp-failure-3 = недостаточно полосы для класса трафика
msrp-failure-4 = ID потока используется другим Talker
msrp-failure-5 = адрес назначения уже используется
msrp-failure-6 = вытеснен потоком более высокого ранга
msrp-failure-7 = заявленная задержка изменилась
msrp-failure-8 = исходящий порт не поддерживает AVB
msrp-failure-9 = используйте другой адрес назначения
msrp-failure-10 = закончились ресурсы MSRP
msrp-failure-11 = закончились ресурсы MMRP
msrp-failure-12 = невозможно сохранить адрес назначения
msrp-failure-13 = приоритет не является приоритетом класса SR
msrp-failure-14 = кадры слишком велики для среды передачи
msrp-failure-15 = достигнут предел входящих портов (fan-in)
msrp-failure-16 = изменилось первое значение зарегистрированного потока
msrp-failure-17 = VLAN заблокирована на исходящем порту
msrp-failure-18 = тегирование VLAN отключено на исходящем порту
msrp-failure-19 = несовпадение приоритета класса SR
msrp-failure-unknown = неизвестная причина
msrp-failure-at = { $reason }, на коммутаторе { $bridge }

## Entity list columns

column-vendor = Производитель
column-entity-image = Изображение сущности
column-vendor-image = Изображение производителя
column-model = Модель
column-state = Состояние
column-entity-model-id = ID модели сущности
column-talker-streams = Потоки Talker
column-listener-streams = Потоки Listener
column-avb-lite = AVB Lite
column-egress = Исходящий трафик
column-wireless = Беспроводная сеть

## Settings file

settings-no-place = Негде хранить настройки: домашняя папка неизвестна.
settings-unusable = Не удалось использовать { $path }: { $error }.
settings-unsaved = Не удалось сохранить { $path }: { $error }.

column-remove = Удалить столбец
column-move-left = Переместить влево
column-move-right = Переместить вправо
column-add = Добавить столбец
common-percent = { $value }{" "}%

## Network view

netmap-empty = Сети для отображения пока нет
netmap-empty-note = Сущности появятся здесь, когда будут прочитаны и сообщат свой домен синхронизации.
netmap-focus-clock-path = Путь синхронизации { $name }
netmap-focus-streams = Потоки { $name }
netmap-showing = Показано: { $what }
netmap-devices = { $count ->
    [one] { $count } устройство
    [few] { $count } устройства
    [many] { $count } устройств
   *[other] { $count } устройства
}
netmap-bridges = { $count ->
    [one] { $count } коммутатор
    [few] { $count } коммутатора
    [many] { $count } коммутаторов
   *[other] { $count } коммутатора
}
netmap-show-map = Показать карту
netmap-show-details = Показать подробности
stream-numbered = Поток { $index }
netmap-bridge = Коммутатор
netmap-access-point = Точка доступа
netmap-plain-switch = Коммутатор без AVB
netmap-device = Устройство
netmap-this-computer = Этот компьютер
netmap-connected = Подключено
netmap-advertised = Объявлено, нет готового Listener
netmap-advertised-unknown-domain = Объявлено, нет готового Listener (домен синхронизации { $listener } неизвестен)
netmap-failed-at = Резервирование не удалось на { $bridge }: { $reason }
netmap-failed = Резервирование не удалось: { $reason }
netmap-no-bridge-on = На { $interface } не слышно коммутатора
netmap-cannot-listen-on = Не удаётся слушать gPTP на { $interface }
netmap-behind-plain-switch = За коммутатором без AVB
netmap-on-this-computer = На этом компьютере
netmap-path-not-reported = Путь не сообщён
netmap-gptp-not-reported = Нет данных gPTP
netmap-domain = Домен gPTP { $domain }
netmap-ptp-domain = Домен PTP { $domain }
netmap-domain-unknown = Домен неизвестен
netmap-synced = Синхронизировано
netmap-sync = Синхронизация
netmap-not-synced = Не синхронизировано
netmap-triib-on = triib на { $interface }
netmap-through-count = транзитом: { $count }
netmap-out = исходящих: { $count }
netmap-in = входящих: { $count }
netmap-failed-count = со сбоем: { $count }
netmap-advertised-only = Только объявлено
netmap-failed-state = Сбой
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-sending-unheard = Отправляет, нет привязанного известного Listener
netmap-no-listener = Нет известного Listener
netmap-apart-no-path = Путь не сообщён; синхронизируется с Grandmaster { $grandmaster }
netmap-apart-unreported = Состояние gPTP не сообщено
netmap-apart-no-neighbor = На интерфейсе этого компьютера не слышно коммутатора
netmap-apart-cannot-listen = Этот компьютер не может слушать gPTP на своём интерфейсе
netmap-apart-plain-switch = Этот компьютер находится за коммутатором без AVB, и ни одно устройство за ним не работает там в AVB Lite, поэтому компьютер не входит ни в один домен
netmap-apart-on-this-computer = Работает на этом компьютере; чтобы увидеть состояние gPTP, прочитайте её с другого компьютера
netmap-switch-state = Не участвует в AVB: обозначает коммутаторы перед устройствами ниже, которые не нашли AVB-коммутатора и перешли на AVB Lite, синхронизируясь с одним и тем же Grandmaster PTP
netmap-clock-tree = Дерево синхронизации
netmap-no-grandmaster = Не слышно Grandmaster
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Требует внимания
netmap-nodes-below = Узлы ниже
netmap-bridges-below = Коммутаторы ниже
netmap-clock-path = Путь синхронизации
netmap-hops = Переходов от Grandmaster
netmap-link-delay = Задержка линка
netmap-bridge-port = Порт коммутатора
netmap-link-drops = Обрывы линка
netmap-synced-to-grandmaster = Синхронизировано с Grandmaster
netmap-host-no-gptp = Не синхронизировано: на этом компьютере не работает gPTP
netmap-host-no-ptp = Не синхронизировано: этот компьютер не следует за Grandmaster этого домена
netmap-link-no-gptp = Не синхронизировано: на его линке не работает gPTP
netmap-ptp-offset-high = Не синхронизировано: { $offset } от Grandmaster, больше 50 µs, допустимых в AVB Lite
netmap-ptp-no-offset = Не синхронизировано: смещение от Grandmaster не измерено
netmap-audio = Аудио
netmap-media-clock-streams = Потоки медиасинхронизации
netmap-audio-streams = Аудиопотоки
netmap-bound = привязано: { $count }
netmap-flowing = Передаётся
netmap-advertised-state = Объявлено
netmap-media-clock-stream = Поток медиасинхронизации
netmap-audio-stream = Аудиопоток
netmap-reaches = Доходит до
netmap-passing-count = { $count ->
    [one] { $count } транзитный поток
    [few] { $count } транзитных потока
    [many] { $count } транзитных потоков
   *[other] { $count } транзитного потока
}
netmap-through = Через
netmap-passing-through = Транзитом
netmap-sending = Отправляет
netmap-receiving = Принимает
netmap-problems = Проблемы
netmap-help-back = Щёлкните по фону, чтобы вернуться к обзору.
netmap-help-stream = Щёлкните по потоку, чтобы открыть его в инспекторе, или по фону, чтобы вернуться к обзору.
netmap-help-clock = Синхронизация идёт от Grandmaster через каждый коммутатор ко всем узлам дерева. Серая пунктирная линия — линк, на котором не работает gPTP. Щёлкните по устройству или его проводу, чтобы открыть его путь синхронизации в инспекторе; щёлкните по фону, чтобы снять выделение.
netmap-help-domains = Каждая область — это домен синхронизации. В домене gPTP синхронизация идёт от Grandmaster вниз через AVB-коммутаторы. В домене PTP устройства работают в AVB Lite и следуют за своим Grandmaster из конца в конец, синхронизированы в пределах 50 µs; пунктирная карточка обозначает коммутаторы без AVB, за которыми часть из них перешла на AVB Lite. Устройство, которое ни с кем не делит свой домен, находится в отдельной области.
netmap-help-media-clock = Только потоки медиасинхронизации (CRF), нарисованные так же, как аудио: один провод для кадров каждого потока на каждом линке, цвет по Talker. Щёлкните по проводу, чтобы открыть его поток в инспекторе, или по устройству, чтобы увидеть его потоки; щёлкните по фону, чтобы снять выделение.
netmap-help-audio = Каждый провод — это кадры одного потока на одном линке; он входит в каждый коммутатор на пути и выходит из него. У multicast-потока один провод, пока пути к его Listener идут вместе, и он ветвится там, где они расходятся; у unicast-потока, какой может отправлять AVB Lite, свой провод от Talker к каждому Listener. Цвет зависит от Talker: у каждого Talker свой оттенок, а каждый его поток — вариация этого оттенка на всех ветвях. Движущиеся точки означают, что аудио передаётся, даже если Talker отправляет поток без известного Listener; неподвижная красная линия — неудавшееся резервирование, неподвижная серая — объявленный поток без готового Listener; обе обрываются там, где заканчивается резервирование. Устройства в среднем столбце подключены напрямую к коммутатору Grandmaster. Щёлкните по проводу, чтобы открыть его поток в инспекторе, по общему проводу, чтобы увидеть всех его Listener, или по устройству, чтобы увидеть его потоки; щёлкните по фону, чтобы снять выделение.

## Connections

matrix-nothing-shown = Нет потоков для отображения
matrix-nothing-shown-note = Измените поиск или фильтры, чтобы увидеть больше потоков.
matrix-empty = Нет потоков для подключения
matrix-empty-note = Потоки Talker и Listener встретятся здесь, когда будут прочитаны сущности, у которых они есть.
matrix-all-streams = Все потоки
matrix-filter = Фильтр потоков
matrix-hide-unconnectable = Скрыть потоки, которые нельзя подключить
matrix-hide-lite = Скрыть сущности, работающие в AVB Lite
matrix-hide-wireless = Скрыть беспроводные сущности
matrix-hidden = { $count ->
    [one] { $count } поток скрыт
    [few] { $count } потока скрыто
    [many] { $count } потоков скрыто
   *[other] { $count } потока скрыто
}
matrix-own = Выходы сущности не подключаются к её собственным входам.
matrix-working = Выполняется.
matrix-waiting-change = Ожидание последнего изменения этого входа.
matrix-connected = Подключено, идёт приём. Щёлкните, чтобы отключить.
matrix-bound-waiting = Привязано, ожидание потока Talker. Щёлкните, чтобы отключить.
matrix-bound-failed = Привязано, но резервирование Talker не удалось: { $reason }. Щёлкните, чтобы отключить.
matrix-bound-formats-differ = Привязано, но форматы различаются: Talker отправляет { $sent }, вход настроен на { $set }. Щёлкните, чтобы отключить.
matrix-formats-match = Форматы совпадают ({ $format }). Щёлкните, чтобы подключить.
matrix-format-must-change = Вход принимает { $sent }, но настроен на { $set }, поэтому может не воспроизводить звук, пока формат не изменится. Щёлкните, чтобы выбрать формат.
matrix-format-input = Изменить вход на
matrix-format-both = Изменить оба на
matrix-format-talker-busy = Talker передаёт поток другим Listener, поэтому его формат не меняется.
matrix-format-anyway = Подключить, не меняя форматы
matrix-incompatible = Вход не принимает { $sent }. Он настроен на { $set }.
matrix-other-domain = Они ведут время в разных доменах синхронизации, поэтому между ними не может пройти ни один поток.
matrix-group-none = Не подключено. Разверните, чтобы подключать потоки по одному.
matrix-group-connected = Подключено: { $count }. Разверните, чтобы увидеть каждое подключение.
matrix-outputs-expand = { $count ->
    [one] { $count } выход потока. Щёлкните по стрелке, чтобы развернуть, или по имени, чтобы открыть в инспекторе.
    [few] { $count } выхода потоков. Щёлкните по стрелке, чтобы развернуть, или по имени, чтобы открыть в инспекторе.
    [many] { $count } выходов потоков. Щёлкните по стрелке, чтобы развернуть, или по имени, чтобы открыть в инспекторе.
   *[other] { $count } выхода потоков. Щёлкните по стрелке, чтобы развернуть, или по имени, чтобы открыть в инспекторе.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } выход потока. Щёлкните по стрелке, чтобы свернуть, или по имени, чтобы открыть в инспекторе.
    [few] { $count } выхода потоков. Щёлкните по стрелке, чтобы свернуть, или по имени, чтобы открыть в инспекторе.
    [many] { $count } выходов потоков. Щёлкните по стрелке, чтобы свернуть, или по имени, чтобы открыть в инспекторе.
   *[other] { $count } выхода потоков. Щёлкните по стрелке, чтобы свернуть, или по имени, чтобы открыть в инспекторе.
}
matrix-inputs-expand = { $count ->
    [one] { $count } вход потока. Щёлкните по стрелке, чтобы развернуть, или по имени, чтобы открыть в инспекторе.
    [few] { $count } входа потоков. Щёлкните по стрелке, чтобы развернуть, или по имени, чтобы открыть в инспекторе.
    [many] { $count } входов потоков. Щёлкните по стрелке, чтобы развернуть, или по имени, чтобы открыть в инспекторе.
   *[other] { $count } входа потоков. Щёлкните по стрелке, чтобы развернуть, или по имени, чтобы открыть в инспекторе.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } вход потока. Щёлкните по стрелке, чтобы свернуть, или по имени, чтобы открыть в инспекторе.
    [few] { $count } входа потоков. Щёлкните по стрелке, чтобы свернуть, или по имени, чтобы открыть в инспекторе.
    [many] { $count } входов потоков. Щёлкните по стрелке, чтобы свернуть, или по имени, чтобы открыть в инспекторе.
   *[other] { $count } входа потоков. Щёлкните по стрелке, чтобы свернуть, или по имени, чтобы открыть в инспекторе.
}
matrix-stream-inspect = { $detail } Щёлкните, чтобы открыть { $entity } в инспекторе.
matrix-point = Наведите на ячейку
matrix-point-note = — увидите её Talker и Listener и совместимы ли их форматы.
matrix-legend-waiting = Привязано, ожидание потока
matrix-legend-trouble = Привязано, есть проблема
matrix-legend-open = Можно подключить
matrix-legend-change = Сначала нужно изменить формат входа
matrix-legend-incompatible = Форматы несовместимы
matrix-legend-other-domain = Другой домен синхронизации
matrix-talker-outputs = Выходы Talker
matrix-listener-inputs = Входы Listener

common-thousands-separator = {" "}

## Diagnostics

diag-since-start = Подсчитано с момента запуска сущности.
diag-stream-input = Вход потока
diag-stream-output = Выход потока
diag-locked = { $count ->
    [0] без захвата синхронизации
    [one] { $number } захват синхронизации
    [few] { $number } захвата синхронизации
    [many] { $number } захватов синхронизации
   *[other] { $number } захвата синхронизации
}
diag-lost-lock = { $count ->
    [0] без потери синхронизации
    [one] { $number } потеря синхронизации
    [few] { $number } потери синхронизации
    [many] { $number } потерь синхронизации
   *[other] { $number } потери синхронизации
}
diag-frames-in = { $count ->
    [one] { $number } принятый кадр
    [few] { $number } принятых кадра
    [many] { $number } принятых кадров
   *[other] { $number } принятого кадра
}
diag-frames-out = { $count ->
    [one] { $number } отправленный кадр
    [few] { $number } отправленных кадра
    [many] { $number } отправленных кадров
   *[other] { $number } отправленного кадра
}
diag-media-locked = { $count ->
    [0] без захвата медиасинхронизации
    [one] { $number } захват медиасинхронизации
    [few] { $number } захвата медиасинхронизации
    [many] { $number } захватов медиасинхронизации
   *[other] { $number } захвата медиасинхронизации
}
diag-lost-media-lock = { $count ->
    [0] без потери медиасинхронизации
    [one] { $number } потеря медиасинхронизации
    [few] { $number } потери медиасинхронизации
    [many] { $number } потерь медиасинхронизации
   *[other] { $number } потери медиасинхронизации
}
diag-interrupted = { $count ->
    [0] без прерываний
    [one] { $number } прерывание
    [few] { $number } прерывания
    [many] { $number } прерываний
   *[other] { $number } прерывания
}
diag-out-of-sequence = { $count ->
    [one] { $number } кадр с нарушением последовательности
    [few] { $number } кадра с нарушением последовательности
    [many] { $number } кадров с нарушением последовательности
   *[other] { $number } кадра с нарушением последовательности
}
diag-media-resets = { $count ->
    [one] { $number } сброс медиа
    [few] { $number } сброса медиа
    [many] { $number } сбросов медиа
   *[other] { $number } сброса медиа
}
diag-timestamps-uncertain = { $count ->
    [0] метки времени достоверны
    [one] недостоверные метки времени { $number } раз
    [few] недостоверные метки времени { $number } раза
    [many] недостоверные метки времени { $number } раз
   *[other] недостоверные метки времени { $number } раза
}
diag-no-timestamp = { $count ->
    [one] { $number } кадр без метки времени
    [few] { $number } кадра без метки времени
    [many] { $number } кадров без метки времени
   *[other] { $number } кадра без метки времени
}
diag-unsupported-format = { $count ->
    [one] { $number } кадр в неподдерживаемом формате
    [few] { $number } кадра в неподдерживаемом формате
    [many] { $number } кадров в неподдерживаемом формате
   *[other] { $number } кадра в неподдерживаемом формате
}
diag-late = { $count ->
    [one] { $number } опоздавший кадр
    [few] { $number } опоздавших кадра
    [many] { $number } опоздавших кадров
   *[other] { $number } опоздавшего кадра
}
diag-early = { $count ->
    [one] { $number } преждевременный кадр
    [few] { $number } преждевременных кадра
    [many] { $number } преждевременных кадров
   *[other] { $number } преждевременного кадра
}
diag-started = { $count ->
    [0] без запусков
    [one] { $number } запуск
    [few] { $number } запуска
    [many] { $number } запусков
   *[other] { $number } запуска
}
diag-stopped = { $count ->
    [0] без остановок
    [one] { $number } остановка
    [few] { $number } остановки
    [many] { $number } остановок
   *[other] { $number } остановки
}
diag-reservation-failed = резервирование Talker не удалось: { $reason }
diag-latency = накопленная задержка { $microseconds } µs

## AVB Lite

lite-active = Активен
lite-active-untagged = Активен, без тега
lite-active-vlan = Активен, VLAN { $vlan }
lite-capable = Поддерживается
lite-mode = Режим
lite-mode-capable = AVB (поддерживает AVB Lite)
lite-because = Причина
lite-fallback-none = причина не указана
lite-fallback-endpoint = дошла декларация другого оконечного устройства, значит, между ними нет AVB-коммутатора
lite-fallback-unanswered = девять запросов peer delay остались без ответа
lite-fallback-responders = на один запрос peer delay ответили двое или больше, значит, коммутатор не поддерживает AVB
lite-fallback-configured = так задал оператор или контроллер
lite-fallback-other = причина, которую профиль не называет
lite-other-profile = Другой профиль
lite-ptp-domain = { $profile }, домен { $domain }
lite-offset = Смещение
lite-offset-from = { $offset } от { $grandmaster }
lite-media-vlan = VLAN для медиа
lite-untagged = Без тега
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] До { $count } Listener на поток, затем multicast
    [few] До { $count } Listener на поток, затем multicast
    [many] До { $count } Listener на поток, затем multicast
   *[other] До { $count } Listener на поток, затем multicast
}
lite-link = Линк
lite-bandwidth = Полоса пропускания
lite-egress-of = { $used } из { $link }, { $share }
lite-egress-of-assumed = { $used } из { $link }, { $share }, предполагается гигабитный линк
lite-egress-reported = Так считает сама сущность по своим допущенным потокам.
lite-egress-worked-out = Рассчитано по форматам её подключённых выходов потоков.
lite-alarm-offset = Смещение PTP { $offset }, больше 50 µs, допустимых в AVB Lite
lite-alarm-egress = Исходящий трафик — { $share } линка, больше допустимых для потоков { $limit }

## AVB Wireless

wireless-station = Станция
wireless-access-point = Точка доступа
wireless-role = Роль
wireless-mode = Режим
wireless-time = Время
wireless-mode-a-ftm = Mode A, 802.1AS через FTM
wireless-mode-a-tm = Mode A, 802.1AS через TM
wireless-mode-b = Mode B, из кадров Beacon
wireless-no-time = Без синхронизации времени
wireless-other-mode = Режим, который профиль не называет
wireless-locked = Синхронизация захвачена
wireless-holdover = Режим удержания
wireless-not-locked = Нет захвата синхронизации
wireless-row-locked = Станция, захват
wireless-row-holdover = Станция, удержание
wireless-row-not-locked = Станция, нет захвата
wireless-row-access-point = { $count ->
    [one] Точка доступа, { $count } станция
    [few] Точка доступа, { $count } станции
    [many] Точка доступа, { $count } станций
   *[other] Точка доступа, { $count } станции
}
wireless-link = Линк
wireless-channel = канал { $channel }
wireless-not-known = Неизвестно
wireless-signal = Сигнал
wireless-rate = Скорость
wireless-ftm-valid = { $share } достоверных
wireless-rtt = круговая задержка { $rtt }
wireless-bursts = { $count ->
    [one] серии из { $count } кадра
    [few] серии из { $count } кадров
    [many] серии из { $count } кадров
   *[other] серии из { $count } кадра
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = точка доступа назначает серии FTM не из трёх и не из двух кадров
wireless-reason-measurement = ни FTM, ни TM с точкой доступа
wireless-reason-signaling = нет сообщения Signaling gPTP-capable от точки доступа
wireless-reason-other = причина, которую профиль не называет
wireless-servo = Ошибка подстройки
wireless-stations = Станции
wireless-station-count = { $count ->
    [one] { $count } станция
    [few] { $count } станции
    [many] { $count } станций
   *[other] { $count } станции
}
wireless-no-ftm = без FTM
wireless-unserved = Listener без обслуживания
wireless-frame-budget = Бюджет кадров
wireless-frame-use = { $budget ->
    [one] { $used } из { $budget } кадра в секунду
    [few] { $used } из { $budget } кадров в секунду
    [many] { $used } из { $budget } кадров в секунду
   *[other] { $used } из { $budget } кадра в секунду
}
wireless-stream-frames = Кадры потоков
wireless-frames-of = станциям: { $readdressed }, без Listener: { $unmapped }, отброшено: { $dropped }, от станций: { $restored }
wireless-class-a-allowed = Разрешён
wireless-class-a-not-allowed = Не разрешён
wireless-alarm-not-locked = Нет захвата синхронизации времени Wi-Fi от точки доступа
wireless-alarm-holdover = Время Wi-Fi в режиме удержания, синхронизация от точки доступа потеряна
wireless-alarm-unserved = { $count ->
    [one] { $count } Listener на порту Wi-Fi без обслуживания, сверх лимита unicast
    [few] { $count } Listener на порту Wi-Fi без обслуживания, сверх лимита unicast
    [many] { $count } Listener на порту Wi-Fi без обслуживания, сверх лимита unicast
   *[other] { $count } Listener на порту Wi-Fi без обслуживания, сверх лимита unicast
}

## Log

log-all = Все
log-warnings = Предупреждения
log-pause = Пауза
log-resume = Продолжить
log-clear = Очистить
log-empty = Здесь появляются все кадры ATDECC, которые triib отправляет и слышит, новые сверху.
log-none-match = Ни один сохранённый кадр не соответствует фильтру.
log-frames = { $count ->
    [one] { $count } кадр
    [few] { $count } кадра
    [many] { $count } кадров
   *[other] { $count } кадра
}
log-shown-of = { $all ->
    [one] { $shown } из { $all } кадра
    [few] { $shown } из { $all } кадров
    [many] { $shown } из { $all } кадров
   *[other] { $shown } из { $all } кадра
}
log-sent = Отправлено
log-heard = Принято
log-not-decoded = Не декодировано
log-warning-short = { $missing ->
    [one] Поле control_data_length выходит за конец кадра на { $missing } октет.
    [few] Поле control_data_length выходит за конец кадра на { $missing } октета.
    [many] Поле control_data_length выходит за конец кадра на { $missing } октетов.
   *[other] Поле control_data_length выходит за конец кадра на { $missing } октета.
}
log-warning-undecodable = Кадр не декодируется: { $error }.
log-warning-long-acmp = Кадр в длинной форме ACMP, которую сущность Milan отправлять не должна (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Назначение каналов
mapping-inputs = Входы
mapping-outputs = Выходы
mapping-port = порт { $number }
mapping-fixed = фиксировано
mapping-not-read = Ещё не прочитано.
mapping-no-clusters = Нет кластеров.
mapping-no-streams = Нет аудиопотоков.
mapping-none = Нет назначений.
mapping-not-mapped = Не назначено
mapping-cluster-numbered = Кластер { $index }

## Presets

presets-note = Пресет хранит источники синхронизации, частоты дискретизации, форматы потоков, регуляторы и подключения каждой сущности. При вызове меняется то, что отличается.
presets-none = Пресетов пока нет.
presets-connections = { $count ->
    [one] { $count } подключение
    [few] { $count } подключения
    [many] { $count } подключений
   *[other] { $count } подключения
}
presets-recall = Вызвать
presets-delete = Удалить
presets-no-place = Негде хранить пресеты: домашняя папка неизвестна.
presets-undeletable = Не удалось удалить { $path }: { $error }.
presets-saved = { $count ->
    [one] Пресет «{ $name }» сохранён: { $count } сущность.
    [few] Пресет «{ $name }» сохранён: { $count } сущности.
    [many] Пресет «{ $name }» сохранён: { $count } сущностей.
   *[other] Пресет «{ $name }» сохранён: { $count } сущности.
}
presets-nothing-differs = Ничто не отличается от «{ $name }».
presets-recalling = { $count ->
    [one] Вызов «{ $name }»: { $count } изменение.
    [few] Вызов «{ $name }»: { $count } изменения.
    [many] Вызов «{ $name }»: { $count } изменений.
   *[other] Вызов «{ $name }»: { $count } изменения.
}
presets-missing = { $report } Нет в сети или не прочитаны: { $missing }.
presets-deleted = Пресет «{ $name }» удалён.
presets-host-note = Он также хранит собственные сущности этого компьютера и перезапускает их при вызове.
presets-host-endpoints = { $count } на этом компьютере
presets-starting-host = Запускаются сущности этого компьютера для «{ $name }»; остальное последует, когда они вернутся.

## Controls

control-numbered = Регулятор { $index }
control-not-shown = Здесь не показывается
control-option = Вариант { $number }

## Network errors

network-permission = triib нужно разрешение на отправку и приём необработанных кадров Ethernet.
network-needs-npcap = triib нужен Npcap для отправки и приёма необработанных кадров Ethernet.
network-npcap-administrators = Npcap разрешает отправлять и принимать необработанные кадры Ethernet только администраторам. Запустите triib от имени администратора или переустановите Npcap без параметра «только для администраторов».

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own entities

host-add-entity = Добавить сущность
host-new-entity = Сущность хоста { $number }
host-failed = Не удалось добавить на этот компьютер: { $reason }
host-needs-clock = Собственным сущностям этого компьютера нужен проводной интерфейс с аппаратными часами PTP
host-no-ptp4l = ptp4l не отвечает, поэтому потоки этого компьютера не могут держать время gPTP
host-starting = Запуск
host-starting-entity = Запуск { $name }
host-setting-up = Настройка { $interface } для AVB
host-no-link = Нет связи на { $interface }
host-starting-ptp4l = Запуск ptp4l
host-no-linuxptp = linuxptp не установлен, поэтому потоки этого компьютера не могут держать время gPTP
host-waiting-network = Ожидание сети
host-elsewhere = triib-endpointd работает на { $interface } от имени другого пользователя или root, поэтому сущности этого компьютера работают там, а не здесь
host-foreign-mrp = Другая программа отправляет декларации MSRP или MVRP на { $interface } с адреса этого компьютера и может отозвать то, что нужно потокам этого компьютера
host-alarm-foreign-mrp = Другая программа на этом компьютере отправляет декларации MSRP или MVRP на { $interface }
host-state = Состояние
host-streaming = Передача
host-waiting = Ожидание Listener
host-listening = Прослушивание
host-bound = Привязан, ожидание Talker
host-unbound = Не привязан
host-audio-from = Звук из
host-audio-to = Звук в
host-channels = Каналы
host-virtual-device = Виртуальное устройство
host-add-stream = Добавить поток
host-add-audio-input = Аудиовход
host-add-audio-output = Аудиовыход
host-add-clock-input = Вход медиасинхронизации
host-add-clock-output = Выход медиасинхронизации
host-remove-stream = Удалить поток
host-silence = Тишина
host-tone = Тестовый тон
host-nowhere = Никуда
host-default-device = Устройство по умолчанию
host-remove = Удалить с этого компьютера

mac-audio-device = Аудиоустройство
mac-no-audio-device = Нет, пока не подключён ни один поток
mac-entity-note = Эту сущность запускает macOS. Пока подключён хотя бы один её поток, macOS создаёт аудиоустройство, через которое приложения воспроизводят и записывают, с числом каналов в каждую сторону, заданным конфигурацией.
mac-open-audio-midi-setup = Открыть «Настройка Audio-MIDI»
mac-avb-off = AVB выключен на { $interface }, поэтому у этого Mac нет собственной сущности AVB. Включите его в браузере сетевых устройств программы «Настройка Audio-MIDI».

listen-section = Прослушивание
listen-start = Слушать
listen-stop = Остановить прослушивание
listen-source = Слушает
listen-received = То, что принимает
listen-sent = То, что отправляет в потоке { $number } ({ $channels } кан.)
listen-sent-needs-macos = Чтобы слушать то, что отправляет этот Mac, нужна macOS 14.2 или новее.
listen-channels = Каналы
listen-channel = Канал { $number }
listen-channel-pair = Каналы { $first }–{ $last }
listen-output = Выход
listen-default-output = Выход по умолчанию
listen-gain = Усиление
listen-gain-db = { $db } dB
listen-starting = Запуск…
listen-waiting-input = Ожидание { $device }: macOS создаёт его, пока подключён хотя бы один поток сущности.
listen-waiting-output = Ожидание { $device }.
listen-playing = Воспроизведение, пики { $db } dB
listen-playing-silent = Воспроизведение, тишина
