use std::convert::TryInto;

fn log (num: u64) -> u16 {
    let log: u16 = (64 - num.leading_zeros() - 1).try_into().unwrap();
    return log;
}



pub fn row_to_int (row: &[u64; 4]) -> u16 {
    let mut int_rep: u16 = 0;
    let base: u16 = 2;
    
    for (i, &i_item) in row.iter().enumerate() {
        if i_item != 0 {
            let power: u32 = (12 - 4*i).try_into().unwrap();
            int_rep = int_rep + base.pow(power)*log(i_item);
        }
    }
    
    return int_rep;
}




pub fn int_to_row (mut int_rep: u16) -> [u64; 4] {
    let mut row: [u64; 4] = [0; 4];
    let base: u64 = 2;
    for i in 0..4 {
        let power: u32 = (12 - 4*i).try_into().unwrap();
        let factor: u16 = base.pow(power).try_into().unwrap();
        let quotient: u16 = int_rep / factor;
        let remainder: u16 = int_rep % factor;
        
        if quotient == 0 {
            row[i] = 0;
        }
        else {
            row[i] = base.pow(quotient.into());
        }
        // println!("i is {} and board[i] is {}", i, board[i]);
        int_rep = remainder;
    }
    
    return row;
}




pub fn row_right(x : u16) -> u16 {
    let mut board_row: [u64; 4] = int_to_row(x);
    println!("initial board_row is {:?}", board_row);
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
    println!("final board_row is {:?}", board_row);
    x
}


pub fn left(x : u64) -> u64 {
    
    
    x
}



pub fn right(x : u64) -> u64 {
    x
}


pub fn up(x : u64) -> u64 {
    x
}

pub fn down(x : u64) -> u64 {
    
    x
}


fn main () {
    
}