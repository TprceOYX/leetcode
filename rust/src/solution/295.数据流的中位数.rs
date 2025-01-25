/*
 * @lc app=leetcode.cn id=295 lang=rust
 *
 * [295] 数据流的中位数
 */

// @lc code=start
struct MedianFinder {
    Data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder { Data: vec![] }
    }

    fn add_num(&mut self, num: i32) {
        let length = self.Data.len();
        if length == 0 || num >= self.Data[length-1]{
            self.Data.push(num);
            return
        }
        if num <= self.Data[0]{
            self.Data.insert(0, num);
            return;
        }
        // 二分查找到插入位置
        let (mut left, mut right) = (0, length as i32 - 1);
        while left <= right {
            let mid = (left + (right - left) / 2) as usize;
            let value = self.Data[mid];
            if value == num {
                left = mid as i32;
                break;
            } else if value > num {
                right = mid as i32 - 1
            } else {
                left = mid as i32 + 1
            }
        }
        self.Data.insert(left as usize, num);
    }

    fn find_median(&self) -> f64 {
        let length = self.Data.len();
        if length % 2 == 1 {
            self.Data[length / 2] as f64
        } else {
            (self.Data[length / 2] + self.Data[length / 2 - 1]) as f64 / 2.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
// @lc code=end