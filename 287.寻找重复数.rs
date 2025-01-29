/*
 * @lc app=leetcode.cn id=287 lang=rust
 *
 * [287] 寻找重复数
 */

// @lc code=start
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
       let (mut slow, mut fast) = (0, 0);
       loop {
           slow = nums[slow] as usize;
           fast = nums[nums[fast] as usize] as usize;
           if slow == fast {
               fast = 0;
               while slow != fast {
                   slow = nums[slow] as usize;
                   fast = nums[fast] as usize;
               }
               return slow as i32;
           }
       }
    }
}
// @lc code=end

