use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // Store a hash set of all of the numbers
        let mut map: HashSet<i32> = HashSet::new();
        for &num in nums.iter() {
            map.insert(num);
        }
        let mut res: i32 = 0;
        for num in &map {
            if map.contains(&(num - 1)) {
                continue;
            }
            let mut sequence: i32 = 1;
            while map.contains(&(num + sequence)) {
                sequence += 1;
            }
            if sequence > res {
                res = sequence;
            }
        }
        res
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
    );
}
