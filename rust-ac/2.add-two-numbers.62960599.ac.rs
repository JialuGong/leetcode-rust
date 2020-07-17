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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut tmp = 0;
        let (mut ptr1, mut ptr2) = (&l1, &l2);
        let mut ans: Box<ListNode> = Box::new(ListNode::new(0));
        let mut p_ans = &mut ans;
        loop {
            let mut node;
            match (ptr1, ptr2) {
                (Some(node1), Some(node2)) => {
                    let val = node1.val + node2.val+tmp;
                    node = Some(Box::new(ListNode::new(val % 10)));
                    tmp = val / 10;
                    ptr1 = &node1.next;
                    ptr2 = &node2.next;
                }
                (Some(node1), None) => {
                    let val=node1.val+tmp;
                    node=Some(Box::new(ListNode::new(val%10)));
                    tmp = val/10;
                    ptr1 = &node1.next;
                }
                (None, Some(node2)) => {
                    let val=node2.val+tmp;
                    node=Some(Box::new(ListNode::new(val%10)));
                    tmp = val/10;
                    ptr2 = &node2.next;
                }
                _ => {
                    if tmp == 0 {
                        break;
                    } else {
                        node=Some(Box::new(ListNode::new(tmp)));
                        tmp = 0;
                    }
                }
            }
            p_ans.next=node;
            p_ans=p_ans.next.as_mut().unwrap();
        }
        ans.next
    }
}
