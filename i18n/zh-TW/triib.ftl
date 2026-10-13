# triib 的介面文字，繁體中文（台灣）。
# 術語見 docs/GLOSSARY.md 與 docs/glossary/zh-TW.md。

## Language

language-name = 繁體中文

## Common

common-close = 關閉
common-more = 更多
common-keep-toolbar-shown = 一律顯示工具列
common-auto-hide-toolbar = 自動隱藏工具列

## Settings

settings-title = 設定
settings-general = 一般
settings-appearance = 外觀
settings-language = 語言
settings-language-system = 系統預設：{ $language }
settings-language-note = 文字欄位沿用系統的輸入語言。
settings-appearance-system = 跟隨系統
settings-appearance-light = 淺色
settings-appearance-dark = 深色
settings-colors = 色彩
settings-system-accent = 使用系統強調色
settings-accent-picked = triib 的配色由下方色彩產生。
settings-accent-omarchy = 來自 Omarchy 主題「{ $theme }」。
settings-accent-desktop = 來自桌面的強調色。
settings-accent-none = 桌面沒有強調色，因此使用下方色彩。
settings-motion = 動態效果
settings-animations = 動畫
settings-animations-note = 內容變化時以彈性與滑動效果轉場。
settings-animations-reduced = 桌面要求減少動態效果，因此 triib 不播放動畫。
settings-outside-control = 外部控制
settings-allow-outside-control = 允許外部控制
settings-allow-outside-control-note = Claude Code 等代理程式可以透過 triib --mcp 檢視和變更你的 AVB 網路。每個新的代理程式接入前，triib 都會先詢問。
settings-allowed-agents = 已允許：{ $agents }
settings-forget-agents = 忘記
agent-prompt-title = 要允許 { $agent } 控制 triib 嗎？
agent-prompt-body = { $agent } 要求使用 triib 的外部控制：檢視並變更你網路上的實體，連接串流，設定格式、時鐘和名稱。你可以在「設定」中關閉外部控制。
agent-prompt-allow = 允許
agent-prompt-deny = 不允許

common-cancel = 取消
common-save = 儲存
common-not-set = 未設定
common-unnamed = 未命名
common-none = 無
common-mac-address = MAC 位址
common-list-separator = {"，"}

## Network interfaces

interface-up = 已連通
interface-link-down = 連結中斷
interface-wireless = 無線
interface-hardware-clock = 硬體時鐘
interface-hardware-clock-named = 硬體時鐘 { $clock }
interface-virtual = 虛擬

## Toolbar

toolbar-choose-interface = 選擇介面
toolbar-interface = 網路介面
toolbar-show-virtual = 顯示虛擬介面
toolbar-hide-virtual = 隱藏虛擬介面
toolbar-connections = 連線
toolbar-network = 網路
toolbar-entities = 實體
toolbar-rediscover = 要求所有實體重新通告
toolbar-rescan = 清除並重新掃描所有實體
toolbar-search = 搜尋實體與串流
toolbar-presets = 預設集
toolbar-log = 記錄
toolbar-inspector = 檢閱器
toolbar-settings = 設定

## The network's state, in place of a view

state-no-interface = 未選擇介面
state-no-interface-note = 選擇 AVB 網路上的介面以探索實體。
state-starting = 正在啟動
state-starting-note = 正在開啟 { $interface }。
state-listening = 正在監聽
state-listening-note = { $interface } 上的實體發出通告後會顯示於此。
state-permission-needed = 需要權限
state-npcap-needed = 需要 Npcap
state-get-npcap = 取得 Npcap
state-copy-command = 複製指令
state-cannot-use = 無法使用 { $interface }
state-try-again = 重試

## Entity list

entities-none-yet = 尚無實體
entities-none-yet-note = 網路上的每個實體，及其角色、SR 類別與時鐘。

## Inspector

