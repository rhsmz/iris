---
description: Step-by-step workflow for setting up the I.R.I.S. backend
---
# I.R.I.S Backend Implementation Workflow

Follow these steps to recursively implement the backend for Project I.R.I.S.

## Step 1: Initialize Workspace
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

## Step 2: Implement Sense Layer (Server)
1. Add `main.rs` that loads environment variables and starts the `axum` server.
2. Create basic `/health` route.
3. Stub the OpenCV camera integration in a `sense::vision` module.

## Step 3: Implement Memory Layer (Database)
1. Connect to local SurrealDB.
2. Define `MemoryNode` struct.
3. Implement `insert_memory` and `spread_activation` functions.

## Step 4: Implement Logic Layer (LLM)
1. Create `OllamaClient` in `logic::reasoning`.
2. Send prompts to `gemma:3n` asynchronously.

## Step 5: Implement Action Layer
1. Create `action::search` using Tavily API client.
2. Orchestrate Sense -> Memory -> Logic -> Action in `main.rs`.
