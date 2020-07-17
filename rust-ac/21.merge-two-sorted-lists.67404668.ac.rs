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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head=Box::new(ListNode::new(0));
        let mut ptr=&mut head;
        let ( mut ptr1,mut ptr2)=(&l1,&l2);
        loop{
            if ptr1.is_none()&&ptr2.is_none(){
                break;
            }else if ptr1.is_none()||(ptr1.is_some()&&ptr2.is_some()&&ptr1.as_ref().unwrap().val>ptr2.as_ref().unwrap().val){
              ptr.next=ptr2.to_owned();
              ptr=ptr.next.as_mut().unwrap();
              ptr2=&ptr2.as_ref().unwrap().next;
            }else{
              ptr.next=ptr1.to_owned();
              ptr=ptr.next.as_mut().unwrap();
              ptr1=&ptr1.as_ref().unwrap().next;
            }
        }
        head.next
        
    }
}
