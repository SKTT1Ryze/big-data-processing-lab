//! Hash Implemation
//! 

use crypto::{
    digest::Digest,
    sha2::Sha256,
};
use std::string::ToString;

pub fn hash_empty() -> String {
    hash_leaf(&0)
}

pub fn hash_leaf<T: ToString>(data: &T) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&data.to_string().as_ref());
    hasher.result_str()
}

pub fn hash_mid<T: ToString>(left: &T, right: &T) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(left.to_string().as_ref());
    hasher.input_str(right.to_string().as_ref());
    hasher.result_str()

}