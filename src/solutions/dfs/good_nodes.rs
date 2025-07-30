// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
// 1448. Count Good Nodes in Binary Tree
// https://leetcode.com/problems/count-good-nodes-in-binary-tree
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, i32::MIN)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let max = max.max(node.val);
                let good = if node.val >= max { 1 } else { 0 };
                good + Self::dfs(&node.left, max) + Self::dfs(&node.right, max)
            }
        }
    }
}
