#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    // 法一：快慢指针
    pub fn kth_to_last(head: Option<Box<ListNode>>, k: i32) -> i32 {
        let mut fast = &head;
        let mut slow = &head;

        for _ in 0..k {
            fast = &fast.as_ref().unwrap().next;
        }

        while let Some(node) = fast {
            fast = &node.next;
            slow = &slow.as_ref().unwrap().next;
        }

        slow.as_ref().unwrap().val
    }
}