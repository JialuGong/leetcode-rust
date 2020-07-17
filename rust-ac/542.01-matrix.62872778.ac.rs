impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for i in 0..m {
            let mut row: Vec<i32> = Vec::new();
            let mut queue: Vec<(usize, usize, i32)> = Vec::new();
            for j in 0..n {
                if matrix[i][j] == 0 {
                    row.push(0);
                } else {
                    let mut min = i32::max_value();
                    queue.push((i, j, 1));
                    while !queue.is_empty() {
                        let cur_node = queue.remove(0);
                        let (x, y, cur) = (cur_node.0, cur_node.1, cur_node.2);

                        //left
                        if y > 0 {
                            if matrix[x][y - 1] == 0 {
                                min = min.min(cur);
                            } else {
                                if x < i && y - 1 < j {
                                    min = min.min(cur + ans[x][y - 1]);
                                } else {
                                    if cur + 1 < min {
                                        queue.push((x, y - 1, cur + 1));
                                    }
                                }
                            }
                        }

                        //right
                        if y + 1 < n {
                            if matrix[x][y + 1] == 0 {
                                min = min.min(cur);
                            } else {
                                if x < i && y + 1 < j {
                                    min = min.min(cur + ans[x][y + 1]);
                                } else {
                                    if cur + 1 < min {
                                        queue.push((x, y + 1, cur + 1));
                                    }
                                }
                            }
                        }

                        //up
                        if x > 0 {
                            if matrix[x - 1][y] == 0 {
                                min = min.min(cur);
                            } else {
                                if x - 1 < i && y < j {
                                    min = min.min(cur + ans[x - 1][y]);
                                } else {
                                    if cur + 1 < min {
                                        queue.push((x - 1, y, cur + 1));
                                    }
                                }
                            }
                        }
                        //down
                        if x + 1 < m {
                            if matrix[x + 1][y] == 0 {
                                min = min.min(cur);
                            } else {
                                if x + 1 < i && y < j {
                                    min = min.min(cur + ans[x + 1][y])
                                } else {
                                    if cur + 1 < min {
                                        queue.push((x + 1, y, cur + 1));
                                    }
                                }
                            }
                        }
                    }
                    row.push(min);
                }
            }
            ans.push(row);
        }
        ans
    }
}
