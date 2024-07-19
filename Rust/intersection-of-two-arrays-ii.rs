use std::cmp;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut retVec = Vec::new();
        let mut counters1 = vec![0; 1001];
        let mut counters2 = vec![0; 1001];
        
        for x in nums1 {
            let idx = x as usize; 
            counters1[idx] = counters1[idx]+1;
        }
        
        for x in nums2 {
            let idx = x as usize; 
            counters2[idx] = counters2[idx]+1;
        }
        
        for x in 0..1001 {
            let idx = x as usize; 
            let curMin = cmp::min(counters1[idx], counters2[idx]);
            if curMin != 0 {
                for i in 0..curMin{
                    retVec.push(x);
                }
            } 
        }

        return retVec;
    }
}
