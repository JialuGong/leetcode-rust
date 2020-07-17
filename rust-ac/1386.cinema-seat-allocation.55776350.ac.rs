impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
  let mut cnt = 0;
  let mut mut_seats: Vec<Vec<i32>> = Vec::new();
  let mut left_flag = true;
  let mut mid_l_flag = true;
  let mut mid_r_flag = true;
  let mut right_flag = true;
  reserved_seats
    .iter()
    .for_each(|item| mut_seats.push(item.to_vec()));
  mut_seats.sort_by(|a, b| a.get(0).unwrap_or(&0).cmp(b.get(0).unwrap_or(&0)));
  mut_seats.push(vec![0,0]);
  let mut last_row = 1;
  let mut row_cnt = 0;
  for row in mut_seats.iter() {
    if row.get(0).unwrap_or(&0) != &last_row{
      println!("{},{},{},{}",left_flag,mid_l_flag,mid_r_flag,right_flag);
      if left_flag&&mid_l_flag{
        if right_flag&&mid_r_flag{cnt+=2;}
        else {cnt+=1}
      }else if right_flag&&mid_r_flag{
        cnt+=1;
      }else if mid_l_flag&&mid_r_flag{
        cnt+=1;
      }
      row_cnt += 1;
      last_row=*row.get(0).unwrap_or(&0);
      left_flag = true;
      mid_l_flag = true;
      mid_r_flag = true;
      right_flag = true;
    }
    match row.get(1).unwrap_or(&0) {
      2 | 3 => left_flag = false,
      4 | 5 => mid_l_flag = false,
      6 | 7 => mid_r_flag = false,
      8 | 9 => right_flag = false,
      _ => {}
    }
  }
  cnt+(n-row_cnt)*2
}

}
