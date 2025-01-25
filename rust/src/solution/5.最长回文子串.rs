/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let (mut start, mut end) = (0, 0);
        let bs = s.as_bytes();
        // 计算以任意位置为中心找到的最长回文子串长度
        for i in 0..bs.len() as i32 {
            let length = method(bs, i, i).max(method(bs, i, i + 1));
            if length > end - start {
                start = i - (length - 1) / 2;
                end = i + length / 2
            }
        }
        // 不包括右边界，所以end+1作为右边界
        s[start as usize..(end + 1) as usize].to_string()
    }
}

pub fn method(s: &[u8], mut left: i32, mut right: i32) -> i32 {
    // 从left、right向两侧移动，找到最长的回文子串
    while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
        left -= 1;
        right += 1
    }
    // 跳出循环时，left+1是实际左边界，right-1是实际右边界
    right - left - 1
}

// @lc code=end

