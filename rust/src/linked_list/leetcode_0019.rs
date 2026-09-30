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
    pub fn remove_kth(list: &Option<Box<ListNode>>, k: usize) {
        if k == 0 {
            // 无效索引
            return;
        }

        let mut curr = list;
        let mut step = k;
        while step > 1 {
            if let Some(node) = curr {
                curr = &node.next;
                step -= 1;
            } else {
                // 链表长度小于k，无需删除
                return;
            }
        }

        if let Some(node) = curr {
            println!("val = {}", node.val);
        }
    }

    pub fn rev(list: &Option<Box<ListNode>>) {
        if let Some(node) = list.as_ref() {
            Self::rev(&node.next);
            println!("{}", node.val);
        }
    }

    // pub fn rev(node: &Option<Box<ListNode>>) -> i32 {
    //     match node {
    //         Some(n) => {
    //             let mut nrth = Self::rev(&n.next);
    //             println!("nrth = {}, val = {}", nrth, n.val);
    //             nrth += 1;
    //             nrth
    //         },
    //         None => { 0 }
    //     }
    // }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::remove_kth(&head, n as usize);
        None
    }
}