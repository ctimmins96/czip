// Module: Main
// 
// Description:
//     Top-Level entry-point
// 

//-- Submodules
mod app;

//-- External Imports

//-- Functions

//-- Structs / Implementations / Enums / Traits

//-- Main
fn main() {
    main3();
}

fn main1() {
    use crate::app::huff::enc_structs::tree::{HuffChild, HuffTree};
    println!("Pushing Items...");
    let mut tree = HuffTree::new();
    tree.push(HuffChild::new(String::from("A"), 12));
    tree.print();
    tree.push(HuffChild::new(String::from("B"), 11));
    tree.print();
    tree.push(HuffChild::new(String::from("C"), 1));
    tree.print();
    tree.push(HuffChild::new(String::from("D"), 2));
    tree.print();
}

fn main2() {
    use crate::app::huff::enc_structs::tree::{HuffChild, HuffTree};
    println!("Pushing Items...");
    let mut tree = HuffTree::new();
    tree.push(HuffChild::new(String::from("A"), 36));
    tree.push(HuffChild::new(String::from("B"), 23));
    tree.push(HuffChild::new(String::from("C"), 13));
    tree.push(HuffChild::new(String::from("D"), 10));
    tree.push(HuffChild::new(String::from("E"), 10));
    tree.print();
}

fn main3() {
    use crate::app::huff::{compress, CompressionResult};
    use crate::app::huff::enc_structs::byte_string::ByteString;
    use crate::app::huff::enc_structs::table::Table;
    let test = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let cmp: CompressionResult = compress(String::from(test), false, 0.67);
    let mut tab: Table = cmp.table.clone();
    tab.flip();
    tab.print();
    let mut payload: ByteString = cmp.payload.clone();
    let mut tmp = String::new();
    let mut mask: u8 = 128;
    let c_byte = payload.next().unwrap();
    let mut is_code = false;
    while mask > 0 && !is_code {
        if (mask & c_byte) > 0 { tmp.push('1'); }
        else { tmp.push('0'); }
        is_code = tab.translate(tmp.clone()).is_some();
        mask = mask >> 1;
    }
    println!("First character: {:} => {:}", tmp, tab.translate(tmp.clone()).unwrap());
}

//-- Test
#[cfg(test)]
mod tests {
    mod huff {
        mod encoding {
            use crate::app::huff::enc_structs::tree::{HuffChild, HuffTree};
            use crate::app::huff::enc_structs::table::Table;
            use crate::app::huff::enc_structs::queue::{PrioItem, PrioQueue};
            use crate::app::huff::enc_structs::byte_string::ByteString;

            #[test]
            fn test_tree() {
               // Break Stuff
               let mut t = Table::new();
               t.push("0", "A");
               assert!(t.len() == 1);
               t.push("1", "C");
               assert!(t.len() == 2);
               assert!(t.peek("0") == "A");
               t.clear();
               assert!(t.len() == 0);
            }

            #[test]
            #[should_panic]
            fn test_tree_peek() {
               // Break Stuff
               let mut t = Table::new();
               t.push("0", "A");
               t.push("1", "C");
               t.peek("2");
            }

            #[test]
            fn test_prio_item() {
                let mut p = PrioItem::new("A");
                assert!(p.is_match("A"));
                assert!(!p.is_match("a"));
                assert!(p.prio() == 1);
                p.push();
                assert!(p.prio() == 2);
                p.pop();
                assert!(p.prio() == 1);
            }

            #[test]
            fn test_prio_queue() {
                // Break Stuff
                let mut q = PrioQueue::new();
                let mut i: usize = 99;
                assert!(!q.has("A", &mut i));
                assert!(i != 0);
                q.push("A");
                assert!(q.size() == 1);
                assert!(q.has("A", &mut i));
                assert!(i == 0);
                assert!(q.peek("A") == 1);
                q.push("A");
                q.push("B");
                assert!(q.peek("A") == 2);
                assert!(q.size() == 2);
                q.push("C");
                q.push("C");
                assert!(q.size() == 3);
                assert!(q.has("C", &mut i));
                assert!(q.peek("C") == 2);
                assert!(i == 1);
                q.push("C");
                assert!(q.has("C", &mut i));
                assert!(q.peek("C") == 3);
                assert!(i == 2);
                assert!(q.cnt() == 6);
                let rm = q.pop();
                assert!(rm.is_match("C"));
                assert!(rm.prio() == 3);
                assert!(!q.has("C", &mut i));
                assert!(q.size() == 2);
                assert!(q.cnt() == 3);
            }

            #[test]
            #[should_panic]
            fn test_prio_queue_peek_1() {
                // Break Stuff
                let q = PrioQueue::new();
                q.peek("A");
            }

            #[test]
            #[should_panic]
            fn test_prio_queue_peek_2() {
                // Break Stuff
                let mut q = PrioQueue::new();
                q.push("A");
                q.peek("B");
            }

            #[test]
            fn test_huff_child() {
                // Break Stuff
                //
                // -- Null Child
                let n = HuffChild::null();
                assert!(n.w() == 0);
                assert!(n.code() == Option::None);

                // -- Empty Child
                let e = HuffChild::empty(12);
                assert!(e.w() == 12);
                assert!(e.code() == Option::None);

                // -- New Child
                let h = HuffChild::new(String::from("A"), 10);
                assert!(h.code().unwrap() == String::from("A"));
                assert!(h.w() == 10);
            }

