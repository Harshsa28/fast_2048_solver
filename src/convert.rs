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
            if row[i] == 65536u64 {
                println!("int_to_row generated 65536, row[i] is {} and int_rep is {} and i is {} and power is {} and factor is {} and quotient is {} and remainder is {}", row[i], int_rep, i, power, factor, quotient, remainder);
            }
        }
        int_rep = remainder;
    }
    
    return row;
}


pub fn board_to_int (board: &[[u64; 4]; 4]) -> u64 {
    let mut int_rep: u64 = 0;
    let base: u64 = 2;
    
    for (i, i_item) in board.iter().enumerate() {
        let power: u32 = (16*(3-i)).try_into().unwrap();
        int_rep += base.pow(power)*(row_to_int(i_item) as u64);
    }
    
    return int_rep;
}



pub fn int_to_board (mut int_rep: u64) -> [[u64; 4]; 4] {
    let mut board: [[u64; 4]; 4] = [[0; 4]; 4];
    
    for i in 0..4 {
        let part: u16 = (int_rep >> (16*(3-i))) as u16;
        board[i] = int_to_row(part);
    }
    
    return board;
}