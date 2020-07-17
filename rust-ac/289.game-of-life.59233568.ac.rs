impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let mut chang_vec:Vec<(usize,usize,i32)>=Vec::new();
    let m=board.len();
    if m!=0{
         let n=board[0].len();
    for i in 0..m{
        for j in 0..n{
            let alive_cell_cnt=Self::get_cell_cnt(&board, i, j, 1);
            if board[i][j]==1{
                if alive_cell_cnt<2||alive_cell_cnt>3{
                    chang_vec.push((i,j,0));
                }
            }else{
                if alive_cell_cnt==3{
                    chang_vec.push((i,j,1));
                }
            }
        }
    }
    chang_vec.iter().for_each(|x|{
        board[x.0][x.1]=x.2;
    });

    }
   
}
fn get_cell_cnt(board: &Vec<Vec<i32>>, x: usize, y: usize, t: i32) -> i32 {
    let mut cnt = 0;
    let m = board.len();
    let n = board[0].len();
    for i in 0..3 {
        for j in 0..3 {
            if x +i>=1 && x + i - 1 < m && y +j>=1 && y + j - 1 < n  {
              
                if board[x + i - 1][y + j - 1] == t {
                    cnt += 1;
                }
            }
        }
       
    }
    if board[x][y]==t {cnt=0.max(cnt-1)};
    cnt
}
}
