pub mod graph;

pub async fn init_memory_layer() {
    println!("Initialize Memory Layer");
    
    // Connect to SurrealDB and initialize the schema
    match graph::connect_to_db().await {
        Ok(_) => println!("Successfully connected to SurrealDB Memory Graph."),
        Err(e) => eprintln!("Failed to connect to SurrealDB: {}", e),
    }
}
