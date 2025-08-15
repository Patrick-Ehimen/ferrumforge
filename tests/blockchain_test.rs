//! Tests for blockchain management functionality
//!
//! This module contains unit tests for the basic blockchain management
//! including blockchain initialization and genesis block creation.

use ferrumforge::blockchain::Blockchain;

#[test]
fn test_blockchain_initialization() {
    let blockchain = Blockchain::new();

    // Verify blockchain has correct initial configuration
    assert_eq!(blockchain.difficulty, 2);
    assert_eq!(blockchain.mining_reward, 100);
    assert_eq!(blockchain.target_block_time, 10);

    // Verify blockchain starts with exactly one block (genesis)
    assert_eq!(blockchain.chain.len(), 1);
}

#[test]
fn test_genesis_block_creation() {
    let genesis = Blockchain::create_genesis_block();

    // Verify genesis block properties
    assert_eq!(genesis.index, 0);
    assert_eq!(genesis.data, "Genesis Block");
    assert_eq!(genesis.previous_hash, "0");
    assert_eq!(genesis.difficulty, 2);
    assert_eq!(genesis.nonce, 0);

    // Verify genesis block has valid hash
    assert!(!genesis.hash.is_empty());
    assert!(genesis.is_valid());
}

#[test]
fn test_get_latest_block() {
    let blockchain = Blockchain::new();
    let latest = blockchain.get_latest_block();

    // Latest block should be the genesis block
    assert_eq!(latest.index, 0);
    assert_eq!(latest.data, "Genesis Block");
    assert_eq!(latest.previous_hash, "0");
}

#[test]
fn test_blockchain_default() {
    let blockchain = Blockchain::default();

    // Default should be equivalent to new()
    assert_eq!(blockchain.chain.len(), 1);
    assert_eq!(blockchain.difficulty, 2);
}
