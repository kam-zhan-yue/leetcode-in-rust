struct Solution;

impl Solution {
    fn is_palindrome(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        for i in 0..len / 2 {
            if chars[i] != chars[len - 1 - i] {
                return false;
            }
        }
        return true;
    }

    fn dfs(s: &String, start: usize, part: &mut Vec<String>, partitions: &mut Vec<Vec<String>>) {
        if start == s.len() {
            partitions.push(part.clone());
            return;
        }

        for end in start + 1..=s.len() {
            if Solution::is_palindrome(&s[start..end]) {
                part.push(s[start..end].to_string());
                Solution::dfs(s, end, part, partitions);
                part.pop();
            }
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = Vec::new();
        let mut part: Vec<String> = Vec::new();

        Solution::dfs(&s, 0, &mut part, &mut res);
        res
    }
}

fn main() {
    println!("{:?}", Solution::partition("aab".to_string()));
}
