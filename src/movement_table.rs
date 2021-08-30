
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



// rl stands for right_left
// send true for right and false for left
fn make_rl_table (mut table: &mut [u32; 4294967296], right_left: bool) {
    for i in 0..4294967296 {
        let first = right_left_16b(((i as u16) >> 16), right_left);
        let second = right_left_16b((i as u16), right_left);
        
        table[i] = ((first << 16) as u32) + (second as u32);
    }
}

pub fn make_right_table (mut right_table: &mut [u32; 4294967296]) { // 4294967296 is 2^32
    make_rl_table(right_table, true);
}

pub fn make_left_table (mut left_table: &mut [u32; 4294967296]) {
    make_rl_table(left_table, false);
}



// ud stands for up_down;
// send true for down and false for up
fn make_ud_table (mut table: &mut [u32; 4294967296], up_down: bool) {
    for i in 0..4294967296 {
        let col1: u32 = (i as u32) & 4042322160; // 11110000111100001111000011110000
        let col1_ele1: u8 = (col1 >> 28) as u8;
        let col1_ele2: u8 = (col1 >> 12) as u8;
        let col1_ele3: u8 = (col1 >> 20) as u8;
        let col1_ele4: u8 = (col1 >> 4) as u8;
        let col1: u16 = ((col1_ele1 as u16) << 12) + ((col1_ele2 as u16) << 8) + ((col1_ele3 as u16) << 4) + (col1_ele4 as u16);
        let col1: u16 = right_left_16b(col1, up_down);
        let col1_ele1: u8 = ((col1 >> 12) as u8) & 15;
        let col1_ele2: u8 = ((col1 >> 8) as u8) & 15;
        let col1_ele3: u8 = ((col1 >> 4) as u8) & 15;
        let col1_ele4: u8 = (col1 as u8) & 15;
        
        let col2: u32 = (i as u32) & 252645135; // 00001111000011110000111100001111
        let col2_ele1: u8 = (col2 >> 24) as u8;
        let col2_ele2: u8 = (col2 >> 8) as u8;
        let col2_ele3: u8 = (col2 >> 16) as u8;
        let col2_ele4: u8 = (col2 >> 0) as u8;
        let col2: u16 = ((col2_ele1 as u16) << 12) + ((col2_ele2 as u16) << 8) + ((col2_ele3 as u16) << 4) + (col2_ele4 as u16);
        let col2: u16 = right_left_16b(col2, up_down);
        let col2_ele1: u8 = ((col2 >> 12) as u8) & 15;
        let col2_ele2: u8 = ((col2 >> 8) as u8) & 15;
        let col2_ele3: u8 = ((col2 >> 4) as u8) & 15;
        let col2_ele4: u8 = (col2 as u8) & 15;
        
        let mut sum: u32 = 0;
        sum += (col1_ele1 as u32) << 28;
        sum += (col1_ele2 as u32) << 12;
        sum += (col1_ele3 as u32) << 20;
        sum += (col1_ele4 as u32) << 4;
        sum += (col2_ele1 as u32) << 24;
        sum += (col2_ele2 as u32) << 8;
        sum += (col2_ele3 as u32) << 16;
        sum += (col2_ele4 as u32) << 0;
        
        table[i] = sum;
    }
}



pub fn make_up_table (mut up_table: &mut [u32; 4294967296]) {
    make_ud_table(up_table, false);
}

pub fn make_down_table (mut down_table: &mut [u32; 4294967296]) {
    make_ud_table(down_table, true);
}