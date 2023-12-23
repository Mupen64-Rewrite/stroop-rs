use thiserror::Error;

#[derive(Debug, Error)]
pub enum StaticMemoryEmulatorError {
    #[error("No matching process found")]
    NoProcessFound,
    #[error("Failed to get process handle")]
    ProcessHandleError(#[from] std::io::Error),
}
