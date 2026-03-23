use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use serde::{Deserialize, Serialize};

static DB: surrealdb::once_cell::sync::Lazy<Surreal<surrealdb::engine::local::Db>> = surrealdb::once_cell::sync::Lazy::new(Surreal::init);

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: String,
    pub concept: String,
    pub vividness: f32,
    pub last_accessed: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatesTo {
    pub weight: f32,
}

pub async fn connect_to_db() -> surrealdb::Result<()> {
    // For development, we'll use an in-memory database.
    // In production, this would be a file or TiKV backend.
    DB.connect::<Mem>(()).await?;
    DB.use_ns("iris").use_db("memory_graph").await?;
    
    // Create initial schema definitions if needed
    // Typically SurrealDB is schemaless by default, but we can enforce strict schema
    Ok(())
}

pub async fn insert_memory(concept: &str, vividness: f32) -> surrealdb::Result<Option<MemoryNode>> {
    let node = MemoryNode {
        id: format!("memory:{}", concept),
        concept: concept.to_string(),
        vividness,
        last_accessed: chrono::Utc::now().timestamp(),
    };
    
    let created: Option<MemoryNode> = DB
        .create(("memory", concept))
        .content(node)
        .await?;
        
    Ok(created)
}

pub async fn spread_activation(concept: &str) -> surrealdb::Result<()> {
    // A query to find related memories and boost their vividness
    // E.g., UPDATE memory SET vividness = vividness + 0.1 WHERE id <-> $concept
    let _ = DB.query("UPDATE memory SET vividness = vividness + 0.1 WHERE <-relates_to->(memory WHERE id = $id)")
        .bind(("id", format!("memory:{}", concept)))
        .await?;
        
    Ok(())
}
