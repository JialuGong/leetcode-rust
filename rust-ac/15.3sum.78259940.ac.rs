impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        //find c_pos;
        let mut positive_pos: usize = 0;
        let mut negetive_pos: usize = 0;
        for i in 0..nums.len() {
            if nums[i] >= 0 && i > 0 && nums[i - 1] < 0 {
                negetive_pos = i;
            }
            if nums[i] > 0 {
                positive_pos = i;
                break;
            }
        }
        for i in positive_pos..nums.len() {
            if i > positive_pos && nums[i] == nums[i - 1] {
                continue;
            }
            for j in (0..negetive_pos).rev() {
                if j < negetive_pos && nums[j] == nums[j+1] {
                    continue;
                }
                let tmp =-(nums[i] + nums[j]);
                if tmp > 0 {
                    for k in i +1..nums.len() {
                        if tmp < nums[k] {
                            break;
                        }
                        if tmp == nums[k] {
                            ans.push(vec![nums[i], nums[j], nums[k]]);
                            break;
                        }
                    }
                } else if tmp < 0 {
                    for k in (0..j).rev() {
                        if tmp > nums[k] {
                            break;
                        }
                        if nums[k] == tmp {
                            ans.push(vec![nums[i], nums[j], nums[k]]);
                            break;
                        }
                    }
                } else {
                    if negetive_pos < positive_pos {
                        ans.push(vec![0, nums[i], nums[j]]);
                    }
                }
            }
        }
        if(positive_pos>negetive_pos&&positive_pos - negetive_pos >= 3 ) ||(*nums.last().unwrap_or(&-1)==0&&nums.len()>=3&&*nums.first().unwrap_or(&-1)==0){
            ans.push(vec![0, 0, 0]);
        }
        ans
    }
}
