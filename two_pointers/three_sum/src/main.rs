struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums: Vec<i32> = nums.clone();
        nums.sort();
        let mut res: Vec<Vec<i32>> = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    // Keep increasing left until it is not a duplicate
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                }
            }
        }
        res
    }
}

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
}
