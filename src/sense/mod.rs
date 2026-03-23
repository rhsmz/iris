pub mod server;
pub mod vision;

pub async fn init_sense_layer() {
    println!("Initialize Sense Layer");
    
    // Spawn server and vision subsystems concurrently
    tokio::spawn(server::start_server());
    tokio::spawn(vision::start_vision_loop());
}