inspector-title = 檢閱器
inspector-entity = 實體
inspector-streams = 串流
inspector-controls = 控制項
inspector-diagnostics = 診斷
inspector-descriptors = 描述元
inspector-select = 選取一個實體以檢視詳細資料。
inspector-offline = { $entity } 已離線。
inspector-rename = 重新命名
inspector-name = 名稱
inspector-identify = 識別
inspector-model-not-read = 尚未讀取其實體模型。
inspector-no-streams = 無串流。
inspector-no-controls = 沒有可顯示的控制項。
inspector-no-diagnostics = 未回報任何介面或計數器。
inspector-reading = 正在讀取描述元，已讀取 { $count } 個。
inspector-read-failed = 無法讀取實體模型：{ $reason }。

entity-section = 實體
entity-name = 名稱
entity-group = 群組
entity-product = 產品
entity-firmware = 韌體
entity-serial-number = 序號
entity-configuration = 組態
entity-configuration-of = { $name }（{ $number }/{ $count }）
entity-milan = Milan
entity-media-clock = 媒體時鐘
entity-clock-domain = 時鐘域
entity-sampling-rate = 取樣率
clock-source-numbered = 時鐘源 { $index }
rate-pull = pull 係數 { $pull }

stream-format = 格式
stream-id = 串流 ID
stream-destination = 目的地
stream-max-transit = 最大傳輸時間

avb-interfaces = AVB 介面
avb-interface = 介面
avb-interface-clock-identity = 時鐘識別碼
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }，域 { $domain }
avb-interface-peer-delay = 對等延遲
avb-interface-running = 執行中
avb-interface-none-reported = 未回報
avb-interface-path = 路徑
avb-interface-own-grandmaster = 本身即為 Grandmaster
avb-interface-hops = 距 Grandmaster { $count } 個躍點
avb-interface-link-up = 連結正常
avb-interface-link-down = 連結中斷
avb-interface-grandmaster-changes = Grandmaster 變更次數
avb-interface-frames-sent = 已傳送訊框數
avb-interface-frames-received = 已接收訊框數
avb-interface-crc-errors = CRC 錯誤

tree-firmware = 韌體 { $version }
tree-descriptor-types = { $count } 種描述元類型
tree-clock = 時鐘
tree-clock-source-from = { $kind }，來自 { $location } { $index }
tree-clock-domain-using = 使用 { $source }
tree-clusters = { $count } 個叢集
tree-maps = { $count } 個對應

advert-not-advertised = 未通告
advert-identity = 身分
advert-entity-id = 實體 ID
advert-entity-model = 實體模型
advert-roles = 角色
advert-talker = 發送端
advert-listener = 接收端
advert-clock = 時鐘
advert-btc = BTC
advert-gptp-domain = gPTP 域
advert-sr-classes = SR 類別
advert-indexes = 實體模型索引
advert-identify-control = 識別控制項
advert-avb-interface = AVB 介面
advert-advertising = 通告
advert-valid-time = 有效時間
advert-available-index = 可用索引
advert-association = 關聯
advert-capabilities = 能力

## Status bar

status-entities = { $count } 個實體
status-not-discovering = 未在探索
status-discovering = 探索中
status-discovering-as = 以 { $controller } 身分探索中
status-stopped = 因錯誤而停止
status-alarm = 警報
status-alarm-of = { $entity }：{ $alarm }
status-alarm-more = { $alarm }，另有 { $count } 項

## Descriptions of entities, shared by the views

role-talker = 發送端 { $count }
role-listener = 接收端 { $count }
role-controller = 控制器
role-none = 無角色
classes-a-and-b = A 與 B
clock-no-gptp = 無 gPTP

read-not-read = 未讀取
read-reading = 正在讀取，已讀取 { $count } 個
read-ready-unreadable = 就緒，{ $count } 個無法讀取
read-ready-cached = 就緒，來自快取
read-ready = 就緒
read-failed = 失敗：{ $reason }

