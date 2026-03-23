# Project I.R.I.S. — 開発タスク管理

このファイルはプロジェクト内の開発進捗を追跡するためのタスクリストです。

## 🟢 フェーズ 1: プロジェクト基盤

- [x] `README.md` の作成
- [x] `GEMINI.md` エージェント設定ファイルの作成（日本の業務仕様）
- [x] `.gemini/` 配下のドキュメント整備（docs / rules / skills / workflows）

## 🟢 フェーズ 2: バックエンド スキャフォールディング

- [x] `cargo init` によるプロジェクト初期化
- [x] **Sense層**: Axum Webサーバー (`/health`, `/api/chat`)
- [x] **Sense層**: OpenCV Vision サブシステム（モック実装）
- [x] **Logic層**: Tokio 非同期オーケストレーションループ
- [x] **Logic層**: Ollama クライアント（Gemma 3n 連携）
- [x] **Memory層**: SurrealDB 接続・グラフスキーマ定義
- [x] **Memory層**: `insert_memory` / `spread_activation` の実装
- [x] **Action層**: Tavily API クライアント（自律リサーチ）
- [x] **Action層**: 応答合成（`synthesize_response`）

## 🟢 フェーズ 3: Docker 開発環境

- [x] `Dockerfile`（マルチステージビルド）
- [x] `docker-compose.yml`（app / surrealdb / ollama）
- [x] `.env.example` の作成
- [x] `.gitignore` の整備
- [x] Dockerfile にユーザー権限設定を追加（`UID/GID` 対応）

## 🔵 フェーズ 4: コア機能の実装

- [ ] SurrealDB の詳細クエリ実装（連想想起ロジック）
- [ ] Ollama クライアントの会話コンテキスト管理
- [ ] Sense → Memory → Logic → Action のフルパイプライン統合
- [ ] Axum ルートと各層の実際の接続

## 🟡 フェーズ 5: 視覚システム

- [ ] OpenCV-Rust の実際のカメラ入力統合
- [ ] 顔認識パイプラインの実装
- [ ] プレゼンス検知とメモリグラフへの連携

## 🟡 フェーズ 6: 管理ダッシュボード

- [ ] Leptos による記憶ネットワーク可視化 UI
- [ ] 記憶の「鮮明度（Vividness）」の可視化

## ⚪ フェーズ 7: 音声合成

- [ ] TTS への感情パラメータ実装
- [ ] Raspberry Pi 5 でのエンドツーエンドテスト
