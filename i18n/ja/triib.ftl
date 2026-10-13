# triib のインターフェイステキスト（日本語）。
# 用語は docs/GLOSSARY.md と docs/glossary/ja.md に従います。

## Language

language-name = 日本語

## Common

common-close = 閉じる
common-more = その他
common-keep-toolbar-shown = ツールバーを常に表示
common-auto-hide-toolbar = ツールバーを自動的に隠す

## Settings

settings-title = 設定
settings-general = 一般
settings-appearance = 外観
settings-language = 言語
settings-language-system = システムのデフォルト：{ $language }
settings-language-note = テキスト欄にはシステムの入力言語で入力します。
settings-appearance-system = システム
settings-appearance-light = ライト
settings-appearance-dark = ダーク
settings-colors = カラー
settings-system-accent = システムのアクセントカラーを使用
settings-accent-picked = 下の色を元に triib の配色を決めます。
settings-accent-omarchy = Omarchy テーマ「{ $theme }」の色です。
settings-accent-desktop = デスクトップのアクセントカラーです。
settings-accent-none = デスクトップにアクセントカラーがないため、下の色を使用します。
settings-motion = モーション
settings-animations = アニメーション
settings-animations-note = 表示が変わるとき、スプリングやスライドで動かします。
settings-animations-reduced = デスクトップで動きを減らす設定になっているため、triib は動きません。
settings-outside-control = 外部からの操作
settings-allow-outside-control = 外部からの操作を許可
settings-allow-outside-control-note = Claude Codeなどのエージェントが、triib --mcpを通じてAVBネットワークを確認し、変更できるようになります。新しいエージェントごとにtriibが確認します。
settings-allowed-agents = 許可済み：{ $agents }
settings-forget-agents = 削除
settings-entities-file = エンティティファイル
settings-entities-file-note = このコンピューター自身のエンティティを記載するファイルです。別のファイルを使うには Enter を押します。まだないファイルはこのファイルのコピーとして作られ、triib-endpointd がそのファイルで再起動します。
settings-entities-file-default = デフォルト
settings-entities-file-full = ~/entities.toml のようなフルパスを指定してください。
agent-prompt-title = { $agent }にtriibの操作を許可しますか？
agent-prompt-body = { $agent }が、triibの外部からの操作を使おうとしています。ネットワーク上のエンティティを確認して変更し、ストリームを接続したり、フォーマット、クロック、名前を設定したりします。外部からの操作は「設定」でオフにできます。
agent-prompt-allow = 許可
agent-prompt-deny = 許可しない

common-cancel = キャンセル
common-save = 保存
common-not-set = 未設定
common-unnamed = 名前なし
common-none = なし
common-mac-address = MAC アドレス
common-list-separator = {"、"}

## Network interfaces

interface-up = アップ
interface-link-down = リンクダウン
interface-wireless = 無線
interface-hardware-clock = ハードウェアクロック
interface-hardware-clock-named = ハードウェアクロック { $clock }
interface-virtual = 仮想

## Toolbar

toolbar-choose-interface = インターフェイスを選択
toolbar-interface = ネットワークインターフェイス
toolbar-filter-interfaces = インターフェイスを絞り込む
toolbar-hide-virtual = 仮想インターフェイスを隠す
toolbar-hide-wireless = 無線インターフェイスを隠す
toolbar-hide-unclocked = ハードウェアクロックのないインターフェイスを隠す
toolbar-connections = 接続
toolbar-network = ネットワーク
toolbar-entities = エンティティ
toolbar-rediscover = すべてのエンティティにアドバタイズを要求
toolbar-rescan = すべてのエンティティをクリアして再スキャン
toolbar-search = エンティティとストリームを検索
toolbar-presets = プリセット
toolbar-log = ログ
toolbar-inspector = インスペクター
toolbar-settings = 設定

## The network's state, in place of a view

state-no-interface = インターフェイス未選択
state-no-interface-note = エンティティを検出するには、AVB ネットワーク上のインターフェイスを選択してください。
state-starting = 起動中
state-starting-note = { $interface } を開いています。
state-listening = 受信待機中
state-listening-note = { $interface } 上のエンティティがアドバタイズすると、ここに表示されます。
state-permission-needed = 権限が必要です
state-npcap-needed = Npcap が必要です
state-get-npcap = Npcap を入手
state-copy-command = コマンドをコピー
state-cannot-use = { $interface } を使用できません
state-try-again = 再試行

