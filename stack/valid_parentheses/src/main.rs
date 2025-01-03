use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let map: HashMap<char, char> = HashMap::from([
            ('(', ')'),
            ('[', ']'),
            ('{', '}'),
        ]);
        let mut stack = Vec::new();
        for c in s.chars() {
            if map.contains_key(&c) {
                stack.push(map[&c]);
            } else if stack.pop() != Some(c) {
                return false
            }
        }
        stack.is_empty()
    }
}

pub fn main() {
    let s = String::from("()");
    let result = Solution::is_valid(s);
    println!("{}", result);
}
