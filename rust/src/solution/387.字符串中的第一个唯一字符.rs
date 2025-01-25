/*
 * @lc app=leetcode.cn id=387 lang=rust
 *
 * [387] 字符串中的第一个唯一字符
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut char_count = HashMap::new();
        for c in s.chars() {
            let count = char_count.entry(c).or_insert(0);
            *count += 1
        }
        for (index, c) in s.chars().enumerate() {
            if *char_count.entry(c).or_default() == 1 {
                return index as i32;
            }
        }
        -1
    }
}
// @lc code=end

