impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans=Vec::new();
        let mut tmp=i32::min_value();
        for num in &nums{
            if tmp>=0{
                ans.push(tmp);
                tmp+=num;
            }else{
                tmp=tmp.max(*num);
            }
        }
        let mut max=tmp;
        for num in &ans{
            max=max.max(*num);
        }
        max
    }
}
