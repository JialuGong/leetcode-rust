impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
  let mut answer:i32=0;
  let mut x:usize=9;
  let mut y:usize=9;

  //find the rook
  for i in 0..8{
    for j in 0..8{
      if board[i][j]=='R'{
        x=i;
        y=j;
      }
    }
  }

  if x==9||y==9{
      return 0;
  }
  //find answer
  for i in x+1..8{
    if board[i][y]=='B'{break;}
    if board[i][y]=='p'{answer+=1;break;}
  }

  for i in (0..x).rev(){
    if board[i][y]=='B'{break;}
    if board[i][y]=='p'{answer+=1;break;}
  }

  for i in y+1..8{
    if board[x][i]=='B'{break;}
    if board[x][i]=='p'{answer+=1;break;}
  }

  for i in (0..y).rev(){
     if board[x][i]=='B'{break;}
    if board[x][i]=='p'{answer+=1;break;}
  }
  answer
}
}
