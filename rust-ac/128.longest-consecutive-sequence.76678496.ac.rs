use std::collections::HashMap;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut num_hash: HashMap<i32, i32> = HashMap::new();
        for num in &nums {
            match (
                num_hash.get(&(num - 1)),
                num_hash.get(&(num + 1)),
                num_hash.get(num),
            ) {
                (None, None, None) => {
                    num_hash.insert(*num, 1);
                }
                (Some(n), None, None) => {
                    let cnt = *n;
                    num_hash.insert(*num, cnt + 1);
                    num_hash.insert(*num - cnt, cnt + 1);
                }
                (None, Some(n), None) => {
                    let cnt = *n;
                    num_hash.insert(*num, cnt + 1);
                    num_hash.insert(*num + cnt, cnt + 1);
                }
                (Some(n1), Some(n2), None) => {
                    let (cnt1, cnt2) = (*n1, *n2);
                    if num_hash.get(num).is_none() {
                        num_hash.insert(*num - cnt1, cnt1 + cnt2 + 1);
                        num_hash.insert(*num + cnt2, cnt1 + cnt2 + 1);
                        num_hash.insert(*num, cnt1 + cnt2 + 1);
                    }
                }
                _ => {}
            }
        }
        num_hash.iter().fold(0, |acc, (key, value)| {
            acc.max(*value)
        })
    }
}
