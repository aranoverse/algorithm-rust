use crate::solution::Solution;

impl Solution {
    /*
     * https://leetcode.com/problems/subsets/
     *
     * Given an integer array nums of unique elements, return all possible subsets (the power set).
     *
     * The solution set must not contain duplicate subsets. Return the solution in any order.
     * Example 1:
     *
     * Input: nums = [1,2,3]
     * Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
     * Example 2:
     *
     * Input: nums = [0]
     * Output: [[],[0]]
     * Constraints:
     *
     * 1 <= nums.length <= 10
     * -10 <= nums[i] <= 10
     * All the numbers of nums are unique.
     */

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let subsets = &mut Vec::new();
        Solution::backtrack(&nums, 0, vec![], subsets);
        subsets.to_vec()
    }

    fn backtrack(nums: &Vec<i32>, start: usize, mut sub_list: Vec<i32>, subsets: &mut Vec<Vec<i32>>) {
        subsets.push(sub_list.clone());
        for i in start..nums.len() {
            sub_list.push(nums[i]);
            Solution::backtrack(nums, i + 1, sub_list.clone(), subsets);
            sub_list.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use test_case::test_case;

    #[test_case([1, 2, 3, 4].to_vec(), [[].to_vec(), [1].to_vec(), [2].to_vec(), [1, 2].to_vec(), [3].to_vec(), [1, 3].to_vec(), [2, 3].to_vec(), [1, 2, 3].to_vec(), [4].to_vec(), [1, 4].to_vec(), [2, 4].to_vec(), [1, 2, 4].to_vec(), [3, 4].to_vec(), [1, 3, 4].to_vec(), [2, 3, 4].to_vec(), [1, 2, 3, 4].to_vec()].to_vec(); "case 2")]
    #[test_case([1, 2, 3].to_vec(), [[].to_vec(), [1].to_vec(), [2].to_vec(), [1, 2].to_vec(), [3].to_vec(), [1, 3].to_vec(), [2, 3].to_vec(), [1, 2, 3].to_vec()].to_vec(); "case 1")]
    fn test_subsets(nums: Vec<i32>, mut subsets: Vec<Vec<i32>>) {
        let mut ret = Solution::subsets(nums);
        ret.sort();
        subsets.sort();
        assert_eq!(ret, subsets);
    }
}