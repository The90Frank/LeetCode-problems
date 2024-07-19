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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn rec_travel(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) -> () {
        if let Some(v) = node {
            let v = v.borrow();

            Self::rec_travel(&v.left, ret);
            ret.push(v.val);
            Self::rec_travel(&v.right, ret);
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = vec![];
        if let Some(v) = root {
            Self::rec_travel(&Some(v), &mut ret);
        }

        return ret;
    }
}
