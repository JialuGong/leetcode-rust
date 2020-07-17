impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
     let mut a_flag=false;
    let mut b_flag=false;
    if rec1[0]<=rec2[0]&&rec2[0]<rec1[2]||rec1[0]>=rec2[0]&&rec2[2]>rec1[0]{a_flag=true;}
    if rec1[1]<=rec2[1]&&rec2[1]<rec1[3]||rec1[1]>=rec2[1]&&rec2[3]>rec1[1]{b_flag=true;}
    a_flag&&b_flag
    
}
}
