impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x<0{
            false
        }else{
            x.to_string().chars().enumerate().fold(true, |acc,(i,ch)|acc&(ch==x.to_string().chars().nth_back(i).unwrap()) )
        }
    }
}
