use std::sync::{Arc, Mutex};

pub struct CmdDurationService {
    command: String,
    args: Vec<String>,
    output: Arc<Mutex<String>>,
}

impl CmdDurationService {
    /// Creates a new `CmdDurationService` instance
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self {
            command,
            args,
            output: Arc::new(Mutex::new(String::new())),
        }
    }
}
