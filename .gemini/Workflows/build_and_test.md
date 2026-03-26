---
description: Build and test execution workflow for the I.R.I.S. project.
---
# I.R.I.S. Build and Test Workflow

This workflow is a procedure for automatically executing the build of the Rust backend, static analysis of the code (Clippy), and testing.

## Step 1: Code Formatting and Static Analysis
Execute this to unify the code style and detect potential bugs.
// turbo
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

## Step 2: Project Build
Compile the entire project.
// turbo
```bash
cargo build
```

## Step 3: Execution of Unit Tests and Integration Tests
Run all tests and confirm that features are working normally.
// turbo
```bash
cargo test
```

## Step 4: (Optional) Release Build
Execute this when performance optimization is necessary or during deployment.
```bash
cargo build --release
```
