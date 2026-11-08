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
    #[cfg(any())]
    // 法一
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l = Box::new(ListNode::new(0));
        let mut ltail = &mut l;

        let mut sum = 0;
        let mut carry = 0;
        let mut p1 = &l1;
        let mut p2 = &l2;
        while let Some(n1) = p1 && let Some(n2) = p2 {
            sum = n1.val + n2.val + carry;
            carry = sum / 10;
            sum %= 10;

            let n = Box::new(ListNode::new(sum));
            ltail.next = Some(n);
            ltail = ltail.next.as_mut().unwrap();

            p1 = &n1.as_ref().next;
            p2 = &n2.as_ref().next;
        }

        while let Some(n1) = p1 {
            sum = n1.val + carry;
            carry = sum / 10;
            sum %= 10;

            let n = Box::new(ListNode::new(sum));
            ltail.next = Some(n);
            ltail = ltail.next.as_mut().unwrap();

            p1 = &n1.as_ref().next;
        }

        while let Some(n2) = p2 {
            sum = n2.val + carry;
            carry = sum / 10;
            sum %= 10;

            let n = Box::new(ListNode::new(sum));
            ltail.next = Some(n);
            ltail = ltail.next.as_mut().unwrap();

            p2 = &n2.as_ref().next;
        }
    
        if carry > 0 {
            let n = Box::new(ListNode::new(carry));
            ltail.next = Some(n);
        }

        l.next
    }

    // 法二：法一改进版
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        let mut carry = 0;
        let mut p1 = l1.as_deref();
        let mut p2 = l2.as_deref();

        while p1.is_some() || p2.is_some() || carry > 0 {
            let mut sum = carry;

            if let Some(n) = p1 {
                sum += n.val;
                p1 = n.next.as_deref();
            }
            if let Some(n) = p2 {
                sum += n.val;
                p2 = n.next.as_deref();
            }

            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}