impl Solution {
    fn maxval_dist(q: &Vec<i32>, dist: &Vec<f64>) -> usize {
        let mut max = dist[q[0] as usize];
        let mut max_idx = q[0];
        for i in q {
            if max < dist[*i as usize] {
                max = dist[*i as usize];
                max_idx = *i;
            }
        }
        return max_idx as usize;
    }

    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let mut matrix_graph = vec![(vec![0.0; n as usize]); n as usize];

        for i in 0..edges.len() {
            let edge = &edges[i];
            let prob = succ_prob[i];
            matrix_graph[edge[0] as usize][edge[1] as usize] = prob;
            matrix_graph[edge[1] as usize][edge[0] as usize] = prob; 
        }
        
        let mut dist = vec![0.0; n as usize];
        let mut q: Vec<i32> = (0..n).collect();
        dist[start_node as usize] = 1.0;

        while q.len() != 0 {
            let u = Self::maxval_dist(&q, &dist);
            q.retain(|&x| x as usize != u);
            let neighbor = matrix_graph[u].clone();
            for i in 0..neighbor.len() {
                if neighbor[i] != 0.0 {
                    let alt = dist[u] * matrix_graph[u][i];
                    if alt > dist[i] {
                        dist[i] = alt;
                    }
                }
            }
        }

        return dist[end_node as usize];       
    }
}
