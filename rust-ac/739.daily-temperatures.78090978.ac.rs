impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![(0, *t.first().unwrap())];
        let mut ans = vec![0; t.len()];
        for (i, num) in t.iter().enumerate() {
            if i == 0 {
                continue;
            }
            while !stack.is_empty() {
                let (index, last_num) = stack.last().unwrap();
                if num <= last_num {
                    break;
                }
                ans[*index] = i as i32 - *index as i32;
                stack.pop();
            }
            stack.push((i, *num));
        }
        ans
    }
}
