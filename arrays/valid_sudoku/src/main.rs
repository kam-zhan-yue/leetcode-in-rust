use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: HashMap<i32, HashSet<char>> = HashMap::new();
        let mut cols: HashMap<i32, HashSet<char>> = HashMap::new();
        let mut squares: HashMap<(i32, i32), HashSet<char>> = HashMap::new();
        for r in 0..9 {
            for c in 0..9 {
                let cell = board[r][c];
                if cell == '.' {
                    continue;
                }
                if let Some(set) = rows.get(&(r as i32)) {
                    if set.contains(&cell) {
                        return false;
                    }
                }
                if let Some(set) = cols.get(&(c as i32)) {
                    if set.contains(&cell) {
                        return false;
                    }
                }
                if let Some(set) = squares.get(&(r as i32 / 3i32, c as i32 / 3i32)) {
                    if set.contains(&cell) {
                        return false;
                    }
                }
                rows.entry(r as i32)
                    .or_insert_with(HashSet::new)
                    .insert(cell);
                cols.entry(c as i32)
                    .or_insert_with(HashSet::new)
                    .insert(cell);
                squares
                    .entry((r as i32 / 3i32, c as i32 / 3i32))
                    .or_insert_with(HashSet::new)
                    .insert(cell);
            }
        }
        return true;
    }
}

fn main() {
    let valid_board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let invalid_board = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    println!(
        "Valid board is valid: {}",
        Solution::is_valid_sudoku(valid_board)
    );
    println!(
        "Invalid board is valid: {}",
        Solution::is_valid_sudoku(invalid_board)
    );
}
