---
description: I.R.I.S. プロジェクトのビルドおよびテスト実行ワークフロー
---
# I.R.I.S. ビルド・テスト ワークフロー

このワークフローは、Rustバックエンドのビルド、コードの静的解析（Clippy）、およびテストを自動的に実行するための手順です。

## Step 1: コードのフォーマットと静的解析
コードのスタイルを統一し、潜在的なバグを検出するために実行します。
// turbo
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

## Step 2: プロジェクトのビルド
プロジェクト全体をコンパイルします。
// turbo
```bash
cargo build
```

## Step 3: ユニットテストと結合テストの実行
すべてのテストを実行し、機能が正常に動作することを確認します。
// turbo
```bash
cargo test
```

## Step 4: （オプション）リリースビルド
パフォーマンス最適化が必要な場合や、デプロイ時に実行します。
```bash
cargo build --release
```
