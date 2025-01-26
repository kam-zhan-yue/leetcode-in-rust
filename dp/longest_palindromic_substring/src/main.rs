struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut res_index: usize = 0;
        let mut res_len: usize = 0;
        let len = s.len();
        let mut dp = vec![vec![false; len]; len];
        let str = s.as_bytes();
        for left in (0..len).rev() {
            for right in left..len {
                if str[left] == str[right] {
                    if (right - left) <= 2 as usize || dp[left + 1][right - 1] {
                        dp[left][right] = true;
                        let palindromic_len = right - left + 1;
                        if (palindromic_len) > res_len {
                            res_index = left;
                            res_len = palindromic_len;
                        }
                    }
                }
            }
        }
        s[res_index..res_index + res_len].to_string()
    }
}

fn main() {
    println!("{:?}", Solution::longest_palindrome(String::from("babad")));
    println!("{:?}", Solution::longest_palindrome(String::from("cbbd")));
}