## Entity list

entities-none-yet = エンティティはまだありません
entities-none-yet-note = ネットワーク上のすべてのエンティティを、ロール、SR クラス、クロックとともに表示します。

## Inspector

inspector-title = インスペクター
inspector-entity = エンティティ
inspector-streams = ストリーム
inspector-controls = コントロール
inspector-diagnostics = 診断
inspector-descriptors = ディスクリプター
inspector-select = エンティティを選択すると詳細を表示します。
inspector-offline = { $entity } はオフラインです。
inspector-rename = 名前を変更
inspector-name = 名前
inspector-identify = 識別
inspector-model-not-read = エンティティモデルをまだ読み込んでいません。
inspector-no-streams = ストリームはありません。
inspector-no-controls = 表示するコントロールはありません。
inspector-no-diagnostics = 報告されたインターフェイスやカウンターはありません。
inspector-reading = ディスクリプターを読み込み中（{ $count } 個完了）。
inspector-read-failed = エンティティモデルを読み込めませんでした：{ $reason }。

entity-section = エンティティ
entity-name = 名前
entity-group = グループ
entity-product = 製品
entity-firmware = ファームウェア
entity-serial-number = シリアル番号
entity-configuration = 構成
entity-configuration-of = { $name }（{ $number }/{ $count }）
entity-milan = Milan
entity-media-clock = メディアクロック
entity-clock-domain = クロックドメイン
entity-sampling-rate = サンプリングレート
clock-source-numbered = ソース { $index }
rate-pull = プル { $pull }

stream-format = フォーマット
stream-id = ストリーム ID
stream-destination = 宛先
stream-max-transit = 最大伝送時間

avb-interfaces = AVB インターフェイス
avb-interface = インターフェイス
avb-interface-clock-identity = クロック ID
avb-interface-grandmaster = グランドマスター
avb-interface-grandmaster-domain = { $grandmaster }、ドメイン { $domain }
avb-interface-peer-delay = ピア遅延
avb-interface-running = 動作中
avb-interface-none-reported = 報告なし
avb-interface-path = パス
avb-interface-own-grandmaster = 自身がグランドマスター
avb-interface-hops = グランドマスターから { $count } ホップ
avb-interface-link-up = リンクアップ
avb-interface-link-down = リンクダウン
avb-interface-grandmaster-changes = グランドマスター変更
avb-interface-frames-sent = 送信フレーム
avb-interface-frames-received = 受信フレーム
avb-interface-crc-errors = CRC エラー

tree-firmware = ファームウェア { $version }
tree-descriptor-types = ディスクリプター { $count } 種類
tree-clock = クロック
tree-clock-source-from = { $kind }、{ $location } { $index } から
tree-clock-domain-using = { $source } を使用
tree-clusters = クラスター { $count } 個
tree-maps = マップ { $count } 個

advert-not-advertised = アドバタイズなし
advert-identity = 識別情報
advert-entity-id = エンティティ ID
advert-entity-model = エンティティモデル
advert-roles = ロール
advert-talker = トーカー
advert-listener = リスナー
advert-clock = クロック
advert-btc = BTC
advert-gptp-domain = gPTP ドメイン
advert-sr-classes = SR クラス
advert-indexes = エンティティモデルのインデックス
advert-identify-control = 識別コントロール
advert-avb-interface = AVB インターフェイス
advert-advertising = アドバタイズ
advert-valid-time = 有効時間
advert-available-index = 利用可能インデックス
advert-association = アソシエーション
advert-capabilities = 機能

## Status bar

status-entities = エンティティ { $count } 個
status-not-discovering = 検出していません
status-discovering = 検出中
status-discovering-as = { $controller } として検出中
status-stopped = エラーにより停止
status-alarm = アラーム
status-alarm-of = { $entity }：{ $alarm }
status-alarm-more = { $alarm } ほか { $count } 件

