use crate::solution::Solution;

impl Solution {
    /// https://leetcode.com/problems/house-robber/
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        if nums.len() == 1 {
            return nums[0];
        }

        let mut max_robbed = Vec::with_capacity(nums.len());
        max_robbed.push(nums[0]);
        max_robbed.push(nums[0].max(nums[1]));

        for idx in 2..nums.len() {
            // formula: max_robbed[i] = max(max_robbed[i-1] + nums[i], max_robbed[i-2])
            max_robbed.insert(idx, max_robbed[idx - 1].max(max_robbed[idx - 2] + nums[idx]));
        }

        max_robbed[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use test_case::test_case;

    #[test_case([1, 0, 0, 6, 1, 2].to_vec(), 9; "case 3")]
    #[test_case([1, 2, 3, 1].to_vec(), 4; "case 2")]
    #[test_case([2, 7, 9, 3, 1].to_vec(), 12; "case 1")]
    fn test_rob(nums: Vec<i32>, max_robbed: i32) {
        assert_eq!(Solution::rob(nums), max_robbed);
    }
}