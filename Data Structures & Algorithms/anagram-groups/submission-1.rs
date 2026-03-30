impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res: HashMap<String, Vec<String>> = HashMap::with_capacity(strs.len());
        for s in strs {
            let mut ch: Vec<char> = s.chars().collect();
            ch.sort_unstable();
            let key = ch.into_iter().collect();

            res.entry(key).or_insert_with(Vec::new).push(s);
        }
        res.into_values().collect()
    }
}
