/*
 * @lc app=leetcode.cn id=242 lang=rust
 *
 * [242] 有效的字母异位词
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let s = s.chars();
        let t = t.chars();
        let mut char_count = HashMap::new();
        for (c1, c2) in s.zip(t) {
            *char_count.entry(c1).or_insert(0) += 1;
            *char_count.entry(c2).or_insert(0) -= 1;
        }
        for (_k, v) in char_count {
            if v != 0 {
                return false;
            }
        }
        true
    }
}
// @lc code=end

