//! Error handling utilities for Sparrow's network interface.

/// Generic type used to type Result across Sparrow's network interface.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
