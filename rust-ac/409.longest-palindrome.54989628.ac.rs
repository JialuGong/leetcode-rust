impl Solution {
    pub  fn longest_palindrome(s: String) -> i32 {
        let mut answer:i32 = 0;
        let mut odd_flag = false;
       for b in b'A'..b'z' + 1{
            let ch=b as char;
            let sum =s.chars().filter(|&a| a==ch).collect::<String>().len();
           answer+=match sum%2{
                1=>{odd_flag=true;sum as i32-1}
                _=>{sum as i32}
            }
        }
        match odd_flag {
             true=> answer+1,
            false => answer,
        }
    }
}
