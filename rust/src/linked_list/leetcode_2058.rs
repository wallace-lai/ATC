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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::new();
        let mut l = &head;
        while let Some(node) = l.as_ref() {
            v.push(node.val);
            l = &node.next;
        }
        // println!("v is {:?}", v);

        let mut points = Vec::new();
        for i in 1..(v.len() - 1) {
            if (v[i] < v[i - 1] && v[i] < v[i + 1]) ||
               (v[i] > v[i - 1] && v[i] > v[i + 1]) {
                points.push(i);
            }
        }
        if points.len() < 2 { return vec![-1, -1]; }

        let max_dist = points[points.len() - 1] - points[0];
        let mut min_dist = usize::MAX;
        for i in 0..(points.len() - 1) {
            if points[i + 1] - points[i] < min_dist {
                min_dist = points[i + 1] - points[i];
            }
        }

        vec![min_dist as i32, max_dist as i32]
    }
}