use std::collections::HashMap;

// A simple platform-agnostic key event
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyboardEvent {
    pub device_id: String,
    pub key_code: u16,
}

// State manager tracking connected keyboards or history
#[derive(Default)]
pub struct KeyboardTracker {
    // Maps Device ID -> List of keycodes pressed
    pub history: HashMap<String, Vec<u16>>,
}

impl KeyboardTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_event(&mut self, event: KeyboardEvent) {
        self.history
            .entry(event.device_id)
            .or_default()
            .push(event.key_code);
    }
}
