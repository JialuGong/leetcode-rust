// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_k_lists(mut lists:  Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head=Box::new(ListNode::new(0));
         let mut ptr=&mut head;
       loop{
           let mut flag=false;
           let mut min=i32::max_value();
           let mut pos=lists.len();
           for i in 0..lists.len(){
               let node:&Option<Box<ListNode>>=&lists[i];
               if node.is_some(){
                   if node.as_ref().unwrap().val<=min{
                       min=node.as_ref().unwrap().val;
                       pos=i;
                   }
               }
           }
           if pos==lists.len(){
               break;
           }else{
               let node:&Option<Box<ListNode>>=lists.get(pos).unwrap();
               ptr.next=node.to_owned();
               ptr=ptr.next.as_mut().unwrap();
               lists[pos]=node.as_ref().unwrap().next.to_owned();
           }
       }
       head.next
       
    }
}
