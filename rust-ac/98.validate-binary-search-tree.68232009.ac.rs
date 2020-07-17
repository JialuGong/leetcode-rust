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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let mut value_queue: Vec<(i64, i64)> = vec![(i32::min_value() as i64-1, i32::max_value() as i64 +1)];
        let mut node_queue: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
       
        while !node_queue.is_empty() {
            let cur_node = &node_queue.remove(0);
            let cur_range = value_queue.remove(0);
            let value = cur_node.as_ref().unwrap().borrow().val;
           
                if value as i64<= cur_range.0 || value as i64 >= cur_range.1 {
                    return false;
                }
            
            //left node
            if cur_node.as_ref().unwrap().borrow().left.is_some() {
                node_queue.push(cur_node.as_ref().unwrap().borrow().left.clone());
                value_queue.push((cur_range.0, cur_range.1.min(value as i64)));
            }
            //right node
            if cur_node.as_ref().unwrap().borrow().right.is_some() {
                node_queue.push(cur_node.as_ref().unwrap().borrow().right.clone());
                value_queue.push((cur_range.0.max(value as i64), cur_range.1))
            }
        }

        true
    }
}
