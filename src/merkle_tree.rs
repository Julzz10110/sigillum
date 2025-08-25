use sha2::{Sha256, Digest};
use std::collections::VecDeque;

pub struct MerkleTree {
    pub root: String,
    leaves: Vec<String>,
    nodes: Vec<String>,
}

impl MerkleTree {
    pub fn new(hashes: Vec<String>) -> Self {
        if hashes.is_empty() {
            return Self {
                root: String::new(),
                leaves: vec![],
                nodes: vec![],
            };
        }

        let leaves = hashes;
        let mut nodes = leaves.clone();
        let mut current_level = VecDeque::from(nodes.clone());

        while current_level.len() > 1 {
            let mut next_level = VecDeque::new();
            
            while !current_level.is_empty() {
                let left = current_level.pop_front().unwrap();
                let right = if current_level.is_empty() {
                    left.clone()
                } else {
                    current_level.pop_front().unwrap()
                };

                let combined = format!("{}{}", left, right);
                let mut hasher = Sha256::new();
                hasher.update(combined.as_bytes());
                let parent_hash = format!("{:x}", hasher.finalize());
                
                next_level.push_back(parent_hash.clone());
                nodes.push(parent_hash);
            }
            
            current_level = next_level;
        }

        let root = current_level.pop_front().unwrap_or_default();

        Self {
            root,
            leaves,
            nodes,
        }
    }

    pub fn get_proof(&self, leaf_hash: &str) -> Option<Vec<String>> {
        let mut proof = Vec::new();
        let mut index = self.leaves.iter().position(|h| h == leaf_hash)?;
        let mut current_nodes = self.leaves.clone();
        let mut current_index = index;

        while current_nodes.len() > 1 {
            if current_index % 2 == 0 {
                // If the current node is left, the right neighbor is needed
                if current_index + 1 < current_nodes.len() {
                    proof.push(current_nodes[current_index + 1].clone());
                }
            } else {
                // If the current node is right, the left neighbor is needed
                proof.push(current_nodes[current_index - 1].clone());
            }

            // Go a level up
            let mut next_level = Vec::new();
            for chunk in current_nodes.chunks(2) {
                if chunk.len() == 2 {
                    let combined = format!("{}{}", chunk[0], chunk[1]);
                    let mut hasher = Sha256::new();
                    hasher.update(combined.as_bytes());
                    next_level.push(format!("{:x}", hasher.finalize()));
                } else {
                    next_level.push(chunk[0].clone());
                }
            }

            current_nodes = next_level;
            current_index /= 2;
        }

        Some(proof)
    }

    pub fn verify_proof(leaf: &str, proof: &[String], root: &str) -> bool {
        let mut current_hash = leaf.to_string();

        for sibling in proof {
            let combined = if current_hash < *sibling {
                format!("{}{}", current_hash, sibling)
            } else {
                format!("{}{}", sibling, current_hash)
            };

            let mut hasher = Sha256::new();
            hasher.update(combined.as_bytes());
            current_hash = format!("{:x}", hasher.finalize());
        }

        current_hash == root
    }
}