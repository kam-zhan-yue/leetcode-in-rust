use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn rot(
        r: usize,
        c: usize,
        fresh: &mut i32,
        queue: &mut VecDeque<(usize, usize)>,
        grid: &mut Vec<Vec<i32>>,
    ) {
        if r >= grid.len() || c >= grid[0].len() {
            return;
        }
        // Rot the orange and add it into the queue
        if grid[r][c] == 1 {
            grid[r][c] = 2;
            queue.push_back((r, c));
            *fresh -= 1;
        }
    }

    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut queue = VecDeque::new();
        let mut fresh: i32 = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 2 {
                    queue.push_back((r, c));
                } else if grid[r][c] == 1 {
                    fresh += 1;
                }
            }
        }

        let mut minutes: i32 = 0;
        while fresh > 0 && queue.len() > 0 {
            for _ in 0..queue.len() {
                if let Some((r, c)) = queue.pop_front() {
                    Solution::rot(r + 1, c, &mut fresh, &mut queue, &mut grid);
                    if r >= 1 {
                        Solution::rot(r - 1, c, &mut fresh, &mut queue, &mut grid);
                    }
                    Solution::rot(r, c + 1, &mut fresh, &mut queue, &mut grid);
                    if c >= 1 {
                        Solution::rot(r, c - 1, &mut fresh, &mut queue, &mut grid);
                    }
                }
            }
            minutes += 1;
        }

        if fresh == 0 {
            return minutes;
        } else {
            return -1;
        }
    }
}

fn main() {
    let valid_grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    let invalid_grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
    println!("{:?}", Solution::oranges_rotting(valid_grid));
    println!("{:?}", Solution::oranges_rotting(invalid_grid));
}
