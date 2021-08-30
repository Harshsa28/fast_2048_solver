#![allow(warnings, unused)]

mod movement;
use movement::{right, left, up, down, row_right, right_left_16b};

mod convert;
use convert::{int_to_row, row_to_int, board_to_int, int_to_board, log};

mod movement_table;
use movement_table::{make_dr_table, make_ul_table};

mod fast_movement;
use fast_movement::{fast_right, fast_left, fast_up, fast_down};

use std::io::{stdin};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
#[macro_use] extern crate prettytable;


fn pretty_print (board: &[[u64; 4]; 4]) {
    let table = ptable!([board[0][0], board[0][1], board[0][2], board[0][3]],
                              [board[1][0], board[1][1], board[1][2], board[1][3]],
                              [board[2][0], board[2][1], board[2][2], board[2][3]],
                              [board[3][0], board[3][1], board[3][2], board[3][3]]);
}



fn main() {
    
    let board: [[u64; 4]; 4] = [[8,2,4,8],
                                [8,2,2,8],
                                [4,8,4,4],
                                [4,0,0,0]];
    
    let mut int_rep: u64 = board_to_int(&board);
    let mut board = int_to_board(int_rep);
    // println!("initial board is :");
    // pretty_print(&board);
    
    let mut dr_table: [u16; 65536] = [0; 65536];
    make_dr_table(&mut dr_table);
    
    
    
    
    // println!("dr_table is: {:?}", dr_table);
    
    let mut ul_table: [u16; 65536] = [0; 65536];
    make_ul_table(&mut ul_table);
    
    for i in 0..100_000_000 {
        fast_up(int_rep, &ul_table);
    }
    
    // println!("initial board is :");
    // pretty_print(&board);
    
    // fast_up(int_rep, &ul_table);
    
    // println!("up board is :");
    // pretty_print(&board);
    
    
    // let mut num: u16 = 65535;
    // println!("num before is {:b}", num);
    // num = num << 8;
    // println!("num after is {:b}", num);
    
    
    
    
    // for i in 0..4 {
    //     let num: u16 = row_to_int(&board[i]);
    //     let left = ul_table[num as usize];
    //     board[i] = int_to_row(left);
    // }
    
    // println!("left board is :");
    // pretty_print(&board);
    
    
    // println!("initial board is :", );
    // pretty_print(&board);
    
    // loop {
    //     let stdin = stdin();
    //     for c in stdin.keys() {
    //         match c.unwrap() {
    //             Key::Right => {
    //                 int_rep = fast_right(int_rep, &dr_table);
    //             },
    //             Key::Left => {
    //                 int_rep = fast_left(int_rep, &ul_table);
    //             },
    //             Key::Up => {
    //                 int_rep = fast_up(int_rep, &ul_table);
    //             },
    //             Key::Down => {
    //                 int_rep = fast_down(int_rep, &dr_table);
    //             },
    //             _ => {
    //                 println!("updated board is :");
    //                 let board = int_to_board(int_rep);
    //                 pretty_print(&board);
    //             },
    //         }
    //     }
    // }
}
