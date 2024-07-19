impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut counter = 0;
        for x in arr.iter(){
            if x%2 == 1 {
                counter = counter+1;
                if counter == 3 {
                    return true;
                }
            } else {
                counter = 0;
            }
        }
        return false;
    }
}
