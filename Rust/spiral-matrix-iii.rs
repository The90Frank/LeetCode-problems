enum Direction {
    Right, 
    Down, 
    Left, 
    Up
}

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut ret_arr : Vec<Vec<i32>> = vec![];

        let spiral_len_molt = 2;
        let mut spiral_len = 1;

        let mut left_up = false;
        let mut left_down = false;
        let mut right_up = false;
        let mut right_down = false;

        let mut x = r_start;
        let mut y = c_start;
        let mut direction = Direction::Right;

        while !left_up || !left_down || !right_up || !right_down {
            for j in 0..spiral_len_molt {
                for i in 0..spiral_len {
                    if x >= 0 && x < rows && y >= 0 && y < cols {
                        ret_arr.push(vec![x, y]);
                    }

                    if x == 0 && y == 0{
                        left_up = true;
                    } 
                    if x == 0 && y == cols-1{
                        right_up = true;
                    }
                    if x == rows-1 && y == 0{
                        left_down = true;
                    }
                    if x == rows-1 && y == cols-1{
                        right_down = true;
                    }

                    match direction {
                        Direction::Right   => y+=1,
                        Direction::Down    => x+=1, 
                        Direction::Left    => y-=1, 
                        Direction::Up      => x-=1,
                    }
                }
                match direction {
                    Direction::Right   => direction=Direction::Down,
                    Direction::Down    => direction=Direction::Left, 
                    Direction::Left    => direction=Direction::Up, 
                    Direction::Up      => direction=Direction::Right,
                }
            }
            spiral_len += 1;
        }

        return ret_arr;
    }
}
