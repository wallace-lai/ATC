# 树状数组

```rust
struct FenwickTree {
    n: usize,
    b: Vec<i64>,
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        Self {
            n: n,
            b: vec![0; n + 1],
        }
    }

    // 从切片初始化（要求切片是 0-indexed的）
    pub fn from_slice(v: &[i32]) -> Self {
        let n = v.len();
        let mut ft = Self::new(n);
        for (i, &x) in v.iter().enumerate() {
            ft.add(i + 1, x);
        }
        ft
    }

    // 单点更新：在位置 i（1-indexed）上增加 delta
    pub fn add(&mut self, mut i: usize, delta: i32) {
        while i <= self.n {
            self.b[i] += delta as i64;
            // lowbit(i) = i & (-i)
            i += i & i.wrapping_neg();
        }
    }

    // 前缀和查询：返回 [1..=i] 的聚合结果
    pub fn query(&self, mut i: usize) -> i32 {
        let mut ans = 0;
        while i > 0 {
            ans += self.b[i];
            i -= i & i.wrapping_neg();
        }
        ans as i32
    }

    // 区间查询：返回 [l..=r] 的聚合结果
    pub fn range_query(&self, l: usize, r: usize) -> i32 {
        self.query(r) - self.query(l)
    }
}
```
