/*
 * @lc app=leetcode.cn id=796 lang=rust
 *
 * [796] 旋转字符串
 */

// @lc code=start
impl Solution {
    pub fn rotate_string(mut s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        for _x in 0..s.len() {
            let c = s.remove(0);
            s.push(c);
            if s.eq(&goal) {
                return true;
            }
        }
        false
    }
}
// @lc code=end
