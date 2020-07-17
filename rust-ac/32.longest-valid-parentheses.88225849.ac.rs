impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut num_stack = Vec::new();
        let mut nums = Vec::new();
        for (i, ch) in s.chars().enumerate() {
            if stack.is_empty() {
                stack.push(ch);
                num_stack.push(i);
            } else {
                if *stack.last().unwrap() == '(' && ch == ')' {
                    stack.pop();
                    nums.push(num_stack.pop().unwrap());
                    nums.push(i);
                } else {
                    stack.push(ch);
                    num_stack.push(i);
                }
            }
        }
        // println!("nums len is {}",nums.len());
        nums.sort();
        if nums.is_empty(){
            return 0;
        }
        let (mut tmp, mut max) = (1, 0);
        for i in 1..nums.len() {
            if nums[i]!=nums[i-1]+1{
                max=max.max(tmp);
                tmp=1;
            }else{
                tmp+=1;
            }
        }
        max.max(tmp)
    }
}
