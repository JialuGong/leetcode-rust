impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let flag: i64 = if x < 0 {
            x = -x;
            -1
        } else {
            1
        };
        let ans: i64 = flag
            * x.to_string()
                .chars()
                .enumerate()
                .map(|(i, x)| {
                    // println!("{}",x as i64-48*10i64.pow(i as u32));
                    (x as i64 - 48) * 10i64.pow(i as u32)
                })
                .sum::<i64>();
       
        if ans > i32::max_value() as i64 || ans < -2147483648 {
            return 0;
        } else {
            return ans as i32;
        }
    }
}
