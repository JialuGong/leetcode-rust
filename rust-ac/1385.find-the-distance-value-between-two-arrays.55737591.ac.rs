impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
  let mut cnt:i32=0;
  for i in &arr1{
    let mut flag=true;
    for j in &arr2{
      if (i-j).abs()<=d {flag=false;break;}  
    }
    if flag {cnt+=1;}
  }
  cnt
}
}
