# triib 的界面文本，简体中文。
# 术语见 docs/GLOSSARY.md 与 docs/glossary/zh-CN.md。

## Language

language-name = 简体中文

## Common

common-close = 关闭
common-more = 更多
common-keep-toolbar-shown = 始终显示工具栏
common-auto-hide-toolbar = 自动隐藏工具栏

## Settings

settings-title = 设置
settings-general = 通用
settings-appearance = 外观
settings-language = 语言
settings-language-system = 系统默认：{ $language }
settings-language-note = 文本框沿用系统的输入语言。
settings-appearance-system = 跟随系统
settings-appearance-light = 浅色
settings-appearance-dark = 深色
settings-colors = 颜色
settings-system-accent = 使用系统强调色
settings-accent-picked = triib 的配色由下方颜色生成。
settings-accent-omarchy = 来自 Omarchy 主题“{ $theme }”。
settings-accent-desktop = 来自桌面的强调色。
settings-accent-none = 桌面没有强调色，因此使用下方颜色。
settings-motion = 动效
settings-animations = 动画
settings-animations-note = 内容变化时以弹性和滑动效果过渡。
settings-animations-reduced = 桌面要求减弱动态效果，因此 triib 不播放动画。
settings-outside-control = 外部控制
settings-allow-outside-control = 允许外部控制
settings-allow-outside-control-note = Claude Code 等智能体可以通过 triib --mcp 查看和更改你的 AVB 网络。每个新智能体接入前，triib 都会先询问。
settings-allowed-agents = 已允许：{ $agents }
settings-forget-agents = 忘记
agent-prompt-title = 允许 { $agent } 控制 triib？
agent-prompt-body = { $agent } 请求使用 triib 的外部控制：查看并更改你网络上的实体，连接流，设置格式、时钟和名称。你可以在“设置”中关闭外部控制。
agent-prompt-allow = 允许
agent-prompt-deny = 不允许

common-cancel = 取消
common-save = 保存
common-not-set = 未设置
common-unnamed = 未命名
common-none = 无
common-mac-address = MAC 地址
common-list-separator = {"，"}

## Network interfaces

interface-up = 已连通
interface-link-down = 链路断开
interface-wireless = 无线
interface-hardware-clock = 硬件时钟
interface-hardware-clock-named = 硬件时钟 { $clock }
interface-virtual = 虚拟

## Toolbar

toolbar-choose-interface = 选择接口
toolbar-interface = 网络接口
toolbar-show-virtual = 显示虚拟接口
toolbar-hide-virtual = 隐藏虚拟接口
toolbar-connections = 连接
toolbar-network = 网络
toolbar-entities = 实体
toolbar-rediscover = 请求所有实体重新通告
toolbar-rescan = 清除并重新扫描所有实体
toolbar-search = 搜索实体和流
toolbar-presets = 预设
toolbar-log = 日志
toolbar-inspector = 检查器
toolbar-settings = 设置

## The network's state, in place of a view

state-no-interface = 未选择接口
state-no-interface-note = 选择 AVB 网络上的接口以发现实体。
state-starting = 正在启动
state-starting-note = 正在打开 { $interface }。
state-listening = 正在监听
state-listening-note = { $interface } 上的实体发出通告后会显示在这里。
state-permission-needed = 需要权限
state-npcap-needed = 需要 Npcap
state-get-npcap = 获取 Npcap
state-copy-command = 复制命令
state-cannot-use = 无法使用 { $interface }
state-try-again = 重试

## Entity list

entities-none-yet = 尚无实体
entities-none-yet-note = 网络上的每个实体，及其角色、SR 类和时钟。

## Inspector

inspector-title = 检查器
inspector-entity = 实体
inspector-streams = 流
inspector-controls = 控制项
inspector-diagnostics = 诊断
inspector-descriptors = 描述符
inspector-select = 选择一个实体以查看详情。
inspector-offline = { $entity } 已离线。
inspector-rename = 重命名
inspector-name = 名称
inspector-identify = 识别
inspector-model-not-read = 尚未读取其实体模型。
inspector-no-streams = 无流。
inspector-no-controls = 没有可显示的控制项。
inspector-no-diagnostics = 未报告任何接口或计数器。
inspector-reading = 正在读取描述符，已读取 { $count } 个。
inspector-read-failed = 无法读取实体模型：{ $reason }。

