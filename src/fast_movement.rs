use super::pretty_print;
use super::int_to_board;
/*
This module is used for fast movement using dr_table and ul_table.
*/

pub fn fast_right(int_rep: u64, dr_table: &[u16; 65536]) -> u64 {
    let mut right: u64 = 0;

    right += (dr_table[(int_rep >> 48) as u16 as usize] as u64) << 48;
    right += (dr_table[(int_rep >> 32) as u16 as usize] as u64) << 32;
    right += (dr_table[(int_rep >> 16) as u16 as usize] as u64) << 16;
    right += dr_table[int_rep as u16 as usize] as u64;

    return right;
}

pub fn fast_left(int_rep: u64, ul_table: &[u16; 65536]) -> u64 {
    let mut left: u64 = 0;
    
    left += (ul_table[(int_rep >> 48) as u16 as usize] as u64) << 48;
    left += (ul_table[(int_rep >> 32) as u16 as usize] as u64) << 32;
    left += (ul_table[(int_rep >> 16) as u16 as usize] as u64) << 16;
    left += ul_table[int_rep as u16 as usize] as u64;
    
    return left;
}


/*

first column: 63-60;...;48-44;...;32-28;...;16-12;... in this order
second column: 0(4);59-56;0(12);43-40;0(12);27-24;0(12);11-8;0(8)
third column: 0(8); 
can convert to : 64-61;000;48-45;000;32-29;000;16-13;000
it's composed of 4 parts, each 16 bits
use bit shifting and count on 0s
*/

pub fn fast_up(int_rep: u64, ul_table: &[u16; 65536]) -> u64 {
    let mut up: u64 = 0;
    
    let mut first: u64 = int_rep & 17294086455919964160; // 1111000000000000111100000000000011110000000000001111000000000000
    first = ul_table[((first >> 48) + (first >> 36) + (first >> 24) + (first >> 12)) as u16 as usize] as u64;
    up += (((first << 48) + (first << 36) + (first << 24) + (first << 12)) & 17294086455919964160);
    
    let mut second: u64 = int_rep & 1080880403494997760; // 0000111100000000000011110000000000001111000000000000111100000000
    second = ul_table[((second >> 44) + (second >> 32) + (second >> 20) + (second >> 8)) as u16 as usize] as u64;
    up += (((second << 44) + (second << 32) + (second << 20) + (second << 8)) & 1080880403494997760);
    
    let mut third: u64 = int_rep & 67555025218437360; // 0000000011110000000000001111000000000000111100000000000011110000
    third = ul_table[((third >> 40) + (third >> 28) + (third >> 16) + (third >> 4)) as u16 as usize] as u64;
    up += (((third << 40) + (third << 28) + (third << 16) + (third << 4)) & 67555025218437360);
    
    let mut fourth: u64 = int_rep & 4222189076152335; // 0000000000001111000000000000111100000000000011110000000000001111
    fourth = ul_table[((fourth >> 36) + (fourth >> 24) + (fourth >> 12) + (fourth >> 0)) as u16 as usize] as u64;
    up += (((fourth << 36) + (fourth << 24) + (fourth << 12) + (fourth << 0)) & 4222189076152335);
    
    return up;
}

pub fn fast_down(int_rep: u64, dr_table: &[u16; 65536]) -> u64 {
    let mut down: u64 = 0;
    
    let mut first: u64 = int_rep & 17294086455919964160; // 1111000000000000111100000000000011110000000000001111000000000000
    first = dr_table[((first >> 48) + (first >> 36) + (first >> 24) + (first >> 12)) as u16 as usize] as u64;
    down += (((first << 48) + (first << 36) + (first << 24) + (first << 12)) & 17294086455919964160);
    
    let mut second: u64 = int_rep & 1080880403494997760; // 0000111100000000000011110000000000001111000000000000111100000000
    second = dr_table[((second >> 44) + (second >> 32) + (second >> 20) + (second >> 8)) as u16 as usize] as u64;
    down += (((second << 44) + (second << 32) + (second << 20) + (second << 8)) & 1080880403494997760);
    
    let mut third: u64 = int_rep & 67555025218437360; // 0000000011110000000000001111000000000000111100000000000011110000
    third = dr_table[((third >> 40) + (third >> 28) + (third >> 16) + (third >> 4)) as u16 as usize] as u64;
    down += (((third << 40) + (third << 28) + (third << 16) + (third << 4)) & 67555025218437360);
    
    let mut fourth: u64 = int_rep & 4222189076152335; // 0000000000001111000000000000111100000000000011110000000000001111
    fourth = dr_table[((fourth >> 36) + (fourth >> 24) + (fourth >> 12) + (fourth >> 0)) as u16 as usize] as u64;
    down += (((fourth << 36) + (fourth << 24) + (fourth << 12) + (fourth << 0)) & 4222189076152335);
    
    return down;
}