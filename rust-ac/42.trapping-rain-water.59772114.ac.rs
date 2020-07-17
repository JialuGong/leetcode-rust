impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
    let mut stack: Vec<(i32, i32)> = Vec::new();
    let mut answer = 0;
    height.iter().for_each(|&x| {
        if stack.is_empty() {
            stack.push((1, x));
        } else {
            let mut cnt = 0;
            if stack[0].1 <= x {
                let minner = stack[0].1;
                while !stack.is_empty() {
                    let cur = stack.pop().unwrap();
                    cnt += cur.0;
                    answer += cur.0 * (minner - cur.1);
                }
                stack.push((1, x));
            } else {
                while stack[stack.len() - 1].1 < x {
                    let cur = stack.pop().unwrap();
                    cnt += cur.0;
                    answer += cur.0 * (x - cur.1);
                }
                stack.push((cnt + 1, x));
            }
        }
    });
    answer
}
}
