impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len()==0{
            return vec![vec![]];
        }
        let mut queue: Vec<Vec<i32>> = Vec::new();
        let mut cnt=1;
        for i in 0..nums.len() {
            let num = nums[i];
            if queue.is_empty() {
                queue.push(vec![num]);
            } else {
                for _k in 0..cnt {
                    let mut cur = queue.remove(0);
                    for j in 0..cur.len() {
                        let mut tmp = cur.clone();
                        tmp.insert(j, num);
                        queue.push(tmp);
                    }
                    cur.push(num);
                    queue.push(cur);
                }
            }
            cnt*=(i+1);
        }
        queue
    }
}
