/*
 * @lc app=leetcode.cn id=724 lang=rust
 *
 * [724] 寻找数组的中心下标
 */

// @lc code=start
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left_sum = 0;
        let mut right_sum = nums.iter().sum::<i32>();
        for i in 0..nums.len() {
            right_sum -= nums[i];
            if left_sum == right_sum {
                return i as i32;
            }
            left_sum += nums[i];
        }
        -1
    }
}
// @lc code=end
