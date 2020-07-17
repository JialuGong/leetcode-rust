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
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut node_stack = vec![(root.clone(), 0usize, true)];
        for (c_layer, c_value) in &Self::rewirte_s(&s) {
            while !node_stack.is_empty() {
                let (tree_node, layer, is_scan) = node_stack.pop().unwrap();
                if *c_layer == layer+1 {
                    if !is_scan {
                        if let Some(rc) = tree_node {
                            rc.borrow_mut().left =
                                Some(Rc::new(RefCell::new(TreeNode::new(*c_value))));
                            node_stack.push((Some(rc.clone()), layer, true));
                            node_stack.push((rc.borrow().left.clone(), *c_layer, false));
                        }
                    } else {
                        tree_node.as_ref().unwrap().borrow_mut().right =
                            Some(Rc::new(RefCell::new(TreeNode::new(*c_value))));
                        node_stack.push((
                            tree_node.as_ref().unwrap().borrow().right.clone(),
                            *c_layer,
                            false,
                        ));
                    }
                    break;
                }
            }
        }
        root.unwrap().borrow().right.clone()
    }
    fn rewirte_s(s: &String) -> Vec<(usize, i32)> {
        let mut ans = Vec::new();
        let mut num: Vec<i32> = Vec::new();
        let mut line_cnt = 1usize;
        for ch in s.chars() {
            match ch {
                '-' => {
                    if !num.is_empty() {
                        ans.push((
                            line_cnt,
                            num.iter()
                                .rev()
                                .enumerate()
                                .map(|(i, x)| x * 10i32.pow(i as u32))
                                .sum::<i32>(),
                        ));
                        num.clear();
                        line_cnt = 1;
                    }
                    line_cnt += 1;
                }
                ch => {
                    num.push(ch as i32 - 48);
                }
            }
        }
        ans.push((
            line_cnt,
            num.iter()
                .rev()
                .enumerate()
                .map(|(i, x)| x * 10i32.pow(i as u32))
                .sum::<i32>(),
        ));
        ans
    }
}
