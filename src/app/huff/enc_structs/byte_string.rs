// Module: ByteString
// 
// Description:
//     String of unorganized bytes that accepts usize elements and returns a string that represents
//     those bytes in utf-8 format
// 

//-- Submodules

//-- External Imports

//-- Functions

//-- Structs / Implementations / Enums / Traits
// Byte-Code String -- ByteString
#[derive(Debug,Clone,PartialEq)]
pub struct ByteString {
    codes: Vec<u8>,
    index: usize, 
    temp: String
}

impl ByteString {
    /// Function: new
    ///
    /// Argument(s):
    ///     - None (Placeholder) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Self) -- Info goes here.
    pub fn new() -> Self {
        let codes: Vec<u8> = Vec::new();
        let index: usize = 0;
        let temp = String::new();
        Self { codes, index, temp }
    }

    /// Function: push
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///     - code (String) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (None) -- Info goes here.
    pub fn push(&mut self, mut code: String) {
        while code.len() > 0 {
            self.temp.push(code.remove(0));
            // Check after pushing a new character if length == 8
            if self.temp.len() == 8 {
                // Empty out temp into a u8 variable and push it into codes.
                let mut transfer: u8 = 0;
                while self.temp.len() > 0 {
                    transfer = transfer << 1;
                    if self.temp.remove(0) == '1' { transfer += 1; }
                }
                self.codes.push(transfer);
            }
        }
    }

    /// Function: as_bits
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (String) -- Info goes here.
    pub fn as_bits(&self) -> String {
        let mut outp = String::new();
        for i in 0..self.codes.len() {
            let mut current = self.codes[i].clone();
            let mut mask: u8 = 128;
            while mask > 0 {
                if current & mask == 0 { outp.push('0'); }
                else { outp.push('1'); }
                mask = mask >> 1;
            }
        }
        let mut tmp = self.temp.clone();
        for i in 0..self.temp.len() {
            outp.push(tmp.remove(0));
        }
        outp
    }

    /// Function: temp_bits
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (u8) -- Info goes here.
    fn temp_bits(&self) -> u8 {
        // Do a thing
        let mut tmp: u8 = 0;
        let tmp_size = self.temp.len();
        let mut s_tmp = self.temp.clone();
        for i in 0..self.temp.len() {
            tmp = tmp << 1;
            if s_tmp.remove(0) == '1' { tmp += 1; }
        }
        tmp = tmp << (8 - (tmp_size));
        tmp
    }

    /// Function: as_utf8
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (String) -- Info goes here.
    pub fn as_utf8(&self) -> String {
        let mut outp = String::new();
        for i in 0..self.codes.len() {
            let mut code = self.codes[i].clone().escape_ascii().to_string();
            while code.len() > 0 {
                outp.push(code.remove(0));
            }
        }
        if self.temp.len() > 0 {
            let tmp = self.temp_bits();
            outp.push(tmp.escape_ascii().to_string().remove(0));
        }
        outp
    }

    /// Function: size
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (usize) -- Info goes here.
    pub fn size(&self) -> usize {
        if self.temp.len() == 0 {
            self.codes.len()
        }
        else {
            self.codes.len() + 1
        }
    }

    /// Function: len
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (usize) -- Info goes here.
    pub fn len(&self) -> usize {
        let bits = self.codes.len() * 8;
        let ret: usize = bits + self.temp.len();
        ret
    }
}

impl Iterator for ByteString {
    type Item = u8;

    /// Function: next
    ///
    /// Argument(s):
    ///     - Referenced-Mutable self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Option<Self::Item>) -- Info goes here.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.codes.len() {
            // Get the represented index from codes
            let c_idx = self.index.clone();
            self.index += 1;
            Option::Some(self.codes[c_idx].clone())
        }
        else if self.index == self.codes.len() {
            // Decode the item in temp
            self.index += 1;
            Option::Some(self.temp_bits())
        }
        else {
            self.index = 0;
            Option::None
        }
    }
}

