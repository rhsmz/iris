# System Architecture: Project I.R.I.S.

## Overview
Project I.R.I.S. is an autonomous, associative memory AI partner built on Raspberry Pi 5. It operates in a local environment that protects privacy without depending on the cloud. It integrates a Rust backend with a large language model (Gemma 3n).

## Layer Configuration

### 1. Sense (Input Layer)
- **Vision (Visual Subsystem)**: Works with OpenCV-Rust to perform face recognition based on images from the camera. Feature extraction is completed locally, and no images are sent to the cloud. The moment the master's face is detected, weight is added to the related nodes in the memory graph.
- **Audio/Text (Audio/Text Subsystem)**: Receives interactions and system commands through an Axum-based web server (`/health`, `/api/chat`).

### 2. Logic (Reasoning Layer)
- **Reasoning (Reasoning Engine)**: Performs reasoning using Gemma 3n running locally via Ollama. In addition to the RAG pattern that dynamically injects necessary episodes into the prompt, it maintains context by keeping recent conversation history in memory. Furthermore, reasoning results are received asynchronously via streaming to improve responsiveness.
- **Orchestrator (Orchestrator)**: Manages the Sense -> Memory -> Logic -> Action pipeline through asynchronous processing using the Tokio runtime.

### 3. Memory (Memory Layer)

#### Graph Memory (SurrealDB)
Saves **only metadata** and manages relationships (edges) between memories:
- `MemoryNode` struct: Concept, tags, Vividness, emotion score, absolute file path of CMS
- `RelatesTo` struct: Edges with relevance (weight) between nodes

#### Episode Memory CMS (File System)
Specific episode text and visual information are saved as Markdown files and dynamically combined as `RetrievedMemory` during search:
```
/var/iris/memories/YYYY/MM/DD/episode_xxxx.md
```

#### Mathematical Model of Memory
Vividness model based on Ebbinghaus's forgetting curve:

$$V = e^{-\frac{t}{S}}$$

- $V$: Retention rate of memory (Vividness)
- $t$: Elapsed time since the last recall
- $S$: Strength of memory (initial value based on emotion score or importance)

#### Spreading Activation (Associative Recall) [Core Mechanism]
When a word appears in a conversation, an activation signal propagates to adjacent nodes on the graph DB, refreshing surrounding memories simultaneously. This enables human-like association such as "Speaking of which, about that...".

#### Personality Core Nodes
`PersonalityNode` with "humor", "sarcasm", and "rust_love" are placed on the graph. Every time a new memory is input, a weak edge is automatically built, applying a consistent "Rusty" bias to the direction of association.

### 4. Action (Output Layer)
- **Response Synthesis**: Generates charming responses based on the "Rusty" persona.
- **Autonomous Research**: Performs autonomous search using the Tavily API if there is insufficient information.

## Container Configuration (Docker)

```
Project Root
├── docker-compose.yml         # Common base (Network, Volume, Environment variables, Ollama, SurrealDB basic settings)
├── docker-compose.dev.yml     # For Windows development (Source code, Cache mount, No camera, Auto build)
└── docker-compose.release.yml # For Raspberry Pi production (Camera mount, Lightweight runtime build)
```

## Technology Stack

| Category | Technology |
|---|---|
| Language | Rust 1.75+ (Stable) |
| Asynchronous Runtime | Tokio |
| Web Server | Axum |
| Database | SurrealDB |
| LLM Runtime | Ollama (Gemma 3n) |
| Visual Processing | OpenCV-Rust |
| Autonomous Research | Tavily API |
| Container | Docker / Docker Compose |
| Target Hardware | Raspberry Pi 5 (16GB) |
