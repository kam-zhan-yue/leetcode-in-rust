use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for &num in nums.iter() {
            map.entry(num).and_modify(|val| *val += 1).or_insert(1);
        }

        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len()];
        for (num, count) in &map {
            buckets[(count - 1) as usize].push(*num);
        }

        let mut res = Vec::new();
        for i in (0..nums.len()).rev() {
            if buckets[i].len() > 0 {
                for &num in buckets[i].iter() {
                    res.push(num);
                    if res.len() >= k as usize {
                        return res;
                    }
                }
            }
        }

        res
    }
}

fn main() {
    println!("{:?}", Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2));
    println!("{:?}", Solution::top_k_frequent(vec![1], 1));
}
