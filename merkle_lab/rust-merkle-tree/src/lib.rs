//! merkle-tree implement Merkle Tree in Rust.
//!
//! It need to recalculate full tree when added
//! or removed element into/from tree.  
//! 
#![feature(test)]

extern crate crypto;
extern crate test;

mod merkle_tree;
mod node;
mod hash;
mod proof;
mod tests;

pub use self::merkle_tree::MerkleTree;
