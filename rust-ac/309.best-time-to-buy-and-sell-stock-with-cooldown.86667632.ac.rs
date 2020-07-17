impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty(){
            return 0;
        }
        let mut hold=vec![-prices[0]];
        let mut unhold=vec![0];
        for i in 1..prices.len(){
            hold.push(hold[i-1].max(if i>1{
                unhold[i-2]-prices[i]
            }else{
                -prices[i]
            }));
            unhold.push(unhold[i-1].max(hold[i-1]+prices[i]));
        }
       *unhold.last().unwrap()
    }
}
