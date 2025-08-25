use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub filename: String,
    pub hash: String,
    pub timestamp: u64,
    pub block_index: Option<usize>,
}

impl Document {
    pub fn new(filename: String, content: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash = format!("{:x}", hasher.finalize());
        
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            filename,
            hash,
            timestamp,
            block_index: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResult {
    pub exists: bool,
    pub block_index: Option<usize>,
    pub timestamp: Option<u64>,
    pub merkle_proof: Option<Vec<String>>,
}