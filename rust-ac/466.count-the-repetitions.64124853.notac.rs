impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s2_chars = s2.chars().collect::<Vec<char>>();
        //get unique char vec
        let mut s1_chars = Vec::new();

        for (_i, ch) in s1.chars().enumerate() {
            for item in &s2_chars {
                if item == &ch {
                    s1_chars.push(*item);
                    break;
                }
            }
        }

        let (mut pos1, mut pos2) = (0, 0);
        loop {
            if pos1 == s1_chars.len() * n1 as usize {
                break;
            }
            if s2_chars[pos2 % s2.len()] == s1_chars[pos1 % s1_chars.len()] {
                pos1 += 1;
                pos2 += 1;
            } else {
                pos1 += 1;
            }
        }
        pos2 as i32 / (s2.len() as i32 * n2)
    }
}