entity-section = 实体
entity-name = 名称
entity-group = 组
entity-product = 产品
entity-firmware = 固件
entity-serial-number = 序列号
entity-configuration = 配置
entity-configuration-of = { $name }（{ $number }/{ $count }）
entity-milan = Milan
entity-media-clock = 媒体时钟
entity-clock-domain = 时钟域
entity-sampling-rate = 采样率
clock-source-numbered = 时钟源 { $index }
rate-pull = pull 系数 { $pull }

stream-format = 格式
stream-id = 流 ID
stream-destination = 目标
stream-max-transit = 最大传输时间

avb-interfaces = AVB 接口
avb-interface = 接口
avb-interface-clock-identity = 时钟标识
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }，域 { $domain }
avb-interface-peer-delay = 对等延迟
avb-interface-running = 正在运行
avb-interface-none-reported = 未报告
avb-interface-path = 路径
avb-interface-own-grandmaster = 自身即为 Grandmaster
avb-interface-hops = 距 Grandmaster { $count } 跳
avb-interface-link-up = 链路连通
avb-interface-link-down = 链路断开
avb-interface-grandmaster-changes = Grandmaster 变更次数
avb-interface-frames-sent = 已发送帧数
avb-interface-frames-received = 已接收帧数
avb-interface-crc-errors = CRC 错误

tree-firmware = 固件 { $version }
tree-descriptor-types = { $count } 种描述符类型
tree-clock = 时钟
tree-clock-source-from = { $kind }，来自 { $location } { $index }
tree-clock-domain-using = 使用 { $source }
tree-clusters = { $count } 个簇
tree-maps = { $count } 个映射

advert-not-advertised = 未通告
advert-identity = 身份
advert-entity-id = 实体 ID
advert-entity-model = 实体模型
advert-roles = 角色
advert-talker = 发送端
advert-listener = 接收端
advert-clock = 时钟
advert-btc = BTC
advert-gptp-domain = gPTP 域
advert-sr-classes = SR 类
advert-indexes = 实体模型索引
advert-identify-control = 识别控制项
advert-avb-interface = AVB 接口
advert-advertising = 通告
advert-valid-time = 有效时间
advert-available-index = 可用索引
advert-association = 关联
advert-capabilities = 能力

## Status bar

status-entities = { $count } 个实体
status-not-discovering = 未在发现
status-discovering = 正在发现
status-discovering-as = 正在以 { $controller } 身份发现
status-stopped = 因错误而停止
status-alarm = 告警
status-alarm-of = { $entity }：{ $alarm }
status-alarm-more = { $alarm }，另有 { $count } 项

## Descriptions of entities, shared by the views

role-talker = 发送端 { $count }
role-listener = 接收端 { $count }
role-controller = 控制器
role-none = 无角色
classes-a-and-b = A 和 B
clock-no-gptp = 无 gPTP

read-not-read = 未读取
read-reading = 正在读取，已读取 { $count } 个
read-ready-unreadable = 就绪，{ $count } 个无法读取
read-ready-cached = 就绪，来自缓存
read-ready = 就绪
read-failed = 失败：{ $reason }

milan-no = 否
milan-before-1-3 = 1.3 之前
milan-certified = { $version }，已通过 { $certification } 认证
milan-not-certified = { $version }，未认证

outcome-status = 状态 { $status }
outcome-no-response = 无响应
outcome-not-possible = 无法执行
outcome-connect = 无法将 { $talker } 连接到 { $listener }：{ $reason }。
outcome-disconnect = 无法断开 { $listener } 的连接：{ $reason }。
outcome-identify = 无法让 { $entity } 执行识别：{ $reason }。
outcome-rename = 无法将 { $what } 重命名为“{ $name }”：{ $reason }。
outcome-rename-group = 无法将 { $entity } 的组重命名为“{ $name }”：{ $reason }。
outcome-format-streaming = 无法更改 { $stream } 的格式：该流正在传输。请先断开连接。
outcome-format = 无法更改 { $stream } 的格式：{ $reason }。
outcome-max-transit-time = 无法更改 { $stream } 的最大传输时间：{ $reason }。
outcome-max-transit-time-streaming = 无法更改 { $stream } 的最大传输时间：该流正在传输。请先断开连接。
outcome-sampling-rate = 无法更改 { $entity } 的采样率：{ $reason }。
outcome-clock-source = 无法更改 { $entity } 的时钟源：{ $reason }。
outcome-map = 无法在 { $entity } 上映射通道：{ $reason }。
outcome-unmap = 无法在 { $entity } 上取消通道映射：{ $reason }。
outcome-control = 无法在 { $entity } 上设置“{ $control }”：{ $reason }。
outcome-control-numbered = 无法在 { $entity } 上设置控制项 { $index }：{ $reason }。

