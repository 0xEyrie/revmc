//! EVM runtime JIT worker implementation.
//!
//! This crate provides the implementation for a JIT (Just-In-Time) compiler worker
//! that operates in a node runtime environment. It includes modules for handling
//! errors, external contexts, and worker functionalities.

#![allow(missing_docs)]
pub mod error;
mod external;
mod handler;
#[cfg(test)]
mod tests;
mod worker;

pub use external::*;
pub use handler::*;
pub use worker::*;
