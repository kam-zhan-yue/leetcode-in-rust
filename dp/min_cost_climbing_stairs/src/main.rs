use std::cmp;
use std::collections::HashMap;

struct Solution;

impl Solution {
    fn dfs(cost: &Vec<i32>, memo: &mut HashMap<i32, i32>, i: i32) -> i32 {
        if i >= cost.len() as i32 {
            return 0;
        }
        if let Some(&value) = memo.get(&i) {
            return value;
        }
        let result =
            cost[i as usize] + cmp::min(Self::dfs(cost, memo, i + 1), Self::dfs(cost, memo, i + 2));
        memo.insert(i, result);
        result
    }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // At each step, you can choose to climb or take the next
        let mut memo: HashMap<i32, i32> = HashMap::new();
        return cmp::min(
            Self::dfs(&cost, &mut memo, 0),
            Self::dfs(&cost, &mut memo, 1),
        );
    }
}

pub fn main() {
    let cost = vec![10, 15, 20];
    println!("{:?}", Solution::min_cost_climbing_stairs(cost));
}
