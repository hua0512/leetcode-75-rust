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
//
// 328. Odd Even Linked List
// https://leetcode.com/problems/odd-even-linked-list
struct Solution;
impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;

        if head.is_none() {
            return None;
        }

        if head.is_some() && head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut odd = head.as_mut().unwrap();
        let mut even_head = odd.next.take().unwrap();
        let mut even = &mut even_head;

        while let Some(node) = even.next.take() {
            odd.next = Some(node);

            // current node's next is the next odd node
            even.next = odd.next.as_mut().unwrap().next.take();

            // move pointers to the next odd and even nodes
            odd = odd.next.as_mut().unwrap();
            if even.next.is_some() {
                even = even.next.as_mut().unwrap();
            } else {
                break;
            }
        }

        odd.next = Some(even_head);
        head
    }
}
