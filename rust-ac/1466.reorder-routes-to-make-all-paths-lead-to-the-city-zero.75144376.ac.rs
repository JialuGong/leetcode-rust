impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut union_set=vec![vec![];n as usize];
        for (i,connect) in connections.iter().enumerate(){
            union_set[connect[0] as usize].push(i);
            union_set[connect[1] as usize].push(i);
        }
        Self::bfs(&connections,0,n,-1,&union_set)
    }
    fn bfs(connections:&Vec<Vec<i32>>,root:i32,n:i32,pre_root:i32,union_set:&Vec<Vec<usize>>)->i32{
        if n==0{
            return 0;
        }
        println!("root is {}",root);
        let mut ans=0;
        for i in &union_set[root as usize]{
            let (num1,num2)=(connections[*i][0],connections[*i][1]);
            if num1==root&&num2!=pre_root{
                ans+=1;
                ans+=Self::bfs(connections,num2,n-1,root,union_set);
            }
            if num2==root&&num1!=pre_root{
                ans+=Self::bfs(connections,num1,n-1,root,union_set);
            }
        }
        ans
    }
}