milan-no = 否
milan-before-1-3 = 1.3 之前
milan-certified = { $version }，已通過 { $certification } 認證
milan-not-certified = { $version }，未認證

outcome-status = 狀態 { $status }
outcome-no-response = 無回應
outcome-not-possible = 無法執行
outcome-connect = 無法將 { $talker } 連線至 { $listener }：{ $reason }。
outcome-disconnect = 無法中斷 { $listener } 的連線：{ $reason }。
outcome-identify = 無法讓 { $entity } 執行識別：{ $reason }。
outcome-rename = 無法將 { $what } 重新命名為「{ $name }」：{ $reason }。
outcome-rename-group = 無法將 { $entity } 的群組重新命名為「{ $name }」：{ $reason }。
outcome-format-streaming = 無法變更 { $stream } 的格式：該串流正在傳輸。請先中斷連線。
outcome-format = 無法變更 { $stream } 的格式：{ $reason }。
outcome-max-transit-time = 無法變更 { $stream } 的最大傳輸時間：{ $reason }。
outcome-max-transit-time-streaming = 無法變更 { $stream } 的最大傳輸時間：該串流正在傳輸。請先中斷連線。
outcome-sampling-rate = 無法變更 { $entity } 的取樣率：{ $reason }。
outcome-clock-source = 無法變更 { $entity } 的時鐘源：{ $reason }。
outcome-map = 無法在 { $entity } 上對應通道：{ $reason }。
outcome-unmap = 無法在 { $entity } 上取消通道對應：{ $reason }。
outcome-control = 無法在 { $entity } 上設定「{ $control }」：{ $reason }。
outcome-control-numbered = 無法在 { $entity } 上設定控制項 { $index }：{ $reason }。

stream-not-connected = 未連線
stream-from = 來自 { $stream }
stream-from-receiving = 來自 { $stream }，接收中
stream-from-waiting = 來自 { $stream }，正在等待發送端
stream-from-failed = 來自 { $stream }，發送端的保留失敗：{ $reason }
stream-sending-to = 正在傳送至 { $destination }

failure-no-response = 未回應
failure-refused = 遭拒絕，狀態為 { $status }
failure-malformed = 回應無法解碼
failure-on-this-computer = 它在本機上執行；請從另一台電腦讀取

msrp-failure-1 = 頻寬不足
msrp-failure-2 = 交換器資源不足
msrp-failure-3 = 該流量類別頻寬不足
msrp-failure-4 = 串流 ID 已由其他發送端使用
msrp-failure-5 = 目的位址已在使用中
msrp-failure-6 = 遭更高等級的串流先佔
msrp-failure-7 = 回報的延遲已變更
msrp-failure-8 = 出口埠不支援 AVB
msrp-failure-9 = 請使用其他目的位址
msrp-failure-10 = MSRP 資源耗盡
msrp-failure-11 = MMRP 資源耗盡
msrp-failure-12 = 無法儲存目的位址
msrp-failure-13 = 優先順序不是 SR 類別優先順序
msrp-failure-14 = 訊框長度超出媒介上限
msrp-failure-15 = 已達扇入埠上限
msrp-failure-16 = 已註冊串流的首值已變更
msrp-failure-17 = VLAN 在出口埠遭封鎖
msrp-failure-18 = 出口埠已停用 VLAN 標記
msrp-failure-19 = SR 類別優先順序不符
msrp-failure-unknown = 未知原因
msrp-failure-at = { $reason }，發生於交換器 { $bridge }

## Entity list columns

column-vendor = 廠商
column-model = 型號
column-state = 狀態
column-entity-model-id = 實體模型 ID
column-talker-streams = 發送端串流
column-listener-streams = 接收端串流
column-avb-lite = AVB Lite
column-egress = 出口流量
column-wireless = 無線

## Settings file

settings-no-place = 沒有可儲存設定的位置：無法得知家目錄。
settings-unusable = 無法使用 { $path }：{ $error }。
settings-unsaved = 無法儲存 { $path }：{ $error }。

