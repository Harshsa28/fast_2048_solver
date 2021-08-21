#![allow(dead_code)]
#![allow(unused_variables)]

use std::convert::TryInto;


fn log_i32 (num : i32) -> u64 {
    let log : u64 = (32 - num.leading_zeros() - 1).try_into().unwrap();
    return log;
}

fn board_to_int (board : &[[i32; 4]; 4]) -> u64 {
    let mut int_rep : u64 = 0;
    let base : u64 = 2;
    
    for (i, &i_item) in board.iter().enumerate() {
        for (j , &j_item) in i_item.iter().enumerate() {
            if board[i][j] != 0 {
                let power : u32 = (60 - 16*i - 4*j).try_into().unwrap();
                int_rep = int_rep + base.pow(power)*log_i32(board[i][j]);
            }
        }
    }
    
    return int_rep;
}



fn main() {
    
    let board : [[i32; 4]; 4] = [[2,8,4,2],
                                [0,0,8,16],
                                [0,0,0,0],
                                [0,0,0,2]];
    
    let int_rep : u64 = board_to_int(&board);
    
    println!("rep is {}", int_rep);
    
    
    
    
}
