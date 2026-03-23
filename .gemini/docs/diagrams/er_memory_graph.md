# ER 図: 記憶グラフ スキーマ

## 概要
SurrealDB 上で管理される記憶ノードと、その関係性（エッジ）のスキーマを示します。
README 2.4 のハイブリッド・ストレージ設計に基づき、SurrealDB には**メタデータのみ**を保存します。

## 図

```mermaid
erDiagram
    MemoryNode {
        string id PK "memory:concept_name"
        string concept "記憶の概念・キーワード"
        float vividness "鮮明度 V = e^(-t/S)  (0.0 ~ 1.0)"
        int last_accessed "最終アクセス UNIX タイムスタンプ"
        string file_path "エピソードファイルのパス（CMSストレージ）"
        float emotion_score "感情スコア（記憶強度 S の初期値）"
        string tags "トピックタグ（カンマ区切り）"
    }

    RelatesTo {
        float weight "関連度の重み (0.0 ~ 1.0)"
    }

    PersonalityNode {
        string id PK "personality:trait_name"
        string trait "人格特性（例: humor / sarcasm / rust_love）"
        float initial_weight "初期エッジ重み（高いほど連想されやすい）"
    }

    MemoryNode ||--o{ RelatesTo : "出発 (spread_activation)"
    RelatesTo }o--|| MemoryNode : "到達"
    PersonalityNode ||--o{ RelatesTo : "バイアス出発"
    RelatesTo }o--|| MemoryNode : "バイアス到達"
```

## 備考
- `vividness` はエビングハウス忘却曲線 $V = e^{-t/S}$ に基づき時間とともに減衰する。
- `spread_activation` により、アクセスされたノードの隣接ノードが自動的に鮮明度を加算される。
- `PersonalityNode` のエッジ重みを高く設定することで、連想方向に「Rusty」な人格バイアスをかける。
- `file_path` に実際のエピソードテキストへの参照（CMS構造: `/var/iris/memories/YYYY/MM/DD/`）を格納する。

## Rust 実装との対応
| ER要素 | Rust 実装 |
|---|---|
| `MemoryNode` | `src/memory/graph.rs` の `MemoryNode` 構造体 |
| `RelatesTo` | `src/memory/graph.rs` の `RelatesTo` 構造体 |
| `spread_activation` | `src/memory/graph.rs` の `spread_activation()` 関数 |

## 更新履歴
| 日付 | 変更内容 |
|---|---|
| 2026-03-24 | 初版作成（基本スキーマ） |
| 2026-03-24 | CMSアーキテクチャ対応・PersonalityNode追加・Rust実装との対応表追記 |
