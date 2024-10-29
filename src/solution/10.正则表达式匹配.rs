/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (sl, pl) = (s.len(), p.len());
        // result[i][j]表示s的前i个字符是否与p的前j个字符匹配
        // 即result[i][j] = is_match(s[0..i]， p[0..j])
        let mut result = vec![vec![false; pl + 1]; sl + 1];
        result[0][0] = true;
        // 判断s中第i个字符是否与p中第j个字符匹配
        let matcher = |i: usize, j: usize| {
            if i == 0 || j == 0 {
                return false;
            }
            // 下标从0开始，所以下标分别是i-1和j-1
            if p[j - 1] as char == '.' {
                return true;
            }
            return s[i - 1] == p[j - 1];
        };
        for i in 0..=sl {
            for j in 1..=pl {
                // p中第j个字符是否为*
                if p[j - 1] as char == '*' {
                    // 第j个字符为*，应该将第j-1和第j个字符视为一个整体
                    if matcher(i, j - 1) {
                        // s[0..i]与p[0..j]匹配的情况有两种
                        // 1. s[0..i-1]与p[0..j]匹配，例如s[0..i]=baa和p[0..j]=baa*，即*号至少用来匹配1次
                        // 2. s[0..i]与p[0..j-2]匹配，例如s[0..i]=ba和p[0..j]=baa*，即*号匹配0次
                        result[i][j] = result[i - 1][j] || result[i][j - 2]
                    } else {
                        // 在s[0..i]与p[0..j]中，此*号只能匹配0次，忽略掉第j-1和第j个字符
                        // 如果s[0..i]与p[0,j-2]匹配，则s[0..i]与p[0..j]匹配
                        result[i][j] = result[i][j - 2]
                    }
                } else {
                    result[i][j] = matcher(i, j) && result[i - 1][j - 1]
                }
            }
        }
        result[sl][pl]
    }

    // 递归方式
    fn matcher(s: &str, p: &str) -> bool {
        let (sb, pb) = (s.as_bytes(), p.as_bytes());
        let (i, j) = (s.len(), p.len());
        if j == 0 {
            return i == 0;
        }
        if i == 0 {
            return j >= 2 && pb[j - 1] as char == '*' && Self::matcher(s, &p[0..j - 2]);
        }
        if pb[j - 1] as char == '*' {
            if sb[i - 1] == pb[j - 2] || pb[j - 2] as char == '.' {
                return (i >= 1 && Self::matcher(&s[0..i - 1], &p))
                    || (j >= 2 && Self::matcher(&s, &p[0..j - 2]));
            } else {
                return j >= 2 && Self::matcher(&s, &p[0..j - 2]);
            }
        } else {
            return (sb[i - 1] == pb[j - 1] || pb[j - 1] as char == '.')
                && Self::matcher(&s[0..i - 1], &p[0..j - 1]);
        }
    }
}
// @lc code=end