## Descriptions of entities, shared by the views

role-talker = トーカー { $count }
role-listener = リスナー { $count }
role-controller = コントローラー
role-none = ロールなし
classes-a-and-b = A と B
clock-no-gptp = gPTP なし

read-not-read = 未読み込み
read-reading = 読み込み中（{ $count } 個完了）
read-ready-unreadable = 準備完了（読み込み不可 { $count } 個）
read-ready-cached = 準備完了（キャッシュから）
read-ready = 準備完了
read-failed = 失敗：{ $reason }

milan-no = 非対応
milan-before-1-3 = 1.3 より前
milan-certified = { $version }、{ $certification } 認証済み
milan-not-certified = { $version }、未認証

outcome-status = ステータス { $status }
outcome-no-response = 応答なし
outcome-not-possible = 実行できません
outcome-connect = { $talker } を { $listener } に接続できませんでした：{ $reason }。
outcome-disconnect = { $listener } を切断できませんでした：{ $reason }。
outcome-identify = { $entity } を識別できませんでした：{ $reason }。
outcome-rename = { $what } の名前を「{ $name }」に変更できませんでした：{ $reason }。
outcome-rename-group = { $entity } のグループ名を「{ $name }」に変更できませんでした：{ $reason }。
outcome-format-streaming = { $stream } はストリーミング中のため、フォーマットを変更できませんでした。先に切断してください。
outcome-format = { $stream } のフォーマットを変更できませんでした：{ $reason }。
outcome-max-transit-time = { $stream } の最大伝送時間を変更できませんでした：{ $reason }。
outcome-max-transit-time-streaming = { $stream } はストリーミング中のため、最大伝送時間を変更できませんでした。先に切断してください。
outcome-sampling-rate = { $entity } のサンプリングレートを変更できませんでした：{ $reason }。
outcome-clock-source = { $entity } のクロックソースを変更できませんでした：{ $reason }。
outcome-map = { $entity } のチャンネルをマッピングできませんでした：{ $reason }。
outcome-unmap = { $entity } のチャンネルのマッピングを解除できませんでした：{ $reason }。
outcome-control = { $entity } の「{ $control }」を設定できませんでした：{ $reason }。
outcome-control-numbered = { $entity } のコントロール { $index } を設定できませんでした：{ $reason }。

stream-not-connected = 未接続
stream-from = { $stream } から
stream-from-receiving = { $stream } から受信中
stream-from-waiting = { $stream } から、トーカーを待機中
stream-from-failed = { $stream } から、トーカーの予約に失敗：{ $reason }
stream-sending-to = { $destination } へ送信中

failure-no-response = 応答がありません
failure-refused = { $status } で拒否されました
failure-malformed = 応答をデコードできません
failure-on-this-computer = このコンピューター上で動作しているため、別のコンピューターから読み込んでください

msrp-failure-1 = 帯域幅不足
msrp-failure-2 = ブリッジのリソース不足
msrp-failure-3 = トラフィッククラスの帯域幅不足
msrp-failure-4 = 他のトーカーが使用中のストリーム ID
msrp-failure-5 = 宛先アドレスが既に使用中
msrp-failure-6 = 上位ランクのストリームによりプリエンプト
msrp-failure-7 = 報告されたレイテンシーが変化
msrp-failure-8 = エグレスポートが AVB 非対応
msrp-failure-9 = 別の宛先アドレスが必要
msrp-failure-10 = MSRP リソース不足
msrp-failure-11 = MMRP リソース不足
msrp-failure-12 = 宛先アドレスを保存できません
msrp-failure-13 = 優先度が SR クラスの優先度ではありません
msrp-failure-14 = 伝送媒体に対してフレームが大きすぎます
msrp-failure-15 = ファンインポートの上限に到達
msrp-failure-16 = 登録済みストリームの FirstValue が変化
msrp-failure-17 = エグレスポートで VLAN がブロック
msrp-failure-18 = エグレスポートで VLAN タグ付けが無効
msrp-failure-19 = SR クラスの優先度が不一致
msrp-failure-unknown = 不明な理由
msrp-failure-at = { $reason }（ブリッジ { $bridge }）

