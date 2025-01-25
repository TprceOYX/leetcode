/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut res = String::new();
        let bs = s.as_bytes();
        let step = (2 * num_rows - 2) as usize;
        for row in 0..num_rows {
            let mut index = row as usize;
            let mut count = 1;
            let s = (num_rows - 1 - row) as usize * 2;
            while index < bs.len() {
                res.push(bs[index] as char);
                count += 1;
                if row == 0 || row == num_rows - 1 {
                    index += step
                } else {
                    if count % 2 == 1 {
                        index += step - s
                    } else {
                        index += s
                    }
                }
            }
        }
        res
    }
}
// @lc code=end

pub fn convert(s: String, num_rows: i32) -> String {
    // 为什么输入类型总是i32
    let num_rows = num_rows as usize;
    // 每行一个String
    let mut rows = vec![String::new(); num_rows];
    // z字形往复的迭代器，01232101232......
    let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();
    // 按迭代器给出的下标访问对应行，推入字符
    iter.zip(s.chars()).for_each(|(i, c)| rows[i].push(c));
    // collect连接每行
    rows.into_iter().collect()
}

