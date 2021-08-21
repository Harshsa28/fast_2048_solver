#![allow(warnings, unused)]

mod movement;
use movement::{left, right, up, down, row_to_int, int_to_row, row_right};


use std::convert::TryInto;


// fn log_i32 (num: u64) -> u64 {
//     let log: u64 = (64 - num.leading_zeros() - 1).try_into().unwrap();
//     return log;
// }

// fn board_to_int (board: &[[u64; 4]; 4]) -> u64 {
//     let mut int_rep: u64 = 0;
//     let base: u64 = 2;
    
//     for (i, &i_item) in board.iter().enumerate() {
//         for (j , &j_item) in i_item.iter().enumerate() {
//             if j_item != 0 {
//                 let power: u32 = (60 - 16*i - 4*j).try_into().unwrap();
//                 int_rep = int_rep + base.pow(power)*log_i32(j_item);
//             }
//         }
//     }
    
//     return int_rep;
// }

fn board_to_int (board: &[[u64; 4]; 4]) -> u64 {
    let mut int_rep: u64 = 0;
    let base: u64 = 2;
    
    for (i, i_item) in board.iter().enumerate() {
        let power: u32 = (16*(3-i)).try_into().unwrap();
        int_rep += base.pow(power)*(row_to_int(i_item) as u64);
    }
    
    return int_rep;
}



fn int_to_board (mut int_rep: u64) -> [[u64; 4]; 4] {
    let mut board: [[u64; 4]; 4] = [[0; 4]; 4];
    let base: u64 = 2;
    for i in 0..4 {
        for j in 0..4 {
            let power: u32 = (60 - 16*i - 4*j).try_into().unwrap();
            let factor: u64 = base.pow(power);
            let quotient: u32 = (int_rep / factor).try_into().unwrap();
            let remainder: u64 = int_rep % factor;
            
            if quotient == 0 {
                board[i][j] = 0;
            }
            else {
                board[i][j] = base.pow(quotient);
            }
            println!("i is {} and j is {} and board[i][j] is {}", i, j, board[i][j]);
            int_rep = remainder;
        }
    }
    
    return board;
}

fn main() {
    
    let board: [[u64; 4]; 4] = [[8,8,8,8],
                                [2,2,2,8],
                                [8,8,4,4],
                                [0,0,0,0]];
    
    let mut int_rep: u64 = board_to_int(&board);
    
    // println!("int_rep u64 is {}", int_rep);
    
    // println!("int_rep bytes is {:b}", int_rep);
    
    let first : u16 = (int_rep >> 48) as u16;
    let second : u16 = (int_rep >> 32) as u16;
    let third : u16 = (int_rep >> 16) as u16;
    let fourth : u16 = int_rep as u16;
    
    // println!("u16 first is {:b}", first);
    // println!("u16 second is {:b}", second);
    // println!("u16 third is {:b}", third);
    // println!("u16 fourth is {:b}", fourth);
    
    // row_right(first);
    // row_right(second);
    // row_right(third);
    // row_right(fourth);
    
    
    
    let mut new_board: [[u64; 4]; 4] = [[0; 4]; 4];
    
    new_board[0] = int_to_row(first);
    new_board[1] = int_to_row(second);
    new_board[2] = int_to_row(third);
    new_board[3] = int_to_row(fourth);
    
    println!("original board is {:?}", board);
    println!("new board is {:?}", new_board);
    
    
    // let board = int_to_board(int_rep);
    
    
}
