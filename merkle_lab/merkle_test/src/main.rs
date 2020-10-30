//! Test for merkle-tree crate
//! 
extern crate rust_merkle_tree;
use rust_merkle_tree::MerkleTree;

#[allow(unused_imports)]
use std::{
    fs,
    vec::Vec,
    thread,
    time,
};
use walkdir::WalkDir;

mod config;
use config::*;

#[allow(unreachable_code)]
fn main() {
    println!("Start MerkleTree...");
    let contexts = get_context(String::from(DATA_PATH));
    let mut mktree = MerkleTree::from_vec(contexts);
    println!("MerkleTree init: {:#?}", mktree);
    loop {
        let new_contexts = get_context(String::from(DATA_PATH));
        if new_contexts.len() != mktree.len() {
            mktree = MerkleTree::from_vec(new_contexts);
            println!("MerkleTree change: {:#?}", mktree);
            continue;
        }
        let old_contexts = mktree.get_vals().unwrap();
        for i in 0..new_contexts.len() {
            if new_contexts[i] != old_contexts[i] {
                mktree = MerkleTree::from_vec(new_contexts.clone());
                println!("MerkleTree change: {:#?}", mktree);
                continue;
            }
        }
        let sleep_sec = time::Duration::from_secs(5);
        thread::sleep(sleep_sec);
    }
    unreachable!();

    // let err_contexts = get_context(String::from(DATA_PATH));
    // let err_mktree = MerkleTree::from_vec(err_contexts);
    // let bake_contexts = get_context(String::from(BAKE_PATH));
    // let bake_mktree = MerkleTree::from_vec(bake_contexts);
    // let err_node = MerkleTree::incorrect_block(&err_mktree, &bake_mktree);
    // match err_node {
    //     Some(e) => println!("Error leaf: {:#?}", e),
    //     None => println!("No error."),
    // }
}

fn get_context(dir: String) -> Vec<String> {
    let mut contexts = Vec::new();
    for entry in WalkDir::new(dir.as_str()) {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();
        if path.len() > dir.len() {
            let context = fs::read_to_string(path).unwrap();
            contexts.push(context);
        }
    }
    contexts
}
