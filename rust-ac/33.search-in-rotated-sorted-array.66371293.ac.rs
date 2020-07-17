impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        } else if nums.len() == 1 {
            if nums[0] == target {
                return 0;
            } else {
                return -1;
            }
        }
        let c_pos = Self::find_c_pos(&nums, 0, nums.len());
        println!("step 1 finish, c pos is {}", c_pos);
        if target >nums[0]&&c_pos!=0 {
            Self::find_target(&nums, 0, c_pos , target)
        } else if target<nums[0]||c_pos==0{
            Self::find_target(&nums, c_pos, nums.len(), target)
        }else{
            0 
        }
    }
    fn find_c_pos(nums: &Vec<i32>, left: usize, right: usize) -> usize {
        if nums[left] > nums[right-1] {
            let mid = left + (right - left) / 2;
            println!("mid is {},left is {},right is {}",mid,left,right);
            if nums[left] < nums[mid] {
                return Self::find_c_pos(nums, mid, right);
            } else if nums[mid] < nums[right - 1] {
                return Self::find_c_pos(nums, left, mid+1);
            } else {
                println!("yes");
                mid
            }
        } else {
            left
        }
    }
    fn find_target(nums: &Vec<i32>, left: usize, right: usize, target: i32) -> i32 {
        if left < right {
            let mid = (right + left) / 2;
            println!("mid is==={} ", mid);
            if target < nums[mid] {
                return Self::find_target(nums, left, mid , target);
            } else if target > nums[mid] {
                return Self::find_target(nums, mid+1 , right, target);
            } else {
                return mid as i32;
            }
        } else {
            -1
        }
    }
}
