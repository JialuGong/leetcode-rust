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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(rc)=>{
                return Self::bfs(&rc.borrow().left,&rc.borrow().right);
            }
            None=>{return true;}
        }
        
    }
    fn bfs(left:&Option<Rc<RefCell<TreeNode>>>,right:&Option<Rc<RefCell<TreeNode>>>)->bool{
       match(left,right){
           (None,None)=>{return true;},
           (Some(rc),None)=>{return false;},
           (None,Some(rc))=>{return false;}
           (Some(rc1),Some(rc2))=>{
               if rc1.borrow().val!=rc2.borrow().val{
                   return false;
               }
               let ans1=Self::bfs(&rc1.borrow().left,&rc2.borrow().right);
               let ans2=Self::bfs(&rc1.borrow().right,&rc2.borrow().left);
               return ans1&ans2;
           }

       }
    }
}
