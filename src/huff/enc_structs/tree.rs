// Module: Tree
// 
// Description:
//     Huffman Encoding Structure Submodule -- Huffman Tree
// 

//-- Submodules

//-- External Imports
use std::panic;
use super::rank::rank_of;

//-- Functions

//-- Structs / Implementations / Enums / Traits
// Huffman Tree -- HuffTree
#[derive(Debug,Clone,PartialEq)]
pub struct HuffTree {
    children: Vec<HuffChild>,
    tokens: Vec<String>
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
        let tokens: Vec<String> = Vec::new();
        Self { children, tokens }
    }

    /// Function: tokens
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Vec<String>) -- Info goes here.
    pub fn tokens(&self) -> Vec<String> {
        // Do a thing
        self.tokens.clone()
    }

    /// Function: codes
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Vec<String>) -- Info goes here.
    pub fn codes(&self) -> Vec<String> {
        let mut toks = self.tokens.iter();
        let mut tmp = toks.next();
        let mut outp: Vec<String> = Vec::new();
        while tmp.is_some() { 
            outp.push(self.code_str(tmp.unwrap()).unwrap());
            tmp = toks.next();
        }
        outp
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
        self.tokens.push(c.code().unwrap());
        self.heapify(c);
        self.reweigh();
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
            Option::Some((((idx - 1) as f32) / 2.0).floor() as usize)
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
    ///     - child (HuffChild) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn heapify(&mut self, child: HuffChild) {
        // -- Knowing that the element is inserted at the end-ish, check that it belongs in this
        // rank 
        // ^WRONG ---- ASSUME NOTHING
        //
        // -- Check all elements, make sure that their respective weight puts them on the correct
        // level, do this by starting at the end of the vector and comparing them to their parent
        // and to other members of their same rank.
        if self.children.len() > 0 {
            // Edge-Case: Check if there is only 2 elements
            if self.children.len() == 1 {
                self.children.push(HuffChild::null());
                self.children.push(child);
                self.swap(0,1);
            }
            else {
                let mut s_idx = 0;
                let mut term = false;
                while !(term) {
                    if self.child_left(s_idx) > 0 && self.child_right(s_idx) > 0 {
                        // It has both children
                        // Check to make sure we are at non-terminated endpoint
                        if self.children[s_idx].code().is_some() {
                            // Bubble down and repeat the loop here
                            let sw_idx = self.child_left(s_idx);
                            self.swap(s_idx, sw_idx);
                        }
                        else {
                            // Check to see which direction has a greater weight
                            if self.children[self.child_left(s_idx)].w() >= self.children[self.child_right(s_idx)].w() {
                                // Left side has a higher weight than the right
                                // (also default case when =)
                                // Check to see if the item has no weight
                                let sw_idx = self.child_right(s_idx);
                                if self.children[sw_idx].w() == 0 {
                                    // Insert child here
                                    self.children.remove(sw_idx);
                                    self.children.insert(sw_idx, child.clone());
                                    term = true;
                                }
                                else {
                                    s_idx = sw_idx;
                                }
                            }
                            else {
                                // Right side has a higher weight than the right
                                let sw_idx = self.child_left(s_idx);
                                if self.children[sw_idx].w() == 0 {
                                    // Insert child here
                                    self.children.remove(sw_idx);
                                    self.children.insert(sw_idx, child.clone());
                                    term = true;
                                }
                                else {
                                    s_idx = sw_idx;
                                }
                            }
                        }
                    }
                    else if self.child_left(s_idx) > 0 && self.child_right(s_idx) == 0 {
                        // It has only the left child
                        // Check to make sure we are at a non-terminated endpoint
                        if self.children[s_idx].code().is_some() {
                            // Bubble down
                            let sw_idx = self.child_left(s_idx);
                            self.swap(s_idx, sw_idx);
                        }
                        // Insert Child at end.
                        self.children.push(child.clone());
                        term = true;
                    }
                    else if self.child_left(s_idx) == 0 && self.child_right(s_idx) > 0 {
                        // It has only the right child
                        // This should never happen
                        panic!("There is no left child! There is a right somehow!!! wormhole???");
                    }
                    else {
                        // It has no children
                        if self.children[s_idx].code().is_none() {
                            // Insert the Child here.
                            self.children.remove(s_idx);
                            self.children.insert(s_idx, child.clone());
                            term = true;
                        }
                        else {
                            // Need to bubble down and re-examine
                            while self.child_left(s_idx) == 0 {
                                self.children.push(HuffChild::null());
                            }
                            let sw_idx = self.child_left(s_idx);
                            self.children.swap(s_idx, sw_idx);
                        }
                    }
                }

                // Go through the items 2 by 2 and sort the child pairs
                let mut idx = 1;
                while idx < self.children.len() {
                    if self.children[idx].code().is_some() && self.children[idx + 1].code().is_some() {
                        // Compare weights
                        if self.children[idx].w() < self.children[idx + 1].w() {
                            // Swap
                            self.swap(idx, idx + 1);
                        }
                    }
                    idx += 2;
                }
            }
        }
        else {
            self.children.push(child);
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

    pub fn print(&self) {
        let mut r = 0;
        for i in 0..self.children.len() {
            if r != rank_of(i) {
                println!("");
                r = rank_of(i);
            }
            print!("\"");
            if self.children[i].code().is_some() {
                print!("{}", self.children[i].code().unwrap());
            }
            else {
                print!("null");
            }
            print!("\" ");
        }
        println!();
        println!();
    }

    /// Function: reweigh
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn reweigh(&mut self) {
        let mut idx = self.children.len() - 1;
        while idx > 0 {
            if self.children[idx].code().is_none() {
                // Check to see if it has children and add their sum
                let mut sum: usize = 0;
                if self.child_left(idx) > 0 {
                    sum += self.children[self.child_left(idx)].w();
                }
                if self.child_right(idx) > 0 {
                    sum += self.children[self.child_right(idx)].w();
                }
                self.children[idx].set(sum);
            }
            idx -= 1;
        }
        if self.children.len() >= 3 {
            let sum = self.children[1].w() + self.children[2].w();
            self.children[0].set(sum);
        }
    }

    /// Function: has_token
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///     - token (Ref str) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (bool) -- Info goes here.
    pub fn has_token(&self, token: &str) -> bool {
        self.tokens.contains(&String::from(token))
    }

    /// Function: code
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///     - token (Ref str) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Option<usize>) -- Info goes here.
    pub fn code(&self, token: &str) -> Option<usize> {
        if self.has_token(token) {
            // Find the index
            let mut idx = 0;
            let mut found = false;
            while !found {
                if self.children[idx].code().is_some() {
                    if self.children[idx].code().clone().unwrap() == String::from(token) {
                        found = true;
                    }
                }
                if !found { idx += 1; }
            }
            Option::Some(idx)
        }
        else { Option::None }
    }

    /// Function: code_str
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///     - token (Ref str) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Option<String>) -- Info goes here.
    pub fn code_str(&self, token: &str) -> Option<String> {
        let u_code = self.code(token);
        if u_code.is_some() {
            let mut idx = u_code.unwrap();
            let mut outp = String::new();
            while idx > 0 {
                if (idx % 2) == 0 {
                    outp.insert(0, '1');
                }
                else {
                    outp.insert(0, '0');
                }
                idx = self.parent(idx).unwrap();
            }
            Option::Some(outp)
        }
        else {
            Option::None
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
        self.children.clear();
        self.tokens.clear();
    }

    /// Function: size
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (usize) -- Info goes here.
    pub fn size(&self) -> usize {
        let mut size = 0;
        for i in 0..self.children.len() {
            if self.children[i].code().is_some() {
                // Add to size equal to the number of bits * weight
                size += self.children[i].w() * rank_of(i);
            }
        }
        size += 16*self.tokens.len();
        size
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

