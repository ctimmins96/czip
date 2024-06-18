// Module: Huff
// 
// Description:
//     Huffman Encoding module. Includes encoding, creating Huffman Trees, and decoding.
// 

//-- Submodules
pub mod enc_structs;
pub mod string_decoding;

//-- External Imports
use self::enc_structs::tree::{HuffTree, HuffChild};
use self::enc_structs::queue::PrioQueue;
use self::enc_structs::table::{Table, Translation};
use self::enc_structs::byte_string::ByteString;

//-- Functions
/// Function: compress
///
/// Argument(s):
///     - payload (String) -- Info goes here.
///     - check_flag (bool) -- Info goes here.
///     - compression_ratio (f32) -- Info goes here.
///
/// Return(s):
///     - ret (CompressionResult) -- Info goes here.
pub fn compress(mut payload: String, check_flag: bool, compress_ratio_min: f32) -> CompressionResult {
    let mut tree = HuffTree::new();
    let mut q = PrioQueue::new();
    let size: usize = payload.len();
    let mut outp = ByteString::new();
    let mut encodable = String::new();
    let mut c_size: usize = 0;
    for i in 0..size {
        let tmp = payload.remove(0);
        q.push(String::from(tmp.clone()).as_str());
        encodable.push(tmp.clone());
        // Check the size of i, if it's greater than 30% of the size
        if check_flag {
            c_size += 8;                     // add 8 bits to the current size
            if i >= (((size as f32)*0.3).floor()) as usize {
                // Check the size, make sure we are still compressing at least 67%
                let mut q_check = q.clone();
                while q_check.size() != 0 {
                    let tmp = q_check.pop();
                    tree.push(HuffChild::new(tmp.code(), tmp.prio() as usize))
                }
                if (tree.size() as f32) / (c_size as f32) >= compress_ratio_min {
                    // Commit the current tree and map to the output String and clear the current items
    
                    // Clear the current data structures
                    encodable.clear();
                    q.clear();
                    c_size = 0;
                }
                tree.clear();
            }
        }
    }
    // Empty out the priority queue into the HuffTree
    while q.size() > 0 {
        let tmp = q.pop();
        tree.push(HuffChild::new(tmp.code(), tmp.prio() as usize));
    }
    while encodable.len() > 0 {
        let token = String::from(encodable.remove(0));
        let code = tree.code_str(token.as_str()).unwrap();
        outp.push(code);
    }
    // With the compression complete,
    let ratio = (outp.size() as f32) / (size as f32);
    CompressionResult {
        bits: outp.len(),
        payload: outp,
        ratio,
        table: Table::from_tree(tree),
        dir: Translation::Forward
    }
}

/// Function: decompress
///
/// Argument(s):
///     - payload (CompressionResult) -- Info goes here.
///
/// Return(s):
///     - ret (String) -- Info goes here.
pub fn decompress(compressed: CompressionResult) -> String {
    let mut payload = compressed.payload.clone();
    let mut c_wrap = payload.next();
    let mut tmp = String::new();
    let mut outp = String::new();
    let mut bit_count: usize = 0;
    while c_wrap.is_some() {
        // Dismantle the byte, bit by bit, to find each code / token
        let c_byte = c_wrap.unwrap();
        let mut mask: u8 = 128;
        while mask > 0 {
            // Update bit_count
            bit_count += 1;
            if bit_count <= compressed.bits {
                // Check the masked value.
                if (c_byte & mask) > 0 { tmp.push('1'); }
                else { tmp.push('0'); }
                // With the updated tmp, check to see if it matches any token
                let code_check = compressed.table.translate(tmp.clone());
                if code_check.is_some() {
                    // Put the token into the output string and clear the temp buffer
                    outp.push_str(code_check.unwrap().as_str());
                    tmp.clear();
                }
            }
            // Update mask
            mask = mask >> 1;
        }
        // Update with the next byte
        c_wrap = payload.next();
    }
    outp
}

//-- Structs / Implementations / Enums / Traits
// Huffman Compression Result -- CompressionResult
pub struct CompressionResult {
    pub payload: ByteString,
    pub ratio: f32,
    pub table: Table,
    pub bits: usize,
    pub dir: Translation
}

impl CompressionResult {
    /// Function: as_str
    ///
    /// Argument(s):
    ///     - Referenced-self -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (String) -- Info goes here.
    pub fn as_str(&self) -> String {
        let mut outp = String::new();
        let mut tab = self.table.to_str();

        outp
    }

    /// Function: from_string
    ///
    /// Argument(s):
    ///     - payload (String) -- Info goes here.
    ///
    /// Return(s):
    ///     - ret (Self) -- Info goes here.
    pub fn from_string(payload: String) -> Self {
        // Do a thing
    }
}

