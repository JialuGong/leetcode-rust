impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
  let mut target:Vec<i32>=Vec::new();
  for i in 0..nums.len(){
    target.insert(*index.get(i).unwrap_or(&0) as usize, *nums.get(i).unwrap_or(&0));
  }
  target
}
}
