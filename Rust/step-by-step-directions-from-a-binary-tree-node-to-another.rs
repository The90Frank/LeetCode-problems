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
    fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut minPrefix = strs[0].as_str();
        let mut minPrefixSize = minPrefix.chars().count();
        for curStr in strs.iter() {
            if curStr.chars().count() < minPrefixSize {
                minPrefixSize = curStr.chars().count();
            }
            let mut x = minPrefixSize+1;
            for x in (0..minPrefixSize+1).rev() {
                if minPrefix[..x] == curStr[..x] {
                    minPrefixSize = x;
                    break;
                }
            }
            if x == 0 {
                minPrefixSize = x;
                break;
            }
        }
        return minPrefix[..minPrefixSize].to_string();
    }

    fn get_path(node: &Option<Rc<RefCell<TreeNode>>>, value: i32) -> (bool, String) {
        if let Some(v) = node {
            let curNode = v.borrow();
            
            if curNode.val == value {
                return (true, "".to_string());
            }
            
            let left = Self::get_path(&curNode.left, value);
            if left.0{
                return (true, "L".to_owned()+&left.1);
            }
            
            let right = Self::get_path(&curNode.right, value);
            if right.0{
                return (true, "R".to_owned()+&right.1);
            } 

            return (false, "".to_string());

        } else {
            return (false, "".to_string());
        }
    }

    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        let (t, start_path) = Self::get_path(&root, start_value);
        let (t, dest_path) = Self::get_path(&root, dest_value);
        let paths = vec![start_path.clone(), dest_path.clone()];
        let s = Self::longest_common_prefix(paths);
        //print!("{} {} {}", s, start_path, dest_path);
        let retS = std::iter::repeat("U").take(start_path.len()-s.len()).collect::<String>() + &dest_path[s.len()..];
        return retS;
    }
}
