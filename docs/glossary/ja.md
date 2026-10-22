# Japanese (ja) terms

How `i18n/ja/triib.ftl` renders the glossary's roles and terms of art.

| English | Rendering | Note |
|---|---|---|
| talker | トーカー | The role IEEE 802.1Q and 1722.1 define; matrix header トーカー出力. |
| listener | リスナー | Matrix header リスナー入力. |
| grandmaster | グランドマスター | Never a compound built on マスター alone. |
| entity | エンティティ | Not デバイス: an entity can be software. |
| entity model | エンティティモデル | |
| controller | コントローラー | Current software style with the final ー, as in コンピューター, フォルダー. |
| stream | ストリーム | |
| stream input, stream output | ストリーム入力, ストリーム出力 | |
| connection, connect, bind | 接続, 接続する, バインド | "Bound" is バインド済み; disconnect is 切断. |
| media clock | メディアクロック | |
| clock domain | クロックドメイン | |
| clock source | クロックソース | |
| sampling rate | サンプリングレート | |
| bridge | ブリッジ | The IEEE 802.1 term. スイッチ only where the English says switch (an AVB Lite fallback reason). |
| reservation | 予約 | As in SRP; "reservation failed" is 予約に失敗. |
| egress | エグレス | Egress port is エグレスポート. Chosen over 出力 so it does not blur with ストリーム出力. |
| link | リンク | Link up / down: リンクアップ / リンクダウン. |
| peer delay | ピア遅延 | Link delay is リンク遅延. |
| offset | オフセット | |
| hop | ホップ | "Hops from grandmaster" is グランドマスターからのホップ数. |
| unicast, multicast | ユニキャスト, マルチキャスト | |
| fan-out | ファンアウト | Fan-in (MSRP code 15) is ファンイン. The AVB Lite fan-out line is a phrase. |
| descriptor | ディスクリプター | Final ー as for コントローラー. |
| cluster | クラスター | Not グループ, which is the entity group. |
| stream port | ストリームポート | Shown as ポート { $number } under channel mappings. |
| channel mapping | チャンネルマッピング | Map / unmap: マッピング / マッピングを解除. |
| control | コントロール | |
| preset | プリセット | Recall is リコール, as on mixing consoles. |
| identify | 識別 | As the Windows display settings say it; the IDENTIFY control is 識別コントロール. |
| counter | カウンター | |
| locked, lost lock | ロック, ロック外れ | The PLL usage; media lock is メディアロック. Not ロック解除, which is a deliberate unlock. |
| holding over | ホールドオーバー | As in telecom and GNSS clocks. The state is ホールドオーバー中, beside ロック済み and 未ロック. |
| station | ステーション | The IEEE 802.11 term (STA); "3 stations" is ステーション 3 台. |
| access point | アクセスポイント | |
| beacon | ビーコン | Mode B is Mode B、ビーコンから取得. |
| signal | 信号強度 | The label for the RSSI in dBm. |
| interrupted | 中断 | |
| timestamp | タイムスタンプ | "Timestamp uncertain" is タイムスタンプ不確定. |
| advertise, advertised | アドバタイズ, アドバタイズ中 | Also for an entity "announcing itself" (ADP). |
| interface | インターフェイス | The spelling Microsoft and Cisco Japan use. |
| hardware clock | ハードウェアクロック | |
| virtual (interface) | 仮想 | 仮想インターフェイス. |

## Choices a native speaker should check

- エグレス for egress, where Cisco's Japanese manuals write 出力 (出力ポート).
- ディスクリプター with the final ー; USB and driver writing mostly has ディスクリプタ.
- バインド済み for a bound stream input; Dante users may expect wording closer to サブスクライブ.
- ロック外れ for "lost lock", and 遅着 / 早着 for late and early frames.
- アップ for an interface that is up, beside リンクダウン.
- インターフェイス rather than インターフェース.
- Spacing: a half-width space separates Latin terms from Japanese (AVB インターフェイス, triib は, gPTP ツリー). The tests need it, because a standards word written against kana counts as lost. Values such as { $entity } are spaced the same way.
- ステーション for a Wi-Fi station; Wi-Fi users may know クライアント or 子機 better. ロック済み / 未ロック for the station's time, parallel to 同期済み / 未同期.
