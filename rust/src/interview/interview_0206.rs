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
    // 法一：转数组后用双指针
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut v = Vec::new();

        let mut curr = head.as_ref();
        while let Some(n) = curr {
            v.push(n.val);
            curr = n.next.as_ref();
        }

        v.iter().eq(v.iter().rev())
    }

    // 法二：快慢指针 + 反转后半段
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        // 数长度
        let mut n = 0;
        let mut curr = head.as_ref();
        while let Some(node) = curr {
          n += 1;
          curr = node.next.as_ref();
        }

        // 把后半段从原链表上切下来（take出next字段，实现move）
        let half = n / 2;
        let mut curr = &mut head;
        for _ in 0..half {
            curr = &mut curr.as_mut().unwrap().next;
        }

        // 此时 curr 指向后半段的起点，如果 n 是奇数，中间那个节点
        // 属于后半段，不影响回文判断。把后半段 move 出来
        let second = curr.take();

        // 反转后半段
        let mut rev = None;
        let mut second = second;
        while let Some(mut node) = second {
            second = node.next.take();
            node.next = rev;
            rev = Some(node);
        }

        // 逐节点比较，如果 n 是奇数，那么 b 比 a 在末尾多一个节点，不影响比较
        let mut a = head.as_ref();
        let mut b = rev.as_ref();
        while let (Some(x), Some(y)) = (a, b) {
            if x.val != y.val { return false; }
            a = x.next.as_ref();
            b = y.next.as_ref();
        }

        true
    }
}