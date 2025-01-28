/*
 * @lc app=leetcode.cn id=946 lang=rust
 *
 * [946] 验证栈序列
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        // 每次弹出一个元素，所有先于此元素的入栈的其它元素都必须后弹出，且弹出的顺序与入栈的顺序相反
        let mut pushed_order = HashMap::new();

        for (value, order) in pushed.iter().zip(1..pushed.len() + 1) {
            pushed_order.insert(value, order);
        }
        let popped = popped.iter().map(|x| pushed_order[x]).collect::<Vec<_>>();
        let mut pop = popped[0];
        let mut bitmap = vec![false; popped.len() + 1];
        bitmap[pop] = true;
        let mut top = pop - 1;
        for x in 1..popped.len() {
            pop = popped[x];
            if pop < top {
                return false;
            }
            bitmap[pop] = true;
            top = pop - 1;
            while bitmap[top] {
                top -= 1;
            }
        }
        true
    }
    pub fn validate_stack_sequences2(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stk = VecDeque::new();
        let mut i = 0;
        for x in pushed {
            stk.push_back(x);
            while !stk.is_empty() && *stk.back().unwrap() == popped[i] {
                stk.pop_back();
                i += 1;
            }
        }
        stk.is_empty()
    }
}
// @lc code=end
