/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // method(l1, l2, 0)
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut sum = 0;
        let mut head = ListNode::new(0);
        let mut ptr = &mut head;
        while l1.is_some() || l2.is_some() || sum > 0 {
            if let Some(x) = l1{
               sum += x.val;
               l1 = &x.next
            }
            if let Some(x) = l2{
                sum += x.val;
                l2 = &x.next
             }
            ptr.next = Some(Box::new(ListNode::new(sum % 10)));
            ptr = ptr.next.as_mut().unwrap();
            sum /= 10
        }
        head.next
    }
}

pub fn method(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    mut num: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && num == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: method(
                // l1不为空执行这个闭包，闭包的返回值即为and_then的返回值（Some）
                // 为空and_then返回None
                // 会转移l1的所有权
                l1.and_then(|x| {
                    num += x.val;
                    x.next
                }),
                l2.and_then(|x| {
                    num += x.val;
                    x.next
                }),
                num / 10,
            ),
            val: num % 10,
        }))
    }
}
// @lc code=end

