use std::collections::HashSet;

fn main() {}

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut row_set = HashSet::new();
            let mut column_set = HashSet::new();
            for j in 0..9 {
                if board[i][j] != '.' && !row_set.insert(board[i][j]) {
                    return false;
                }

                if board[j][i] != '.' && !column_set.insert(board[j][i]) {
                    return false;
                }
            }
        }

        let mut blocks = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                blocks.push((
                    i * 3 .. (i + 1) * 3,
                    j * 3 .. (j + 1) * 3,
                ))
            }
        }

        for (row_range, column_range) in blocks {
            let mut block_set = HashSet::new();
            for i in row_range {
                for j in column_range.clone() {
                    if board[i][j] != '.' && !block_set.insert(board[i][j]) {
                        return false;
                    }
                }
            }
        }
        
        true
    }
}