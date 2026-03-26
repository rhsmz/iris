# Sequence Diagram: Chat Endpoint Flow

## Overview
Shows the processing flow of Sense -> Memory -> Logic -> Action at the `POST /api/chat` endpoint.
Reflects the hybrid storage (CMS) design and Spreading Activation from README 2.4.

## Diagram

```mermaid
sequenceDiagram
    participant User as User
    participant Axum as Sense Layer (Axum)
    participant Logic as Logic Layer (Reasoning)
    participant Memory as Memory Layer (SurrealDB)
    participant FS as Episode CMS (FileSystem)
    participant Ollama as Local LLM (Gemma 3n)
    participant Action as Action Layer (Tavily)

    User->>Axum: POST /api/chat { message }
    Axum->>Logic: ask_with_context(message)

    Note over Logic,Memory: Spreading Activation
    Logic->>Memory: spread_activation(message)
    Memory-->>Logic: Completed updating Vividness of personality and related nodes

    Note over Logic,FS: RAG Context Extraction
    Logic->>Memory: fetch_top_memories(limit)
    Memory->>FS: Read episode based on path
    FS-->>Memory: Markdown episode entity
    Memory-->>Logic: RetrievedMemory group (metadata + body)

    Note over Logic,Ollama: Prompt Synthesis and Reasoning
    Logic->>Logic: Synthesis of Rusty persona + memory context + recent conversation history
    Logic->>Ollama: Send prompt (stream: true)
    Ollama-->>Logic: Streaming JSON chunk response
    Logic->>Logic: Execute callback for chunk reception

    alt If there is unknown information
        Logic->>Action: Search with Tavily API (asynchronous)
        Action-->>Logic: Search results
        Logic->>Ollama: Re-prompt (with search results)
        Ollama-->>Logic: Final response
    end

    Note over Logic,Memory: Saving New Memory and Association Bias
    Logic->>FS: Write response content as Markdown
    FS-->>Logic: file_path
    Logic->>Memory: insert_memory (Save node + build Personality edge)
    Memory-->>Logic: Completion of saving

    Logic-->>Axum: Completion of reasoning
    Axum-->>User: ChatResponse { reply }
```

## Correspondence with Implementation Files
| Participant in Diagram | Source File |
|---|---|
| Sense Layer (Axum) | `src/sense/server.rs` |
| Memory Layer (SurrealDB) | `src/memory/graph.rs` |
| Logic Layer (Ollama) | `src/logic/reasoning.rs` |
| Action Layer (Tavily) | `src/action/research.rs` |

## Update History
| Date | Changes |
|---|---|
| 2026-03-24 | Initial version created |
| 2026-03-24 | Added CMS architecture (FileSystem), Spreading Activation, and implementation correspondence table |
