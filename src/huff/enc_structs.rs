// Module: Encoding Structures
// 
// Description:
//     Huffman Encoding Structures Sub-Module
// 

//-- Submodules

//-- External Imports
use std::panic;

//-- Functions

//-- Structs / Implementations / Enums / Traits


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
            let mut is_found = self.keys.get(idx).unwrap().to_owned() == key.clone().to_owned();
            while idx < self.size && !is_found {
                idx += 1;
                is_found = self.keys.get(idx).unwrap().to_owned() == key.clone().to_owned();
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
}

// Huffman Tree -- HuffTree
pub struct HuffTree {
    children: Vec<HuffChild>
}

impl HuffTree {
}

// Huffman Tree Child -- HuffChild
pub struct HuffChild {
    value: Option<String>,
    weight: usize
}

impl HuffChild {

    /// Function: null
    ///
    /// Argument(s):
    ///     - None
    ///
    /// Return(s):
    ///     - ret (Self) -- Info goes here.
    pub fn null() -> Self {
        Self { value: Option::None, weight: 0 }
    }

    /// Function: new
    ///
    /// Argument(s):
    ///     - value (String) -- Info goes here.
    ///     - weight (usize) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Self) -- Info goes here.
    pub fn new(value: String, wght: usize) -> Self {
        Self { value: Option::Some(value), weight: wght }
    }

    /// Function: val
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Option<String>) -- Info goes here.
    pub fn code(&self) -> Option<String> {
        self.value.clone()
    }

    /// Function: val
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (usize) -- Info goes here.
    pub fn w(&self) -> usize {
        self.weight.clone()
    }
}

// Priority Queue -- PrioQueue
#[derive(Debug,Clone,PartialEq)]
pub struct PrioQueue {
    size: usize,
    count: usize,
    q: Vec<PrioItem>,
}

impl PrioQueue {
     /// Function: new
     ///
     /// Argument(s):
     ///     - None.
     ///
     /// Return(s):
     ///     - ret (Self) -- Info goes here.
     pub fn new() -> Self {
         let q: Vec<PrioItem> = Vec::new();
         let size: usize = 0;
         let count: usize = 0;
         Self { size, count, q }
     }

     /// Function: size
     ///
     /// Argument(s):
     ///     - Referenced-self -- Info goes here.
     ///
     /// Return(s):
     ///     - ret (usize) -- Info goes here.
     pub fn size(&self) -> usize {
         self.size
     }

     /// Function: has
     ///
     /// Argument(s):
     ///     - Referenced-self -- Info goes here.
     ///     - Referenced-str (Placeholder) -- Info goes here.
     ///
     /// Return(s):
     ///     - ret (bool) -- Info goes here.
     pub fn has(&self, code: &str, i: &mut usize) -> bool {
         if self.size != 0 {
             // Loop through each item to verify if the given one is a match
             *i = 0 as usize;
             while *i < self.size && !self.q[*i].is_match(code) {
                 *i += 1;
             }
             *i < self.size
         }
         else {
             false
         }
     }

     /// Function: push
     ///
     /// Argument(s):
     ///     - Referenced-Mutable self -- Info goes here.
     ///     - code (Ref str) -- Info goes here.
     ///
     /// Return(s):
     ///     - ret (None) -- Info goes here.
     pub fn push(&mut self, code: &str) {
         let mut idx: usize = 0;
         if !self.has(code, &mut idx) {
             // push new code to the queue
             self.q.push(PrioItem::new(code));
             self.size += 1;
         }
         else {
             self.q[idx].push();
         }
         self.check(idx);
         self.count += 1;
     }

