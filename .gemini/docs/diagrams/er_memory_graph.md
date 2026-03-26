# ER Diagram: Memory Graph Schema

## Overview
Shows the schema of memory nodes and their relationships (edges) managed on SurrealDB.
Based on the hybrid storage design in README 2.4, **only metadata** is stored in SurrealDB.

## Diagram

```mermaid
erDiagram
    MemoryNode {
        string id PK "memory:concept_name"
        string concept "Concept/Keyword of memory"
        float vividness "Vividness V = e^(-t/S)  (0.0 ~ 1.0)"
        int last_accessed "Last accessed UNIX timestamp"
        string file_path "Path to episode file (CMS storage)"
        float emotion_score "Emotion score (Initial value of memory strength S)"
        string tags "Topic tags (comma-separated)"
    }

    RelatesTo {
        float weight "Relevance weight (0.0 ~ 1.0)"
    }

    PersonalityNode {
        string id PK "personality:trait_name"
        string trait "Personality trait (e.g., humor / sarcasm / rust_love)"
        float initial_weight "Initial edge weight (higher is easier to associate)"
    }

    MemoryNode ||--o{ RelatesTo : "Source (spread_activation)"
    RelatesTo }o--|| MemoryNode : "Destination"
    PersonalityNode ||--o{ RelatesTo : "Bias Source"
    RelatesTo }o--|| MemoryNode : "Bias Destination"
```

## Notes
- `vividness` decays over time based on Ebbinghaus's forgetting curve $V = e^{-t/S}$.
- By `spread_activation`, vividness is automatically added to adjacent nodes of the accessed node.
- By setting high edge weights for `PersonalityNode`, a "Rusty" personality bias is applied to the direction of association.
- `file_path` stores a reference to the actual episode text (CMS structure: `/var/iris/memories/YYYY/MM/DD/`).

## Correspondence with Rust Implementation
| ER Element | Rust Implementation |
|---|---|
| `MemoryNode` | `MemoryNode` struct in `src/memory/graph.rs` |
| `RelatesTo` | `RelatesTo` struct in `src/memory/graph.rs` |
| `spread_activation` | `spread_activation()` function in `src/memory/graph.rs` |

## Update History
| Date | Changes |
|---|---|
| 2026-03-24 | Initial version created (Basic schema) |
| 2026-03-24 | Added CMS architecture support, PersonalityNode, and correspondence table with Rust implementation |