stream-not-connected = 未连接
stream-from = 来自 { $stream }
stream-from-receiving = 来自 { $stream }，正在接收
stream-from-waiting = 来自 { $stream }，正在等待发送端
stream-from-failed = 来自 { $stream }，发送端的预留失败：{ $reason }
stream-sending-to = 正在发送至 { $destination }

failure-no-response = 未响应
failure-refused = 被拒绝，状态为 { $status }
failure-malformed = 响应无法解码
failure-on-this-computer = 它在本机上运行；请从另一台计算机读取

msrp-failure-1 = 带宽不足
msrp-failure-2 = 交换机资源不足
msrp-failure-3 = 该流量类别带宽不足
msrp-failure-4 = 流 ID 已被另一个发送端使用
msrp-failure-5 = 目标地址已被使用
msrp-failure-6 = 被更高等级的流抢占
msrp-failure-7 = 报告的延迟已改变
msrp-failure-8 = 出口端口不支持 AVB
msrp-failure-9 = 请使用其他目标地址
msrp-failure-10 = MSRP 资源耗尽
msrp-failure-11 = MMRP 资源耗尽
msrp-failure-12 = 无法存储目标地址
msrp-failure-13 = 优先级不是 SR 类优先级
msrp-failure-14 = 帧长度超出介质上限
msrp-failure-15 = 已达到扇入端口上限
msrp-failure-16 = 已注册流的首值发生变化
msrp-failure-17 = VLAN 在出口端口被阻塞
msrp-failure-18 = 出口端口禁用了 VLAN 标签
msrp-failure-19 = SR 类优先级不匹配
msrp-failure-unknown = 未知原因
msrp-failure-at = { $reason }，发生在交换机 { $bridge }

## Entity list columns

column-vendor = 厂商
column-model = 型号
column-state = 状态
column-entity-model-id = 实体模型 ID
column-talker-streams = 发送端流
column-listener-streams = 接收端流
column-avb-lite = AVB Lite
column-egress = 出口流量
column-wireless = 无线

## Settings file

settings-no-place = 没有可保存设置的位置：主文件夹未知。
settings-unusable = 无法使用 { $path }：{ $error }。
settings-unsaved = 无法保存 { $path }：{ $error }。

column-remove = 移除列
column-move-left = 左移
column-move-right = 右移
column-add = 添加列
common-percent = { $value }%

## Network view

