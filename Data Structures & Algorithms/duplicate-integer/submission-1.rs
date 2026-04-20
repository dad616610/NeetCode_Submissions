impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let s: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        s.len() != len
    }
}
