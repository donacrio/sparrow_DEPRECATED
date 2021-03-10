//! Error handling utilities.

/// Generic type used to type Result across Sparrow codebase.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
