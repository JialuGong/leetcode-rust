impl Solution {
    pub fn get_last_moment(n: i32,  left: Vec<i32>,  right: Vec<i32>) -> i32 {
      let left=left.iter().fold(0, |acc,x| acc.max(*x));
      let right=right.iter().fold(0, |acc,x|acc.max(n-*x));
      left.max(right)
    }
}
