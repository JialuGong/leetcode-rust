impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        if strs.len()==0{
            return "".to_string();
        }
       let standard=strs.remove(0);
        let ans_len=strs.iter().fold(standard.len(),|acc,item|{
            Self::common_prefix(&standard, acc, item)
        });
        standard[0..ans_len].to_string()
    }
    fn common_prefix(standart:&String,len:usize,compare_str:&String)->usize{
        let mut ans=0;
        for i in 0..len.min(compare_str.len()){
            if standart.chars().nth(i).unwrap()==compare_str.chars().nth(i).unwrap(){
                ans+=1;
            }else{
                break;
            }
        }
        ans
    }
}
