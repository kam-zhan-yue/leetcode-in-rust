struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left: usize = 0;
        let mut right: usize = numbers.len() - 1;
        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum == target {
                return vec![left as i32 + 1, right as i32 + 1];
            } else if sum > target {
                right -= 1;
            } else {
                left += 1;
            }
        }
        return vec![-1, -1]
    }
}

fn main() {
    let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", result);
}
