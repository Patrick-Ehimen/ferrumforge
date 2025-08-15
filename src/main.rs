use ferrumforge::{Blockchain, BlockchainResult};

fn main() -> BlockchainResult<()> {
    println!("FerrumForge - Blockchain Simulator");

    // Create a new blockchain
    let blockchain = Blockchain::new();
    println!("Created blockchain with {} blocks", blockchain.chain.len());
    println!("Genesis block: {:?}", blockchain.get_latest_block());

    Ok(())
}
