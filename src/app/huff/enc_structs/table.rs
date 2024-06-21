// Module: table
// 
// Description:
//     Huffman Encoding Structure -- Table
// 

//-- Submodules
use super::tree::HuffTree;

//-- Standard Imports
use std::panic;

//-- External Imports
use regex::Regex;

//-- Functions

//-- Structs / Implementations / Enums / Traits
// Translation Direction Enumeration
#[derive(PartialEq, Eq)]
pub enum Translation {
    Forward,
    Backward
}

// Huffman Encoding Table -- Table
#[derive(Debug, Clone)]
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

    /// Function: translate
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///     - key (String) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Option<String>) -- Info goes here.
    pub fn translate(&self, key: String) -> Option<String> {
        if self.keys.contains(&key) {
            let mut idx = 0;
            while self.keys[idx] != key { idx += 1; }
            Option::Some(String::from(self.codes[idx].as_str().clone()))
        }
        else { Option::None }
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

    /// Function: flip
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn flip(&mut self) {
        let size = self.codes.len();
        for i in 0..size {
            let tmp1 = self.keys.remove(0);
            let tmp2 = self.codes.remove(0);
            self.codes.push(tmp1);
            self.keys.push(tmp2);
        }
    }

    /// Function: print
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn print(&self) {
        for i in 0..self.codes.len() {
            println!("{:} -> \"{:}\"", self.keys[i], self.codes[i]);
        }
    }

    /// Function: to_str
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (String) -- Info goes here.
    pub fn to_str(&self) -> String {
        let mut outp = String::new();
        for i in 0..self.codes.len() {
            let temp = format!("|{}={}", self.keys[i].clone(), self.codes[i].clone());
            let mut parser = temp.chars();
            let mut c_char = parser.next();
            while c_char.is_some() {
                outp.push(c_char.unwrap());
                c_char = parser.next();
            }
        }
        outp
    }

    /// Function: from_str
    ///
    /// Argument(s):
    ///     - payload (String) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Self) -- Info goes here
    pub fn from_str(payload: String) -> Result<Self, &'static str> {
        let re_forward = Regex::new(r"\|(.)=([01]*)").unwrap();
        let re_backward = Regex::new(r"\|([01]*)=(.)").unwrap();
        let mut size: usize = 0;
        let mut keys: Vec<String> = Vec::new();
        let mut codes: Vec<String> = Vec::new();

        if re_forward.is_match(payload.clone().as_str()) && re_backward.is_match(payload.clone().as_str()) {
            Err("the string payload does not match any expected patterns.")
        }
        else if re_forward.is_match(payload.clone().as_str()) {
            for (_, [key, code]) in re_forward.captures_iter(payload.as_str()).map(|c| c.extract()) {
                size += 1;
                keys.push(String::from(key));
                codes.push(String::from(code));
            }
            Ok(Self { keys, codes, size })
        }
        else if re_backward.is_match(payload.clone().as_str()) {
            for (_, [key, code]) in re_backward.captures_iter(payload.as_str()).map(|c| c.extract()) {
                size += 1;
                keys.push(String::from(key));
                codes.push(String::from(code));
            }
            Ok(Self { keys, codes, size })
        }
        else {
            Err("the string payload does not match any expected patterns.")
        }
    }
}