column-remove = 移除欄
column-move-left = 左移
column-move-right = 右移
column-add = 新增欄
common-percent = { $value }%

## Network view

netmap-empty = 尚無可顯示的網路
netmap-empty-note = 實體經讀取並回報其在 gPTP 樹中的位置後，會顯示於此。
netmap-focus-clock-path = { $name } 的時鐘路徑
netmap-focus-streams = { $name } 的串流
netmap-showing = 正在顯示 { $what }
netmap-devices = { $count } 台裝置
netmap-bridges = { $count } 台交換器
netmap-show-map = 顯示拓樸圖
netmap-show-details = 顯示詳細資料
stream-numbered = 串流 { $index }
netmap-bridge = 交換器
netmap-access-point = 存取點
netmap-device = 裝置
netmap-this-computer = 本機
netmap-connected = 已連線
netmap-advertised = 已通告，沒有就緒的接收端
netmap-advertised-off-tree = 已通告，沒有就緒的接收端（{ $listener } 不在 gPTP 樹上）
netmap-failed-at = 在 { $bridge } 保留失敗：{ $reason }
netmap-failed = 保留失敗：{ $reason }
netmap-no-bridge-on = 在 { $interface } 上未偵測到交換器
netmap-cannot-listen-on = 無法在 { $interface } 上監聽 gPTP
netmap-on-this-computer = 在本機上
netmap-path-not-reported = 未回報路徑
netmap-gptp-not-reported = 未回報 gPTP
netmap-off-tree = 不在 gPTP 樹上
netmap-off-ptp = 不在 PTP 樹上
netmap-not-lite = 未執行 AVB Lite
netmap-lite-not-reported = 未回報 AVB Lite
netmap-synced = 已同步
netmap-not-synced = 未同步
netmap-triib-on = { $interface } 上的 triib
netmap-through-count = 途經 { $count }
netmap-out = 傳送 { $count }
netmap-in = 接收 { $count }
netmap-failed-count = 失敗 { $count }
netmap-advertised-only = 僅通告
netmap-failed-state = 失敗
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = 不在 gPTP 樹上：它本身即為 Grandmaster
netmap-apart-no-path = 未回報其路徑；它跟隨 Grandmaster { $grandmaster }
netmap-apart-unreported = 未回報其 gPTP 狀態
netmap-apart-no-neighbor = 本機介面上未偵測到交換器
netmap-apart-cannot-listen = 本機無法在其介面上監聽 gPTP
netmap-apart-on-this-computer = 它在本機上執行；請從另一台電腦讀取以查看其 gPTP 狀態
netmap-apart-not-lite = 它未執行 AVB Lite，因此不跟隨 Grandmaster
netmap-apart-lite-unreported = 它未回報任何 AVB Lite 資訊，因此無從得知它跟隨誰
netmap-clock-tree = 時鐘樹
netmap-no-grandmaster = 未偵測到 Grandmaster
netmap-grandmaster-is = Grandmaster：{ $grandmaster }
netmap-needs-attention = 需要注意
netmap-nodes-below = 下游節點
netmap-bridges-below = 下游交換器
netmap-clock-path = 時鐘路徑
netmap-hops = 距 Grandmaster 躍點數
netmap-link-delay = 連結延遲
netmap-bridge-port = 交換器埠
netmap-link-drops = 連結中斷次數
netmap-synced-to-grandmaster = 已與 Grandmaster 同步
netmap-host-no-gptp = 未同步：本機未執行 gPTP
netmap-link-no-gptp = 未同步：其連結未執行 gPTP
netmap-ptp-offset-high = 未同步：相對 Grandmaster 偏移 { $offset }，超出 AVB Lite 允許的 50 µs
netmap-ptp-no-offset = 未同步：未測得相對 Grandmaster 的偏移
netmap-audio = 音訊
netmap-media-clock-streams = 媒體時鐘串流
netmap-audio-streams = 音訊串流
netmap-bound = 已綁定 { $count } 個
netmap-flowing = 傳輸中
netmap-advertised-state = 已通告
netmap-media-clock-stream = 媒體時鐘串流
netmap-audio-stream = 音訊串流
netmap-reaches = 到達
netmap-passing-count = { $count } 個串流途經此處
netmap-through = 途經
netmap-passing-through = 途經的串流
netmap-sending = 傳送
netmap-receiving = 接收
netmap-problems = 問題
netmap-help-back = 按一下背景即可返回概覽。
netmap-help-stream = 按一下串流即可檢閱，或按一下背景返回概覽。
netmap-help-ptp = 在 AVB Lite 中，時鐘從 Grandmaster 端對端傳到每台裝置，途經的交換器不參與其中，因此均不顯示。裝置只要跟隨 Grandmaster 的偏移在 50 µs 以內，就處於同步狀態。按一下裝置或其線路即可檢閱其時鐘；按一下背景即可清除。
netmap-help-clock = 時鐘從 Grandmaster 出發，經由每台交換器傳到樹上的每個節點。灰色虛線表示未執行 gPTP 的連結。按一下裝置或其線路即可檢閱其時鐘路徑；按一下背景即可清除。
netmap-help-media-clock = 僅顯示媒體時鐘（CRF）串流，畫法與音訊相同：每個串流一條線路，依發送端著色。按一下線路即可檢閱其串流，按一下裝置可檢視其串流；按一下背景即可清除。
netmap-help-audio = 每個串流都有自己的線路，進出它經過的每台交換器。顏色依發送端區分：每個發送端有一種色相，其串流為該色相的不同深淺。移動的圓點表示音訊正在傳輸；靜止的紅線表示保留失敗，靜止的灰線表示已通告但沒有就緒的接收端；兩者都止於保留停止之處。中間一欄的裝置直接接到 Grandmaster 所在的交換器。按一下線路即可檢閱其串流，按一下裝置可檢視其串流；按一下背景即可清除。

