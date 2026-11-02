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
    // 法一：哈希表，单次遍历
    // 时间复杂度：O(n)
    // 空间复杂度：O(n)
    pub fn remove_duplicate_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut dummy = Some(Box::new(
            ListNode {
                val: 0,
                next: head
            }
        ));

        // prev 指向当前已保留的最后一个节点
        let mut prev = &mut dummy;

        while let Some(curr) = &prev.as_mut().unwrap().next {
            if set.contains(&curr.val) {
                // 如果哈希表中已存在，删除 curr 节点
                // 通过 take() 将 next 的所有权取出，再赋值为 next.next
                let next = prev.as_mut().unwrap().next.take();
                prev.as_mut().unwrap().next = next.unwrap().next;
            } else {
                // 如果值不存在，则加入集合，并移动 prev 指针
                set.insert(curr.val);
                prev = &mut prev.as_mut().unwrap().next;
            }
        }

        dummy.unwrap().next
    }

    // 法二：双重循环
    pub fn remove_duplicate_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(
            ListNode {
                val: 0,
                next: head
            }
        ));
        let mut curr = &mut dummy;

        // 外层循环：遍历每一个可能作为基准的节点
        while let Some(node) = &mut curr.as_mut().unwrap().next {
            let val = node.val;
            let mut runner = node;

            // 内层循环：删除后续所有与基准相同的节点
            while let Some(next_node) = &mut runner.next {
                if next_node.val == val {
                    // 发现重复节点，删除它
                    let to_delete = runner.next.take();
                    runner.next = to_delete.unwrap().next;
                } else {
                    runner = runner.next.as_mut().unwrap();
                }
            }

            // 当前基准处理完毕，移动到下一个
            curr = &mut curr.as_mut().unwrap().next;
        }

        dummy.unwrap().next
    }
}