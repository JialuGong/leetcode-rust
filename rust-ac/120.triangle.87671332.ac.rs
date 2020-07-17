impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut nums = Vec::new();
        for num in triangle.last().unwrap(){
            nums.push(*num);
        }
       
        let len=triangle.len();
        for i in (0..len-1).rev(){
            let mut tmp=Vec::new();
            for j in 0..i+1{
                tmp.push(triangle[i][j]+nums[j].min(nums[j+1]));
            }
            nums=tmp;
        }
       *nums.first().unwrap()
    }
}