netmap-empty = 暂无可显示的网络
netmap-empty-note = 实体被读取并报告其在 gPTP 树中的位置后，会显示在这里。
netmap-focus-clock-path = { $name } 的时钟路径
netmap-focus-streams = { $name } 的流
netmap-showing = 正在显示 { $what }
netmap-devices = { $count } 台设备
netmap-bridges = { $count } 台交换机
netmap-show-map = 显示拓扑图
netmap-show-details = 显示详情
stream-numbered = 流 { $index }
netmap-bridge = 交换机
netmap-access-point = 接入点
netmap-device = 设备
netmap-this-computer = 本机
netmap-connected = 已连接
netmap-advertised = 已通告，无就绪的接收端
netmap-advertised-off-tree = 已通告，无就绪的接收端（{ $listener } 不在 gPTP 树上）
netmap-failed-at = 在 { $bridge } 预留失败：{ $reason }
netmap-failed = 预留失败：{ $reason }
netmap-no-bridge-on = 在 { $interface } 上未检测到交换机
netmap-cannot-listen-on = 无法在 { $interface } 上监听 gPTP
netmap-on-this-computer = 在本机上
netmap-path-not-reported = 未报告路径
netmap-gptp-not-reported = 未报告 gPTP
netmap-off-tree = 不在 gPTP 树上
netmap-off-ptp = 不在 PTP 树上
netmap-not-lite = 未运行 AVB Lite
netmap-lite-not-reported = 未报告 AVB Lite
netmap-synced = 已同步
netmap-not-synced = 未同步
netmap-triib-on = { $interface } 上的 triib
netmap-through-count = 途经 { $count }
netmap-out = 发送 { $count }
netmap-in = 接收 { $count }
netmap-failed-count = 失败 { $count }
netmap-advertised-only = 仅通告
netmap-failed-state = 失败
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = 不在 gPTP 树上：它自身即为 Grandmaster
netmap-apart-no-path = 未报告其路径；它跟随 Grandmaster { $grandmaster }
netmap-apart-unreported = 未报告其 gPTP 状态
netmap-apart-no-neighbor = 本机接口上未检测到交换机
netmap-apart-cannot-listen = 本机无法在其接口上监听 gPTP
netmap-apart-on-this-computer = 它在本机上运行；请从另一台计算机读取以查看其 gPTP 状态
netmap-apart-not-lite = 它未运行 AVB Lite，因此不跟随 Grandmaster
netmap-apart-lite-unreported = 它未报告任何 AVB Lite 信息，因此无法得知它跟随谁
netmap-clock-tree = 时钟树
netmap-no-grandmaster = 未检测到 Grandmaster
netmap-grandmaster-is = Grandmaster：{ $grandmaster }
netmap-needs-attention = 需要关注
netmap-nodes-below = 下游节点
netmap-bridges-below = 下游交换机
netmap-clock-path = 时钟路径
netmap-hops = 距 Grandmaster 跳数
netmap-link-delay = 链路延迟
netmap-bridge-port = 交换机端口
netmap-link-drops = 链路断开次数
netmap-synced-to-grandmaster = 已与 Grandmaster 同步
netmap-host-no-gptp = 未同步：本机未运行 gPTP
netmap-link-no-gptp = 未同步：其链路未运行 gPTP
netmap-ptp-offset-high = 未同步：相对 Grandmaster 偏移 { $offset }，超出 AVB Lite 允许的 50 µs
netmap-ptp-no-offset = 未同步：未测得相对 Grandmaster 的偏移
netmap-audio = 音频
netmap-media-clock-streams = 媒体时钟流
netmap-audio-streams = 音频流
netmap-bound = 已绑定 { $count } 个
netmap-flowing = 传输中
netmap-advertised-state = 已通告
netmap-media-clock-stream = 媒体时钟流
netmap-audio-stream = 音频流
netmap-reaches = 到达
netmap-passing-count = { $count } 个流途经此处
netmap-through = 途经
netmap-passing-through = 途经的流
netmap-sending = 发送
netmap-receiving = 接收
netmap-problems = 问题
netmap-help-back = 点击背景返回概览。
netmap-help-stream = 点击某个流以检查它，或点击背景返回概览。
netmap-help-ptp = 在 AVB Lite 中，时钟从 Grandmaster 端到端地传到每台设备，途经的交换机不参与其中，因此均不显示。设备只要跟随 Grandmaster 的偏移在 50 µs 以内，就处于同步状态。点击设备或其连线可检查其时钟；点击背景可清除。
netmap-help-clock = 时钟从 Grandmaster 出发，经过每台交换机到达树上的每个节点。灰色虚线表示未运行 gPTP 的链路。点击设备或其连线可检查其时钟路径；点击背景可清除。
netmap-help-media-clock = 仅显示媒体时钟（CRF）流，画法与音频相同：每个流一条连线，按发送端着色。点击连线可检查其流，点击设备可查看其流；点击背景可清除。
netmap-help-audio = 每个流都有自己的连线，进出它经过的每台交换机。颜色按发送端区分：每个发送端有一种色相，其流为该色相的不同深浅。移动的圆点表示音频正在传输；静止的红线表示预留失败，静止的灰线表示已通告但无就绪的接收端；两者都止于预留停止之处。中间一列的设备直接接入 Grandmaster 所在的交换机。点击连线可检查其流，点击设备可查看其流；点击背景可清除。

