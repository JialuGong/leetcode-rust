impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let max_str:String;
        let min_str:String;
        if str1.len()>str2.len(){
            max_str=str1;
            min_str=str2;
        }else{
            max_str=str2;
            min_str=str1;
        }
         let min_len: usize = min_str.len();
        let max_len: usize = max_str.len();
        for i in (1..min_len + 1).rev() {
            if min_len % i != 0 || max_len % i != 0 {
                continue;
            } else {
                let mut flag1 = true;
                let mut flag2 = true;
                let comp_str = &min_str[0..i];
                for j in 0..min_len / i {
                    if &min_str[j * i..j * i + i] != comp_str {
                        flag1 = false;
                        break;
                    }
                }
                for k in 0..max_len / i {
                    if &max_str[k * i..k * i + i] != comp_str {
                        flag2 = false;
                        break;
                    }
                }
                if flag1 && flag2 {
                    return comp_str.to_string();
                }
            }
        }
        return ("").to_string();
       
    }
    
}
