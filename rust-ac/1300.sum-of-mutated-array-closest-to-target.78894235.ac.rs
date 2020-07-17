impl Solution {
    pub fn find_best_value(mut arr: Vec<i32>, target: i32) -> i32 {
        arr.sort();
        let mut sum=arr.iter().sum::<i32>();
        if sum<=target{
            return *arr.last().unwrap();
        }else{
            let (mut min_value,mut ans)=(i32::max_value(),i32::max_value());
            for (i,num) in arr.iter().rev().enumerate(){
                sum-=num;
                if target<sum{
                    continue;
                }
                let value=(target-sum)/(i+1) as i32;
                let next=*arr.iter().nth_back(i+1).unwrap_or(&0);
                if value<next{
                    continue;
                }
                let plus=sum+(value+1)*(i as i32+1);
                let cur=sum+(value)*(i as i32+1);
                let (cur_min_value,cur_ans)=if (plus-target).abs()<(cur-target).abs(){
                    ((plus-target).abs(),value+1)
                }else{
                     ((cur-target).abs(),value)
                };
               //  println!("sum is {},value is {},cur_min_value is {}",sum,value,cur_min_value);
                if cur_min_value<min_value{
                    min_value=cur_min_value;
                    ans=cur_ans;
                }
            }
            ans
        }
        
    }
}
