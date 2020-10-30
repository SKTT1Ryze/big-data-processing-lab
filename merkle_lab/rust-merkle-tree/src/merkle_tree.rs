use std::{
    collections::{
        BTreeMap,
        VecDeque,
        vec_deque::Iter,
    },
    fmt,
    fmt::Display,
    rc::Rc,
};
use crate::node::Node;
use crate::hash::*;
use crate::proof::Proof;

#[derive(Debug)]
pub enum ProofNode {
    Left(String),
    Right(String),
}

/// MerkleTree struct represents merkle binary tree with values of type `T` and map of nodes.  
// #[derive(Debug)]
pub struct MerkleTree<T: ToString + Display + Clone + fmt::Debug> {
    root: Node<T>,
    height: usize,
    count: usize,
    storage: VecDeque<Rc<T>>,
    nodes: BTreeMap<usize, VecDeque<Node<T>>>,
}

impl<T: ToString + Display + Clone + fmt::Debug> fmt::Debug for MerkleTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MerkleTree")
            .field("root", &self.root)
            .field("height", &self.height)
            .field("count", &self.count)
            .field("storage", &self.storage)
            .finish()
    }
}

impl<T: ToString + Display + Clone + fmt::Debug> MerkleTree<T> {
    pub fn new() -> Self {
        Self {
            root: Node::empty(),
            height: 0,
            count: 0,
            storage: VecDeque::new(),
            nodes: BTreeMap::new(),
        }
    }

    pub fn from_vec(data: Vec<T>) -> Self {
        if data.is_empty() {
            Self::new()
        } else {
            let blocks = data.into_iter()
                .map(|e| Rc::new(e))
                .collect::<VecDeque<Rc<T>>>();
            let mut tree = Self {
                root: Node::empty(),
                height: 0,
                count: 0,
                storage: blocks,
                nodes: BTreeMap::new(),
            };
            tree.calculate();
            tree
        }
    }

    pub fn push(&mut self, val: T) {
        self.storage.push_back(Rc::new(val));
        self.count = self.storage.len();
        self.calculate();
    }

    pub fn remove(&mut self, idx: usize) -> bool {
        if let Some(_) = &self.storage.remove(idx) {
            self.count = self.storage.len();
            self.calculate();
            true
        } else {
            false
        }
    }

    pub fn get_idx(&self, idx: usize) -> Option<&T> {
        if let Some(e) = self.storage.get(idx) {
            Some(e.as_ref())
        } else {
            None
        }
    }

