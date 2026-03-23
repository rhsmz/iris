---
description: I.R.I.S. バックエンド環境をセットアップするためのステップバイステップ・ワークフロー
---
# I.R.I.S バックエンド実装ワークフロー

Project I.R.I.S. のバックエンドを段階的かつ再帰的に実装する際は、以下の手順に従ってください。

## Step 1: ワークスペースの初期化
必要なクレートを追加し、プロジェクトの基盤を作成します。
// turbo
```bash
cargo init --bin .
cargo add tokio -F full
cargo add axum
cargo add serde -F derive
cargo add serde_json
cargo add surrealdb
cargo add reqwest -F json
```

## Step 2: Sense 層（サーバー）の実装
1. 環境変数を読み込み、`Axum` サーバーを起動する `main.rs` を追加する。
2. 基本的な `/health` ルートを作成・確認する。
3. `sense::vision` モジュール内で OpenCV カメラ連携のスタブ（代替実装）を作成する。

## Step 3: Memory 層（データベース）の実装
1. ローカルの SurrealDB に接続する。
2. `MemoryNode` 構造体を定義する。
3. 記憶を保存する `insert_memory` と、連想を広げる `spread_activation` 関数を実装する。

## Step 4: Logic 層（LLM推論）の実装
1. `logic::reasoning` 内に `OllamaClient` を作成する。
2. ローカルの `gemma:3n` モデルに対して非同期でプロンプトを送信する処理を実装する。

## Step 5: Action 層（出力・行動）の実装
1. Tavily API のクライアントを利用した `action::search` を作成する。
2. `main.rs` 内で Sense -> Memory -> Logic -> Action の一連の流れをオーケストレーション（統合・制御）する。
