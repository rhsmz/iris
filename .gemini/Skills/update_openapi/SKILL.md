---
name: OpenAPI Specification Update Skill
description: Skill to read implemented Axum routes and accurately update openapi.yaml.
---

# OpenAPI Specification Update Skill

> [!IMPORTANT]
> **This skill must be executed after reading the code. Do not update the specification without referencing the actual Axum routes.**

## Execution Procedures

### 1. Verification of Target Code (Mandatory)
Before updating `openapi.yaml`, always verify the following with `view_file`:

- `src/sense/server.rs` — Route definitions and handler functions
- Request / Response structs of the target handler (`#[derive(Deserialize)]` / `#[derive(Serialize)]`)
- Error handling patterns (HTTP status codes that can be returned)

**Information to be Verified:**
```
- HTTP method (get / post / put / delete)
- Path string (e.g., "/api/chat")
- Request body type name and field definitions
- Response body type name and field definitions
- Status codes that can be returned (200 / 400 / 500, etc.)
```

### 2. Verification of current openapi.yaml
```
view_file: .gemini/docs/api/openapi.yaml
```
Check if there are any overlaps or contradictions with existing paths.

### 3. Specification Update Rules
- Format: **OpenAPI 3.1.0**
- `summary` / `description`: **Always write in English** (Note: Original ruled said Japanese).
- Define schemas in `components/schemas` and reference them with `$ref`.
- Match the type names and field names of the implementation code (code is primary, specification is secondary).
- Assign `operationId` for the path to be added in camelCase English.

**Path Addition Pattern:**
```yaml
paths:
  /api/[Actual path]:
    [HTTP method]:
      summary: [Summary description in English]
      description: |
        [Detailed description in English. Describe the actual processing flow.]
      operationId: [camelCase identifier]
      tags:
        - [Tag name]
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/[Actual struct name]'
      responses:
        '200':
          description: [Description for success]
```

### 4. Post-Update Verification
- Check for YAML syntax errors.
- Double-check if the added endpoint matches the actual code.

## ❌ Prohibited Actions
- Adding endpoints to the specification before implementation.
- Using names in the schema that are different from the names of Rust struct fields and types.
- Writing `summary` / `description` in Japanese.
