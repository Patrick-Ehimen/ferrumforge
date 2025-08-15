use ferrumforge::*;

#[cfg(test)]
mod tests {
    use ferrumforge::*;

    // Test block creation and field initialization
    #[test]
    fn test_block_creation() {
        let index = 1;
        let data = "Test block data".to_string();
        let previous_hash = "prevhash".to_string();
        let difficulty = 2;
        // Create a new block
        let block = Block::new(index, data.clone(), previous_hash.clone(), difficulty);
        println!("Created block: {:?}", block);
        // Check if block fields are set correctly
        assert_eq!(block.index, index);
        assert_eq!(block.data, data);
        assert_eq!(block.previous_hash, previous_hash);
        assert_eq!(block.difficulty, difficulty);
        // Ensure timestamp is set
        println!("Block timestamp: {}", block.timestamp);
        assert!(block.timestamp > 0);
        // Nonce should be initialized to 0
        println!("Block nonce: {}", block.nonce);
        assert_eq!(block.nonce, 0);
        // Hash should not be empty
        println!("Block hash: {}", block.hash);
        assert!(!block.hash.is_empty());
    }

    // Test block hash calculation logic
    #[test]
    fn test_block_hash_calculation() {
        // Manually construct a block
        let block = Block {
            index: 1,
            timestamp: 1234567890,
            data: "Test block data".to_string(),
            previous_hash: "prevhash".to_string(),
            hash: String::new(),
            nonce: 0,
            difficulty: 2,
        };
        println!("Testing hash calculation for block: {:?}", block);
        // Calculate expected hash using the same logic as Block::calculate_hash
        let expected = crate::crypto::hashing::hash_data(&format!(
            "{}{}{}{}{}{}",
            block.index,
            block.timestamp,
            block.data,
            block.previous_hash,
            block.nonce,
            block.difficulty
        ));
        println!("Expected hash: {}", expected);
        let calculated = block.calculate_hash();
        println!("Calculated hash: {}", calculated);
        // Assert that calculate_hash returns the expected value
        assert_eq!(calculated, expected);
    }
}
