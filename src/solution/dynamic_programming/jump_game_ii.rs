use std::ops::Add;
use std::process::id;
use crate::solution::Solution;

impl Solution {
    // https://leetcode.com/problems/jump-game-ii/
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut min_steps = vec![0 as i32; nums.len()];

        for idx in 1..nums.len() {
            // Initialize max values
            min_steps[idx] = idx as i32;
            for before_idx in 0..idx {
                // nums[before_idx] means the gap from before_idx to idx ,
                // If it can reach the index, compare the  min_steps[idx] and (number[before_idx] + 1) .
                if idx - before_idx <= nums[before_idx] as usize {
                    min_steps[idx] = min_steps[idx].min(min_steps[before_idx] + 1)
                }
            }
        }

        min_steps[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use test_case::test_case;

    #[test_case([2, 3, 1, 1, 4].to_vec(), 2; "case 1")]
    #[test_case([2, 3, 1, 1, 4, 0, 1, 3].to_vec(), 3; "case 2")]
    fn test_can_jump(nums: Vec<i32>, min_step: i32) {
        assert_eq!(Solution::jump(nums), min_step);
    }
}