/*
 * @lc app=leetcode.cn id=409 lang=rust
 *
 * [409] 最长回文串
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut char_count = HashMap::new();
        s.chars()
            .for_each(|c| *char_count.entry(c).or_insert(0) += 1);
        let mut res = 0;
        let mut odd = 0;
        for v in char_count.values() {
            let remain = *v % 2;
            res += *v - remain;
            if remain == 1 {
                odd = 1
            }
        }
        res + odd
    }
}
// @lc code=end