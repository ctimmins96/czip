// Module: Main
// 
// Description:
//     Top-Level entry-point
// 

//-- Submodules
mod huff;

//-- External Imports

//-- Functions

//-- Structs / Implementations / Enums / Traits

//-- Main
fn main() {
    main1();
}

fn main1() {
    use crate::huff::enc_structs::{HuffChild, HuffTree};
    println!("Pushing Items...");
    let mut tree = HuffTree::new();
    tree.push(HuffChild::new(String::from("A"), 12));
    println!("\n\nDebug Tree: {:?}", tree);
    tree.push(HuffChild::new(String::from("B"), 11));
    println!("Current Tree\n{:?}", tree.as_str());
    println!("\n\nDebug Tree: {:?}", tree);
}

//-- Test
#[cfg(test)]
mod tests {
    mod huff {
        mod encoding {
            use crate::huff::enc_structs::{Table, PrioItem, PrioQueue, HuffChild, HuffTree};

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
                assert!(t.as_str().eq("\"null\"\n\"A\" \"B\""));
            }
        }
    }
}

