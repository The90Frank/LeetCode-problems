fn is_all_same(arr: &Vec<i32>) -> bool {
    if arr.is_empty() {
        return true;
    }
    let first = arr[0];
    return arr.iter().all(|&item| item == first);
}

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 3 || grid[0].len() < 3 {
            return 0;
        }
        let mut counter = 0;

        for pivot_x in 0..grid.len()-2 {
            'nextMatrix: for pivot_y in 0..grid[pivot_x].len()-2 {
                let mut value = vec![0; 9];
                let mut sum_x = vec![0,0,0];
                let mut sum_y = vec![0,0,0];
                let mut diags = vec![0,0];  
                for x in 0..3 {
                    for y in 0..3 {
                        let val = grid[pivot_x+x][pivot_y+y];
                        if val > 9 || val <= 0 {
                            continue 'nextMatrix;
                        }
                        
                        value[(val-1) as usize] += 1;
                        sum_x[x] += val;
                        sum_y[y] += val;
                        if (  x == y  ) { diags[0] += val; }
                        if ( x+y == 2 ) { diags[1] += val; }
                    }
                }
                if (is_all_same(&value) &&
                    is_all_same(&sum_x) && 
                    is_all_same(&sum_y) && 
                    is_all_same(&diags) && 
                    sum_x[0]==sum_y[0]  && 
                    sum_x[0]==diags[0]  ){
                        counter += 1;
                    }
            }
        }

        return counter;
    }
}
