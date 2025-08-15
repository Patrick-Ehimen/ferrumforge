//! FerrumForge - A comprehensive blockchain simulator in Rust
//!
//! This library provides educational blockchain functionality including:
//! - Basic blockchain structure and operations
//! - Proof-of-work mining and consensus
//! - Chain validation and integrity checking
//! - Difficulty adjustment algorithms
//! - Serialization and persistence

pub mod blockchain;
pub mod error;
pub mod mining;
pub mod utils;

// Re-export main types for convenience
pub use blockchain::{Block, Blockchain};
pub use error::{BlockchainError, BlockchainResult};
pub use mining::MiningEngine;
pub use utils::ValidationEngine;
