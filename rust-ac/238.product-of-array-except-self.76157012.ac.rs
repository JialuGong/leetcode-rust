impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let (mut front,mut back,mut ans)=(vec![1],vec![1],Vec::new());
        let mut multi=1;
        for num in &nums{
            multi*=num;
            front.push(multi);
        }
        front.push(1);
        multi =1;
        for num in nums.iter().rev(){
            multi*=num;
            back.push(multi);
        }
        back.push(1);
        back.reverse();
        for i in 1..nums.len()+1{
           ans.push(front[i-1]*back[i+1]);
           
        }
        ans

    }
}
