/*
 * @lc app=leetcode.cn id=151 lang=rust
 *
 * [151] 反转字符串中的单词
 */

// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let s = s.trim();
        let b = s.as_bytes();
        let length = b.len() as i32;
        let (mut start, mut end) = (length - 1, length - 1);
        let mut res: Vec<u8> = vec![];
        while start >= 0 {
            while start >= 0 && b[start as usize] != b' ' {
                start -= 1
            }
            res.extend_from_slice(&b[(start + 1) as usize..(end + 1) as usize]);
            res.push(b' ');
            while start >= 0 && b[start as usize] == b' ' {
                start -= 1
            }
            end = start
        }
        res.remove(res.len()-1);
        String::from_utf8(res).unwrap()
    }
}
// @lc code=end