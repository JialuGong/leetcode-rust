const I32_MAX:i32=2_147_483_647;
const I32_MIN:i32=-2_147_483_648;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
    let mut answer:i64 = 1;
    let mut num_str = String::new();
    let mut num_start_flag=false;
    let mut sign_start_flag=false;
    let mut zero_start_flag=false;
    for ch in str.chars() {
        match ch {
            ' ' => {
                if num_start_flag||sign_start_flag||zero_start_flag{
                    break;
                }
            },
            '0'=>{
                zero_start_flag=true;
                if num_start_flag{num_str.push(ch);}
            }
            '1'..='9' => {
                num_start_flag=true;
                num_str.push(ch);
            }
            '-' => {
                if sign_start_flag||num_start_flag||zero_start_flag{
                    break;
                }
                sign_start_flag=true;
                answer*=-1;
            }
            '+'=>{
               if sign_start_flag||num_start_flag||zero_start_flag{
                   break;
               }
               sign_start_flag=true;
            },
            _ => {
                break;
            }
        }
    }
    if num_str.len() <= 0 {
        return 0;
    } else {
        if num_str.len()<=10{
           let num:i64=num_str
            .chars()
            .rev()
            .enumerate()
            .map(|(i, ch)| (ch as i64 - 48) * i64::pow(10,i as u32))
            .sum();
            answer*=num;
            if answer>I32_MAX as i64{
                return I32_MAX;
            }else if answer<I32_MIN as i64{
                return I32_MIN;
            }else{
                return answer as i32;
            }
        }else{
            match answer {
               1=>{
                   return I32_MAX;
               },
               _=>{
                   return I32_MIN;
               },
            }
        }
    }
}
}
