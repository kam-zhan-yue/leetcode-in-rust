struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let n = s.len();
        let mut dp = vec![0; n + 1];
        dp[n] = 1;

        let mut chars = s.chars();
        for (i, c) in chars.clone().rev().enumerate() {
            if c == '0' {
                dp[i] = 0;
            } else {
                dp[i] = dp[i + 1];
                if i + 1 < n {
                    let i = i as usize;
                    if chars.nth(i).unwrap() == '1' || (chars.nth(i).unwrap() == '2' && String::from("0123456").contains(chars.nth(i + 1).unwrap())) {
                        dp[i] += dp[i + 2];
                    }
                }
            }
        }

        return dp[0];
    }
}
fn main() {
    println!("{:?}", Solution::num_decodings(String::from("12")));
    println!("{:?}", Solution::num_decodings(String::from("226")));
}
