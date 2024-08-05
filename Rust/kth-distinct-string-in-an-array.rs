impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut is_unique_arr = vec![true; arr.len()];

        for idx in 0..arr.len()-1 {
            if is_unique_arr[idx]{
                for sub_idx in idx+1..arr.len() {
                    if is_unique_arr[sub_idx] && arr[idx] == arr[sub_idx] {
                        is_unique_arr[sub_idx] = false;
                        is_unique_arr[idx] = false;
                    }
                }
            }
        }

        let mut counter = 0;
        for idx in 0..arr.len() {
            if is_unique_arr[idx] {
                counter += 1;
                if counter == k {
                    return arr[idx].clone();
                }
            }
        }

        return "".to_string();
    }
}
