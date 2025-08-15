//! Block structure and operations
//!
//! This module defines the core Block struct and its associated operations
//! including creation, hashing, and validation.

use crate::error::{BlockchainError, BlockchainResult};
use serde::{Deserialize, Serialize};

/// Core block structure containing all necessary fields for blockchain operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: u32,
}

impl Block {
    /// Creates a new block with the given parameters
    /// The hash will be calculated and the nonce will be set to 0
    pub fn new(index: u64, data: String, previous_hash: String, difficulty: u32) -> Self {
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            difficulty,
        };
        block.hash = block.calculate_hash();
        block
    }

    /// Calculates the hash for this block using SHA-256
    pub fn calculate_hash(&self) -> String {
        use std::fmt::Write;

        // Pre-allocate string with estimated capacity to reduce allocations
        let mut block_data = String::with_capacity(128);

        // Use write! macro for more efficient string concatenation
        // This concatenates all block fields into a single string for hashing
        // Fields included: index, timestamp, data, previous_hash, nonce, difficulty
        write!(
            &mut block_data,
            "{}{}{}{}{}{}",
            self.index,         // Block position in chain
            self.timestamp,     // When block was created
            self.data,          // Block payload/content
            self.previous_hash, // Hash of previous block (chain linkage)
            self.nonce,         // Proof-of-work value for mining
            self.difficulty     // Mining difficulty level
        )
        .expect("Writing to string should never fail");

        // Generate SHA-256 hash of the concatenated block data
        crate::crypto::hashing::hash_data(&block_data)
    }

    /// Validates this block's internal consistency
    pub fn is_valid(&self) -> bool {
        // Check if the stored hash matches the calculated hash
        self.hash == self.calculate_hash()
    }

    /// Mines this block by finding a valid nonce
    pub fn mine_block(&mut self) -> BlockchainResult<()> {
        // Implementation will be added in later tasks
        Ok(())
    }
}
