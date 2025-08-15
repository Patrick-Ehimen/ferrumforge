# FerrumForge 🔗

A comprehensive blockchain simulator with proof-of-work mining, built in Rust for educational purposes.

## Features

- **Basic Blockchain**: Create blocks, manage chains, validate integrity
- **Proof-of-Work Mining**: Mine blocks with adjustable difficulty
- **Dynamic Difficulty**: Automatic difficulty adjustment based on block timing
- **Serialization**: Save and load blockchain state (JSON/Binary)
- **CLI Interface**: Easy-to-use command-line interface
- **Performance Monitoring**: Mining statistics and hash rate tracking

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Build from Source

```bash
git clone <your-repo-url>
cd ferrumforge
cargo build --release
```

## Usage

### Basic Commands

```bash
# Create a new block with data
ferrumforge create-block "Hello, Blockchain!"

# Mine a block with proof-of-work
ferrumforge mine-block "Mined block data"

# Validate the entire blockchain
ferrumforge validate-chain

# Show the current blockchain
ferrumforge show-chain

# Display blockchain statistics
ferrumforge chain-stats

# Set mining difficulty (number of leading zeros)
ferrumforge set-difficulty 4

# Export blockchain to file
ferrumforge export-chain blockchain.json

# Import blockchain from file
ferrumforge import-chain blockchain.json
```

### Examples

```bash
# Start with a simple block
ferrumforge create-block "Genesis transaction"

# Mine some blocks with increasing difficulty
ferrumforge set-difficulty 2
ferrumforge mine-block "First mined block"

ferrumforge set-difficulty 3
ferrumforge mine-block "Second mined block"

# Check the results
ferrumforge show-chain
ferrumforge chain-stats
```

## Project Structure

```
src/
├── lib.rs              # Library exports
├── main.rs             # CLI entry point
├── error.rs            # Error handling
├── blockchain/         # Core blockchain functionality
│   ├── block.rs        # Block structure and operations
│   └── chain.rs        # Blockchain management
├── cli/                # Command-line interface
│   ├── commands.rs     # Command execution
│   └── parser.rs       # Argument parsing
├── crypto/             # Cryptographic operations
│   └── hashing.rs      # SHA-256 hashing
├── mining/             # Mining and proof-of-work
│   ├── engine.rs       # Mining engine
│   └── difficulty.rs   # Difficulty adjustment
├── storage/            # Serialization and persistence
│   ├── serialization.rs # JSON/Binary serialization
│   └── persistence.rs  # File I/O operations
└── utils/              # Utility functions
    ├── formatting.rs   # Output formatting
    ├── validation.rs   # Validation helpers
    └── performance.rs  # Performance monitoring
```

## Learning Objectives

This project teaches:

- **Rust Programming**: Structs, enums, error handling, modules
- **Blockchain Concepts**: Blocks, hashing, chain validation
- **Proof-of-Work**: Mining algorithms, difficulty adjustment
- **Cryptography**: SHA-256 hashing, hash validation
- **Serialization**: JSON and binary data formats
- **CLI Development**: Command parsing, user interaction
- **Performance**: Hash rate calculation, optimization

## Development

### Running Tests

```bash
cargo test
```

### Running Benchmarks

```bash
cargo bench
```

### Development Mode

```bash
cargo run -- <command> <args>
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run `cargo test` and `cargo clippy`
6. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Educational Use

FerrumForge is designed for educational purposes to demonstrate blockchain concepts and Rust programming. It is not intended for production use or real cryptocurrency applications.

## Acknowledgments

- Built with Rust and the amazing Rust ecosystem
- Inspired by Bitcoin's proof-of-work consensus mechanism
- Educational blockchain concepts from various online resources
