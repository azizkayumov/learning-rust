// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/

// Definition for singly-linked list.
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

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    -1
}

#[test]
fn test_get_decimal_value() {
    let expected = 5;

    let mut node3 = ListNode::new(1);

    let mut node2 = ListNode::new(0);
    node2.next = Option::Some(Box::new(node3));

    let mut node1 = ListNode::new(1);
    node1.next = Option::Some(Box::new(node2));

    let output = get_decimal_value(Option::Some(Box::new(node1)));
    assert_eq!(expected, output);
}
