//! Blockchain core functionality module
//!
//! This module contains the core blockchain implementation including:
//! - Block structure and operations
//! - Blockchain management
//! - Mining and proof-of-work
//! - Chain validation
//! - Difficulty adjustment

pub mod block;
pub mod chain;

// Re-export main types for convenience
pub use block::Block;
pub use chain::Blockchain;
