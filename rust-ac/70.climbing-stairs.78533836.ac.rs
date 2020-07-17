impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut ans=vec![];
        for i in 0..n as usize{
            if i<1{
                ans.push(1);
            }else if  i<2{
                ans.push(ans.last().unwrap()+1);
            }else {
                ans.push(ans[i-1]+ans[i-2]);
            }
        }

        *ans.last().unwrap_or(&1)
    }
}
