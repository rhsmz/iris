# Project I.R.I.S. — 開発タスク管理

このファイルはプロジェクト内の開発進捗を追跡するためのタスクリストです。
README のロードマップと連動しています。

---

## 🟢 フェーズ 1: プロジェクト基盤

- [x] `README.md` の作成・更新
- [x] `GEMINI.md` エージェント設定ファイルの作成（日本の業務仕様）
- [x] `.gemini/` 配下のドキュメント整備（docs / rules / skills / workflows）
- [x] `TASKS.md` の作成（本ファイル）
- [x] システムアーキテクチャの基本設計
- [x] エピソード記憶のハイブリッド・ストレージ構造（CMS設計）の策定
- [x] Dockerによるコンテナ化・開発環境の分離

---

## 🟢 フェーズ 2: バックエンド スキャフォールディング

- [x] `cargo init` によるプロジェクト初期化
- [x] **Sense層**: Axum Webサーバー (`/health`, `/api/chat`)
- [x] **Sense層**: OpenCV Vision サブシステム（モック実装）
- [x] **Logic層**: Tokio 非同期オーケストレーションループ
- [x] **Logic層**: Ollama クライアント（Gemma 3n 連携）
- [x] **Memory層**: SurrealDB 接続・グラフスキーマ定義（`MemoryNode` / `RelatesTo`）
- [x] **Memory層**: `insert_memory` / `spread_activation` の実装
- [x] **Action層**: Tavily API クライアント（自律リサーチ）
- [x] **Action層**: 応答合成（`synthesize_response`）

---

## 🟢 フェーズ 3: Docker 開発環境

- [x] `Dockerfile`（マルチステージビルド・非rootユーザー設定）
- [x] `docker-compose.yml`（iris-core / surrealdb / ollama の3サービス構成）
- [x] `.env.example` の作成
- [x] `.gitignore` の整備
- [x] Dockerfile にユーザー権限設定を追加（`UID/GID` 対応）

---

## 🔵 フェーズ 4: コア機能の実装

- [ ] **グラフベースの連想想起ロジックの実装（SurrealDB + Rust）**
  - [ ] Spreading Activation アルゴリズムの実装
  - [ ] 記憶ノードの鮮明度（Vividness）自動減衰処理
  - [ ] 人格コアノード（ユーモア・皮肉・Rustへの愛）の設計・配置
- [ ] **Rust による Ollama API クライアントの実装**
  - [ ] 会話コンテキスト管理（マルチターン対応）
  - [ ] エピソード記憶の動的プロンプト注入（CMSアーキテクチャ）
- [ ] Sense → Memory → Logic → Action のフルパイプライン統合
- [ ] Axum ルートと各層の実際の接続

---

## 🟡 フェーズ 5: 視覚システム

- [ ] **カメラ接続と OpenCV による顔認識パイプラインの実装**
  - [ ] OpenCV-Rust の実際のカメラ入力統合
  - [ ] 顔認識ロジック（特徴量抽出）
  - [ ] プレゼンス検知とメモリグラフへのノード重み加算

---

## 🟡 フェーズ 6: エピソード記憶 CMS

- [ ] ファイルシステム記憶ストレージ（`/var/iris/memories/YYYY/MM/DD/` 階層）
- [ ] SurrealDB へのメタデータ（タグ・感情スコア・ファイルパス）保存
- [ ] 連想発火時のファイル取得・プロンプト注入 Retrieval 実装

---

## 🟡 フェーズ 7: 管理ダッシュボード

- [ ] Leptos による記憶ネットワーク可視化 UI
- [ ] 記憶の「鮮明度（Vividness）」の可視化

---

## ⚪ フェーズ 8: 音声合成

- [ ] TTS への感情パラメータ実装
- [ ] Raspberry Pi 5 でのエンドツーエンドテスト
