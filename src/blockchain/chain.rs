//! Blockchain management and operations
//!
//! This module handles the blockchain structure and high-level operations
//! including chain creation, block addition, and chain validation.

use crate::blockchain::Block;
use crate::error::{BlockchainError, BlockchainResult};
use serde::{Deserialize, Serialize};

/// Main blockchain structure containing the chain of blocks and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
    pub mining_reward: u64,
    pub target_block_time: u64, // in seconds
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 2, // Start with low difficulty
            mining_reward: 100,
            target_block_time: 10, // 10 seconds target
        };

        let genesis_block = Self::create_genesis_block();
        blockchain.chain.push(genesis_block);
        blockchain
    }

    /// Creates the genesis block (first block in the chain)
    pub fn create_genesis_block() -> Block {
        Block::new(0, "Genesis Block".to_string(), "0".to_string(), 2)
    }

    /// Returns a reference to the latest block in the chain
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Chain should never be empty")
    }

    /// Adds a new block to the chain with the given data
    pub fn add_block(&mut self, data: String) -> BlockchainResult<()> {
        // Implementation will be added in later tasks
        Ok(())
    }

    /// Validates the entire blockchain
    pub fn is_chain_valid(&self) -> bool {
        // Implementation will be added in later tasks
        true
    }

    /// Adjusts the mining difficulty based on recent block times
    pub fn adjust_difficulty(&mut self) {
        // Implementation will be added in later tasks
    }

    /// Gets the average mining time for the last N blocks
    pub fn get_mining_time(&self, blocks_to_check: usize) -> u64 {
        // Implementation will be added in later tasks
        self.target_block_time
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}
