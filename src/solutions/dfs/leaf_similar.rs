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
// 872. Leaf-Similar Trees
// https://leetcode.com/problems/leaf-similar-trees
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let l1 = Self::leaves(&root1);
        let l2 = Self::leaves(&root2);

        l1.eq(l2)
    }

    fn is_leaf(node: &TreeNode) -> bool {
        node.left.is_none() && node.right.is_none()
    }

    fn leaves(tree: &Option<Rc<RefCell<TreeNode>>>) -> Box<dyn Iterator<Item = i32>> {
        if let Some(root) = tree {
            let root = root.borrow();
            if Self::is_leaf(&root) {
                Box::new(std::iter::once(root.val))
            } else {
                Box::new(Self::leaves(&root.left).chain(Self::leaves(&root.right)))
            }
        } else {
            Box::new(std::iter::empty())
        }
    }
}
