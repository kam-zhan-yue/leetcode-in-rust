use std::cmp::{max, min};

struct Solution;

impl Solution {
    fn rob_normal(nums: &[i32]) -> i32 {
        let mut rob_1: i32 = 0;
        let mut rob_2: i32 = 0;
        for i in (0..nums.len()).rev() {
            let max_rob = max(rob_1 + nums[i], rob_2);
            rob_1 = rob_2;
            rob_2 = max_rob;
        }
        rob_2
    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        min(
            Solution::rob_normal(&nums[..nums.len() - 1]),
            Solution::rob_normal(&nums[1..nums.len()]),
        )
    }
}

fn main() {
    println!("{:?}", Solution::rob(vec![2, 3, 2]));
    println!("{:?}", Solution::rob(vec![1, 2, 3, 1]));
}
