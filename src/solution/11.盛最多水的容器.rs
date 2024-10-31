/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let (mut left, mut right) = (0, height.len() - 1);
        while right > left {
            if height[right] > height[left] {
                result = result.max((right - left) as i32 * height[left]);
                left += 1
            } else {
                result = result.max((right - left) as i32 * height[right]);
                right -= 1
            }
        }
        result
    }
}
// @lc code=end