## Connections

matrix-nothing-shown = 没有可显示的流
matrix-nothing-shown-note = 更改搜索或筛选条件以查看更多流。
matrix-empty = 没有可连接的流
matrix-empty-note = 读取到带有流的实体后，发送端流和接收端流会在这里交汇。
matrix-all-streams = 所有流
matrix-connectable-only = 隐藏无法连接的项
matrix-none-hidden = 显示的所有流均可连接
matrix-hidden = 已隐藏 { $count } 个流
matrix-own = 实体的输出不能连接到其自身的输入。
matrix-working = 正在处理。
matrix-waiting-change = 正在等待此输入的上一次更改完成。
matrix-connected = 已连接并正在接收。点击断开连接。
matrix-bound-waiting = 已绑定，正在等待发送端的流。点击断开连接。
matrix-bound-failed = 已绑定，但发送端的预留失败：{ $reason }。点击断开连接。
matrix-bound-formats-differ = 已绑定，但格式不同：发送端发送 { $sent }，输入设置为 { $set }。点击断开连接。
matrix-formats-match = 格式匹配（{ $format }）。点击连接。
matrix-format-must-change = 该输入支持 { $sent }，但当前设置为 { $set }，因此在更改格式前可能无法播放。点击选择格式。
matrix-format-input = 将输入改为
matrix-format-both = 将两端都改为
matrix-format-talker-busy = 发送端正在向其他接收端发送流，因此保持其格式不变。
matrix-format-anyway = 不更改格式直接连接
matrix-incompatible = 该输入不支持 { $sent }，当前设置为 { $set }。
matrix-group-none = 未连接。展开以逐一连接流。
matrix-group-connected = 已连接 { $count } 个。展开以查看每个连接。
matrix-outputs-expand = { $count } 个流输出。点击箭头展开，点击名称进行检查。
matrix-outputs-collapse = { $count } 个流输出。点击箭头折叠，点击名称进行检查。
matrix-inputs-expand = { $count } 个流输入。点击箭头展开，点击名称进行检查。
matrix-inputs-collapse = { $count } 个流输入。点击箭头折叠，点击名称进行检查。
matrix-stream-inspect = { $detail }点击以检查 { $entity }。
matrix-point = 指向一个单元格
matrix-point-note = 即可查看其发送端和接收端，以及二者格式是否匹配。
matrix-legend-waiting = 已绑定，正在等待流
matrix-legend-trouble = 已绑定，出现问题
matrix-legend-open = 可连接
matrix-legend-change = 需先更改输入格式
matrix-legend-incompatible = 格式无法匹配
matrix-talker-outputs = 发送端输出
matrix-listener-inputs = 接收端输入

common-thousands-separator = {","}
common-decimal-separator = {"."}

## Diagnostics

diag-since-start = 自实体启动以来的计数。
diag-stream-input = 流输入
diag-stream-output = 流输出
diag-locked = { $count ->
    [0] 未锁定
   *[other] 锁定 { $number } 次
}
diag-lost-lock = { $count ->
    [0] 未失锁
   *[other] 失锁 { $number } 次
}
diag-frames-in = 接收 { $number } 帧
diag-frames-out = 发送 { $number } 帧
diag-media-locked = { $count ->
    [0] 媒体未锁定
   *[other] 媒体锁定 { $number } 次
}
diag-lost-media-lock = { $count ->
    [0] 媒体未失锁
   *[other] 媒体失锁 { $number } 次
}
diag-interrupted = { $count ->
    [0] 未中断
   *[other] 中断 { $number } 次
}
diag-out-of-sequence = { $number } 帧乱序
diag-media-resets = 媒体重置 { $number } 次
diag-timestamps-uncertain = { $count ->
    [0] 时间戳未出现不确定
   *[other] 时间戳不确定 { $number } 次
}
diag-no-timestamp = { $number } 帧无时间戳
diag-unsupported-format = { $number } 帧格式不受支持
diag-late = { $number } 帧迟到
diag-early = { $number } 帧早到
diag-started = { $count ->
    [0] 未启动
   *[other] 启动 { $number } 次
}
diag-stopped = { $count ->
    [0] 未停止
   *[other] 停止 { $number } 次
}
diag-reservation-failed = 发送端的预留失败：{ $reason }
diag-latency = 累计延迟 { $microseconds } µs

