// Module: table
// 
// Description:
//     Huffman Encoding Structure -- Table
// 

//-- Submodules

//-- External Imports

//-- Functions

use super::tree::HuffTree;

//-- Structs / Implementations / Enums / Traits
// Translation Direction Enumeration
#[derive(PartialEq, Eq)]
pub enum Translation {
    Forward,
    Backward
}

// Huffman Encoding Table -- Table
#[derive(Debug)]
pub struct Table {
    /// Tree Structure; Serves as the lookup-table for the Huffman Encoding
    keys: Vec<String>,
    codes: Vec<String>,
    size: usize
}

impl Table {
    /// Function: new
    ///
    /// Argument(s):
    ///     - None 
    ///
    /// Return(s):
    ///     - ret (Self) -- Info goes here.
    pub fn new() -> Self {
        let k: Vec<String> = Vec::new();
        let c: Vec<String> = Vec::new();
        Self { keys: k, codes: c, size: 0 }
    }

    /// Function: len
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (usize) -- Info goes here.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Function: push
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///     - key (String) -- Info goes here.
    ///     - code (String) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (null) -- Info goes here.
    pub fn push(&mut self, key: &str, code: &str) {
        self.keys.push(key.to_owned());
        self.codes.push(code.to_owned());
        self.size += 1;
    }

    /// Function: peek
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///     - key (string) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (String) -- Info goes here.
    pub fn peek(&self, key: &str) -> &str {
        // Find the index with the corresponding key
        if self.keys.contains(&(key.to_owned())) {
            // Find the key index now that we know it exists.
            let mut idx = 0;
            let mut is_found = self.keys.get(idx).unwrap().to_owned() == key.to_owned();
            while idx < self.size && !is_found {
                idx += 1;
                is_found = self.keys.get(idx).unwrap().to_owned() == key.to_owned();
            }
            self.codes.get(idx).unwrap().as_str()
        }
        else {
            panic!("Key not found in Keys! Key: {}", key);
        }
    }

    /// Function: clear
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn clear(&mut self) {
        self.keys.clear();
        self.codes.clear();
        self.size = 0;
    }

    /// Function: from_tree
    ///
    /// Argument(s):
    ///     - tree (HuffTree) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Self) -- Info goes here.
    pub fn from_tree(tree: HuffTree) -> Self {
        Self::from_tree_dir(tree, Translation::Forward)
    }

    /// Function: from_tree_dir
    ///
    /// Argument(s):
    ///     - tree (HuffTree) -- Info goes here.
    ///     - translation (bool) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Self) -- Info goes here.
    pub fn from_tree_dir(tree: HuffTree, trsl: Translation) -> Self {
        if trsl == Translation::Forward {
            let keys: Vec<String> = tree.tokens();
            let codes: Vec<String> = tree.codes();
            let size: usize = keys.len();
            Self { keys, codes, size }
        }
        else {
            let keys: Vec<String> = tree.codes();
            let codes: Vec<String> = tree.tokens();
            let size: usize = keys.len();
            Self { keys, codes, size }
        }

    }

    /// Function: as_str
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (String) -- Info goes here.
    pub fn as_str(&self) -> String {
        // Do a thing
        String::new()
    }
}

