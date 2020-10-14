/*
You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.
*/

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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);

        let mut one = l1.unwrap();
        let mut two = l2.unwrap();
        
        let mut sol = Solution::make_node(one.val + two.val);
        dummy.next.get_or_insert(Box::new(sol.0));

        let mut curr = &mut dummy.next;
        while one.next.is_some() || two.next.is_some() {
            match curr{
                None => break,
                Some(now) => {
                    one = one.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
                    two = two.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
                    sol = Solution::make_node(one.val + two.val + res.1);
                    now.next.get_or_insert(Box::new(sol.0));

                    curr = &mut now.next;
                }
            }
        }

        if sol.1 > 0 {
            if let Some(now) = curr{
                now.next.get_or_insert(Box::new(ListNode::new(sol.1)));
            }
        }

        dummy.next
    }

    fn make_node(mut result: i32) -> (ListNode, i32) {
        let remainder;
        if result > 9 {
            remainder = 1;
            result = result - 10;
        } else {
            remainder = 0;
        }
        (ListNode::new(result), remainder)
    }
}