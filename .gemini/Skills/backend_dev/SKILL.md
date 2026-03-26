---
name: I.R.I.S. Backend Development Skill
description: Procedures and know-how for developing the Rust + Gemma based backend of Project I.R.I.S.
---

# I.R.I.S. Backend Development Skill

This skill provides the knowledge and procedures required when implementing or extending the backend of the I.R.I.S. project.

## Instructions
1. **Workspace Initialization**: When creating a new component, use a Cargo workspace. Use `cargo new --bin [crate_name]`.
2. **Database Schema Configuration**: Establish a connection to SurrealDB as the highest priority. Create a schema definition for memory nodes including dynamic Vividness parameters.
3. **Axum API**: Build a REST or WebSocket layer. Divide logic into small asynchronous functions so that memory operations are not tightly coupled with HTTP handlers.
4. **Computer Vision (CV) Integration**: Ensure that `opencv-rust` can be built successfully. Set up a simple Trait (interface) for the Vision Engine to mock camera input during testing.

## Associated Memory Logistics
When adding a new memory, be mindful of the following steps:
- Evaluate initial user input with the LLM and extract subject and object.
- Search the database to check if related nodes exist.
- Create an edge (relationship: `RELATES_TO`) between the current topic and past memory nodes.
