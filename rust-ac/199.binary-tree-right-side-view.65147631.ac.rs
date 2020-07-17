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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ptr = &root;
        let mut ans = Vec::new();
        Solution::dfs(&root, &mut ans, 0);
        ans
    }
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>, depth: usize) {
        match root {
            Some(rc) => {
                if ans.get(depth).is_none() {
                    ans.push(rc.as_ref().borrow().val);
                }

                if rc.as_ref().borrow().right.is_some() {
                    Solution::dfs(&rc.as_ref().borrow().right, ans, depth + 1);
                }
                if rc.as_ref().borrow().left.is_some(){
                    Solution::dfs(&rc.as_ref().borrow().left, ans, depth + 1);
                }
                
            }
            None => return,
        }
    }
}
