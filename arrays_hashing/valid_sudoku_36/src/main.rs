use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_set = HashSet::new();
        let mut col_set = HashSet::new();
        let mut sub_box_sets = vec![HashSet::new(); 9];
        for row_index in 0..board.len() {
            for column_index in 0..board.len() {
                if let Some(digit) = board[row_index][column_index].to_digit(10) {
                    if !row_set.insert((row_index, digit)) {
                        return false;
                    }
                    if !col_set.insert((column_index, digit)) {
                        return false;
                    }
                    //Sub box, we do 3 * for the row because in order to jump to another box.
                    if !sub_box_sets[3 * (row_index / 3) + column_index / 3].insert(digit) {
                        return false;
                    }
                }
            }
        }
        return true;
    }
    //using bits
    pub fn is_valid_sudoku_better(board: Vec<Vec<char>>) -> bool {
        let mut cols: [u16; 9] = [0; 9];
        let mut boxes: [u16; 9] = [0; 9];

        for r in 0..9 {
            let mut rows: [u16; 9] = [0; 9];
            for c in 0..9 {
                match board[r][c] {
                    '.' => continue,
                    char => {
                        let b: usize = (r / 3) * 3 + (c / 3);
                        let curr = 1 << (char.to_digit(10).unwrap());
                        //Bitwise and
                        if rows[r] & curr != 0 || cols[c] & curr != 0 || boxes[b] & curr != 0 {
                            return false;
                        }
                        //Bitwise or
                        rows[r] |= curr;
                        cols[c] |= curr;
                        boxes[b] |= curr;
                    }
                }
            }
        }

        return true;
    }
}

fn main() {
    dbg!(Solution::is_valid_sudoku(vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ]));
    dbg!(Solution::is_valid_sudoku_better(vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ]));
}
