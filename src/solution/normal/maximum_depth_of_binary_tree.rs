use std::cell::RefCell;
use std::rc::Rc;

use crate::solution::Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(&root, 0)
    }

    pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        let mut depth_now = depth;
        if let Some(node) = node {
            depth_now = depth + 1;
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();


            let depth_left = depth_now.max(Solution::dfs(&left, depth_now));
            let depth_right = depth_now.max(Solution::dfs(&right, depth_now));
            depth_now = depth_left.max(depth_right)
        }
        depth_now
    }
}