## Connections

matrix-nothing-shown = 沒有可顯示的串流
matrix-nothing-shown-note = 變更搜尋或篩選條件以查看更多串流。
matrix-empty = 沒有可連線的串流
matrix-empty-note = 讀取到具有串流的實體後，發送端串流與接收端串流會在此交會。
matrix-all-streams = 所有串流
matrix-connectable-only = 隱藏無法連線的項目
matrix-none-hidden = 顯示的所有串流皆可連線
matrix-hidden = 已隱藏 { $count } 個串流
matrix-own = 實體的輸出無法連線至其自身的輸入。
matrix-working = 處理中。
matrix-waiting-change = 正在等待此輸入的上一次變更完成。
matrix-connected = 已連線且正在接收。按一下即可中斷連線。
matrix-bound-waiting = 已綁定，正在等待發送端的串流。按一下即可中斷連線。
matrix-bound-failed = 已綁定，但發送端的保留失敗：{ $reason }。按一下即可中斷連線。
matrix-bound-formats-differ = 已綁定，但格式不同：發送端傳送 { $sent }，輸入設定為 { $set }。按一下即可中斷連線。
matrix-formats-match = 格式相符（{ $format }）。按一下即可連線。
matrix-format-must-change = 此輸入支援 { $sent }，但目前設定為 { $set }，因此在變更格式前可能無法播放。按一下選擇格式。
matrix-format-input = 將輸入改為
matrix-format-both = 將兩端都改為
matrix-format-talker-busy = 發送端正在向其他接收端傳送串流，因此維持其格式不變。
matrix-format-anyway = 不變更格式直接連線
matrix-incompatible = 此輸入不支援 { $sent }，目前設定為 { $set }。
matrix-group-none = 未連線。展開即可逐一連線串流。
matrix-group-connected = 已連線 { $count } 個。展開即可查看每個連線。
matrix-outputs-expand = { $count } 個串流輸出。按一下箭頭展開，按一下名稱檢閱。
matrix-outputs-collapse = { $count } 個串流輸出。按一下箭頭收合，按一下名稱檢閱。
matrix-inputs-expand = { $count } 個串流輸入。按一下箭頭展開，按一下名稱檢閱。
matrix-inputs-collapse = { $count } 個串流輸入。按一下箭頭收合，按一下名稱檢閱。
matrix-stream-inspect = { $detail }按一下即可檢閱 { $entity }。
matrix-point = 指向一個儲存格
matrix-point-note = 即可查看其發送端與接收端，以及兩者的格式是否相符。
matrix-legend-waiting = 已綁定，正在等待串流
matrix-legend-trouble = 已綁定，發生問題
matrix-legend-open = 可連線
matrix-legend-change = 須先變更輸入格式
matrix-legend-incompatible = 格式無法相符
matrix-talker-outputs = 發送端輸出
matrix-listener-inputs = 接收端輸入

