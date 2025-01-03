use std::collections::HashMap;

struct Solution;

impl Solution {
    fn dfs(i: i32, n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if i > n {
            return 0;
        }
        else if i == n {
            return 1;
        }
        if memo.contains_key(&i) {
            return memo[&i];
        }
        let result = Self::dfs(i + 1, n, memo) + Self::dfs(i + 2, n, memo);
        memo.insert(i, result);
        result
    }
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        return Self::dfs(0, n, &mut memo)
    }
}

pub fn main() {
    let n = 3;
    let result = Solution::climb_stairs(n);
    println!("{}", result);
}