## AVB Lite

lite-active = 已激活
lite-active-untagged = 已激活，不带标签
lite-active-vlan = 已激活，VLAN { $vlan }
lite-capable = 支持
lite-mode = 模式
lite-mode-capable = AVB（支持 AVB Lite）
lite-because = 原因
lite-fallback-none = 未给出原因
lite-fallback-endpoint = 收到了另一个终端的声明，因此两者之间没有 AVB 交换机
lite-fallback-unanswered = 九次对等延迟请求均未得到响应
lite-fallback-responders = 一次对等延迟请求收到两个或以上的响应，因此该交换机不是 AVB 交换机
lite-fallback-configured = 由操作员或控制器设置
lite-fallback-other = 该规范未定义的原因
lite-other-profile = 其他规范
lite-ptp-domain = { $profile }，域 { $domain }
lite-offset = 偏移
lite-offset-from = 相对 { $grandmaster } 偏移 { $offset }
lite-media-vlan = 媒体 VLAN
lite-untagged = 不带标签
lite-unicast = 单播
lite-fanout = 每个流最多逐一发送给 { $count } 个接收端，之后改用组播
lite-link = 链路
lite-bandwidth = 带宽
lite-egress-of = { $used }（共 { $link }），{ $share }
lite-egress-of-assumed = { $used }（共 { $link }），{ $share }，按千兆链路估算
lite-egress-reported = 根据实体统计的已准入流。
lite-egress-worked-out = 根据已连接流输出的格式推算。
lite-alarm-offset = PTP 偏移 { $offset }，超出 AVB Lite 允许的 50 µs
lite-alarm-egress = 出口流量达到链路的 { $share }，超出流可占用的 { $limit }

## AVB Wireless

wireless-station = 站点
wireless-access-point = 接入点
wireless-role = 角色
wireless-mode = 模式
wireless-time = 时间
wireless-mode-a-ftm = Mode A，基于 FTM 的 802.1AS
wireless-mode-a-tm = Mode A，基于 TM 的 802.1AS
wireless-mode-b = Mode B，基于信标
wireless-no-time = 无时间同步
wireless-other-mode = 该规范未定义的模式
wireless-locked = 已锁定
wireless-holdover = 保持中
wireless-not-locked = 未锁定
wireless-row-locked = 站点，已锁定
wireless-row-holdover = 站点，保持中
wireless-row-not-locked = 站点，未锁定
wireless-row-access-point = 接入点，{ $count } 个站点
wireless-link = 链路
wireless-channel = 信道 { $channel }
wireless-not-known = 未知
wireless-signal = 信号强度
wireless-rate = 发送速率
wireless-ftm-valid = 有效率 { $share }
wireless-rtt = 往返时间 { $rtt }
wireless-bursts = 每次突发 { $count } 帧
wireless-not-as-capable = FALSE，{ $reason }
wireless-reason-bursts = 接入点分配的 FTM 突发既非 3 帧也非 2 帧
wireless-reason-measurement = 与接入点之间既无 FTM 也无 TM
wireless-reason-signaling = 接入点未发出支持 gPTP 的 Signaling 消息
wireless-reason-other = 该规范未定义的原因
wireless-servo = 伺服误差
wireless-stations = 站点
wireless-station-count = { $count } 个站点
wireless-no-ftm = 不支持 FTM
wireless-unserved = 未获服务的接收端
wireless-frame-budget = 帧预算
wireless-frame-use = 每秒 { $used } / { $budget } 帧
wireless-stream-frames = 流数据帧
wireless-frames-of = 发往站点 { $readdressed }，无接收端 { $unmapped }，丢弃 { $dropped }，来自站点 { $restored }
wireless-class-a-allowed = 允许
wireless-class-a-not-allowed = 不允许
wireless-alarm-not-locked = Wi-Fi 时间未锁定到接入点
wireless-alarm-holdover = Wi-Fi 时间处于保持状态，已与接入点失锁
wireless-alarm-unserved = Wi-Fi 端口上有 { $count } 个接收端未获服务，超出单播上限

## Log

