impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut minPrefix = strs[0].as_str();
        let mut minPrefixSize = minPrefix.chars().count();
        for curStr in strs.iter() {
            if curStr.chars().count() < minPrefixSize {
                minPrefixSize = curStr.chars().count();
            }
            let mut x = minPrefixSize+1;
            for x in (0..minPrefixSize+1).rev() {
                if minPrefix[..x] == curStr[..x] {
                    minPrefixSize = x;
                    break;
                }
            }
            if x == 0 {
                minPrefixSize = x;
                break;
            }
        }
        return minPrefix[..minPrefixSize].to_string();
    }
}
