struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // DP array to store the longest length of the subsequence at the index
        let mut dp: Vec<i32> = vec![1; nums.len()];
        // Start from the back
        for i in (0..nums.len()).rev() {
            for j in i..nums.len() {
                if nums[i] < nums[j] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18])
    );
    println!("{:?}", Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));
    println!("{:?}", Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7]));
}
