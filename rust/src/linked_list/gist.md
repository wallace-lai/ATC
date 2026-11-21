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

# 三、回溯

## 3.1 排列问题

### 生成所有排列（序列不含重复数字）

```rust
/// nums为原始序列
/// ans存储生成的排列
/// nums[0..=k-1]范围内为已选择的数，nums[k..]范围内是还未被选择的数
fn backtrack(nums: &mut Vec<i32>, k: usize, ans: &mut Vec<Vec<i32>>) {
    // 所有的数都填充完毕
    if k == nums.len() {
        ans.push(nums.clone());
        return;
    }

    // nums[k..]范围内是还未选的数，从其中选择一个数填充到nums[k]中
    for i in k..nums.len() {
        // 选择下标i处的数填充
        nums.swap(k, i);
        backtrack(nums, k + 1, ans);
        // 撤销选择
        nums.swap(k, i);
    }
}
```

### 生成所有排列（序列包含重复数字）

```rust
// nums 必须已经排序，去重才能成功
fn backtrack(nums: &[i32], used: &mut [bool], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if path.len() == nums.len() {
        ans.push(path.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] { continue; }
        // 树层去重：同层元素只取第一个
        if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] { continue; }

        used[i] = true;
        path.push(nums[i]);
        backtrack(nums, used, path, ans);
        path.pop();
        used[i] = false;
    }
}
```

### `next_permutation`算法

`next_permutation` 算法本身就不需要额外去重，只要初始序列排序一次即可

```rust
// nums 必须为已排序后的字典序最小序列
fn next_permutation(nums: &mut [i32]) -> bool {
    let n = nums.len();
    if n < 2 { return false; }

    // 从右往左找第一个升序对 a[i - 1] < a[i]
    // 用 >= 而不是 >：重复元素之间不构成升序关系，必须继续往左找
    let mut i = n - 1;
    while i > 0 && nums[i - 1] >= nums[i] { i -= 1; }
    if i == 0 { return false; } // 已经是完全降序，最后一个排列

    // 从右往左找第一个严格大于 a[i - 1] 的数
    // 用 <= 而不是 <：跳过相等元素，交换才不会“原地踏步”
    let mut j = n - 1;
    while nums[j] <= nums[i - 1] { j -= 1; }

    // 交换后，将后缀翻转成最小（升序）
    nums.swap(i - 1, j);
    nums[i..].reverse();
    true
}
```

## 3.2 子集问题

### 生成所有子集（序列不含重复数字）

#### 法一：回溯

```rust
// 时间复杂度：O(n * 2^n)
// 空间复杂度：O(n)
fn dfs(nums: &[i32], curr: usize, t: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if curr == nums.len() {
        ans.push(t.clone());
        return;
    }

    // 选
    t.push(nums[curr]);
    dfs(nums, curr + 1, t, ans);
    t.pop();

    // 不选
    dfs(nums, curr + 1, t, ans);
}
```

#### 法二：二进制位迭代（nums.len() < 32）

```rust
// 法二：二进制位迭代，nums.len() < 32
// 时间复杂度：O(n * 2^n)
// 空间复杂度：O(n)
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut t = Vec::with_capacity(n);
    let mut ans = Vec::with_capacity(2usize.pow(n as u32));

    for mask in 0..(1usize << n) {
        t.clear();
        for i in 0..n {
            if (mask & (1usize << i)) != 0 {
                t.push(nums[i]);
            }
        }
        ans.push(t.clone());
    }

    ans
}
```

### 生成所有子集（序列含重复数字）

#### 法一：回溯

```rust
// nums 必须是已排序的序列
fn dfs(nums: &[i32], mut i: usize, t: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if i == nums.len() {
        ans.push(t.clone());
        return;
    }

    // 把相同元素看成一组，比如 nums = [1, 1, 1, 2]，一个
    // 子集真正关心的是：选0个、1个、2个还是3个1，而不关心选
    // 的是第几个1

    // 当前的这个 x 已经选了，接下来考虑后面的元素。后面相同
    // 的 x 还可以继续选，用来表示选2个、3个相同的元素

    // 如果当前的这个 x 不选，那么后面那些相同的 x 也不能通过
    // 跳过当前，选后面的方式来选，否则会造成重复

    // 选
    let x = nums[i];
    t.push(x);
    dfs(nums, i + 1, t, ans);
    t.pop();

    // 不选
    i += 1;
    while i < nums.len() && nums[i] == x {
        i += 1;
    }
    dfs(nums, i, t, ans);
}
```

### 生成所有子集（通用解法）

```rust
// 通用解法，无论是否存在相同元素
fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    
    fn dfs(g: &[(i32, usize)], i: usize, t: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if i == g.len() {
            ans.push(t.clone());
            return;
        }

        let (v, c) = g[i];
        // 对于这一组，选择 k 个，k 决定了从中选择多少个 v
        for k in 0..=c {
            let base = t.len();
            t.extend(std::iter::repeat(v).take(k));
            dfs(g, i + 1, t, ans);
            t.truncate(base);
        }
    }

    nums.sort_unstable();

    // 分组：(值，出现次数)
    let mut group: Vec<(i32, usize)> = Vec::new();
    for &x in nums.iter() {
        match group.last_mut() {
            Some((v, c)) if *v == x => { *c += 1 },
            _ => { group.push((x, 1)) },
        }
    }

    let mut t = vec![];
    let mut ans = vec![];
    dfs(&group, 0, &mut t, &mut ans);
    ans
}
```
