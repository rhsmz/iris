# System Architecture: Project I.R.I.S.

## Overview
Project I.R.I.S. is a self-hosted, autonomous associated memory AI partner built on a Raspberry Pi 5. It integrates a Rust backend with the Gemma 3n LLM.

## Layers

### 1. Sense (Input Layer)
- **Vision Subsystem**: OpenCV-Rust integration for facial recognition. Processes local video feeds to detect the user's presence without cloud dependencies.
- **Audio/Text Subsystem**: Axum-based web server for handling incoming dialogue and system commands.

### 2. Logic (Reasoning Layer)
- **Reasoning Engine**: Gemma 3n (4B/9B) executed locally via Ollama. Handles multimodal inputs.
- **Orchestrator**: Tokio runtime managing asynchronous tasks, determining whether to trigger memory storage, retrieval, or action generation.

### 3. Memory (Storage Layer)
- **Graph Engine**: SurrealDB is used to store conversational contexts, user facts, and associations as a connected graph graph.
- **Vividness Model**: Episodic memories possess a fading mechanism (vividness: $V = e^{-t / S}$).
- **Associative Retrieval**: Accessing a memory node propagates activation to connected nodes, simulating "spreading activation."

### 4. Action (Output Layer)
- **Response Synthesis**: Generating dialogue that reflects the 'Rusty' persona (refined yet possessing human-like quirks).
- **Autonomous Research**: Utilizing the Tavily API to actively seek information when the local knowledge base is insufficient.
