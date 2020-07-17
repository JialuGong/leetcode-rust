use std::collections::HashMap;
impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut hash: HashMap<i32, i32> = HashMap::new();
        deck.iter().for_each(|x| {
            *hash.entry(*x).or_insert(0) += 1;
        });
        let ans = hash.iter().fold(
            0,
            |acc, x| if acc == 0 { *x.1 } else { Self::gcd(acc, *x.1) },
        );
        if ans <= 1 {
            false
        } else {
            true
        }
    }
    fn gcd(x: i32, y: i32) -> i32 {
        let (mut max, mut min) = (x.max(y), x.min(y));
        while min > 1 {
            if max == min {
                return max;
            }
            let old_max=max;
            max = (max - min).max(min);
            min = (old_max - min).min(min);
            
        }
        1
    }
}
