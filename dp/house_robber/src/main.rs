struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut rob_1: i32 = 0;
        let mut rob_2: i32 = 0;
        for i in (0..nums.len()).rev() {
            // Get the maximum of robbing or not robbing
            let max_rob = std::cmp::max(rob_1 + nums[i], rob_2);
            rob_1 = rob_2;
            rob_2 = max_rob;
        }
        rob_2
    }
}

fn main() {
    println!("{:?}", Solution::rob(vec![1, 2, 3, 1]));
    println!("{:?}", Solution::rob(vec![2, 7, 9, 3, 1 ]));
}
