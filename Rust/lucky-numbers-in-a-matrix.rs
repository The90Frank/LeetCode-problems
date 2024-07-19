impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let cardCol = matrix[0].len();
        let cardRow = matrix.len();
        
        let mut ret : Vec<i32> = vec![];
        let mut minsRow : Vec<i32> = vec![i32::MAX; cardRow];
        let mut maxsCol : Vec<i32> = vec![       0; cardCol];
        for i in 0..cardRow {
            for j in 0..cardCol {
                let curVal = matrix[i][j];
                if curVal<minsRow[i] {
                    minsRow[i]=curVal;
                }
                if curVal>maxsCol[j] {
                    maxsCol[j]=curVal;
                }
            }
        }
        for i in 0..cardRow {
            for j in 0..cardCol {
                if minsRow[i] == maxsCol[j] {
                    ret.push(minsRow[i]);
                }
            }
        }
        return ret;
    }
}
