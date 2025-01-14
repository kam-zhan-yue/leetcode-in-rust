struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        // Need a 2-dimensional dp array to represent the palindrome
        let n = s.len();
        let mut dp: Vec<Vec<bool>> = vec![vec![false; n]; n];
        let mut substrings: i32 = 0;
        let str = s.as_bytes();
        for left in (0..s.len()).rev() {
            for right in left..s.len() {
                if str[left] == str[right] {
                    if (right - left) < 2 || dp[left + 1][right - 1] {
                        substrings += 1;
                        dp[left][right] = true
                    }
                }
            }
        }
        substrings
    }
}

fn main() {
    println!("{:?}", Solution::count_substrings("abc".to_string()));
    println!("{:?}", Solution::count_substrings("aaa".to_string()));
}
