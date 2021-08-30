
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

pub fn make_right_table (mut right_table: &mut [u32; 4294967296]) { // 4294967296 is 2^32
    for i in 0..4294967296 {
        let first = right_left_16b(((i as u16) >> 16), true);
        let second = right_left_16b((i as u16), true);
        
        right_table[i] = ((first << 16) as u32) + (second as u32);
    }
}

pub fn make_left_table (mut left_table: &mut [u32; 4294967296]) {
    for i in 0..4294967296 {
        let first = right_left_16b(((i as u16) >> 16), false);
        let second = right_left_16b((i as u16), false);
        
        left_table[i] = ((first << 16) as u32) + (second as u32);
    }
}