struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix: Vec<i32> = vec![1; nums.len()];
        let mut suffix: Vec<i32> = vec![1; nums.len()];
        let mut res: Vec<i32> = vec![1; nums.len()];
        for i in 1..nums.len() {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }
        for i in (0..nums.len() - 1).rev() {
            suffix[i] = suffix[i + 1] * nums[i + 1];
        }

        for i in 0..nums.len() {
            res[i] = prefix[i] * suffix[i];
        }
        res
    }
}

fn main() {
    println!("{:?}", Solution::product_except_self(vec![1, 2, 3, 4]));
    println!("{:?}", Solution::product_except_self(vec![-1, 1, 0, -3, 3]));
}
