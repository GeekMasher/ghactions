//! Errors for the core library
use thiserror::Error;

/// Actions Error
#[derive(Error, Debug, PartialEq)]
pub enum ActionsError {
    /// Failed to load the environment
    #[error("Failed to load environment: `{0}`")]
    FailedLoading(String),

    /// Failed to get input value from environment
    #[error("Failed to get input value: `{0}`")]
    InputError(String),

    /// Input Type Error
    #[error("Input Type Error: `{0}` (Expected: `{1}`)")]
    InputTypeError(String, String),

    /// Octocrab Error
    #[cfg(feature = "octocrab")]
    #[error("Octocrab Error: `{0}`")]
    OctocrabError(String),

    /// Failed parsing the repository reference
    #[error("Unable to parse repo reference: `{0}`")]
    RepositoryReferenceError(String),

    /// Not Implemented
    #[error("Not Implemented")]
    NotImplemented,
}
