use rdev::{listen, EventType, Key};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct MouseListener {
    is_listening: Arc<AtomicBool>,
}

impl MouseListener {
    pub fn new() -> Self {
        MouseListener {
            is_listening: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start_listening(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_listening.store(true, Ordering::SeqCst);

        let is_listening = Arc::clone(&self.is_listening);

        std::thread::spawn(move || {
            if let Err(error) = listen(move |event| {
                if !is_listening.load(Ordering::SeqCst) {
                    return;
                }

                match event.event_type {
                    EventType::MouseMove { x, y } => {
                        // Handle mouse move
                        // println!("Mouse moved: ({}, {})", x, y);
                    }
                    EventType::ButtonPress(button) => {
                        // Handle button press
                        // println!("Button pressed: {:?}", button);
                    }
                    EventType::ButtonRelease(button) => {
                        // Handle button release
                        // println!("Button released: {:?}", button);
                    }
                    _ => {}
                }
            }) {
                eprintln!("Error: {:?}", error);
            }
        });

        Ok(())
    }

    pub fn stop_listening(&mut self) {
        self.is_listening.store(false, Ordering::SeqCst);
    }
}

impl Default for MouseListener {
    fn default() -> Self {
        Self::new()
    }
}
