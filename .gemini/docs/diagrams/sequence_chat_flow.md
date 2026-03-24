# シーケンス図: チャットエンドポイント フロー

## 概要
`POST /api/chat` エンドポイントにおける、Sense → Memory → Logic → Action の処理フローを示します。
README 2.4 のハイブリッド・ストレージ（CMS）設計および Spreading Activation を反映しています。

## 図

```mermaid
sequenceDiagram
    participant User as ユーザー
    participant Axum as Sense層 (Axum)
    participant Logic as Logic層 (Reasoning)
    participant Memory as Memory層 (SurrealDB)
    participant FS as エピソードCMS (FileSystem)
    participant Ollama as Local LLM (Gemma 3n)
    participant Action as Action層 (Tavily)

    User->>Axum: POST /api/chat { message }
    Axum->>Logic: ask_with_context(message)

    Note over Logic,Memory: Spreading Activation
    Logic->>Memory: spread_activation(message)
    Memory-->>Logic: 人格・関連ノードのVividness更新完了

    Note over Logic,FS: RAG コンテキスト抽出
    Logic->>Memory: fetch_top_memories(limit)
    Memory->>FS: パスに基づくエピソードの読み出し
    FS-->>Memory: Markdownエピソード実体
    Memory-->>Logic: RetrievedMemory群（メタデータ＋本文）

    Note over Logic,Ollama: プロンプト合成と推論
    Logic->>Logic: Rustyペルソナ＋記憶コンテキスト＋直近会話履歴の合成
    Logic->>Ollama: プロンプト送信 (stream: true)
    Ollama-->>Logic: Streaming JSONチャンク応答
    Logic->>Logic: チャンク受信用コールバック実行

    alt 不明な情報がある場合
        Logic->>Action: Tavily API で検索（非同期）
        Action-->>Logic: 検索結果
        Logic->>Ollama: 再プロンプト（検索結果付き）
        Ollama-->>Logic: 最終応答
    end

    Note over Logic,Memory: 新規記憶の保存と連想バイアス
    Logic->>FS: 応答内容をMarkdownとして書き出し
    FS-->>Logic: file_path
    Logic->>Memory: insert_memory（ノード保存＋Personalityエッジ構築）
    Memory-->>Logic: 保存完了

    Logic-->>Axum: 推論完了
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
