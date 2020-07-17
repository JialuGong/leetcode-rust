impl Solution {
     pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len()<=1{
            return 0;
        }
        let mut next_reach=0;
        let mut steps=0;
        let mut reach:i32=0;
        for i in 0..nums.len(){
            next_reach=next_reach.max(i as i32 +nums[i]);
            if next_reach >=nums.len() as i32-1{
                return steps+1;
            }
            if i==reach as usize{
                steps+=1;
                reach=next_reach;
            }
        }
        steps
    }
}