## Entity list columns

column-vendor = ベンダー
column-model = モデル
column-state = 状態
column-entity-model-id = エンティティモデル ID
column-talker-streams = トーカーストリーム
column-listener-streams = リスナーストリーム
column-avb-lite = AVB Lite
column-egress = エグレス
column-wireless = 無線

## Settings file

settings-no-place = 設定を保存する場所がありません：ホームフォルダーが不明です。
settings-unusable = { $path } を使用できませんでした：{ $error }。
settings-unsaved = { $path } を保存できませんでした：{ $error }。

column-remove = 列を削除
column-move-left = 左へ移動
column-move-right = 右へ移動
column-add = 列を追加
common-percent = { $value }%

## Network view

netmap-empty = 表示するネットワークはまだありません
netmap-empty-note = エンティティは、読み込まれて gPTP ツリー上の位置を報告すると、ここに表示されます。
netmap-focus-clock-path = { $name } のクロックパス
netmap-focus-streams = { $name } のストリーム
netmap-showing = { $what } を表示中
netmap-devices = デバイス { $count } 台
netmap-bridges = ブリッジ { $count } 台
netmap-show-map = マップを表示
netmap-show-details = 詳細を表示
stream-numbered = ストリーム { $index }
netmap-bridge = ブリッジ
netmap-access-point = アクセスポイント
netmap-device = デバイス
netmap-this-computer = このコンピューター
netmap-connected = 接続済み
netmap-advertised = アドバタイズ中、リスナー未準備
netmap-advertised-off-tree = アドバタイズ中、リスナー未準備（{ $listener } は gPTP ツリー外）
netmap-failed-at = { $bridge } で予約に失敗：{ $reason }
netmap-failed = 予約に失敗：{ $reason }
netmap-no-bridge-on = { $interface } でブリッジが見つかりません
netmap-cannot-listen-on = { $interface } で gPTP を受信できません
netmap-on-this-computer = このコンピューター上
netmap-path-not-reported = パス未報告
netmap-gptp-not-reported = gPTP 未報告
netmap-off-tree = gPTP ツリー外
netmap-off-ptp = PTP ツリー外
netmap-not-lite = AVB Lite 未参加
netmap-lite-not-reported = AVB Lite 未報告
netmap-synced = 同期済み
netmap-not-synced = 未同期
netmap-triib-on = { $interface } 上の triib
netmap-through-count = 通過 { $count }
netmap-out = 送信 { $count }
netmap-in = 受信 { $count }
netmap-failed-count = 失敗 { $count }
netmap-advertised-only = アドバタイズのみ
netmap-failed-state = 失敗
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = gPTP ツリー外：自身がグランドマスターです
netmap-apart-no-path = パスが報告されていません。グランドマスター { $grandmaster } に従っています
netmap-apart-unreported = gPTP の状態を報告していません
netmap-apart-no-neighbor = このコンピューターのインターフェイスでブリッジが見つかりません
netmap-apart-cannot-listen = このコンピューターはインターフェイスで gPTP を受信できません
netmap-apart-on-this-computer = このコンピューター上で動作しています。gPTP の状態は別のコンピューターから読み込んで確認してください
netmap-apart-not-lite = AVB Lite を実行していないため、グランドマスターに従っていません
netmap-apart-lite-unreported = AVB Lite について何も報告していないため、何に従っているかは不明です
netmap-clock-tree = クロックツリー
netmap-no-grandmaster = グランドマスターが見つかりません
netmap-grandmaster-is = グランドマスター：{ $grandmaster }
netmap-needs-attention = 要確認
netmap-nodes-below = 下位のノード
netmap-bridges-below = 下位のブリッジ
netmap-clock-path = クロックパス
netmap-hops = グランドマスターからのホップ数
netmap-link-delay = リンク遅延
netmap-bridge-port = ブリッジポート
netmap-link-drops = リンクダウン回数
netmap-synced-to-grandmaster = グランドマスターに同期済み
netmap-host-no-gptp = 未同期：このコンピューターは gPTP を実行していません
netmap-link-no-gptp = 未同期：リンク上で gPTP が動作していません
netmap-ptp-offset-high = 未同期：グランドマスターに対し { $offset }、AVB Lite の許容値 50 µs を超過
netmap-ptp-no-offset = 未同期：グランドマスターからのオフセットを測定していません
netmap-audio = オーディオ
netmap-media-clock-streams = メディアクロックストリーム
netmap-audio-streams = オーディオストリーム
netmap-bound = バインド済み { $count }
netmap-flowing = 伝送中
netmap-advertised-state = アドバタイズ中
netmap-media-clock-stream = メディアクロックストリーム
netmap-audio-stream = オーディオストリーム
netmap-reaches = 到達点
netmap-passing-count = 通過ストリーム { $count } 本
netmap-through = 通過
netmap-passing-through = 通過中
netmap-sending = 送信中
netmap-receiving = 受信中
netmap-problems = 問題
netmap-help-back = 背景をクリックすると概要に戻ります。
netmap-help-stream = ストリームをクリックすると詳細を表示し、背景をクリックすると概要に戻ります。
netmap-help-ptp = AVB Lite ではクロックはグランドマスターから各デバイスへエンドツーエンドで伝わり、途中のスイッチは関与しないため表示されません。デバイスは、グランドマスターに 50 µs 以内で従っている間は同期済みです。デバイスまたはその配線をクリックするとクロックを表示し、背景をクリックすると解除します。
netmap-help-clock = クロックはグランドマスターから各ブリッジを経て、ツリー上のすべてのノードへ伝わります。灰色の破線は gPTP が動作していないリンクです。デバイスまたはその配線をクリックするとクロックパスを表示し、背景をクリックすると解除します。
netmap-help-media-clock = メディアクロック（CRF）ストリームだけを、オーディオと同じ方法で描きます。ストリームごとに 1 本の配線を、トーカー別に色分けします。配線をクリックするとそのストリームを、デバイスをクリックするとそのデバイスのストリームを表示し、背景をクリックすると解除します。
netmap-help-audio = ストリームごとに専用の配線があり、通過するブリッジごとに出入りします。色はトーカー別で、トーカーごとに色相を割り当て、そのストリームは同じ色相の濃淡で示します。動く点はオーディオが流れていることを示します。動かない赤い線は予約の失敗、動かない灰色の線はリスナーの準備ができていないアドバタイズ中のストリームで、どちらも予約が止まった位置で途切れます。中央の列のデバイスは、グランドマスターのブリッジに直接接続しています。配線をクリックするとそのストリームを、デバイスをクリックするとそのデバイスのストリームを表示し、背景をクリックすると解除します。

