impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut p = strs[0].clone();
        for s in strs.iter().skip(1) {
            while !s.starts_with(&p) {
                p.pop();
            }
        }
        p
    }
}
