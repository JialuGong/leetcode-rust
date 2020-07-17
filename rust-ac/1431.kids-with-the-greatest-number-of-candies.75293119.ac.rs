impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max=candies.iter().fold(0, |acc,x| acc.max(*x));
        let ans=candies.iter().map(|x| {if extra_candies+x>=max{true}else{false}}).collect::<Vec<bool>>();
        ans
    }
}
