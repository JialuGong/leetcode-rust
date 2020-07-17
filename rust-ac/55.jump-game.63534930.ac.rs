impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let m = nums.len();
        if nums[0]==0{
            if m==1{
                return true;
            }else{
                return false;
            }
        }
        for i in 0..m {
            if nums[i] == 0 {
                let n = i;
                let mut flag = true;
                let mut k = 1;
                for j in (0..n ).rev() {
                    if nums[j] > k {
                        flag = false;
                        break;
                    }
                     k+=1;
                }
                if flag &&i!=m-1{
                    return false;
                }
            }
        }
        true
    }
}
