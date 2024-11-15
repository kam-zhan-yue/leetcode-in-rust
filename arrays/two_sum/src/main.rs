use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            let remainder = target - num;
            if let Some(value) = hash.get(&remainder) {
                return vec![*value, i as i32];
            }
            hash.insert(num, i as i32);
        }
        return Vec::new();
    }
}

pub fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result); // Output should be [0, 1]
}