## Connections

matrix-nothing-shown = 表示するストリームがありません
matrix-nothing-shown-note = 検索条件やフィルターを変更すると、ほかのストリームも表示されます。
matrix-empty = 接続できるストリームがありません
matrix-empty-note = ストリームを持つエンティティが読み込まれると、トーカーストリームとリスナーストリームがここに並びます。
matrix-all-streams = すべてのストリーム
matrix-connectable-only = 接続できないものを隠す
matrix-none-hidden = 表示中のストリームはすべて接続できます
matrix-hidden = { $count } 本のストリームを非表示
matrix-own = エンティティの出力は、自身の入力には接続できません。
matrix-working = 処理中です。
matrix-waiting-change = この入力への直前の変更を待っています。
matrix-connected = 接続済みで受信中です。クリックすると切断します。
matrix-bound-waiting = バインド済みで、トーカーのストリームを待っています。クリックすると切断します。
matrix-bound-failed = バインド済みですが、トーカーの予約に失敗しました：{ $reason }。クリックすると切断します。
matrix-bound-formats-differ = バインド済みですが、フォーマットが異なります：トーカーは { $sent } を送信し、入力は { $set } に設定されています。クリックすると切断します。
matrix-formats-match = フォーマット一致（{ $format }）。クリックすると接続します。
matrix-format-must-change = 入力は { $sent } を受けられますが、{ $set } に設定されているため、フォーマットを変更するまで再生されない場合があります。クリックしてフォーマットを選びます。
matrix-format-input = 入力を変更
matrix-format-both = 両方を変更
matrix-format-talker-busy = トーカーはほかのリスナーにもストリームしているため、フォーマットは変えません。
matrix-format-anyway = フォーマットを変えずに接続
matrix-incompatible = 入力は { $sent } を受けられません。{ $set } に設定されています。
matrix-group-none = 未接続。展開するとストリームを個別に接続できます。
matrix-group-connected = { $count } 本接続済み。展開すると個別に表示します。
matrix-outputs-expand = ストリーム出力 { $count } 本。矢印をクリックすると展開し、名前をクリックすると詳細を表示します。
matrix-outputs-collapse = ストリーム出力 { $count } 本。矢印をクリックすると折りたたみ、名前をクリックすると詳細を表示します。
matrix-inputs-expand = ストリーム入力 { $count } 本。矢印をクリックすると展開し、名前をクリックすると詳細を表示します。
matrix-inputs-collapse = ストリーム入力 { $count } 本。矢印をクリックすると折りたたみ、名前をクリックすると詳細を表示します。
matrix-stream-inspect = { $detail } クリックすると { $entity } の詳細を表示します。
matrix-point = セルにポインターを合わせると
matrix-point-note = トーカーとリスナー、両者のフォーマットが合うかどうかを表示します。
matrix-legend-waiting = バインド済み、ストリーム待ち
matrix-legend-trouble = バインド済み、問題あり
matrix-legend-open = 接続可能
matrix-legend-change = 先に入力フォーマットの変更が必要
matrix-legend-incompatible = フォーマットに互換性なし
matrix-talker-outputs = トーカー出力
matrix-listener-inputs = リスナー入力

