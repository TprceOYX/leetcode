package main

/*
 * @lc app=leetcode.cn id=142 lang=golang
 *
 * [142] 环形链表 II
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func detectCycle(head *ListNode) *ListNode {
	fast, slow := head, head
	// 假设从链表头走a步进入环，环的边数量为b
	// fast步进为2，slow步进为1，假设循环轮次为n时两个指针相遇
	// 那么slow向前移动了n步，fast向前移动了2n步
	// 另外fast遇到slow之前一定已经在环中绕了x圈
	// 所以fast移动步数也等于n+xb，那么可以知道循环轮次n=xb
	for {
		if fast == nil || fast.Next == nil { // 无环的情况
			return nil
		}
		fast = fast.Next.Next
		slow = slow.Next
		if fast == slow {
			break
		}
	}
	// slow指针每次到达环的入口时，其前进的步数一定等于a+xb
	// 即走a步后第一次到达环入口，之后每前进b步重新回到环入口
	// 所以让fast重新回到链表头，并且步进为1，这样fast和slow走a步后都会到达环入口
	fast = head
	for fast != slow {
		fast = fast.Next
		slow = slow.Next
	}
	return fast
}

// @lc code=end