     /// Function: check
     ///
     /// Argument(s):
     ///     - Referenced-Mutable self -- Info goes here.
     ///     - idx (usize) -- Info goes here.
     ///
     /// Return(s):
     ///     - ret (None) -- Info goes here.
     fn check(&mut self, idx: usize) {
         if idx < self.size {
             if self.size > 1 {
                 // Check the element above and below the found index
                 if idx == 0 {
                     // Check the index above only
                     if self.q[idx].prio() > self.q[idx + 1].prio() {
                         // Swap the two HuffChild Items
                         self.swap(idx, idx + 1);
                         self.check(idx + 1);
                     }
                 }
                 else if idx == (self.size - 1) {
                     // Check the index below only
                     if self.q[idx].prio() < self.q[idx - 1].prio() {
                         // Swap the two children
                         self.swap(idx, idx - 1);
                         self.check(idx - 1);
                     }
                 }
                 else {
                     if self.q[idx].prio() > self.q[idx + 1].prio() {
                         self.swap(idx, idx + 1);
                         self.check(idx + 1);
                     }
                     else if self.q[idx].prio() < self.q[idx - 1].prio() {
                         self.swap(idx, idx - 1);
                         self.check(idx - 1);
                     }
                 }
             }
         }
     }

     /// Function: swap
     ///
     /// Argument(s):
     ///     - Referenced-Mutable self -- Info goes here.
     ///     - i1 (usize) -- Info goes here.
     ///     - i2 (usize) -- Info goes here.
     ///
     /// Return(s):
     ///     - ret (None) -- Info goes here.
     fn swap(&mut self, i1: usize, i2: usize) {
         let temp = self.q.remove(i1);
         self.q.insert(i2, temp);
     }

     /// Function: peek
     ///
     /// Argument(s):
     ///     - Referenced-self -- Info goes here.
     ///     - code (Ref str) -- Info goes here.
     ///
     /// Return(s):
     ///     - ret (u8) -- Info goes here.
     pub fn peek(&self, code: &str) -> u8 {
         let mut idx: usize = 0;
         if self.has(code, &mut idx) {
             self.q.get(idx).unwrap().priority
         }
         else {
             panic!("Specified code wasn't found! Code: {}", code)
         }
     }
    
    /// Function: pop
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///     - code (Ref str) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (PrioItem) -- Info goes here.
    pub fn pop(&mut self) -> PrioItem {
        self.count -= self.q[self.size - 1].prio() as usize;
        self.size -= 1;
        self.q.remove(self.size)
    }

    /// Function: remove
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///     - code (Ref str) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (PrioItem) -- Info goes here.
    pub fn remove(&mut self, code: &str) -> PrioItem {
        let mut idx: usize = 0;
        if self.has(code, &mut idx) {
            self.size -= 1;
            self.count -= self.q[idx].prio() as usize;
            self.q.remove(idx)
        }
        else {
            panic!("Specified code was not found! Code: {}", code);
        }
    }

    /// Function: cnt
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (usize) -- Info goes here.
    pub fn cnt(&self) -> usize {
        self.count
    }

    /// Function: sequence
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (String) -- Info goes here.
    pub fn sequence(&self) -> String {
        let mut seq = String::from("");
        let mut idx = 0;
        while idx < self.size {
            seq += self.q[idx].code.as_str();
            idx += 1;
        }
        seq.to_owned()
    }
}

// Priority Item -- PrioItem
#[derive(Debug,Clone,PartialEq)]
pub struct PrioItem {
    code: String,
    priority: u8
}

impl PrioItem {

    /// Function: new
    ///
    /// Argument(s):
    ///     - code (Ref str) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Self) -- Info goes here.
    pub fn new(code: &str) -> Self {
        PrioItem { code: code.to_owned(), priority: 1 }
    }

    /// Function: push
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn push(&mut self) {
        self.priority += 1;
    }

    /// Function: pop
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret () -- Info goes here.
    pub fn pop(&mut self) {
        self.priority -= 1;
    }

    /// Function: prio
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (u8) -- Info goes here.
    pub fn prio(&self) -> u8 {
        self.priority.clone()
    }


    /// Function: match
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///     - code (Referenced-String) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (bool) -- Info goes here.
    pub fn is_match(&self, code: &str) -> bool {
        self.code.eq(code)
    }
}

