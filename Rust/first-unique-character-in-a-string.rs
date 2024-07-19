impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let n = 256;
        let mut v = vec![0; n];
        for c in s.chars() { 
            let idx = c as usize;
            v[idx] = v[idx]+1;
        }
        for (i, c) in s.chars().enumerate() { 
            let idx = c as usize;
            if v[idx] == 1 {
                return i as i32;
            }
        }
        return -1;
    }
}
