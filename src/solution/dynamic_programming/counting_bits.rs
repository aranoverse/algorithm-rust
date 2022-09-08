use std::ops::Shl;
use crate::solution::Solution;

impl Solution {
    /// https://leetcode.com/problems/counting-bits/
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut bit_nums = vec![0; (n + 1) as usize];
        // idx>>1 represents the pre index,
        // idx & 1 represents odd or even , compensate for  `>> 1`
        for idx in 1..bit_nums.len() {
            bit_nums[idx] = bit_nums[idx >> 1] + (idx & 1) as i32;
        }

        bit_nums
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use test_case::test_case;

    #[test_case(2, [0, 1, 1].to_vec(); "case 2")]
    #[test_case(5, [0, 1, 1, 2, 1, 2].to_vec(); "case 1")]
    fn test_count_bits(n: i32, bit_nums: Vec<i32>) {
        assert_eq!(Solution::count_bits(n), bit_nums);
    }
}