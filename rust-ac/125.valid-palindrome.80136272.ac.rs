impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let (mut left, mut right) = (0, s.len() - 1);
        let chars:Vec<char>=s.chars().collect();
        while left < right&&left<s.len() {
            let (ch_l, ch_r) = (chars[left], chars[right]);
            //  println!("ch_l is {},ch_r is {}",ch_l,ch_r);
            if ch_l.is_ascii_alphanumeric() && ch_r.is_ascii_alphanumeric() {
                // println!("inner :: ch_l is {},ch_r is {}",ch_l,ch_r);
                if !ch_l.eq_ignore_ascii_case(&ch_r) {
                    return false;
                }
                left += 1;
                right -= 1;
            }
            else if !ch_l.is_ascii_alphanumeric() {
                left += 1;
            }
            else if !ch_r.is_ascii_alphanumeric() {
                right -= 1;
            }
        }
        true
    }
}
