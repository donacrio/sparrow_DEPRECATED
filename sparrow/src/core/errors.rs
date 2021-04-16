//! Error handling utilities for Sparrow's core.

/// Generic type used to type Result across Sparrow's core codebase.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
