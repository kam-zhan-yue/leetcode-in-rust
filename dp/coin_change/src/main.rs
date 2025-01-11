use std::cmp;

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // Create an array that holds the minimum number of coins to
        // get to the value (index)
        let mut dp: Vec<i32> = vec![i32::MAX; (amount + 1) as usize];
        // We know that it takes 0 coins to get 0
        dp[0] = 0;
        for i in 1..amount + 1 {
            let a = i as usize;
            for &coin in coins.iter() {
                // If the coin is valid
                if i - coin >= 0 {
                    let i = a - coin as usize;
                    if dp[i] != i32::MAX {
                        dp[a] = cmp::min(dp[a], 1 + dp[i]);
                    }
                }
            }
        }
        if dp[amount as usize] == i32::MAX {
            return -1;
        }
        dp[amount as usize]
    }
}

fn main() {
    // println!("{:?}", Solution::coin_change(vec![1, 2, 5], 11));
    println!("{:?}", Solution::coin_change(vec![2], 3));
}
