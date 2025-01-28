/*
 * @lc app=leetcode.cn id=415 lang=rust
 *
 * [415] 字符串相加
 */

// @lc code=start
use std::iter;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        if num2.len() > num1.len() {
            return Solution::add_strings(num2, num1);
        }
        let mut carry = 0;
        let mut res = num1
            .chars()
            .rev()
            .zip(
                num2.chars()
                    .rev()
                    .chain(iter::repeat('0').take(num1.len() - num2.len())), // 补齐
            )
            .map(|(a, b)| {
                let cur = carry + a.to_digit(10).unwrap() + b.to_digit(10).unwrap();
                carry = cur / 10;
                char::from_digit(cur % 10, 10).unwrap()
            })
            .collect::<Vec<_>>();
        if carry > 0 {
            res.push('1');
        }
        res.iter().rev().collect()
    }
}
// @lc code=end