common-thousands-separator = {","}
common-decimal-separator = {"."}

## Diagnostics

diag-since-start = 自實體啟動以來的計數。
diag-stream-input = 串流輸入
diag-stream-output = 串流輸出
diag-locked = { $count ->
    [0] 未鎖定
   *[other] 鎖定 { $number } 次
}
diag-lost-lock = { $count ->
    [0] 未失鎖
   *[other] 失鎖 { $number } 次
}
diag-frames-in = 接收 { $number } 個訊框
diag-frames-out = 傳送 { $number } 個訊框
diag-media-locked = { $count ->
    [0] 媒體未鎖定
   *[other] 媒體鎖定 { $number } 次
}
diag-lost-media-lock = { $count ->
    [0] 媒體未失鎖
   *[other] 媒體失鎖 { $number } 次
}
diag-interrupted = { $count ->
    [0] 未中斷
   *[other] 中斷 { $number } 次
}
diag-out-of-sequence = { $number } 個訊框順序錯亂
diag-media-resets = 媒體重設 { $number } 次
diag-timestamps-uncertain = { $count ->
    [0] 時間戳記未出現不確定
   *[other] 時間戳記不確定 { $number } 次
}
diag-no-timestamp = { $number } 個訊框沒有時間戳記
diag-unsupported-format = { $number } 個訊框格式不受支援
diag-late = { $number } 個訊框延遲到達
diag-early = { $number } 個訊框提早到達
diag-started = { $count ->
    [0] 未啟動
   *[other] 啟動 { $number } 次
}
diag-stopped = { $count ->
    [0] 未停止
   *[other] 停止 { $number } 次
}
diag-reservation-failed = 發送端的保留失敗：{ $reason }
diag-latency = 累積延遲 { $microseconds } µs

## AVB Lite

lite-active = 作用中
lite-active-untagged = 作用中，未標記
lite-active-vlan = 作用中，VLAN { $vlan }
lite-capable = 支援
lite-mode = 模式
lite-mode-capable = AVB（支援 AVB Lite）
lite-because = 原因
lite-fallback-none = 未提供原因
lite-fallback-endpoint = 收到另一個端點的宣告，因此兩者之間沒有 AVB 交換器
lite-fallback-unanswered = 九次對等延遲要求均未獲回應
lite-fallback-responders = 一次對等延遲要求獲得兩個以上的回應，因此該交換器不是 AVB 交換器
lite-fallback-configured = 由操作人員或控制器設定
lite-fallback-other = 該規範未定義的原因
lite-other-profile = 其他規範
lite-ptp-domain = { $profile }，域 { $domain }
lite-offset = 偏移
lite-offset-from = 相對 { $grandmaster } 偏移 { $offset }
lite-media-vlan = 媒體 VLAN
lite-untagged = 未標記
lite-unicast = 單播
lite-fanout = 每個串流最多逐一傳送給 { $count } 個接收端，之後改用多播
lite-link = 連結
lite-bandwidth = 頻寬
lite-egress-of = { $used }（共 { $link }），{ $share }
lite-egress-of-assumed = { $used }（共 { $link }），{ $share }，假設為 Gigabit 連結
lite-egress-reported = 依實體統計的已准入串流計算。
lite-egress-worked-out = 依已連線串流輸出的格式推算。
lite-alarm-offset = PTP 偏移 { $offset }，超出 AVB Lite 允許的 50 µs
lite-alarm-egress = 出口流量達到連結的 { $share }，超出串流可佔用的 { $limit }

