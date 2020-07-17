impl Solution{
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ans=0;
        let mut left_pos=0;
        let mut right_pos=height.len()-1;
        loop{
            if left_pos>=right_pos{break;}
            ans=ans.max((right_pos-left_pos) as i32 *i32::min(height[left_pos],height[right_pos]));
            if height[left_pos]<height[right_pos]{
                left_pos+=1;
            }else{
                right_pos-=1;
            }
        }
        ans
    }
}
