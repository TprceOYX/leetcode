/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let m = HashMap::from([
            (b'I', 1),
            (b'V', 5),
            (b'X', 10),
            (b'L', 50),
            (b'C', 100),
            (b'D', 500),
            (b'M', 1000),
        ]);
        let s = s.as_bytes();
        let mut res = 0;
        for i in 0..s.len() - 1 {
            let (a, b) = (m[&s[i]], m[&s[i + 1]]);
            if a >= b {
                res += a
            } else {
                res -= a
            }
        }
        res + m[&s[s.len() - 1]]
    }
}
// @lc code=end

