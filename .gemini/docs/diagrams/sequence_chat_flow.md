# シーケンス図: チャットエンドポイント フロー

## 概要
`POST /api/chat` エンドポイントにおける、Sense → Memory → Logic → Action の処理フローを示します。
README 2.4 のハイブリッド・ストレージ（CMS）設計および Spreading Activation を反映しています。

## 図

```mermaid
sequenceDiagram
    participant User as ユーザー
    participant Axum as Sense層 (Axum)
    participant Memory as Memory層 (SurrealDB)
    participant FS as エピソードCMS (FileSystem)
    participant Logic as Logic層 (Ollama/Gemma 3n)
    participant Action as Action層 (Tavily)

    User->>Axum: POST /api/chat { message }

    Note over Axum,Memory: Spreading Activation
    Axum->>Memory: キーワードで記憶ノードを検索
    Memory-->>Axum: 関連ノード群（鮮明度・ファイルパス付き）

    Note over Axum,FS: CMS Retrieval - 動的コンテキスト注入
    Axum->>FS: 鮮明度の高いノードのファイルパスを取得
    FS-->>Axum: エピソードテキスト（Markdownファイル）

    Note over Axum,Logic: RAG プロンプト構築
    Axum->>Logic: プロンプト送信（記憶コンテキスト＋エピソード注入）
    Logic-->>Axum: Gemma 3n 推論結果

    alt 不明な情報がある場合
        Axum->>Action: Tavily API で検索（非同期）
        Action-->>Axum: 検索結果
        Axum->>Logic: 再プロンプト（検索結果付き）
        Logic-->>Axum: 最終応答
    end

    Note over Axum,Memory: 新しい記憶の保存
    Axum->>Memory: insert_memory（新ノード＋メタデータ）
    Axum->>FS: エピソードをMarkdownファイルとして書き出し
    Memory-->>Axum: 保存完了

    Axum-->>User: ChatResponse { reply }
```

## 実装ファイルとの対応
| 図中の参加者 | ソースファイル |
|---|---|
| Sense層 (Axum) | `src/sense/server.rs` |
| Memory層 (SurrealDB) | `src/memory/graph.rs` |
| Logic層 (Ollama) | `src/logic/reasoning.rs` |
| Action層 (Tavily) | `src/action/research.rs` |

## 更新履歴
| 日付 | 変更内容 |
|---|---|
| 2026-03-24 | 初版作成 |
| 2026-03-24 | CMSアーキテクチャ（FileSystem）・Spreading Activation・実装対応表を追加 |
