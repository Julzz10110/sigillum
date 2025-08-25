use crate::merkle_tree::MerkleTree;
use crate::models::Document;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: usize,
    pub timestamp: i64,
    pub previous_hash: String,
    pub merkle_root: String,
    pub documents: Vec<Document>,
    pub hash: String,
}

impl Block {
    pub fn new(index: usize, previous_hash: String, documents: Vec<Document>) -> Self {
        let hashes: Vec<String> = documents.iter().map(|d| d.hash.clone()).collect();
        let merkle_tree = MerkleTree::new(hashes);
        let merkle_root = merkle_tree.root.clone();
        
        let timestamp = Utc::now().timestamp();
        let hash = Self::calculate_hash(index, timestamp, &previous_hash, &merkle_root);

        Self {
            index,
            timestamp,
            previous_hash,
            merkle_root,
            documents,
            hash,
        }
    }

    fn calculate_hash(index: usize, timestamp: i64, previous_hash: &str, merkle_root: &str) -> String {
        use sha2::{Sha256, Digest};
        
        let input = format!("{}{}{}{}", index, timestamp, previous_hash, merkle_root);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_documents: Vec<Document>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "0".to_string(), vec![]);
        
        Self {
            chain: vec![genesis_block],
            pending_documents: vec![],
        }
    }

    pub fn add_document(&mut self, document: Document) {
        self.pending_documents.push(document);
    }

    pub fn mine_block(&mut self) -> Option<Block> {
        if self.pending_documents.is_empty() {
            return None;
        }

        let documents = std::mem::take(&mut self.pending_documents);
        let previous_block = self.chain.last().unwrap();
        
        let new_block = Block::new(
            previous_block.index + 1,
            previous_block.hash.clone(),
            documents,
        );

        self.chain.push(new_block.clone());
        Some(new_block)
    }

    pub fn verify_document(&self, document_hash: &str) -> Option<(usize, Vec<String>)> {
        for block in &self.chain {
            for (i, doc) in block.documents.iter().enumerate() {
                if doc.hash == document_hash {
                    let hashes: Vec<String> = block.documents.iter().map(|d| d.hash.clone()).collect();
                    let merkle_tree = MerkleTree::new(hashes);
                    let proof = merkle_tree.get_proof(document_hash).unwrap_or_default();
                    
                    return Some((block.index, proof));
                }
            }
        }
        None
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Check the hash block
            let calculated_hash = Block::calculate_hash(
                current.index,
                current.timestamp,
                &previous.hash,
                &current.merkle_root,
            );

            if calculated_hash != current.hash {
                return false;
            }

            // Check the connection with the previous block
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}

pub type SharedBlockchain = Arc<RwLock<Blockchain>>;