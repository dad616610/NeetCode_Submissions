impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut arr: Vec<i32> = Vec::with_capacity(2 * nums.len());
        for i in 0..2 {
            for n in &nums {
                arr.push(*n)
            }
        }
        arr
    }
}
