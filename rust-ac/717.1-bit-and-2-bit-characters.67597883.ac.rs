impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut cnt = 0;
        for i in (0..bits.len()-1).rev() {
            if bits[i] == 0 {
                break;
            }
            cnt += 1;
        }
        if cnt % 2 == 1 {
            return false;
        } else {
            return true;
        }
    }
}
