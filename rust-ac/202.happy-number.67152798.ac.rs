impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut mut_n = n;
        let mut prev = vec![];
        loop {
            if mut_n == 1 {
                return true;
            } else {
                for num in &prev{
                    if mut_n==*num{
                        return false;
                    }
                }
                prev.push(mut_n);
                let mut tmp = 0;
                while mut_n > 0 {
                    tmp += (mut_n % 10).pow(2);
                    mut_n /= 10
                }
                mut_n = tmp;
            }
        }
        
    }
}
