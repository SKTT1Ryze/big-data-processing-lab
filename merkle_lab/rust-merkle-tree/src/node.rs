//! Node Implemation
//! 

use std::{
    fmt::Display,
    rc::Rc,
};

use crate::hash::*;

#[derive(Clone, Debug)]
pub enum Node<T: ToString + Display> {
    Mid {
        left: Box<Node<T>>,
        right: Box<Node<T>>,
        hash: String,
    },
    Leaf {
        data: Rc<T>,
        hash: String,
    },
    Empty {hash: String},
}

impl<T: Display> Node<T> {
    pub fn empty() -> Self {
        Node::Empty { hash: hash_empty()}
    }

    pub fn hash(&self) -> Option<&String> {
        match *self {
            Node::Mid { ref hash, ..} |
            Node::Leaf { ref hash, ..} |
            Node::Empty { ref hash, ..} => Some(hash),
        }
    }

    pub fn create_leaf(val: Rc<T>) -> Node<T> {
        let hash = hash_leaf(val.as_ref());
        Node::Leaf {
            data: val,
            hash: hash,
        }
    }

    pub fn create_mid(left: Node<T>, right: Node<T>) -> Node<T> {
        let hash = hash_mid(left.hash().unwrap(), right.hash().unwrap());
        Node::Mid {
            left: Box::new(left),
            right: Box::new(right),
            hash: hash,
        }
    }

}
