struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
/*
 * @lc app=leetcode.cn id=876 lang=rust
 *
 * [876] 链表的中间结点
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut h1, mut h2) = (head.as_ref(), head.as_ref());
        while h2.is_some() && h2.unwrap().next.is_some(){
            h1 = h1.unwrap().next.as_ref();
            h2 = h2.unwrap().next.as_ref().unwrap().next.as_ref();
        }
        Some(h1.unwrap().to_owned())
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        // println!("res: {}",Solution::is_subsequence(s, t))
    }
}
