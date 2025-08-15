//! Error handling for FerrumForge blockchain simulator
//!
//! This module defines comprehensive error types for blockchain operations,
//! including block validation, chain management, mining, and serialization.

use std::fmt;

/// Comprehensive error type for blockchain operations
#[derive(Debug, Clone, PartialEq)]
pub enum BlockchainError {
    // Block-related errors
    InvalidBlock(String),
    InvalidHash(String),
    InvalidNonce(String),

    // Chain-related errors
    InvalidChain(String),
    ChainValidationError(String),
    GenesisBlockError(String),

    // Mining-related errors
    MiningError(String),
    DifficultyAdjustmentError(String),

    // Serialization errors
    SerializationError(String),
    DeserializationError(String),

    // I/O errors
    FileError(String),

    // Crypto errors
    CryptoError(String),
    InvalidInput(String),
}

impl fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockchainError::InvalidBlock(msg) => write!(f, "Invalid block: {}", msg),
            BlockchainError::InvalidHash(msg) => write!(f, "Invalid hash: {}", msg),
            BlockchainError::InvalidNonce(msg) => write!(f, "Invalid nonce: {}", msg),
            BlockchainError::InvalidChain(msg) => write!(f, "Invalid chain: {}", msg),
            BlockchainError::ChainValidationError(msg) => {
                write!(f, "Chain validation error: {}", msg)
            }
            BlockchainError::GenesisBlockError(msg) => write!(f, "Genesis block error: {}", msg),
            BlockchainError::MiningError(msg) => write!(f, "Mining error: {}", msg),
            BlockchainError::DifficultyAdjustmentError(msg) => {
                write!(f, "Difficulty adjustment error: {}", msg)
            }
            BlockchainError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            BlockchainError::DeserializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
            BlockchainError::FileError(msg) => write!(f, "File error: {}", msg),
            BlockchainError::CryptoError(msg) => write!(f, "Cryptographic error: {}", msg),
            BlockchainError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for BlockchainError {}

// Conversion implementations for common error types
impl From<std::io::Error> for BlockchainError {
    fn from(error: std::io::Error) -> Self {
        BlockchainError::FileError(error.to_string())
    }
}

impl From<serde_json::Error> for BlockchainError {
    fn from(error: serde_json::Error) -> Self {
        BlockchainError::SerializationError(error.to_string())
    }
}

impl From<bincode::Error> for BlockchainError {
    fn from(error: bincode::Error) -> Self {
        BlockchainError::SerializationError(error.to_string())
    }
}

/// Result type alias for blockchain operations
pub type BlockchainResult<T> = Result<T, BlockchainError>;
