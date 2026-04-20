impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len(){
            return false;
        }
        let mut a = [0i32; 26];
        for (_s, _t) in s.bytes().zip(t.bytes()) {
            a[(_s - b'a') as usize] += 1;
            a[(_t - b'a') as usize] -= 1;
        }
        a.iter().all(|&v| v == 0)
    }
}
