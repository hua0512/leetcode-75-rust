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
// 2095. Delete the Middle Node of a Linked List
// https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list
impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        if head.is_none() {
            return None;
        }

        let mut head = head.unwrap();

        if head.next.is_none() {
            return None;
        }

        // think: we use fast and slow pointers
        let mut slow = &raw mut head;
        let mut fast = &raw const head;
        let mut prev = slow;

        // think:
        // when fast traverse 2n, slow traverse n
        // so we get the middle node
        // 0 indexed, so we need the previous node
        while unsafe {(*fast).next.is_some()} {
            fast = unsafe {&raw const *(*fast).next.as_ref().unwrap()};
            if let Some(next) = unsafe {(*fast).next.as_ref()}{
                fast = &raw const *next;
            }
            prev = slow;
            slow = unsafe {&raw mut *(*slow).next.as_mut().unwrap()};
        }

        // delete middle
        unsafe {
            (*prev).next = (*prev).next.take().unwrap().next;
        }

        Some(head)

    }
}
