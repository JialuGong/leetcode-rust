use std::char;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut ans=String::new();
        let mut next=0;
        let (mut iter1,mut iter2)=(a.chars().rev(),b.chars().rev());
       loop{
           match (iter1.next(),iter2.next()) {
               (Some(ch1),Some(ch2))=>{
                   next+=ch1 as u32 -48+ch2 as u32-48;
                   ans.push(char::from_digit(next%2,10).unwrap());
                   next/=2;
               }
               (Some(ch1),None)=>{
                   next+=ch1 as u32 -48;
                   ans.push(char::from_digit(next%2,10).unwrap());
                   next/=2;
               }
               (None,Some(ch1))=>{
                   next+=ch1 as u32 -48;
                   ans.push(char::from_digit(next%2,10).unwrap());
                   next/=2;

               }
               (None,None)=>{
                   break;
               }
           }
       }
       if next!=0{
           ans.push('1');
       }
       ans.chars().rev().collect()
    }
}
