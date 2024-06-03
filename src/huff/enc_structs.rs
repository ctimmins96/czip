// Module: Encoding Structures
// 
// Description:
//     Huffman Encoding Structures Sub-Module
// 

//-- Submodules

//-- External Imports
use std::panic;

//-- Functions
/// Function: rank_of
///
/// Argument(s):
///     - idx (usize) -- Info goes here.
///
/// Return(s):
///     - ret (usize) -- Info goes here.
pub fn rank_of(idx: usize) -> usize {
    let mut r: usize = 0;
    let mut c_idx: usize = 0;
    while idx > c_idx {
        r += 1;
        c_idx = 2*c_idx + 2;
    }
    r
}

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
#[derive(Debug,Clone,PartialEq)]
pub struct HuffTree {
    children: Vec<HuffChild>
}

impl HuffTree {
    /// Function: new
    ///
    /// Argument(s):
    ///     - None
    ///
    /// Return(s):
    ///     - ret (Self) -- Info goes here.
    pub fn new() -> Self {
        let children: Vec<HuffChild> = Vec::new();
        Self { children }
    }

    /// Function: push
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///     - c (HuffChild) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn push(&mut self, c: HuffChild) {
        self.children.push(c);
        self.heapify();
    }

    /// Function: child_left
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///     - idx:usize (Placeholder) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (usize) -- Info goes here.
    pub fn child_left(&self, idx:usize) -> usize {
        if (2*idx + 1) < self.children.len() {
            2*idx + 1
        }
        else {
            0
        }
    }

    /// Function: child_right
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///     - idx:usize (Placeholder) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (usize) -- Info goes here.
    pub fn child_right(&self, idx:usize) -> usize {
        if (2*idx + 2) < self.children.len() {
            2*idx + 2
        }
        else {
            0
        }
    }

    /// Function: parent
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///     - idx (usize) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Option<usize>) -- Info goes here.
    pub fn parent(&self, idx: usize) -> Option<usize> {
        if idx == 0 { Option::None }
        else {
            Option::Some(((idx as f32) / 2.0).floor() as usize)
        }
    }

    /// Function: rank
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (usize) -- Info goes here.
    pub fn rank(&self) -> usize {
        let mut rnk: usize = 0;
        let mut idx: usize = self.child_left(0 as usize);
        while idx > 0 {
            rnk += 1;
            idx = self.child_left(idx);
        }
        rnk
    }

    /// Function: fluff
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn fluff(&mut self) {
        // Pad the vector to the next rank
        let rank = self.rank();
        let mut idx = 0;
        while self.child_left(idx) > 0 {
            idx = self.child_left(idx);
        }
        let start = idx;
        while self.children.len() < (start + ((2.0_f32).powi(rank as i32)) as usize) {
            self.children.push(HuffChild::null());
        }
    }

    /// Function: prune
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn prune(&mut self) {
        // Assuming that the array is already sorted
        if self.children.len() > 1 {
            // Start at end of the array and work back by powers of 2 until we find a HuffChild
            // that is not null 
            while self.children[self.children.len() - 1].code() == Option::None {
                self.children.pop();
            }
        }
    }

    /// Function: heapify
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn heapify(&mut self) {
        // -- Knowing that the element is inserted at the end-ish, check that it belongs in this
        // rank 
        // ^WRONG ---- ASSUME NOTHING
        //
        // -- Check all elements, make sure that their respective weight puts them on the correct
        // level, do this by starting at the end of the vector and comparing them to their parent
        // and to other members of their same rank.
        if self.children.len() > 1 {
            // Edge-Case: Check if there is only 2 elements
            if self.children.len() == 2 {
                self.children.push(HuffChild::empty(self.sum_of(0)));
                self.swap(0,1);
                self.swap(0,2);
            }
            else {
                // Determine what rank and where in the rank the end of the array is.
                let r = self.rank();

                // Check the item at the end and see what rank the new item can fit into
                let mut n_idx = self.children.len() - 1;
                let c_ref = self.children[n_idx].clone();
                let mut ranks: Vec<usize> = Vec::new();
                for i in 0..n_idx {
                    let cmp: f32 = (self.children[i].w() as f32) / (c_ref.w() as f32);
                    if cmp > 0.5 && cmp < 2.0 {
                        if !ranks.contains(&rank_of(i.clone())) {
                            ranks.push(rank_of(i.clone()));
                        }
                    }
                }

                // Ranks found, now we see which side makes the most sense to insert on.
                // Check viable endpoints
                let mut s_idx = 0;
                let mut term = false;
                while !term {
                    if self.child_left(s_idx) > 0 && self.child_right(s_idx) > 0 {
                        // It has both children
                        // Check to see which direction has a greater weight
                        if self.children[self.child_left(s_idx)].w() >= self.children[self.child_right(s_idx)].w() {
                            // Left side has a higher weight than the right
                            // (also default case when =)
                            // Make sure the right path is valid
                            if self.children[self.child_right(s_idx)].code().is_none() {
                                // Right child is a valid path
                                s_idx = self.child_right(s_idx);
                            }
                        }
                        else {
                            // Right side has a higher weight than the right
                            // Make sure left path is valid
                            if self.children[self.child_left(s_idx)].code().is_none() {
                                // Left child is a valid path
                                s_idx = self.child_left(s_idx);
                            }
                        }
                    }
                    else if self.child_left(s_idx) > 0 && self.child_right(s_idx) == 0 {
                        // It has only the left child
                    }
                    else if self.child_left(s_idx) == 0 && self.child_right(s_idx) > 0 {
                        // It has only the right child
                    }
                    else {
                        // It has no children
                    }
                }
                  
                // If no viable endpoints persist, bubble down then heapify
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
    pub fn swap(&mut self, i1: usize, i2: usize) {
        self.children.swap(i1, i2);
    }

    /// Function: sum_of
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///     - idx (usize) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (usize) -- Info goes here.
    pub fn sum_of(&self, idx: usize) -> usize {
        let mut sum = self.children[idx].w();
        if self.child_left(idx) > 0 {
            sum += self.sum_of(self.child_left(idx));
        }
        if self.child_right(idx) > 0 {
            sum += self.sum_of(self.child_right(idx));
        }
        sum
    }

    /// Function: as_str
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (str) -- Info goes here.
    pub fn as_str(&self) -> String {
        let mut ret = String::from("");
        let mut r = 0;
        for i in 0..self.children.len() {
            if r != rank_of(i) {
                ret.push('\n');
                r = rank_of(i);
            }
            ret.push('\"');
            if self.children[i].code().is_some() {
                for c in self.children[i].code().unwrap().chars() {
                    ret.push(c);
                }
            }
            else {
                for c in "null".chars() {
                    ret.push(c);
                }
            }
            ret.push('\"');
            ret.push(' ');
        }
        ret.clone()
    }
}

// Huffman Tree Child -- HuffChild
#[derive(Debug,Clone,PartialEq)]
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

    /// Function: empty
    ///
    /// Argument(s):
    ///     - wght (usize) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Self) -- Info goes here.
    pub fn empty(wght: usize) -> Self {
        Self { value: Option::None, weight: wght }
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

    /// Function: set
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///     - n_weight (usize) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn set(&mut self, n_weight: usize) {
        self.weight = n_weight;
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

