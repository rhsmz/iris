# システムアーキテクチャ: Project I.R.I.S.

## 概要
Project I.R.I.S. は、Raspberry Pi 5上に構築される自律型・連想記憶AIパートナーです。クラウドに依存せず、プライバシーを守ったローカル環境で動作します。Rustバックエンドと大規模言語モデル（Gemma 3n）を統合しています。

## レイヤー構成

### 1. Sense (入力層)
- **Vision（視覚サブシステム）**: OpenCV-Rustと連携し、カメラからの映像をもとに顔認識を行います。特徴量抽出はローカルで完結し、クラウドへの画像送信は一切行いません。主人の顔を検知した瞬間、記憶グラフ内の関連ノードに重みが加算されます。
- **Audio/Text（音声・テキストサブシステム）**: AxumベースのWebサーバーを介して対話やシステムコマンドを受け付けます（`/health`, `/api/chat`）。

### 2. Logic (思考層)
- **Reasoning（推論エンジン）**: Ollamaを経由してローカルで稼働するGemma 3n を活用し推論を行います。必要なエピソードをプロンプトに動的注入するRAGパターンに加え、直近の会話履歴をインメモリで保持して文脈を維持します。また、推論結果はストリーミングで非同期受信されレスポンス性を高めます。
- **Orchestrator（オーケストレーター）**: Tokioランタイムを用いた非同期処理により、Sense → Memory → Logic → Action のパイプラインを管理します。

### 3. Memory (記憶層)

#### グラフ記憶（SurrealDB）
**メタデータのみ**を保存し、記憶間の関係性（エッジ）を管理します：
- `MemoryNode` 構造体: 概念（Concept）、タグ、鮮明度（Vividness）、感情スコア、CMSの絶対ファイルパス
- `RelatesTo` 構造体: ノード間の関連度（重み）を持つエッジ

#### エピソード記憶 CMS（ファイルシステム）
具体的なエピソードテキストや視覚情報はMarkdownファイルとして保存し、検索時に `RetrievedMemory` として動的に結合します：
```
/var/iris/memories/YYYY/MM/DD/episode_xxxx.md
```

#### 記憶の数学モデル
エビングハウスの忘却曲線に基づく鮮明度（Vividness）モデル：

$$V = e^{-\frac{t}{S}}$$

- $V$: 記憶の保持率（鮮明度）
- $t$: 最後に想起してからの経過時間
- $S$: 記憶の強度（感情スコアや重要度による初期値）

#### Spreading Activation（連想想起）[コア・メカニズム]
ある単語が会話に登場すると、グラフDB上の隣接ノードに活性化信号が伝播し、周辺の記憶を同時にリフレッシュします。これにより「そういえば、あれについても……」という人間らしい連想が可能になります。

#### 人格コア・ノード
グラフ上に「ユーモア (humor)」「皮肉 (sarcasm)」「Rustへの愛 (rust_love)」を持つ `PersonalityNode` を配置します。新しい記憶が入力されるたびに弱いエッジが自動構築され、連想の方向に一貫した「Rusty」なバイアスをかけます。

### 4. Action (出力層)
- **Response Synthesis（応答生成）**: 「Rusty」ペルソナに基づいた愛嬌ある応答を生成します。
- **Autonomous Research（自律リサーチ）**: 不足情報があれば Tavily API を使用して自律検索を行います。

## コンテナ構成（Docker）

```
Project Root
├── docker-compose.yml         # 共通ベース（ネットワーク・ボリューム・環境変数・Ollama・SurrealDB基本設定）
├── docker-compose.dev.yml     # Windows開発用（ソースコード・キャッシュマウント、カメラなし、自動ビルド）
└── docker-compose.release.yml # Raspberry Pi本番用（カメラマウント、軽量ランタイムビルド）
```

## 技術スタック

| 区分 | 技術 |
|---|---|
| 言語 | Rust 1.75+ (Stable) |
| 非同期ランタイム | Tokio |
| Webサーバー | Axum |
| データベース | SurrealDB |
| LLMランタイム | Ollama (Gemma 3n) |
| 視覚処理 | OpenCV-Rust |
| 自律リサーチ | Tavily API |
| コンテナ | Docker / Docker Compose |
| ターゲットハードウェア | Raspberry Pi 5 (16GB) |
