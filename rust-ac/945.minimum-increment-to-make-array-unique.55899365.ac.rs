impl Solution {
    pub fn min_increment_for_unique(a: Vec<i32>) -> i32 {
  let mut mut_a:Vec<i32> =a.clone();
  mut_a.sort();
  let mut pre=-1;
  let mut cnt=0;
  mut_a.iter().for_each(|&x|{
    if x<=pre{
      cnt+=pre+1-x;
      pre=pre+1;
    }else{
      pre=x;
    }
  });
  cnt
}
}
