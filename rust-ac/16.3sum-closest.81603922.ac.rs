impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let len = nums.len();
        let mut min = target - nums[0..3].iter().sum::<i32>();
        for second in 1..len - 1 {
            let (mut first, mut third) = (second - 1usize, second + 1usize);
            let new_target = target - nums[second];
             //println!("new target is {},second is {},nums[second] is {}=========", new_target,second,nums[second]);
            loop {
                let cur = new_target - nums[first] - nums[third];
               // println!("new target is {},cur is {}", new_target, cur);
                if min == 0 {
                    break;
                }
                min = if min.abs() < cur.abs() { min } else { cur };
                if first <= 0 && third >= (len - 1) {
                    break;
                }
                if nums[first] + nums[third] > new_target {
                    if first == 0 {
                        
                        break;
                    } else {
                        first -= 1;
                    }
                }
                else if nums[first] + nums[third] < new_target {
                    if third == len - 1 {
                        
                        break;
                    } else {
                        third += 1;
                    }
                }
            }
        }
        target - min
    }
}
