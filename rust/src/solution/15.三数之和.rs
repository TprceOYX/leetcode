/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::real_three_sum(nums)
    }
    fn real_three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let length = nums.len();
        nums.sort();
        if nums[0] > 0 || nums[length - 1] < 0 {
            // 全为正数或者全为负数
            return res;
        }
        let max_two_sum = nums[length - 1] + nums[length - 2];
        for i in 0..length - 2 {
            let first = nums[i];
            if first > 0 {
                // 后两个数也一定是正数，不必继续寻找
                break;
            }
            if (i > 0 && first == nums[i - 1]) || first + max_two_sum < 0 {
                // 跳过重复的数和过小的数
                continue;
            }
            let (mut start, mut end) = (i + 1, length - 1);
            while end > start {
                let sum = first + nums[start] + nums[end];
                if sum == 0 {
                    res.push(vec![first, nums[start], nums[end]]);
                    while start + 1 < end && nums[start] == nums[start + 1] {
                        start += 1
                    }
                    start += 1;
                    while start < end - 1 && nums[end] == nums[end - 1] {
                        end -= 1
                    }
                    end -= 1
                } else if sum > 0 {
                    end -= 1
                } else {
                    start += 1
                }
            }
        }
        res
    }
}
// @lc code=end