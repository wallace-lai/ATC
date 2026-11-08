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
    // 法一：双哑节点
    // 思路：将所有节点都 move 出来，按照 <x / >= x分到两条新链表尾部，最后拼起来
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        // 两个哑节点
        let mut l = Box::new(ListNode::new(0));
        let mut r = Box::new(ListNode::new(0));

        // tail游标，始终指向链表的最后一个节点
        let mut ltail = &mut l;
        let mut rtail = &mut r;

        let mut curr = head;
        while let Some(mut node) = curr {
            // 关键：先把 node.next take走，让 node 变成干净的孤立节点
            curr = node.next.take();

            if node.val < x {
                ltail.next = Some(node);
                ltail = ltail.next.as_mut().unwrap();
            } else {
                rtail.next = Some(node);
                rtail = rtail.next.as_mut().unwrap();
            }
        }

        // 拼接：小链表表尾 -> 大链表表头
        ltail.next = r.next;
        l.next
    }
}