common-thousands-separator = {","}
common-decimal-separator = {"."}

## Diagnostics

diag-since-start = エンティティの起動以降のカウントです。
diag-stream-input = ストリーム入力
diag-stream-output = ストリーム出力
diag-locked = { $count ->
    [0] ロックなし
   *[other] ロック { $number } 回
}
diag-lost-lock = { $count ->
    [0] ロック外れなし
   *[other] ロック外れ { $number } 回
}
diag-frames-in = 受信フレーム { $number }
diag-frames-out = 送信フレーム { $number }
diag-media-locked = { $count ->
    [0] メディアロックなし
   *[other] メディアロック { $number } 回
}
diag-lost-media-lock = { $count ->
    [0] メディアロック外れなし
   *[other] メディアロック外れ { $number } 回
}
diag-interrupted = { $count ->
    [0] 中断なし
   *[other] 中断 { $number } 回
}
diag-out-of-sequence = 順序が乱れたフレーム { $number }
diag-media-resets = メディアリセット { $number } 回
diag-timestamps-uncertain = { $count ->
    [0] タイムスタンプ不確定なし
   *[other] タイムスタンプ不確定 { $number } 回
}
diag-no-timestamp = タイムスタンプのないフレーム { $number }
diag-unsupported-format = 非対応フォーマットのフレーム { $number }
diag-late = 遅着フレーム { $number }
diag-early = 早着フレーム { $number }
diag-started = { $count ->
    [0] 開始なし
   *[other] 開始 { $number } 回
}
diag-stopped = { $count ->
    [0] 停止なし
   *[other] 停止 { $number } 回
}
diag-reservation-failed = トーカーの予約に失敗：{ $reason }
diag-latency = 累積レイテンシー { $microseconds } µs

## AVB Lite

