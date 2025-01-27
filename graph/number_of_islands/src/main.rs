struct Solution;

impl Solution {
    fn dfs(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
        // Check if out of bounds
        if row >= grid.len() || col >= grid[0].len() {
            return;
        }
        if grid[row][col] == '0' {
            return;
        }
        // Update the current cell
        grid[row][col] = '0';
        Solution::dfs(grid, row + 1, col);
        Solution::dfs(grid, row, col + 1);
        if row >= 1 {
            Solution::dfs(grid, row - 1, col);
        }
        if col >= 1 {
            Solution::dfs(grid, row, col - 1);
        }
    }
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut islands: i32 = 0;

        let rows = grid.len();
        let cols = grid[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    islands += 1;
                    Solution::dfs(&mut grid, i, j);
                }
            }
        }
        islands
    }
}

fn main() {
    let grid_1 = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];

    let grid_2 = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    println!("{:?}", Solution::num_islands(grid_1));
    println!("{:?}", Solution::num_islands(grid_2));
}
