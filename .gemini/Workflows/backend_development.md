---
description: Step-by-step workflow for setting up the I.R.I.S. backend environment.
---
# I.R.I.S Backend Implementation Workflow

Follow the procedures below when implementing the backend of Project I.R.I.S. in a step-by-step and recursive manner.

## Step 1: Workspace Initialization
Add necessary crates and create the foundation of the project.
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

## Step 2: Implementation of Sense Layer (Server)
1. Add `main.rs` to read environment variables and start the `Axum` server.
2. Create and verify the basic `/health` route.
3. Create a stub (alternative implementation) for OpenCV camera integration within the `sense::vision` module.

## Step 3: Implementation of Memory Layer (Database)
1. Connect to local SurrealDB.
2. Define the `MemoryNode` struct.
3. Implement `insert_memory` for saving memories and the `spread_activation` function for expanding associations.

## Step 4: Implementation of Logic Layer (LLM Reasoning)
1. Create `OllamaClient` within `logic::reasoning`.
2. Implement processing to asynchronously send prompts to the local `gemma:3n` model.

## Step 5: Implementation of Action Layer (Output/Action)
1. Create `action::search` using the client of the Tavily API.
2. Orchestrate (integrate/control) the sequence of flow Sense -> Memory -> Logic -> Action within `main.rs`.