lite-active = アクティブ
lite-active-untagged = アクティブ、タグなし
lite-active-vlan = アクティブ、VLAN { $vlan }
lite-capable = 対応
lite-mode = モード
lite-mode-capable = AVB（AVB Lite 対応）
lite-because = 理由
lite-fallback-none = 理由の通知なし
lite-fallback-endpoint = 別のエンドポイントの宣言がそのまま届いたため、間に AVB ブリッジはありません
lite-fallback-unanswered = ピア遅延要求 9 回に応答なし
lite-fallback-responders = 1 回のピア遅延要求に 2 台以上が応答したため、スイッチは AVB ブリッジではありません
lite-fallback-configured = オペレーターまたはコントローラーが設定
lite-fallback-other = プロファイルに定義のない理由
lite-other-profile = 別のプロファイル
lite-ptp-domain = { $profile }、ドメイン { $domain }
lite-offset = オフセット
lite-offset-from = { $grandmaster } に対し { $offset }
lite-media-vlan = メディア VLAN
lite-untagged = タグなし
lite-unicast = ユニキャスト
lite-fanout = ストリームあたり最大 { $count } リスナー、それ以上はマルチキャスト
lite-link = リンク
lite-bandwidth = 帯域幅
lite-egress-of = { $used } / { $link }、{ $share }
lite-egress-of-assumed = { $used } / { $link }、{ $share }（ギガビットリンクと仮定）
lite-egress-reported = エンティティが数えた許可済みストリームに基づく値です。
lite-egress-worked-out = 接続済みストリーム出力のフォーマットから算出した値です。
lite-alarm-offset = PTP オフセット { $offset }、AVB Lite の許容値 50 µs を超過
lite-alarm-egress = エグレスがリンクの { $share }、ストリームの上限 { $limit } を超過

## AVB Wireless

wireless-station = ステーション
wireless-access-point = アクセスポイント
wireless-role = ロール
wireless-mode = モード
wireless-time = 時刻
wireless-mode-a-ftm = Mode A、FTM による 802.1AS
wireless-mode-a-tm = Mode A、TM による 802.1AS
wireless-mode-b = Mode B、ビーコンから取得
wireless-no-time = 時刻同期なし
wireless-other-mode = プロファイルに定義のないモード
wireless-locked = ロック済み
wireless-holdover = ホールドオーバー中
wireless-not-locked = 未ロック
wireless-row-locked = ステーション、ロック済み
wireless-row-holdover = ステーション、ホールドオーバー中
wireless-row-not-locked = ステーション、未ロック
wireless-row-access-point = アクセスポイント、ステーション { $count } 台
wireless-link = リンク
wireless-channel = チャンネル { $channel }
wireless-not-known = 不明
wireless-signal = 信号強度
wireless-rate = 送信レート
wireless-ftm-valid = 有効 { $share }
wireless-rtt = 往復 { $rtt }
wireless-bursts = バーストあたり { $count } フレーム
wireless-not-as-capable = FALSE、{ $reason }
wireless-reason-bursts = アクセスポイントが許可する FTM バーストのフレーム数が 3 または 2 以外
wireless-reason-measurement = アクセスポイントとの間で FTM も TM も利用不可
wireless-reason-signaling = アクセスポイントからの gPTP 対応 Signaling メッセージなし
wireless-reason-other = プロファイルに定義のない理由
wireless-servo = サーボ誤差
wireless-stations = ステーション
wireless-station-count = ステーション { $count } 台
wireless-no-ftm = FTM 非対応
wireless-unserved = 配信されないリスナー
wireless-frame-budget = フレームバジェット
wireless-frame-use = 毎秒 { $used } / { $budget } フレーム
wireless-stream-frames = ストリームフレーム
wireless-frames-of = ステーション宛 { $readdressed }、リスナーなし { $unmapped }、破棄 { $dropped }、ステーションから { $restored }
wireless-class-a-allowed = 許可
wireless-class-a-not-allowed = 不許可
wireless-alarm-not-locked = Wi-Fi の時刻がアクセスポイントに未ロック
wireless-alarm-holdover = Wi-Fi の時刻がホールドオーバー中、アクセスポイントとのロック外れ
wireless-alarm-unserved = Wi-Fi ポートのリスナー { $count } 台に未配信、ユニキャストの上限を超過

## Log

log-all = すべて
log-warnings = 警告
log-pause = 一時停止
log-resume = 再開
log-clear = クリア
log-empty = triib が送受信するすべての ATDECC フレームを、新しい順にここに表示します。
log-none-match = フィルターに一致するフレームはありません。
log-frames = { $count } フレーム
log-shown-of = { $shown } / { $all } フレーム
log-sent = 送信
log-heard = 受信
log-not-decoded = 未デコード
log-warning-short = control_data_length の値がフレームの末尾を { $missing } オクテット超えています。
log-warning-undecodable = デコードできません：{ $error }。
log-warning-long-acmp = 長い形式の ACMP です。Milan エンティティはこの形式を送信できません（Milan 1.3、5.5.2.2）。

