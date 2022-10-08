use std::rc::Rc;
use std::cell::RefCell;

use crate::solution::Solution;
use crate::solution::normal::tree::TreeNode;


impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dfs_max_path_sum(&root).unwrap()
    }
}

pub fn dfs_max_path_sum(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    return if let Some(node) = node {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        let left_sum = dfs_max_path_sum(&left);
        let right_sum = dfs_max_path_sum(&right);

        let mut sum = node.borrow().val;

        if left_sum.is_some() && right_sum.is_some() {
            let left_sum = left_sum.unwrap();
            let right_sum = right_sum.unwrap();
            sum = left_sum.max(right_sum).max(left_sum + right_sum + sum).max(sum).max(sum + left_sum).max(sum + right_sum);
        } else if left_sum.is_some() && right_sum.is_none() {
            let left_sum = left_sum.unwrap();
            sum = left_sum.max(left_sum + sum).max(sum);
        } else if left_sum.is_none() && right_sum.is_some() {
            let right_sum = right_sum.unwrap();
            sum = right_sum.max(right_sum + sum).max(sum);
        }

        Some(sum)
    } else { None };
}