log-all = 全部
log-warnings = 警告
log-pause = 暂停
log-resume = 继续
log-clear = 清除
log-empty = triib 发送和收到的每个 ATDECC 帧都会显示在这里，最新的在最前。
log-none-match = 已保存的帧中没有符合筛选条件的。
log-frames = { $count } 帧
log-shown-of = { $shown } / { $all } 帧
log-sent = 发送
log-heard = 收到
log-not-decoded = 未解码
log-warning-short = 其 control_data_length 声明的长度超出帧末尾 { $missing } 个字节。
log-warning-undecodable = 无法解码：{ $error }。
log-warning-long-acmp = 此帧采用长格式 ACMP，Milan 实体不得发送此格式（Milan 1.3，5.5.2.2）。

## Channel mappings

mapping-section = 通道映射
mapping-inputs = 输入
mapping-outputs = 输出
mapping-port = 端口 { $number }
mapping-fixed = 固定
mapping-not-read = 尚未读取。
mapping-no-clusters = 无簇。
mapping-no-streams = 无音频流。
mapping-none = 无映射。
mapping-not-mapped = 未映射
mapping-cluster-numbered = 簇 { $index }

## Presets

presets-note = 预设保存每个实体的时钟源、采样率、流格式、控制项和连接。调用时只更改有差异的部分。
presets-none = 尚未保存预设。
presets-connections = { $count } 个连接
presets-recall = 调用
presets-delete = 删除
presets-no-place = 没有可保存预设的位置：主文件夹未知。
presets-undeletable = 无法删除 { $path }：{ $error }。
presets-saved = 已保存“{ $name }”，包含 { $count } 个实体。
presets-nothing-differs = 与“{ $name }”没有差异。
presets-recalling = 正在调用“{ $name }”：{ $count } 项更改。
presets-missing = { $report }以下实体不在此处或尚未读取：{ $missing }。
presets-deleted = 已删除“{ $name }”。
presets-host-note = 也会保存本机自己的实体，并在调用时重新启动它们。
presets-host-endpoints = 本机 { $count } 个
presets-starting-host = 正在为“{ $name }”启动本机的实体；它们恢复后再应用其余部分。

## Controls

control-numbered = 控制项 { $index }
control-not-shown = 此处不显示
control-option = 选项 { $number }

## Network errors

network-permission = triib 需要权限才能收发原始以太网帧。
network-needs-npcap = triib 需要 Npcap 才能收发原始以太网帧。
network-npcap-administrators = Npcap 仅允许管理员收发原始以太网帧。请以管理员身份运行 triib，或在不勾选“仅限管理员”选项的情况下重新安装 Npcap。

matrix-stream-format = { $format }。
matrix-stream-format-state = { $format }。{ $state }。

## This computer's own entities

host-add-entity = 添加实体
host-show-mine = 只显示本机自己的实体
host-show-all = 显示所有实体
host-new-entity = 主机实体 { $number }
host-failed = 无法添加到本机：{ $reason }
host-needs-clock = 本机自己的实体需要带 PTP 硬件时钟的有线接口
host-no-ptp4l = ptp4l 没有应答，本机的流无法保持 gPTP 时间
host-elsewhere = triib-endpointd 正以其他用户或 root 身份在 { $interface } 上运行，因此本机的实体在那里运行，而不是在这里
host-foreign-mrp = 另一个程序正从本机的地址在 { $interface } 上发出 MSRP 或 MVRP 声明，可能撤回本机的流所需的内容
host-alarm-foreign-mrp = 本机上的另一个程序在 { $interface } 上发出 MSRP 或 MVRP 声明
host-state = 状态
host-streaming = 正在发送
host-waiting = 正在等待接收端
host-listening = 正在监听
host-bound = 已绑定，正在等待发送端
host-unbound = 未绑定
host-audio-from = 音频来自
host-audio-to = 音频送往
host-channels = 声道数
host-virtual-device = 虚拟设备
host-add-stream = 添加流
host-add-audio-input = 音频输入
host-add-audio-output = 音频输出
host-add-clock-input = 媒体时钟输入
host-add-clock-output = 媒体时钟输出
host-remove-stream = 移除流
host-silence = 静音
host-tone = 测试音
host-nowhere = 不输出
host-default-device = 默认设备
host-remove = 从本机移除
