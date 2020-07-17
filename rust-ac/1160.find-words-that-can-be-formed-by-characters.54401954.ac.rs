use std::collections::HashMap;
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut answer :i32=0;
    let mut chars_hash:HashMap<char,i32> =HashMap::new();
    for ch in chars.chars(){
        let num=chars_hash.entry(ch).or_insert(0);
        *num+=1;
    }
    for word in words{
        let mut word_hash:HashMap<char,i32>=HashMap::new();
        for ch in word.chars(){
            let num=word_hash.entry(ch).or_insert(0);
            *num+=1;
        }
        let mut flag=true;
        for (key,value) in word_hash{
            match chars_hash.get(&key) {
                Some(num) => {
                    if *num<value{
                        flag=false;
                    }
                },
                None => {
                    flag=false;
                },
            }
        }
        if flag{
            answer+=word.len() as i32;
        }
    }
    answer

}
}
