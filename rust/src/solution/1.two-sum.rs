/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_index: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            num_index.insert(nums[i], i);
        }
        for i in 0..nums.len() {
            if let Some(j) = num_index.get(&(target - nums[i])) {
                if i != *j {
                    return vec![i as i32, *j as i32];
                }
            }
        }
        vec![]
    }
}
// @lc code=end
