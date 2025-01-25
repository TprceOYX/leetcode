/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let (mut reverted, mut x) = (0, x);
        while x > reverted {
            reverted = reverted * 10 + x % 10;
            x /= 10
        }
        return x == reverted || x == reverted / 10;
    }
}
// @lc code=end

