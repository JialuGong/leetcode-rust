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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
         let mut ans = Vec::new();
        let mut stack: Vec<(Option<Rc<RefCell<TreeNode>>>, bool)> = vec![(root.clone(), false)];
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            let flag = node.1;
            match node.0.clone() {
                Some(rc) => {
                    if node.1 {
                        ans.push(rc.as_ref().borrow().val);
                    } else {
                        stack.push((node.0.clone(), true));
                        stack.push((rc.as_ref().borrow().right.clone(), false));
                        stack.push((rc.as_ref().borrow().left.clone(), false));
                        
                    }
                }
                None => {}
            }
        }

        ans

    }
}
