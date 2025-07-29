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
//
// 2130. Maximum Twin Sum of a Linked List
// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list
struct Solution;

impl Solution {

    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut current = head.as_ref();
        let mut n = 0;

        // 1. compute the length of the linkedlist
        while current.is_some() {
            n += 1;
            current = current.unwrap().next.as_ref();
        }

        // 2. reverse the first half of linkedlist
        let mut head = head;
        let mut prev = None;
        let mut next = None;

        for _ in 0..n / 2 {
            if let Some(mut node) = head {
                next = node.next.take();
                node.next = prev;
                prev = Some(node);
                head = next;
            }
        }

        // 3. compute the maximum sum of the pairs
        let mut max_sum = 0;

        while let (Some(node1), Some(node2)) = (prev, head) {
            max_sum = max_sum.max(node1.val + node2.val);
            prev = node1.next;
            head = node2.next;
        }

        max_sum
    }
}
