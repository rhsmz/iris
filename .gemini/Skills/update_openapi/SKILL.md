---
name: OpenAPI 仕様書更新スキル
description: 実装済みの Axum ルートを読み取り、openapi.yaml を正確に更新するスキルです。
---

# OpenAPI 仕様書更新スキル

> [!IMPORTANT]
> **このスキルはコードを読んでから実行すること。実際の Axum ルートを参照せずに仕様書を更新してはならない。**

## 実行手順

### 1. 対象コードの確認（必須）
`openapi.yaml` を更新する前に、必ず以下を `view_file` で確認すること：

- `src/sense/server.rs` — ルート定義・ハンドラ関数
- 対象ハンドラの Request / Response 構造体（`#[derive(Deserialize)]` / `#[derive(Serialize)]`）
- エラーハンドリングのパターン（返しうる HTTP ステータスコード）

**確認すべき情報:**
```
- HTTPメソッド（get / post / put / delete）
- パス文字列（例: "/api/chat"）
- リクエストボディの型名とフィールド定義
- レスポンスボディの型名とフィールド定義
- 返しうるステータスコード（200 / 400 / 500 等）
```

### 2. 現在の openapi.yaml の確認
```
view_file: .gemini/docs/api/openapi.yaml
```
既存のパスと重複・矛盾がないかを確認する。

### 3. 仕様書の更新ルール
- 形式: **OpenAPI 3.1.0**
- `summary` / `description`: **必ず日本語**で記述する
- スキーマは `components/schemas` に定義し、`$ref` で参照する
- 実装コードの型名・フィールド名と一致させる（コードが正、仕様書が従）
- 追加するパスの `operationId` はキャメルケース英語で付与する

**パス追加パターン:**
```yaml
paths:
  /api/[実際のパス]:
    [HTTPメソッド]:
      summary: [日本語の機能説明]
      description: |
        [日本語の詳細説明。実際の処理フローを記述。]
      operationId: [キャメルケースの識別子]
      tags:
        - [タグ名]
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/[実際の構造体名]'
      responses:
        '200':
          description: [成功時の説明]
```

### 4. 更新後の確認
- YAML の構文エラーがないかを確認する
- 追加したエンドポイントが実際のコードと一致しているか再確認する

## ❌ やってはいけないこと
- 実装前のエンドポイントを仕様書に追加する
- Rustの構造体フィールドと型が異なる名称をスキーマに使用する
- `summary` / `description` を英語で記述する
