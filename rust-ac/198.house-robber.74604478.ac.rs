impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![];
        for (i, num) in nums.iter().enumerate() {
            let max = i32::max(
                num + if i < 2 {
                    0
                } else {
                    *dp.get(i - 2).unwrap_or(&0)
                },
                if i < 1 {
                    0
                } else {
                    *dp.get(i - 1).unwrap_or(&0)
                },
            );
            dp.push(max);
        }
        *dp.last().unwrap_or(&0)
    }
}