            #[test]
            fn test_huff_tree() {
                // Break Stuff
                let mut t = HuffTree::new();

                t.push(HuffChild::new(String::from("A"), 12));
                t.push(HuffChild::new(String::from("B"), 11));
                t.push(HuffChild::new(String::from("C"), 1));
                assert!(t.has_token("A"));
                assert!(t.has_token("B"));
                assert!(t.has_token("C"));
                assert!(t.code("A").unwrap() == 1);
                assert!(t.code_str("A").unwrap() == "0");
                assert!(t.code("B").unwrap() == 5);
                assert!(t.code_str("B").unwrap() == "10");
                assert!(t.code("C").unwrap() == 6);
                assert!(t.code_str("C").unwrap() == "11");
                assert!(t.code("D").is_none());
            }

            #[test]
            fn test_byte_string() {
                // Break Stuff
                let mut b = ByteString::new();
                b.push(String::from("001"));
                b.push(String::from("101"));
                b.push(String::from("10"));
                b.push(String::from("00111"));
                b.push(String::from("001"));
                assert!(b.as_bits() == String::from("0011011000111001"));
                assert!(b.as_utf8() == String::from("69"));

                let mut s = b.clone();
                s.push(String::from("001011"));
                assert!(s.as_bits() == "0011011000111001001011");
                println!("Payload: {:}", s.as_utf8());
                assert!(s.as_utf8() == "69,");
                s.push(String::from("11"));
                println!("Payload: {:}", s.as_utf8());
                assert!(s.as_utf8() == "69/");

                let s_str = s.as_utf8();
                let mut tmp = String::new();
                let mut parser = s_str.bytes();
                let mut c_wrap = parser.next();
                while c_wrap.is_some() {
                    let c_byte = c_wrap.unwrap();
                    let mut mask: u8 = 128;
                    while mask > 0 {
                        if (mask & c_byte) > 0 { tmp.push('1'); }
                        else { tmp.push('0'); }
                        mask = mask >> 1;
                    }
                    c_wrap = parser.next();
                }
                assert!(tmp == "001101100011100100101111");
            }

            #[test]
            fn test_byte_string_encoding() {
                // Break Stuff
                let mut s = ByteString::new();
                s.push(String::from("001"));
                s.push(String::from("101"));
                s.push(String::from("10"));
                s.push(String::from("00111"));
                s.push(String::from("001"));
                s.push(String::from("001011"));
                s.push(String::from("11"));

                let s_str = s.as_utf8();
                let mut tmp = String::new();
                let mut parser = s_str.bytes();
                let mut c_wrap = parser.next();
                while c_wrap.is_some() {
                    let c_byte = c_wrap.unwrap();
                    let mut mask: u8 = 128;
                    while mask > 0 {
                        if (mask & c_byte) > 0 { tmp.push('1'); }
                        else { tmp.push('0'); }
                        mask = mask >> 1;
                    }
                    c_wrap = parser.next();
                }
                assert!(tmp == "001101100011100100101111");

                tmp = String::new();
                parser = s_str.bytes();
                c_wrap = parser.next();
                while c_wrap.is_some() {
                    let c_byte = c_wrap.unwrap();
                    let mut mask: u8 = 128;
                    let mut buffer: u8 = 0;
                    while mask > 0 {
                        if (mask & c_byte) > 0 { buffer += mask; }
                        mask = mask >> 1;
                    }
                    tmp.push(buffer as char);
                    c_wrap = parser.next();
                }
                assert!(tmp == String::from("69/"));
            }

            #[test]
            fn test_table() {
                // Break Stuff
                let mut tab = Table::new();
                tab.push("101", "A");
                tab.push("110", "B");
                tab.push("111", "C");
                tab.push("00", " ");
                assert!(tab.to_str() == "|101=A|110=B|111=C|00= ");

                // Test Table from String.
                let tab2 = Table::from_str(String::from("|101=A|110=B|111=C|00= "));
                assert!(tab.to_str() == tab2.unwrap().to_str());

                // Test Table from String in reverse method.
                let tab3 = Table::from_str(String::from("|A=101|B=110|C=111| =00"));
                assert!(tab3.unwrap().to_str() == String::from("|A=101|B=110|C=111| =00"));

                // Test the error returns of the from_str methods
                assert!(Table::from_str(String::from("|101>A")).is_err());
            }
        }
        use crate::app::huff::{compress, decompress, CompressionResult};

        #[test]
        fn compress_test() {
            // Break Stuff
            let test = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
            let mut cmp: CompressionResult = compress(String::from(test), false, 0.67);

            // Time to check each token
            cmp.table.flip();
            cmp.table.print();
            let mut parser = cmp.payload.clone();
            let mut chars = test.chars();
            let mut tmp = String::new();
            let mut c_wrap = parser.next();
            while c_wrap.is_some() {
                let c_byte = c_wrap.unwrap();
                let mut mask: u8 = 128;
                while mask > 0 {
                    if (mask & c_byte) > 0 {
                        tmp.push('1');
                    }
                    else {
                        tmp.push('0');
                    }
                    let code_check = cmp.table.translate(tmp.clone());
                    if code_check.is_some() {
                        // Now check the translated value is the same as the string at this index
                        let mut c_code = code_check.unwrap();
                        let n_char = chars.next();
                        if n_char.is_some() {
                            let v1 = n_char.unwrap();
                            let v2 = c_code.pop().unwrap();
                            assert!(v1 == v2);
                        }
                        tmp.clear();
                    }
                    mask = mask >> 1;
                }
                c_wrap = parser.next();
            }
        }

        #[test]
        fn compress_decompress_test() {
            // Break Stuff
            let test = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
            let mut cmp = compress(String::from(test), false, 0.67);
            cmp.table.flip();
            let dcmp = decompress(cmp);
            println!("Expected: {:}\n\nDecoded: {:}", test.clone(), dcmp.clone());
            assert!(dcmp == test);
        }
    }
}

