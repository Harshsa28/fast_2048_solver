#![allow(warnings, unused)]

mod movement;
use movement::{right, left, up, down, row_right};

mod convert;
use convert::{int_to_row, row_to_int, board_to_int, int_to_board};


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
    
    println!("initial board is :", );
    pretty_print(&board);
    
    
    
    // println!("printing table");
    // table.printstd();
    // println!("printed table");
    
    
    
    loop {
        // println!("in loop");
        let stdin = stdin();
        for c in stdin.keys() {
            // println!("c is {:?}", c);
            match c.unwrap() {
                Key::Right => {
                    int_rep = right(int_rep);
                },
                Key::Left => {
                    int_rep = left(int_rep);
                },
                Key::Up => {
                    int_rep = up(int_rep);
                },
                Key::Down => {
                    int_rep = down(int_rep);
                },
                _ => {
                    println!("updated board is :");
                    let board = int_to_board(int_rep);
                    pretty_print(&board);
                },
            }
        }
    }
    
    
    
    // println!("int_rep u64 is {}", int_rep);
    // println!("original board is {:?}", int_to_board(int_rep));
    
    // int_rep = up(int_rep);
    
    // println!("int_rep up is {}", int_rep);
    // println!("up board is {:?}", int_to_board(int_rep));
    
    // int_rep = left(int_rep);
    
    // println!("int_rep left is {}", int_rep);
    // println!("left board is {:?}", int_to_board(int_rep));
    
    // int_rep = right(int_rep);
    
    // println!("int_rep right is {}", int_rep);
    // println!("right board is {:?}", int_to_board(int_rep));
    
    
    // println!("int_rep bytes is {:b}", int_rep);
    
    // let first : u16 = (int_rep >> 48) as u16;
    // let second : u16 = (int_rep >> 32) as u16;
    // let third : u16 = (int_rep >> 16) as u16;
    // let fourth : u16 = int_rep as u16;
    
    // // println!("u16 first is {:b}", first);
    // // println!("u16 second is {:b}", second);
    // // println!("u16 third is {:b}", third);
    // // println!("u16 fourth is {:b}", fourth);
    
    // // row_right(first);
    // // row_right(second);
    // // row_right(third);
    // // row_right(fourth);
    
    
    
    // let mut new_board: [[u64; 4]; 4] = [[0; 4]; 4];
    
    // new_board[0] = int_to_row(first);
    // new_board[1] = int_to_row(second);
    // new_board[2] = int_to_row(third);
    // new_board[3] = int_to_row(fourth);
    
    // println!("original board is {:?}", board);
    // println!("new board is {:?}", new_board);
    
    
    // let board = int_to_board(int_rep);
    
    
}
