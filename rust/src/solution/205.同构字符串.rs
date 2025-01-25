/*
 * @lc app=leetcode.cn id=205 lang=rust
 *
 * [205] 同构字符串
 */

// @lc code=start
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut m = HashMap::new();
        let mut used = HashSet::new();
        for (c1, c2) in s.chars().zip(t.chars()) {
            let exist = m.contains_key(&c1);
            let c = m.entry(c1).or_insert(c2);
            if *c != c2 || (!exist && used.contains(&c2)) {
                return false;
            }
            used.insert(c2);
        }
        true
    }
}
// @lc code=end

