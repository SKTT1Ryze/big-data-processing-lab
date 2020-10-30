//! Proof Implemation
//! 

use std::fmt::Display;
use crate::hash::{hash_leaf, hash_mid};
use crate::merkle_tree::ProofNode;

#[derive(Debug)]
pub struct Proof<T: Display> {
    root_hash: String,
    val: T,
    path: Vec<ProofNode>,
}

impl<T> Proof<T>
    where T: Display
{
    pub fn new(root_hash: String, val: T, path: Vec<ProofNode>) -> Self {
        Self {
            root_hash,
            val,
            path,
        }
    }

    pub fn validate(&self, root_hash: &str) -> bool {
        let mut hash = hash_leaf(&self.val);
        for node in &self.path {
            hash = match node {
                &ProofNode::Left(ref proof_hash) => hash_mid(proof_hash, &hash),
                &ProofNode::Right(ref proof_hash) => hash_mid(&hash, proof_hash),
            };
        }
        root_hash == hash
    }
}