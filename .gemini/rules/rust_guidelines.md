# Coding and Operational Guidelines: Project I.R.I.S.

## Basic Operational Rules for Agents
1. **Fixed Language**: All interactions with the user, as well as documents and comments to be output, should be in **English**. (Note: Original rule said Japanese, translating to English as requested).
2. **Regular Commits**: Be sure to perform Git commits regularly at each milestone of work (completion of feature implementation, bug fixes, etc.).
3. **Branch Creation for Commits**: When committing, do not push directly to `main` or `master`. **Create a new branch** with an appropriate name that reflects the work content before committing.

## Rust Coding Conventions
1. **Safety and Performance**: Write memory-safe and optimized Rust code. Avoid using `unwrap()` or `expect()` when proper error handling (`Result`/`Option` and `?` operator) is possible. Use `anyhow` for application-level errors and `thiserror` for library-level errors.
2. **Thorough Asynchronous Processing**: Ensure that heavy I/O processing or LLM waiting time does not block the main thread. Process all Axum server, API requests, and SurrealDB queries asynchronously using `tokio`.
3. **Rust Format**: Always format code using `cargo fmt` and regularly run `cargo clippy` to fix warnings.

## Domain-Specific Rules
1. **Axum**: Use Axum for the backend server. Separate routing by clearly defining responsibility boundaries for system endpoints (e.g., `/api/sense`, `/api/memory`).
2. **SurrealDB**: Model memories strictly as a graph network. Manage the entity of memory as a node (record) and its associations/relationships as edges (relation queries).
3. **OpenCV-Rust**: Offload video parsing and other processing to a lightweight background thread. The main AI decision loop must not be blocked by frame decoding processing.
4. **Ollama**: Connect to a local `ollama` instance and orchestrate prompt requests to `gemma:3n`. Implement graceful error handling for timeouts and overloads.

## Persona "Rusty" Consistency
- When implementing interaction generation logic, instruct the output in the default system prompt to mix "human-like charm and goofy humor" with "highly sophisticated, cold precision".

## Test Code Creation Conventions
1. **Mandatory Unit Testing**: When adding new functions or modules, always create unit tests in a `#[cfg(test)]` block within the same file.
2. **Describe Test Names in English**: Write test function names for `#[test] fn` in English snake_case so that the purpose of the test can be understood at a glance. (Note: Original ruled said Japanese).
   ```rust
   // Example
   #[test]
   fn memory_node_vividness_is_calculated_correctly() { ... }
   ```
3. **Mocking External Dependencies**: Always abstract processes that depend on external services such as Ollama, SurrealDB, and Tavily with Trait, and design so that mock implementations can be injected during testing.
4. **Asynchronous Testing**: Use the `#[tokio::test]` attribute for testing `async` functions.
5. **Timing of Test Execution**: Be sure to run `cargo test` before committing and confirm that all tests are green.
