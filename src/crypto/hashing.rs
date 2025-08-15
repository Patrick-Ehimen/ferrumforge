use sha2::{Digest, Sha256};

/// Hashes the given data using SHA-256 and returns the hex string
pub fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
