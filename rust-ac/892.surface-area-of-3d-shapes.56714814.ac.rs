impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    let mut cnt = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let current=grid[i][j];
            if current!=0{ cnt += 2 + 4 * current;}
            cnt -= Self::get_one_surface(&grid, i as i32- 1, j as i32).min(current);
            cnt -= Self::get_one_surface(&grid, i as i32, j as i32 - 1).min(current);
            cnt -= Self::get_one_surface(&grid, i as i32 + 1, j as i32).min(current);
            cnt -= Self::get_one_surface(&grid, i as i32, j as i32+ 1).min(current);
        }
    }
    cnt
}

fn get_one_surface(grid: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    let blank_vec:Vec<i32>=vec![];
    if i < 0 {
        return 0;
    }
    if j < 0 {
        return 0;
    }
    match (grid.get(i as usize).unwrap_or(&blank_vec)).get(j as usize) {
        Some(&num) => num,
        None => 0,
    }
}
}
