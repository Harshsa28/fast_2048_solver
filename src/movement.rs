// mod convert;
// use convert::{row_to_int, int_to_row};

use super::{int_to_row, row_to_int};

use std::convert::TryInto;


pub fn row_right(mut board_row: &mut [u64; 4]){
    // let mut board_row: [u64; 4] = int_to_row(x);
    // println!("initial board_row is {:?}", board_row);
    let mut cmp: u64 = 0;
    let mut free_spot = 3;
    let mut match_spot = 4;
    for i in (0..4).rev() {
        if board_row[i] != 0 {
            if board_row[i] == cmp {
                board_row[match_spot] = 2*cmp;
                match_spot = 4;
                board_row[i] = 0;
                cmp = 0;
            } else {
                cmp = board_row[i];
                if free_spot > i {
                    board_row[free_spot] = board_row[i];
                    board_row[i] = 0;
                }
                match_spot = free_spot;
                if free_spot > 0{
                    free_spot -= 1;
                }
            }
        }
    }
    // println!("final board_row is {:?}", board_row);
    // return row_to_int(&board_row);
    // return board_row;
}

// true if right, false if left
fn right_left(int_rep: u64, right_or_left: bool) -> u64 {
    let mut right_left: u64 = 0;
    let base: u64 = 2;
    
    for i in 0..4 {
        let part: u16 = (int_rep >> (16*(3-i))) as u16;
        let power: u32 = (16*(3-i)).try_into().unwrap();
        
        let mut row: [u64; 4] = int_to_row(part);
        if !right_or_left {
            row.reverse();
        }
        row_right(&mut row);
        if !right_or_left {
            row.reverse();
        }
        let row_int: u64 = row_to_int(&row) as u64;
        
        right_left += base.pow(power)*(row_int);
    }
    
    return right_left;
}

pub fn right(int_rep : u64) -> u64 {
    return right_left(int_rep, true);
}

pub fn left(int_rep : u64) -> u64 {
    return right_left(int_rep, false);
}

// true if up, false if down
fn up_down(int_rep: u64, up_or_down: bool) -> u64 {
    let mut up_down: u64 = 0;
    let base: u64 = 2;
    
    for i in 0..4 {
        let mut part: u16 = 0;
        
        for j in 0..4 {
            let power: u32 = (12-(4*j)).try_into().unwrap();
            let exp: u16 = (base as u16).pow(power);
            
            let bit_shift_by: u16 = (60-(4*i)-(16*j)).try_into().unwrap();
            let bits4: u16 = ((int_rep >> bit_shift_by) & 15u64) as u16;
            
            part += exp*bits4;
        }
        
        let mut row: [u64; 4] = int_to_row(part);
        if up_or_down {
            row.reverse();
        }
        row_right(&mut row);
        if up_or_down {
            row.reverse();
        }
        let mut row_int: u16 = row_to_int(&row);
        
        for j in 0..4 {
            let power: u32 = (60-(4*i)-(16*j)).try_into().unwrap();
            let exp: u64 = base.pow(power);
            
            let bit_shift_by: u16 = (12-(4*j)).try_into().unwrap();
            let bits4: u64 = ((row_int >> bit_shift_by) & 15u16) as u64;
            
            up_down += exp*bits4;
        }
    }
    
    return up_down;
}


pub fn up(int_rep : u64) -> u64 {
    return up_down(int_rep, true);
}

pub fn down(int_rep : u64) -> u64 {
    return up_down(int_rep, false);
}

