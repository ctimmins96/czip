// Module: queue
// 
// Description:
//     Priority Queue Encoding Structure
// 

//-- Submodules

//-- External Imports
use std::panic;

//-- Functions

//-- Structs / Implementations / Enums / Traits
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

    /// Function: clear
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn clear(&mut self) {
        self.size = 0;
        self.count = 0;
        self.q.clear();
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

    /// Function: code
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (&'static str) -- Info goes here.
    pub fn code(&self) -> String {
        String::from(self.code.clone())
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

