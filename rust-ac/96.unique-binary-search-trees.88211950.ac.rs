impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut nums = vec![1,1];
        for i in 2..(n+1) as usize {
           nums.push( (0..i).fold(0, |acc,x|acc+nums[x]*nums[i-x-1]));
        }
        nums[n as usize]
    }
}
