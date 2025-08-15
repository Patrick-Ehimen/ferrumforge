/// Mining module containing difficulty adjustment and mining engine implementations
pub mod difficulty;
/// Core mining engine implementation
pub mod engine;

/// Re-export of the main mining engine for convenient access
pub use engine::MiningEngine;
