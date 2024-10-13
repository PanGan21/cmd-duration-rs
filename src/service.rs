use std::{
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    time::Instant,
};

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

    /// Starts the command in a separate thread and tracks its duration
    pub fn run(&mut self) {
        let start_time = Instant::now();
        let output_clone = Arc::clone(&self.output);

        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to execute command");
    }
}