    pub fn get_vals(&self) -> Option<Vec<T>> {
        if self.storage.is_empty() {
            None
        } else {
            let vals = self.storage
                .iter()
                .map(|e| e.as_ref().clone())
                .collect::<Vec<T>>();
            Some(vals)
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_empty(self) -> bool {
        self.storage.is_empty()
    }

    pub fn root_hash(&self) -> Option<&String> {
        self.root.hash()
    }

    pub fn iter(&self) -> Iter<Rc<T>> {
        self.storage.iter()
    }

    pub fn get_proof(&self, val: T) -> Proof<T> {
        let path = self.get_needed_hashes_for_proof(&val);
        Proof::new(self.root_hash().unwrap().clone(), val.clone(), path)
    }

    fn calculate(&mut self) {
        self.count = self.storage.len();
        self.height = calculate_height(self.count);
        self.root = Node::empty();
        self.nodes.clear();
        let mut cur_level = self.height;
        if !self.storage.is_empty() {
            let mut leaves = VecDeque::new();
            for val in &self.storage {
                let leaf = Node::create_leaf(val.clone());
                leaves.push_back(leaf);
            }
            self.nodes.insert(cur_level, leaves);

            while cur_level > 0 {
                let pre_level = cur_level - 1;
                let pre_row = {
                    let mut row = VecDeque::new();
                    let cur_row = self.nodes.get(&cur_level).unwrap();
                    for i in (0..cur_row.len()).step_by(2) {
                        let left = cur_row.get(i).unwrap();
                        let right = cur_row.get(i + 1).unwrap_or(left);
                        let node = Node::create_mid(left.clone(), right.clone());
                        row.push_back(node);
                    }
                    row
                };
                self.nodes.insert(pre_level, pre_row);
                cur_level -= 1;
            }
            assert!(cur_level == 0);
            self.root = self.nodes.get(&0).unwrap()[0].clone();
        }
    }

    fn get_needed_hashes_for_proof(&self, value: &T) -> Vec<ProofNode> {
        let mut level = self.height;
        let mut next_hash = hash_leaf(&value);
        let mut needed_hashes = Vec::new();

        while level > 0 {
            if let Some(index) = self.get_element_index(level, &next_hash) {
                let nodes = self.nodes.get(&level).unwrap();
                match nodes.get(index) {
                    Some(&Node::Leaf { ref hash, .. }) |
                    Some(&Node::Mid { ref hash, .. }) => {
                        if index % 2 == 0 {
                            if let Some(sibling_node) = nodes.get(index + 1) {
                                needed_hashes.push(ProofNode::Right(sibling_node
                                                                        .hash()
                                                                        .unwrap()
                                                                        .clone()));
                                next_hash = hash_mid(hash, sibling_node.hash().unwrap());
                            } else {
                                needed_hashes.push(ProofNode::Right(hash.clone()));
                                next_hash = hash_mid(hash, hash);
                            }
                        } else {
                            if let Some(sibling_node) = nodes.get(index - 1) {
                                needed_hashes.push(ProofNode::Left(sibling_node
                                                                       .hash()
                                                                       .unwrap()
                                                                       .clone()));
                                next_hash = hash_mid(sibling_node.hash().unwrap(), hash);
                            }
                        }
                    }
                    _ => continue,
                };
            }
            level -= 1;
        }
        needed_hashes
    }

    fn get_element_index(&self, level: usize, hash: &String) -> Option<usize> {
        let row_hashes = self.nodes
            .get(&level)
            .unwrap()
            .iter()
            .map(|e| e.hash().unwrap())
            .collect::<Vec<&String>>();
        row_hashes.iter().position(|&s| s == hash)
    }

    pub fn incorrect_block(a_tree: &MerkleTree<T>, b_tree: &MerkleTree<T>) -> Option<Node<T>> {
        if a_tree.root_hash() == b_tree.root_hash() {
            return None;
        } else {
            Self::find_incorrect_block(a_tree.root.clone(), b_tree.root.clone())
        }
    }

    #[allow(unused_variables)]
    pub fn find_incorrect_block(err_root: Node<T>, bake_root: Node<T>) -> Option<Node<T>>{
        match err_root {
            Node::Empty{hash} => {
                panic!("Empty Node");
            },
            Node::Mid{
                left: e_left,
                right: e_right,
                hash: e_hash,
            } => {
                match bake_root {
                    Node::Mid{
                        left: b_left,
                        right: b_right,
                        hash: b_hash,
                    } => {
                        if e_hash == b_hash {
                            return None;
                        } else {
                            match Self::find_incorrect_block(*e_left.clone(), *b_left.clone()) {
                                None => {
                                    return Self::find_incorrect_block(*e_right.clone(), *b_right.clone())
                                },
                                n => return n,
                            }
                            
                        }
                    },
                    _ => panic!("Two trees have different shape."),
                }
            },
            Node::Leaf{
                data,
                hash,
            } => {
                return Some(Node::create_leaf(data.clone()));
            },
        }
    }
}


pub fn calculate_height(count: usize) -> usize {
    if count > 0 {
        let height = (count as f64).log2();
        if height - height.floor() > 0.0 {
            (height + 1.0) as usize
        } else {
            height as usize
        }
    } else {
        0
    }
}
