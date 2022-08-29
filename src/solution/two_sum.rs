use crate::solution::Solution;
use std::collections::HashMap;

impl Solution {
    //! https://leetcode.cn/problems/two-sum/
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indexies: Vec<i32> = Vec::with_capacity(2);
        let mut num_to_index: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for i in 0..nums.len() {
            let num = nums[i];
            let left = target - num;

            if num_to_index.contains_key(&left) {
                indexies.insert(0, i as i32);
                indexies.insert(1, *num_to_index.get(&left).unwrap());
                break;
            }

            num_to_index.insert(num, i as i32);
        }

        indexies
    }
}
