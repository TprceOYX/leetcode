/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(a: Vec<i32>, b: Vec<i32>) -> f64 {
        let (m, n) = (a.len(), b.len());
        if m > n {
            return Self::find_median_sorted_arrays(b, a);
        }
        // 中位数将a+b的集合分成两部分，第一部分的最大值小于等于第二部分的最小值
        // 如果m+n为偶数，第一部分的元素数量为(m+n)/2，中位数为(max(part1) + min(part2)) / 2
        // 如果m+n为奇数，可以把中位数归入第一部分，这样第一部分元素数为(m+n+1)/2，中位数为max(part1)

        // 假设a数组有i个数位于第一部分，b数组有j个数位于第一部分，满足以下条件
        // 1.i+j=(m+n+1)/2，i的取值范围为[0, m]
        // 2.max(a[i-1], b[j-1]) <= min(a[i], b[j])，下标是从0开始的，第i个数的下标是i-1
        //   因为a[i-1]<=a[i] && b[j-1]<=b[j]，所以只需判断a[i-1]<=b[j] && b[j-1]<=a[i]
        //   i=0以及j=0时，a[i-1]和b[j-1]置为-∞
        //   i=m以及j=n时，a[i]和b[j]置为∞

        let (mut i, mut j) = (0 as usize, (m + n + 1) / 2);
        // 遍历i的取值，找到符合a[i-1]<=b[j] && b[j-1]<=a[i]条件的值
        loop {
            let a1 = if i <= 0 { i32::MIN } else { a[i - 1] };
            let a2 = if i >= m { i32::MAX } else { a[i] };
            let b1 = if j <= 0 { i32::MIN } else { b[j - 1] };
            let b2 = if j >= n { i32::MAX } else { b[j] };
            if a1 <= b2 && b1 <= a2 {
                return if (m + n) % 2 == 1 {
                    a1.max(b1) as f64
                } else {
                    (a1.max(b1) + a2.min(b2)) as f64 / 2.0
                };
            }
            i += 1;
            j -= 1;
        }
    }
}
// @lc code=end

