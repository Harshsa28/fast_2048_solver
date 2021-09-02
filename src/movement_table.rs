
use std::convert::TryInto;
use super::right_left_16b;


/*
This module is used to create dr_table and ul_table.

Going up is same as going left after you select the correct bits.
Going down is same as goign right after you select the correct bits

So up-table is the same as left-table and down-table is the same as right-table.
*/


// "dr" stands for "down-right"
pub fn make_dr_table (mut dr_table: &mut [u16; 65536]) { // param should be [0; 65536]
    // let mut dr_table: [u16; 65536] = [0; 65536];
    
    for i in 0..65536 {
        dr_table[i] = right_left_16b(i.try_into().unwrap(), true);
    }
}

// "ul" stands for "up-left"
pub fn make_ul_table(mut ul_table: &mut [u16; 65536]) { // param should be [0; 65536]
    // let mut ul_table: [u16; 65536] = [0; 65536];
    
    for i in 0..65536 {
        ul_table[i] = right_left_16b(i.try_into().unwrap(), false);
    }
}