## Channel mappings

mapping-section = チャンネルマッピング
mapping-inputs = 入力
mapping-outputs = 出力
mapping-port = ポート { $number }
mapping-fixed = 固定
mapping-not-read = まだ読み込んでいません。
mapping-no-clusters = クラスターはありません。
mapping-no-streams = オーディオストリームはありません。
mapping-none = マッピングはありません。
mapping-not-mapped = 未マッピング
mapping-cluster-numbered = クラスター { $index }

## Presets

presets-note = プリセットには、各エンティティのクロックソース、サンプリングレート、ストリームフォーマット、コントロール、接続を保存します。リコールすると、異なる部分だけを変更します。
presets-none = 保存されたプリセットはまだありません。
presets-connections = 接続 { $count } 件
presets-recall = リコール
presets-delete = 削除
presets-no-place = プリセットを保存する場所がありません：ホームフォルダーが不明です。
presets-undeletable = { $path } を削除できませんでした：{ $error }。
presets-saved = 「{ $name }」を保存しました（エンティティ { $count } 個）。
presets-nothing-differs = 「{ $name }」と異なる点はありません。
presets-recalling = 「{ $name }」をリコール中：変更 { $count } 件。
presets-missing = { $report } ネットワーク上にないか未読み込み：{ $missing }。
presets-deleted = 「{ $name }」を削除しました。
presets-host-note = このコンピューター自身のエンティティも保存し、リコール時に再起動します。
presets-host-endpoints = このコンピューター上に { $count } 件
presets-starting-host = 「{ $name }」のために、このコンピューターのエンティティを起動しています。戻り次第、残りを適用します。

## Controls

control-numbered = コントロール { $index }
control-not-shown = ここには表示されません
control-option = オプション { $number }

## Network errors

network-permission = triib が raw イーサネットフレームを送受信するには権限が必要です。
network-needs-npcap = triib が raw イーサネットフレームを送受信するには Npcap が必要です。
network-npcap-administrators = Npcap は管理者にのみ raw イーサネットフレームの送受信を許可しています。triib を管理者として実行するか、管理者限定のオプションを外して Npcap を再インストールしてください。

matrix-stream-format = { $format }。
matrix-stream-format-state = { $format }。{ $state }。

## This computer's own entities

host-add-entity = エンティティを追加
host-show-mine = このコンピューター自身のエンティティだけを表示
host-show-all = すべてのエンティティを表示
host-new-entity = ホストのエンティティ { $number }
host-failed = このコンピューターに追加できませんでした：{ $reason }
host-needs-clock = このコンピューター自身のエンティティには、PTP ハードウェアクロックを備えた有線インターフェイスが必要です
host-no-ptp4l = ptp4l が応答しないため、このコンピューターのストリームは gPTP 時刻を保てません
host-elsewhere = triib-endpointd が { $interface } 上で別のユーザーまたは root として動作しているため、このコンピューターのエンティティはここではなくそちらで動作します
host-foreign-mrp = 別のプログラムがこのコンピューターのアドレスから { $interface } 上で MSRP または MVRP を宣言しているため、このコンピューターのストリームに必要なものが取り下げられる可能性があります
host-alarm-foreign-mrp = このコンピューター上の別のプログラムが { $interface } で MSRP または MVRP を宣言中
host-state = 状態
host-streaming = 送信中
host-waiting = リスナーを待機中
host-listening = 受信待機中
host-bound = バインド済み、トーカーを待機中
host-unbound = バインドなし
host-audio-from = オーディオの入力元
host-audio-to = オーディオの出力先
host-channels = チャンネル数
host-virtual-device = 仮想デバイス
host-add-stream = ストリームを追加
host-add-audio-input = オーディオ入力
host-add-audio-output = オーディオ出力
host-add-clock-input = メディアクロック入力
host-add-clock-output = メディアクロック出力
host-remove-stream = ストリームを削除
host-silence = 無音
host-tone = テストトーン
host-nowhere = 出力しない
host-default-device = 既定のデバイス
host-remove = このコンピューターから削除
