use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut charactor_hash:HashMap<char,usize>=HashMap::new();
        let (mut ans,mut tmp)=(0,0);
        let mut pos:usize=0;
        for (i,item) in s.chars().enumerate(){
            if charactor_hash.get(&item).is_none()||*charactor_hash.get(&item).unwrap()<pos{
                tmp+=1;
                charactor_hash.insert(item,i);
            }else{
                ans=ans.max(tmp);
                let k= charactor_hash.get(&item).unwrap();
                tmp-=*k as i32-pos as i32;
                pos=*k+1;
                charactor_hash.insert(item,i);
            }
        }
        ans.max(tmp)
    }
}
