impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        let sum = if nums.iter().sum::<i32>() % k == 0 {
            nums.iter().sum::<i32>() / k
        } else {
            return false;
        };
        nums.sort_by(|a,b|b.cmp(a));
        let ans = Self::find_k(&mut nums, sum, sum);
        // println!("sum is {}", sum);
        // for num in &nums {
        //     println!("{}", num);
        // }
        ans
    }
    fn find_k(nums: &mut Vec<i32>, k: i32, origin: i32) -> bool {
        if *nums.iter().max().unwrap() == 0 {
            return true;
        } else if *nums.iter().min().unwrap() > k {
            return false;
        } else {
            let mut ans = false;
            for i in 0..nums.len() {
                if nums[i] == 0 {
                    // println!("is zero");
                    continue;
                }
                if nums[i] > k {
                    ans = ans || false;
                } else if nums[i] == k {
                    nums[i] = 0;
                    ans = ans || Self::find_k(nums, origin, origin);
                    nums[i] = k;
                } else {
                    let tmp = nums[i];
                    nums[i] = 0;
                    ans = ans || Self::find_k(nums, k - tmp, origin);
                    nums[i] = tmp;
                }
            }
            return ans;
        }
    }
}