## AVB Wireless

wireless-station = 站台
wireless-access-point = 存取點
wireless-role = 角色
wireless-mode = 模式
wireless-time = 時間
wireless-mode-a-ftm = Mode A，基於 FTM 的 802.1AS
wireless-mode-a-tm = Mode A，基於 TM 的 802.1AS
wireless-mode-b = Mode B，基於信標
wireless-no-time = 無時間同步
wireless-other-mode = 該規範未定義的模式
wireless-locked = 已鎖定
wireless-holdover = 保持中
wireless-not-locked = 未鎖定
wireless-row-locked = 站台，已鎖定
wireless-row-holdover = 站台，保持中
wireless-row-not-locked = 站台，未鎖定
wireless-row-access-point = 存取點，{ $count } 個站台
wireless-link = 連結
wireless-channel = 頻道 { $channel }
wireless-not-known = 未知
wireless-signal = 訊號強度
wireless-rate = 傳送速率
wireless-ftm-valid = 有效率 { $share }
wireless-rtt = 往返時間 { $rtt }
wireless-bursts = 每次叢發 { $count } 個訊框
wireless-not-as-capable = FALSE，{ $reason }
wireless-reason-bursts = 存取點分配的 FTM 叢發既非 3 個也非 2 個訊框
wireless-reason-measurement = 與存取點之間既無 FTM 也無 TM
wireless-reason-signaling = 存取點未發出支援 gPTP 的 Signaling 訊息
wireless-reason-other = 該規範未定義的原因
wireless-servo = 伺服誤差
wireless-stations = 站台
wireless-station-count = { $count } 個站台
wireless-no-ftm = 不支援 FTM
wireless-unserved = 未獲服務的接收端
wireless-frame-budget = 訊框預算
wireless-frame-use = 每秒 { $used } / { $budget } 個訊框
wireless-stream-frames = 串流訊框
wireless-frames-of = 傳往站台 { $readdressed }，無接收端 { $unmapped }，丟棄 { $dropped }，來自站台 { $restored }
wireless-class-a-allowed = 允許
wireless-class-a-not-allowed = 不允許
wireless-alarm-not-locked = Wi-Fi 時間未鎖定至存取點
wireless-alarm-holdover = Wi-Fi 時間處於保持狀態，已與存取點失鎖
wireless-alarm-unserved = Wi-Fi 埠上有 { $count } 個接收端未獲服務，超出單播上限

## Log

log-all = 全部
log-warnings = 警告
log-pause = 暫停
log-resume = 繼續
log-clear = 清除
log-empty = triib 傳送與收到的每個 ATDECC 訊框都會顯示於此，最新的在最前面。
log-none-match = 已存的訊框中沒有符合篩選條件的。
log-frames = { $count } 個訊框
log-shown-of = { $shown } / { $all } 個訊框
log-sent = 傳送
log-heard = 收到
log-not-decoded = 未解碼
log-warning-short = 其 control_data_length 宣稱的長度超出訊框結尾 { $missing } 個位元組。
log-warning-undecodable = 無法解碼：{ $error }。
log-warning-long-acmp = 此訊框採用長格式 ACMP，Milan 實體不得傳送此格式（Milan 1.3，5.5.2.2）。

