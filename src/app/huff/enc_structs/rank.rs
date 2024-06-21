// Module: rank
// 
// Description:
//     rank calculation module
// 

//-- Submodules

//-- External Imports

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

