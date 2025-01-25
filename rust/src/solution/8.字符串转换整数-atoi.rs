/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] 字符串转换整数 (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut res: i32 = 0;
        let mut stop = false;
        let mut flag = 1;
        let bs = s.trim().as_bytes();
        for b in bs {
            let c = *b as char;
            match c {
                '0'..='9' => {
                    stop = true;
                    let bit = flag * (b - '0' as u8) as i32;
                    if res > i32::MAX / 10 || res < i32::MIN / 10 {
                        return if res > 0 { i32::MAX } else { i32::MIN };
                    }
                    res *= 10;
                    if (bit >= 0 && res > i32::MAX - bit) || (bit <= 0 && res < i32::MIN - bit) {
                        return if res > 0 { i32::MAX } else { i32::MIN };
                    }
                    res += bit
                }
                '+' | '-' => {
                    if !stop {
                        flag = if c == '+' { 1 } else { -1 };
                        stop = true
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        res
    }
}
// @lc code=end