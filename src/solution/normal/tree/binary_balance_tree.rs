use std::cell::RefCell;
use std::rc::Rc;

use crate::solution::normal::tree::TreeNode;
use crate::solution::Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        dfs(&root, 0) != -1
    }
}

pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
    let mut depth_now = depth;
    if let Some(node) = node {
        depth_now = depth + 1;
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();


        let depth_left = dfs(&left, depth_now);
        let depth_right = dfs(&right, depth_now);

        depth_now = if depth_left == -1 || depth_right == -1 || depth_left - depth_right > 1 || depth_right - depth_left > 1 {
            -1
        } else {
            depth_left.max(depth_right).max(depth_now)
        }
    }
    depth_now
}

pub fn dfs_v2(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut depth_now = 0;
    if let Some(node) = node {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();


        let depth_left = dfs_v2(&left);
        let depth_right = dfs_v2(&right);

        depth_now = if depth_left == -1 || depth_right == -1 || depth_left - depth_right > 1 || depth_right - depth_left > 1 {
            -1
        } else if depth_left > depth_right {
            depth_left + 1
        } else {
            depth_right + 1
        }
    }

    depth_now
}