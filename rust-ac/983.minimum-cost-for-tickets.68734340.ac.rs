impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut days_cost = vec![0];
        let (day, week, month) = (costs[0], costs[1], costs[2]);
        let mut i = 0;
        for j in 1..366 {
            if i == days.len() {
                break;
            }
            if j == days[i] {
                i += 1;
                days_cost.push(
                    (i32::min(
                        days_cost[if j > 1 { j as usize - 1 } else { 0 }] + day,
                        days_cost[if j > 7 { j as usize - 7 } else { 0 }] + week,
                    ))
                    .min(days_cost[if j > 30 { j as usize - 30 } else { 0 }] + month),
                );
            }else{
                  days_cost.push(days_cost[j as usize -1]);
            }
        }

        *days_cost.last().unwrap()
    }
}
