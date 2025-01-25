/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let s0 = &strs[0];
        for (i, b) in s0.bytes().enumerate() {
            for j in 1..strs.len() {
                if i >= strs[j].len() || strs[j].as_bytes()[i] != b {
                    return s0[..i].to_string();
                }
            }
        }
        s0.clone()
    }
}

// @lc code=end