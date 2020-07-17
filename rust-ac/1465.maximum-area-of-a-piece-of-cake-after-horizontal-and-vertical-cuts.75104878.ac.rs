impl Solution {
    pub fn max_area(h: i32, w: i32, mut horizontal_cuts: Vec<i32>, mut vertical_cuts: Vec<i32>) -> i32 {
        horizontal_cuts.sort();
        vertical_cuts.sort();
        let (mut h_max, mut w_max) = (0, 0);
        let mut prev = 0;
        for num in &horizontal_cuts {
            h_max = h_max.max(num - prev);
            prev = *num;
        }
        h_max = h_max.max(h -prev);
        prev = 0;
        for num in &vertical_cuts {
            w_max = w_max.max(num - prev);
            prev = *num;
        }
        w_max = w_max.max(w - prev);
      ( ( h_max as i64*w_max as i64)%1000000007) as i32
    }
}
