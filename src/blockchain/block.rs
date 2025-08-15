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
        // Concatenate all block fields except hash
        let block_data = format!(
            "{}{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce, self.difficulty
        );
        crate::crypto::hashing::hash_data(&block_data)
    }

    /// Validates this block's internal consistency
    pub fn is_valid(&self) -> bool {
        // Implementation will be added in later tasks
        true
    }

    /// Mines this block by finding a valid nonce
    pub fn mine_block(&mut self) -> BlockchainResult<()> {
        // Implementation will be added in later tasks
        Ok(())
    }
}
