impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut curr_sum = nums[0];
        let mut max_sum = nums[0];
        for i in 1..n {
            curr_sum = max(nums[i], curr_sum + nums[i]);
            max_sum = max(curr_sum, max_sum);
        }
        max_sum
    }
}
