/*
 * @lc app=leetcode.cn id=704 lang=rust
 *
 * [704] 二分查找
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (1, nums.len());
        while left <= right {
            let mid = left + (right - left) / 2;
            let n = nums[mid - 1];
            if n == target {
                return mid as i32 - 1;
            } else if n < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        -1
    }
}
// @lc code=end
