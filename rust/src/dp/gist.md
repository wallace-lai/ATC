## 数位 DP

定义

数位 DP 是动态规划的一种，专门用来解决"在某个整数区间 [0, N]（或 [L, R]）内，统计满足与数的各位数字有关的条件的数字有多少个 / 满足条件的贡献总和是多少"这类问题。

它的核心是：从高位到低位逐位枚举数字，用记忆化搜索避免重复计算。

https://chat.deepseek.com/share/wj1mzjymq9z2uyigft


## 最长上升子序列（LIS）

最长上升子序列：给定一个数组 nums，子序列指从数组中按原顺序挑选出若干元素（可以不连续），保持相对顺序不变。最长上升子序列就是在所有子序列中，长度最长且严格递增的那一个

## 1. 一维 LIS

### 法一：动态规划

时间：`O(n^2)`
空间：`O(n^2)`

```rust
// 返回严格 LIS 的长度
fn lis(nums: &[i32]) -> usize {
    let n = nums.len();
    if n == 0 { return 0; }

    // dp[i] 为以 nums[i] 结尾的严格 LIS 长度
    let mut dp = vec![1; n];
    let mut ans = 1;

    for i in 0..n {
        for j in 0..i {
            // 如果是求非降 LIS 长度，则
            // if nums[j] <= nums[i] {
            if nums[j] < nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        ans = ans.max(dp[i]);
    }

    ans
}
```

### 法二：贪心 + 二分（partition_point）

时间：`O(nlogn)`
空间：`O(n)`

```rust
// 返回严格 LIS 的长度
fn lis(nums: &[i32]) -> usize {
    let mut tails = Vec::new();

    for &x in nums {
        // 如果是求非降 LIS 的长度，则是
        // let i = tails.partition_point(|&v| v <= x);

        // 第一个 >= x 的位置（lower_bound）
        let i = tails.partition_point(|&v| v < x);
        if i == tails.len() {
            tails.push(x);
        } else {
            tails[i] = x;
        }
    }

    tails.len()
}
```

### 法三：贪心 + 二分（手写 lower_bound）

时间：`O(nlogn)`
空间：`O(n)`

```rust
// 找到第一个 >=t 的位置
fn lower_bound(nums: &[i32], t: i32) -> usize {
    // 左闭右开区间
    let (mut l, mut r) = (0, nums.len());
    while l < r {
        let mid = l + (r - l) / 2;
        if nums[mid] >= t {
            r = mid;        // mid可能是答案，但还需往前找
        } else {
            l = mid + 1;    // mid和左边都不满足 >= t
        }
    }
    l   // l == nums.len()时，表示 >=t 位置不存在
}

fn lis(nums: &[i32]) -> usize {
    // tails[i] 为所有长度为 i+1 的上升子序列中，结尾元素的最小值
    let mut tails = Vec::new();

    for &x in nums {
        // 第一个 >= x 的位置（lower_bound）
        let i = lower_bound(&tails, x);
        if i == tails.len() {
            tails.push(x);
        } else {
            tails[i] = x;
        }
    }

    tails.len()
}
```

## 2. 二维偏序最长链

### 法一：消掉一维后做 LIS

面试题 17.08. 马戏团人塔

```rust
// 法一：二维偏序最长链
// 思路：消掉一维后做 LIS
pub fn best_seq_at_index(height: Vec<i32>, weight: Vec<i32>) -> i32 {
    let n = height.len();
    if n == 0 { return 0; }

    // 打包成 (身高，体重)
    let mut people: Vec<(i32, i32)> = height.into_iter()
        .zip(weight.into_iter())
        .collect();

    // 身高升序：身高相同，体重降序（关键）
    people.sort_by(|a, b| {
        a.0.cmp(&b.0).then(b.1.cmp(&a.1))
    });

    // 对体重要求严格 LIS
    let mut tails = Vec::with_capacity(n);
    for &(_, w) in people.iter() {
        let i = tails.partition_point(|&v| v < w);
        if i == tails.len() {
            tails.push(w);
        } else {
            tails[i] = w;
        }
    }

    tails.len() as i32
}
```

## 3. 带权三维偏序最长链

### 法一：排序 + DP

时间：`O(n²)`
空间：`O(n)`

面试题 08.13. 堆箱子

带权指定是题目要求的是高度和（高度就是权重），而不是箱子个数。

```rust
// 法一：带权三维偏序最长链
// 思路：
// 1. 按某一维度排序（比如 w 升序，w 相同按 d 升序，再相同按 h 升序）
// 2. dp[i] 为以第 i 个箱子为塔底时能堆出的最大高度
// 3. 转移：对所有 j<i，如果 b[j] 三个维度都严格小于 b[i]，则可以将
// b[i] 放到 b[j] 上面
pub fn pile_box(mut b: Vec<Vec<i32>>) -> i32 {
    let n = b.len();
    if n == 0 { return 0; }

    // 排序：按 w 升序，w 相同按 d 升序，再相同按 h 升序
    b.sort_by(|x, y| {
        x[0].cmp(&y[0])
            .then(x[1].cmp(&y[1]))
            .then(x[2].cmp(&y[2]))
    });

    let mut dp = vec![0; n];
    let mut ans = 0;

    for i in 0..n {
        dp[i] = b[i][2];    // 至少能单独放这一个箱子
        for j in 0..i {
            // 三个都严格小，才能放 i 上面
            if b[j][0] < b[i][0] &&
                b[j][1] < b[i][1] &&
                b[j][2] < b[i][2] {
                dp[i] = dp[i].max(dp[j] + b[i][2]);
            }
        }
        ans = ans.max(dp[i]);
    }

    ans
}
```
