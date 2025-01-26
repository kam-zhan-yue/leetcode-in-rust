struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // Create a filtered strign with no whitespace and alphanumeric characters
        let filtered: String = s
            .chars()
            .into_iter()
            .filter(|c| !!c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();
        // Check whether the reverse of the filtered string is the same
        filtered == filtered.chars().rev().collect::<String>()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::is_palindrome(String::from("A man, a plan, a canal: Panama"))
    );
    println!("{:?}", Solution::is_palindrome(String::from("race a car")));
}
