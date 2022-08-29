use crate::solution::Solution;
use std::collections::HashMap;

impl Solution {
    //! https://leetcode.cn/problems/two-sum/
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indexies: Vec<i32> = Vec::with_capacity(2);
        let mut num_to_index = HashMap::with_capacity(nums.len());

        for (idx, num) in nums.iter().enumerate() {
            let left = target - num;

            if let Some(left_index) = num_to_index.get(&left) {
                indexies.insert(0, idx as i32);
                indexies.insert(1, *left_index);
                break;
            }

            num_to_index.insert(*num, idx as i32);
        }

        indexies
    }
}
