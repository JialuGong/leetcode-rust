impl Solution {
  fn get_max_common_factor(x: i32, y: i32) -> i32 {
    let mut larger_number = i32::max(x, y);
    let mut smaller_number = i32::min(x, y);
    while larger_number != smaller_number {
      let tmp_lar = larger_number;
      let tmp_sma = smaller_number;
      larger_number = i32::max(tmp_lar - tmp_sma, tmp_sma);
      smaller_number = i32::min(tmp_lar - tmp_sma, tmp_sma);
    }
    smaller_number
  }

  pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
    if z == 0 {
      return true;
    }
    if x == 0 {
      if y == z {
        return true;
      } else {
        return false;
      }
    }
    if y == 0 {
      if x == z {
        return true;
      } else {
        return false;
      }
    }
    let divisor = Self::get_max_common_factor(x, y);
    if z % divisor != 0 {
      false
    } else {
      if z > x + y {
        false
      } else {
        true
      }
    }
  }
}
