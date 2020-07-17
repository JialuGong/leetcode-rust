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
        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();
        let mut ptr = &l1;
        loop {
            match &ptr {
                Some(node) => {
                    stack1.push(node.val);
                    ptr=&node.next;
                },
                None=>{break;},
            }
        }
        ptr=&l2;
        loop{
          match &ptr {
            Some(node)=>{
              stack2.push(node.val);
              ptr=&node.next;
            }
            None=>{break;}
          }
        }
        let mut tmp=0;
        let mut ans:Option<Box<ListNode>>=Option::None;
        while !(stack1.is_empty()&&stack2.is_empty())||tmp!=0{
          let (val1,val2)=(stack1.pop().unwrap_or(0),stack2.pop().unwrap_or(0));
          let mut val=val1+val2+tmp;
          tmp=val/10;
          val=val%10;
          let mut node=Box::new(ListNode::new(val));
          node.next=ans;
          ans=Option::Some(node);
        }
        ans
    }
}
