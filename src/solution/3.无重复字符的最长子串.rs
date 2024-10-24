/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut result, mut start, mut end) = (0, 0, 0);
        let mut char_index = HashMap::new();
        let bs = s.as_bytes();
        while end < bs.len() {
            let c = bs[end];
            // 遇到重复元素，更新最大长度和start
            if let Some(before) = char_index.insert(c, end) {
                result = result.max(end - start);
                start = start.max(before + 1) // start只能向后不能向前，否则会有重复字符
            }
            end += 1
        }
        // 如果s中所有字符不重复，在此处更新最大长度
        result.max(end - start) as i32
    }
}
// @lc code=end