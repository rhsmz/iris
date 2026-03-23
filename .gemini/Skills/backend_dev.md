---
name: I.R.I.S. Backend Development Skill
description: Instructions for developing the Project I.R.I.S. Rust + Gemma backend.
---

# I.R.I.S. Backend Development Skill

This skill provides the required knowledge for implementing the I.R.I.S. backend.

## Instructions
1. **Initialize Workspace**: Start by creating a Cargo workspace. Use `cargo new --bin [crate_name]`.
2. **Database Schema Setup**: First establish the connection to SurrealDB. Create a memory schema definition containing dynamic vividness parameters.
3. **Axum API**: Build the REST/WebSocket layer. The logic should be broken into discrete async functions to avoid tightly coupling memory operations to HTTP handlers.
4. **Computer Vision Interop**: Ensure `opencv-rust` is successfully building. Set up a simple trait for the Vision Engine to mock camera input during testing.

## Associated Memory Logistics
Remember that adding a memory requires evaluating the initial user input with the LLM to extract subjects/objects, checking the database for nodes, and creating edges (`RELATES_TO`) between the current topic and previous nodes.
