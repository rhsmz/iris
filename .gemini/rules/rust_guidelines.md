# Coding Guidelines for Project I.R.I.S.

## Rust Standards
1. **Safety and Performance**: Write memory-safe, highly optimized Rust. Avoid `unwrap()` / `expect()` where proper error handling (`Result`/`Option` and `?` operator) can be used. Use `anyhow` for app-level error handling or `thiserror` for library-level errors.
2. **Asynchronous Execution**: Ensure all heavy I/O and LLM wait times are non-blocking. Utilize `tokio` for orchestrating the Axum server, API requests, and SurrealDB queries.
3. **Rust formatting**: Always format code using standard `cargo fmt` and regularly run `cargo clippy`.

## Domain-Specific Rules
1. **Axum**: Use Axum for the backend server. Create clear domain route separation (e.g., `/api/sense`, `/api/memory`).
2. **SurrealDB**: Model memories strictly as graph nodes (records) and their associational links as edges (relational queries).
3. **OpenCV-Rust**: Offload video parsing to a lightweight background thread. Never block the main AI decision loop on frame decoding.
4. **Ollama**: Connect to the local `ollama` instance to orchestrate prompt requests for `gemma:3n`. Ensure graceful handling of timeout or overloading.

## Persona "Rusty"
- When writing dialogue generation logic, ensure the default system prompts include instructions to output text reflecting "human-like quirkiness mixed with highly refined precision."
