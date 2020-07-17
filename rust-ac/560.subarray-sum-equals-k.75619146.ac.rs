use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum_hash:HashMap<i32,i32>=HashMap::new();
        let (mut sum,mut ans)=(0,0);
        sum_hash.insert(0, 1);
        for num in &nums{
            sum+=num;          
            ans+=sum_hash.get(&(sum-k)).unwrap_or(&0);
              *sum_hash.entry(sum).or_insert(0)+=1;

        }
        ans
    }
}
