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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len()==0{
            return None;
        }else{
             Self::recursion_bst(&nums, 0, nums.len()-1)
        }
      
    }
    fn recursion_bst(nums:&Vec<i32>,begin:usize,end:usize)->Option<Rc<RefCell<TreeNode>>>{
        if begin==end{
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[begin]))))
        }else{
            let mid=(end-begin)/2+begin;
            let root=Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            if mid!=begin&&mid>0{
                root.borrow_mut().left=Self::recursion_bst(nums, begin, mid-1);
            }
            if mid<nums.len(){
                root.borrow_mut().right=Self::recursion_bst(nums, mid+1, end);
            }
            Some(root)
        }
    }
}
