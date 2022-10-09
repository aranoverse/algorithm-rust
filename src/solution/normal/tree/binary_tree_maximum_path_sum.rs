use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Add;

use crate::solution::Solution;
use crate::solution::normal::tree::TreeNode;


impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_path_sum = i32::MIN;
        dfs_max_path_sum(&root, &mut max_path_sum);
        max_path_sum
    }
}

pub fn dfs_max_path_sum(node: &Option<Rc<RefCell<TreeNode>>>, max_path_sum: &mut i32) -> (i32) {
    let mut sum = 0;
    if let Some(node) = node {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        // If less than zero , do not add
        let left_sum = dfs_max_path_sum(&left, max_path_sum).max(0);
        let right_sum = dfs_max_path_sum(&right, max_path_sum).max(0);

        *max_path_sum = (*max_path_sum).max(left_sum + right_sum + node.borrow().val);
        sum = left_sum.max(right_sum) + node.borrow().val;
    }

    sum
}