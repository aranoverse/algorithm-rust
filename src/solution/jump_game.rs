use std::cmp::max;
use crate::solution::Solution;

impl Solution {
    /// https://leetcode.com/problems/jump-game/
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut current_position = 0;
        let mut left_steps = 0;

        while current_position < nums.len() {
            left_steps = max(nums[current_position], left_steps - 1);

            current_position += 1;

            if left_steps == 0 {
                break;
            }
        }

        current_position == nums.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use test_case::test_case;

    #[test_case([2, 3, 1, 1, 4].to_vec(), true; "case 1")]
    #[test_case([1].to_vec(), true; "case 2")]
    #[test_case([1, 2, 3].to_vec(), true; "case 3")]
    fn test_can_jump(nums: Vec<i32>, can_jump: bool) {
        assert_eq!(Solution::can_jump(nums), can_jump);
    }
}