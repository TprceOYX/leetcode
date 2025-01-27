package main

type ListNode struct {
	Val  int
	Next *ListNode
}

/*
 * @lc app=leetcode.cn id=160 lang=golang
 *
 * [160] 相交链表
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

func getIntersectionNode(headA, headB *ListNode) *ListNode {
	a, b := headA, headB
	// 假设headA共有a条边，headB共有b条边，两条链表有c条公共边
	// 让指针a从headA的头部向后移动，到达尾部后再从headB的头部向后移动
	// 让指针b从headB的头部向后移动，到达尾部后再从headA的头部向后移动
	// 如果链表不相交，两个指针都走了a+b步到达nil节点
	// 如果两个链表相交，两个指针都走了a+b-c步到达相交节点
	// 还有一种a=b的特殊情况
	for a != b {
		if a == nil {
			a = headB
		} else {
			a = a.Next
		}

		if b == nil {
			b = headA
		} else {
			b = b.Next
		}
	}
	return a
}

// @lc code=end
