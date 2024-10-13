use thiserror::Error;

/// Custom error type for the WaitService.
#[derive(Debug, Error)]
pub enum CmdDurationServiceError {}

pub type Result<T> = std::result::Result<T, CmdDurationServiceError>;
