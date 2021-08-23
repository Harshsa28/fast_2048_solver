#![allow(warnings, unused)]

mod movement;
use movement::{right, left, up, down, row_right, right_left_16b};

mod convert;
use convert::{int_to_row, row_to_int, board_to_int, int_to_board};

mod movement_table;
use movement_table::{make_dr_table, make_ul_table};

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
    
    let board: [[u64; 4]; 4] = [[8,8,8,8],
                                [8,2,2,8],
                                [4,8,4,4],
                                [4,0,0,0]];
    
    let mut int_rep: u64 = board_to_int(&board);
    let board = int_to_board(int_rep);
    
    let mut dr_table: [u16; 65536] = [0; 65536];
    make_dr_table(&mut dr_table);
    
    // println!("dr_table is: {:?}", dr_table);
    
    // let mut ul_table: [u16; 65536] = [0; 65536];
    // make_ul_table(&mut ul_table);
    
    
    // println!("initial board is :", );
    // pretty_print(&board);
    
    // loop {
    //     let stdin = stdin();
    //     for c in stdin.keys() {
    //         match c.unwrap() {
    //             Key::Right => {
    //                 int_rep = right(int_rep);
    //             },
    //             Key::Left => {
    //                 int_rep = left(int_rep);
    //             },
    //             Key::Up => {
    //                 int_rep = up(int_rep);
    //             },
    //             Key::Down => {
    //                 int_rep = down(int_rep);
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
