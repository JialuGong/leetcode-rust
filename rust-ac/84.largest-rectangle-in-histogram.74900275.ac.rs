impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut stack = vec![0];
        let mut cnt_stack = Vec::new();

        let mut max_value = 0;
        heights.push(0);
        for (i, num) in heights.iter().enumerate() {
            let mut left=i;
            while num <stack.last().unwrap() {
                let last = stack.pop().unwrap();
                 left = cnt_stack.pop().unwrap();
                max_value = max_value.max(last * (i - left) as i32);
            }
            stack.push(*num);
            cnt_stack.push(left);
        }
        max_value
    }
}

