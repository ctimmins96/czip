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
    println!("Hello, world!");
}

//-- Test
#[cfg(test)]
mod tests {
    mod huff {
        mod encoding {
            use crate::huff::enc_structs::{Table, PrioItem, PrioQueue};

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
                assert!(p.prio() == 0);
                p.push();
                assert!(p.prio() == 1);
                p.pop();
                assert!(p.prio() == 0);
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
                assert!(q.peek("A") == 0);
                q.push("A");
                q.push("B");
                assert!(q.peek("A") == 1);
                assert!(q.size() == 2);
                q.push("C");
                q.push("C");
                assert!(q.size() == 3);
                assert!(q.has("C", &mut i));
                assert!(q.peek("C") == 1);
                assert!(i == 2);
                q.push("C");
                assert!(q.has("C", &mut i));
                assert!(q.peek("C") == 2);
                assert!(i == 2);
                assert!(q.cnt() == 6);
                let rm = q.pop();
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
        }
    }
}

