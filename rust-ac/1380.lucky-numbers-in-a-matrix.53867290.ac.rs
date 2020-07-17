impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
      let m=matrix.len();
    let n=(&matrix[0]).len();
    println!("{},{}",m,n);
    let mut answer:Vec<i32> =Vec::new();
    for i in 0..m{
        let mut row_min:i32=2147483647;
        let mut col_index:usize=0;
        for j in 0..n{
            let num:i32=matrix[i][j];
         
            if row_min>num{
                row_min=num;
                col_index=j;
            }
        }
          let mut flag:bool=true;
            for k in 0..m{
                let temp=matrix[k][col_index];
                if temp>row_min{
                    flag=false;
                    break;
                }
            }
            if flag{
                answer.push(row_min);
            }
    }
    answer
    }
}
