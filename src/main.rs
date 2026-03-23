mod action;
mod memory;
mod sense;
mod logic;

#[tokio::main]
async fn main() {
    println!("Starting Project I.R.I.S. orchestrator...");

    // Initialize Layers
    memory::init_memory_layer().await;
    action::init_action_layer().await;
    logic::init_logic_layer().await;
    sense::init_sense_layer().await;

    // Keep the main thread alive
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for event");
    
    println!("I.R.I.S. shutting down. Good night.");
}
