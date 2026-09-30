# 链表

基于Box指针的单链表定义：

```rust
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
```

## 遍历

```rust
    // 遍历链表（消耗版本）
    pub fn traverse1(list: Option<Box<ListNode>>) {
        let mut l = list;
        while let Some(node) = l {
            println!("{}", node.val);
            l = node.next;
        }
    }
```

```rust
    // 遍历链表（借用版本）
    pub fn traverse2(list: &Option<Box<ListNode>>) {
        let mut l = list;
        while let Some(node) = l.as_ref() {
            println!("{}", node.val);
            l = &node.next;
        }
    }
```

```rust
    // 遍历链表（可变版本）
    pub fn traverse3(list: &mut Option<Box<ListNode>>) {
        let mut l = list;
        while let Some(node) = l.as_mut() {
            println!("{}", node.val);
            node.val += 1;
            l = &mut node.next;
        }
    }
```

```rust
    // 遍历链表（逆序，借用版本）
    pub fn traverse4(list: &Option<Box<ListNode>>) {
        if let Some(node) = list.as_ref() {
            Self::traverse4(&node.next);
            println!("{}", node.val);
        }
    }
```

## 构造

```rust
    // 从数组构造链表（顺序，消耗数组）
    pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: -1, next: None });
        let mut tail = &mut dummy;

        for val in v {
            tail.next = Some(Box::new(ListNode {
                val: val,
                next: None
            }));

            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
```

```rust
    // 从数组构造链表（顺序，借用数组）
    pub fn from_slice(slice: &[i32]) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: -1, next: None });
        let mut tail = &mut dummy;

        for val in slice {
            tail.next = Some(Box::new(ListNode {
                val: *val,
                next: None
            }));

            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
```

```rust
    // 从数组构造链表（逆序，消耗数组）
    pub fn from_vec_rev(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for val in v.into_iter().rev() {
            head = Some(Box::new(ListNode {
                val: val,
                next: head
            }));
        }

        head
    }
```

```rust
    // 从数组构造链表（逆序，借用版本）
    pub fn from_slice_rev(slice: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for val in slice.iter().rev() {
            head = Some(Box::new(ListNode {
                val: *val,
                next: head
            }));
        }

        head
    }
```
