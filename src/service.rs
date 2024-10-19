use crossterm::{cursor::MoveTo, execute, terminal};
use std::{
    io::{self, Read, Write},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use crate::utils::{get_hours, get_minutes, get_seconds};

pub struct CmdDurationService {
    command: String,
    args: Vec<String>,
}

impl CmdDurationService {
    /// Creates a new `CmdDurationService` instance
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }

    /// Starts the command in a separate thread and tracks its duration
    pub fn run(&mut self) {
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let output = Arc::new(Mutex::new(String::new()));
        let output_clone = Arc::clone(&output);
        let running = Arc::new(AtomicBool::new(true)); // Flag to indicate if the process is still running
        let running_clone = running.clone();

        let start_time = Instant::now();
        let mut child = cmd.spawn().expect("Failed to execute command");
        let mut child_output = child.stdout.take().expect("Failed to capture stdout");

        thread::spawn(move || {
            let mut buffer = [0; 1024];
            while let Ok(bytes_read) = child_output.read(&mut buffer) {
                if bytes_read == 0 {
                    break;
                }
                let mut output_locked = output_clone.lock().unwrap();
                output_locked.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
            }
            running_clone.store(false, Ordering::SeqCst);
        });
        // Start the duration printing
        Self::log_duration_and_output(output, running, start_time);

        // Wait for the command to finish
        let _ = child.wait().expect("Failed to wait on child process");

        // Ensure everything is printed before exiting
        thread::sleep(Duration::from_secs(1));
        print!("\n");
    }

    /// Continuously logs the duration and outputs any command results
    fn log_duration_and_output(
        output: Arc<Mutex<String>>,
        running: Arc<AtomicBool>,
        start: Instant,
    ) {
        let tick_duration = Duration::from_millis(250);
        let mut output_accumulator = String::new();

        while running.load(Ordering::SeqCst) {
            thread::sleep(tick_duration);

            let current_output = output.lock().unwrap().clone();
            if current_output != output_accumulator {
                let output_diff = current_output
                    .strip_prefix(&output_accumulator)
                    .unwrap_or("");
                output_accumulator = current_output.clone();

                Self::clear_time();
                print!("\r{}", output_diff);
                io::stdout().flush().unwrap();
            }

            Self::log_duration(start);
        }
    }

    fn clear_time() {
        print!("\x1b[2K\r"); // Clear the last line
        io::stdout().flush().unwrap();
    }

    /// Logs the current duration the command has been running for
    fn log_duration(start: Instant) {
        let current_time = start.elapsed();

        let duration_message = format!(
            "command running since: {}:{}:{}",
            get_hours(current_time),
            get_minutes(current_time),
            get_seconds(current_time)
        );

        let (cols, lines) = terminal::size().unwrap();

        // Calculate the position to move the cursor to
        let message_length = duration_message.len() as u16;
        let position = cols.saturating_sub(message_length).saturating_sub(1); // Adjust to leave a space before the message

        // Move the cursor to the calculated position
        execute!(io::stdout(), MoveTo(position, lines - 1)).unwrap();

        print!("{}", duration_message);
        io::stdout().flush().unwrap();
    }
}
