# 一、链表

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
fn traverse1(list: Option<Box<ListNode>>) {
    let mut l = list;
    while let Some(node) = l {
        println!("{}", node.val);
        l = node.next;
    }
}
```

```rust
// 遍历链表（借用版本）
fn traverse2(list: &Option<Box<ListNode>>) {
    let mut l = list;
    while let Some(node) = l.as_ref() {
        println!("{}", node.val);
        l = &node.next;
    }
}
```

```rust
// 遍历链表（可变版本）
fn traverse3(list: &mut Option<Box<ListNode>>) {
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
fn traverse4(list: &Option<Box<ListNode>>) {
    if let Some(node) = list.as_ref() {
        Self::traverse4(&node.next);
        println!("{}", node.val);
    }
}
```

## 构造

```rust
// 从数组构造链表（顺序，消耗数组）
fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
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
fn from_slice(slice: &[i32]) -> Option<Box<ListNode>> {
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
fn from_vec_rev(v: Vec<i32>) -> Option<Box<ListNode>> {
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
fn from_slice_rev(slice: &[i32]) -> Option<Box<ListNode>> {
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

# 二、二叉树

## 2.1 定义

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
```

## 2.2 遍历

### 前序遍历

### 中序遍历（左根右）

```rust
fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
    if let Some(n) = root {
        inorder(&n.borrow().left, v);
        v.push(n.borrow().val);
        inorder(&n.borrow().right, v);
    }
}
```

### 后续遍历

### 层序遍历

```rust
// v存储了从上到下每个层的值
fn level_order(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<Vec<i32>>) {
    if root.is_none() { return; }
    let mut q = VecDeque::new();
    
    q.push_back(root.unwrap());     // 前面已经判空
    while !q.is_empty() {
        let len = q.len();
        let mut level = Vec::with_capacity(len);

        for _ in 0..len {
            // 取出当前层的一个节点
            let curr_rc = q.pop_front().unwrap();
            // 借用一个不可变引用计数
            let curr = curr_rc.borrow();
            // 节点值入当前层的value向量
            level.push(curr.val);

            if let Some(left) = curr.left.clone() {
                q.push_back(left);
            }
            if let Some(right) = curr.right.clone() {
                q.push_back(right);
            }
        }

        v.push(level);
    }
}
```

### 最近公共祖先LCA

针对不同的二叉树，LCA问题有多个不同版本：

|版本|关键条件|最优复杂度|
|---|---|---|
|普通二叉树|无任何有序性|$O(n)$|
|二叉搜索树|中序有序|$O(h)$|
|带parent指针|可向上回溯|$O(h)$|

普通二叉树的LCA解法：自底向上递归，找到第一个能同时罩住p和q的节点

```rust
// 限制：默认p和q一定存在于树中
fn lca(node: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let node = match node {
        None => return None,
        Some(n) => n,
    };
    let val = node.borrow().val;
    if val == p || val == q {   // 命中了p或者q，返回自身
        return Some(node.clone());
    }

    let left = lca(node.borrow().left.clone(), p, q);
    let right = lca(node.borrow().right.clone(), p, q);
    match (left, right) {
        (Some(_), Some(_)) => Some(node.clone()),   // p和q分居两侧，当前即为 LCA
        (Some(l), None) => Some(l),     // 只在一侧
        (None, Some(r)) => Some(r),
        (None, None) => None,
    }
}
```

二叉搜索树的LCA解法：利用有序性往下走
- 若 `p->val < root->val && q->val < root->val`：两者都在左子树，往左走；
- 若`p->val > root->val && q->val > root->val`：两者都在右子树，往右走；
- 否则（分居两侧，或者其中一个等于root）：root就是LCA；

```rust
fn lca(node: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut cur = root;
    while let Some(node) = cur {
        let val = node.borrow().val;
        if p < val && q < val {
            let left = node.borrow().left.clone();
            cur = left;
        } else if p > val && q > val {
            let right = node.borrow().right.clone();
            cur = right;
        } else {
            return Some(node);
        }
    }
    None
}
```

带parent指针二叉树解法：等价于两条链表求第一个公共节点

解法A：深度对其法
- 分别从p和q向上走到根，记录p和q的深度；
- 让较深的那个先向上走`|depth_p - depth_q|`步；
- 让后两者同步向上，第一次相遇的节点就是LCA；

解法B：哈希表
- 从p一路向上把祖先放进HashSet中；
- 在从q向上走，第一个出现在HashSet中的节点就是LCA；


## 2.3 构造
### 根据升序数组构造二叉搜索树

```rust
// nums是升序数组的切片
fn build_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() { return None; }

    let mid = nums.len() / 2;
    let mut node = TreeNode::new(nums[mid]);
    node.left = build(&nums[..mid]);
    node.right = build(&nums[mid + 1..]);
    Some(Rc::new(RefCell::new(node)))
}
```

