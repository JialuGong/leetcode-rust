impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
         if intervals.is_empty() || intervals[0].is_empty() {
            return intervals;
        }
         intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let mut ans = Vec::new();
        let mut begin = intervals[0][0];
        let mut end = intervals[0][1];
        for i in 1..intervals.len() {
            let item = &intervals[i];
            if item[0] <= end {
                if item[1] > end {
                    end = item[1];
                }
            } else {
                ans.push(vec![begin, end]);
                begin = item[0];
                end = item[1];
            }
        }
        ans.push(vec![begin, end]);
        ans
    }
}
