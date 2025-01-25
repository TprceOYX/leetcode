/*
 * @lc app=leetcode.cn id=12 lang=rust
 *
 * [12] 整数转罗马数字
 */

// @lc code=start
const values: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
const symbols: [&str; 13] = [
    "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut res = String::new();
        for (index, value) in values.iter().enumerate() {
            while num >= *value {
                num -= *value;
                res.insert_str(res.len(), symbols.get(index).unwrap());
            }
            if num <= 0 {
                break;
            }
        }
        res
    }
}
// @lc code=end

