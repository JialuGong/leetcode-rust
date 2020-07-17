impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
      arr.sort();
      let mut prev=arr[1];
     let  tmp=arr[1]-arr[0];
      for i in 2..arr.len(){
        if arr[i]-prev!=tmp{
          return false;
        }
        prev=arr[i];
      }
      return true;
    }
}