## Channel mappings

mapping-section = 通道對應
mapping-inputs = 輸入
mapping-outputs = 輸出
mapping-port = 埠 { $number }
mapping-fixed = 固定
mapping-not-read = 尚未讀取。
mapping-no-clusters = 無叢集。
mapping-no-streams = 無音訊串流。
mapping-none = 無對應。
mapping-not-mapped = 未對應
mapping-cluster-numbered = 叢集 { $index }

## Presets

presets-note = 預設集會保存每個實體的時鐘源、取樣率、串流格式、控制項與連線。套用時只會變更有差異的部分。
presets-none = 尚未儲存預設集。
presets-connections = { $count } 個連線
presets-recall = 套用
presets-delete = 刪除
presets-no-place = 沒有可儲存預設集的位置：無法得知家目錄。
presets-undeletable = 無法刪除 { $path }：{ $error }。
presets-saved = 已儲存「{ $name }」，包含 { $count } 個實體。
presets-nothing-differs = 與「{ $name }」沒有差異。
presets-recalling = 正在套用「{ $name }」：{ $count } 項變更。
presets-missing = { $report }以下實體不在此處或尚未讀取：{ $missing }。
presets-deleted = 已刪除「{ $name }」。
presets-host-note = 也會保存本機自己的發送端和接收端，並在套用時重新啟動它們。
presets-host-endpoints = 本機 { $count } 個
presets-starting-host = 正在為「{ $name }」啟動本機的發送端和接收端；它們恢復後再套用其餘部分。

## Controls

control-numbered = 控制項 { $index }
control-not-shown = 此處不顯示
control-option = 選項 { $number }

## Network errors

network-permission = triib 需要權限才能傳送與接收原始乙太網路訊框。
network-needs-npcap = triib 需要 Npcap 才能傳送與接收原始乙太網路訊框。
network-npcap-administrators = Npcap 僅允許系統管理員傳送與接收原始乙太網路訊框。請以系統管理員身分執行 triib，或在不勾選「僅限系統管理員」選項的情況下重新安裝 Npcap。

matrix-stream-format = { $format }。
matrix-stream-format-state = { $format }。{ $state }。

## This computer's own talkers and listeners

host-add-talker = 新增發送端
host-add-listener = 新增接收端
host-add-device = 新增裝置
host-show-mine = 只顯示本機自己的發送端和接收端
host-show-all = 顯示所有實體
host-new-talker = 主機發送端 { $number }
host-new-listener = 主機接收端 { $number }
host-new-device = 主機裝置 { $number }
host-failed = 無法新增到本機：{ $reason }
host-needs-clock = 本機自己的發送端和接收端需要具備 PTP 硬體時鐘的有線介面
host-no-ptp4l = ptp4l 沒有回應，本機的串流無法保持 gPTP 時間
host-elsewhere = triib-endpointd 正以其他使用者或 root 身分在 { $interface } 上執行，因此本機的發送端和接收端在那裡執行，而不是在這裡
host-foreign-mrp = 另一個程式正從本機的位址在 { $interface } 上發出 MSRP 或 MVRP 宣告，可能撤回本機串流所需的內容
host-alarm-foreign-mrp = 本機上的另一個程式在 { $interface } 上發出 MSRP 或 MVRP 宣告
host-state = 狀態
host-streaming = 正在發送
host-waiting = 正在等待接收端
host-listening = 正在監聽
host-bound = 已綁定，正在等待發送端
host-unbound = 未綁定
host-audio-from = 音訊來源
host-audio-to = 音訊送往
host-channels = 聲道數
host-virtual-device = 虛擬裝置
host-add-output-stream = 新增輸出串流
host-add-input-stream = 新增輸入串流
host-remove-stream = 移除串流
host-silence = 靜音
host-tone = 測試音
host-nowhere = 不輸出
host-default-device = 預設裝置
host-remove = 從本機移除
