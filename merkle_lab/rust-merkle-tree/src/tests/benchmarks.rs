#![cfg(test)]

use test::Bencher;
use crate::merkle_tree::MerkleTree;

#[bench]
fn benchmark_good_validation(b: &mut Bencher) {
    let data = (0..10000).collect::<Vec<_>>();
    let db = MerkleTree::from_vec(data);
    let root_hash = db.root_hash();
    let proof = db.get_proof(557);

    b.iter(|| { proof.validate(root_hash.unwrap()); })
}

#[bench]
fn benchmark_bad_validation(b: &mut Bencher) {
    let data = (0..10000).collect::<Vec<_>>();
    let db = MerkleTree::from_vec(data);
    let root_hash = db.root_hash();
    let proof = db.get_proof(242342342);

    b.iter(|| { proof.validate(root_hash.unwrap()); })
}

#[bench]
fn benchmark_creation_from_vec_with_100_elements(b: &mut Bencher) {
    b.iter(|| {
               let v = (0..100).collect::<Vec<_>>();
               let _ = MerkleTree::from_vec(v);
           });
}

#[bench]
fn benchmark_creation_from_vec_with_1000_elements(b: &mut Bencher) {
    b.iter(|| {
               let v = (0..1000).collect::<Vec<_>>();
               let _ = MerkleTree::from_vec(v);
           });
}
