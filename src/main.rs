#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::convert::TryInto;


fn log_i32 (num: u64) -> u64 {
    let log: u64 = (64 - num.leading_zeros() - 1).try_into().unwrap();
    return log;
}

fn board_to_int (board: &[[u64; 4]; 4]) -> u64 {
    let mut int_rep: u64 = 0;
    let base: u64 = 2;
    
    for (i, &i_item) in board.iter().enumerate() {
        for (j , &j_item) in i_item.iter().enumerate() {
            if j_item != 0 {
                let power: u32 = (60 - 16*i - 4*j).try_into().unwrap();
                int_rep = int_rep + base.pow(power)*log_i32(j_item);
            }
        }
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
    
    let board: [[u64; 4]; 4] = [[2,8,4,2],
                                [0,0,8,16],
                                [0,0,0,0],
                                [0,0,0,2]];
    
    let int_rep: u64 = board_to_int(&board);
    
    println!("rep is {}", int_rep);
    
    let board = int_to_board(int_rep);
    
    
}
