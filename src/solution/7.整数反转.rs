/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut res: i32 = 0;
        let mut x: i32 = x;
        while x != 0 {
            let bit: i32 = x % 10;
            x = (x - bit) / 10;
            match res.checked_mul(10) {
                Some(r) => match r.checked_add(bit) {
                    Some(r) => res = r,
                    None => return 0,
                },
                None => return 0,
            }
        }
        res
    }
}
// @lc code=end