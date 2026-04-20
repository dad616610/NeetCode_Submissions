impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut a = [0; 26];
        let mut b = [0; 26];
        for _s in s.chars() {
            a[(_s as usize) - ('a' as usize)] += 1;
        }
        for _t in t.chars() {
            b[(_t as usize) - ('a' as usize)] += 1;
        }
        for i in 0..26 {
            if a[i] != b[i] {
                return false;
            }
        }
        true
    }
}
