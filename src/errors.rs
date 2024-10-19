use std::io;
use thiserror::Error;

/// Custom error type for the WaitService.
#[derive(Debug, Error)]
pub enum CmdDurationServiceError {
    #[error("Failed to execute the command")]
    CommandExecutionError(#[from] io::Error),

    #[error("Failed to capture stdout")]
    StdoutCaptureError,
}

pub type Result<T> = std::result::Result<T, CmdDurationServiceError>;
