use std::collections::HashMap;
impl Solution {
    pub fn new21_game(n: i32, k: i32, w: i32) -> f64 {
        if k == 0 {
            return 1f64;
        }
        let mut state_hash: HashMap<i32, f64> = HashMap::new();

        Self::rec(n, k, w, &mut state_hash, 0f64)
    }
    fn rec(n: i32, k: i32, w: i32, state_hash: &mut HashMap<i32, f64>, mut ans: f64) -> f64 {
        match state_hash.get(&k) {
            Some(&var) => var,
            None => {
                for num in 1..w + 1 {
                    let tmp = 1f64 / w as f64;
                    if k <= num {
                        if n >= num {
                            ans += tmp;
                        }
                    } else {
                        ans += tmp * Self::rec(n - num, k - num, w, state_hash, ans);
                    }
                }
                state_hash.insert(k, ans);
                ans
            }
        }
    }
}
