#![allow(dead_code)]
use std::time::Duration;
use tokio::time::sleep;

pub trait VisionEngine {
    fn detect_user_presence(&self) -> bool;
}

pub struct MockVisionEngine {
    pub user_present: bool,
}

impl VisionEngine for MockVisionEngine {
    fn detect_user_presence(&self) -> bool {
        self.user_present
    }
}

pub async fn start_vision_loop() {
    println!("Starting Vision Subsystem...");
    let engine = MockVisionEngine {
        user_present: false,
    };

    loop {
        // Sleep to simulate frame processing time
        sleep(Duration::from_secs(2)).await;

        let presence = engine.detect_user_presence();
        // In the future this will send a message or event indicating user presence
        if presence {
            println!("[Vision] I see you.");
        } else {
            // Un-comment to trace:
            // println!("[Vision] Scanning...");
        }
    }
}
