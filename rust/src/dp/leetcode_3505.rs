struct Solution;

use std::collections::HashMap;
use std::collections::BinaryHeap;

struct LazyRemoveHeap {
    sum: i64,                   // 堆中元素总和
    heap: BinaryHeap<i64>,      // 实际堆
    size: usize,                // 堆中实际元素大小
    count: HashMap<i64, u32>,   // 记录元素需要删除的次数
}

impl LazyRemoveHeap {
    fn new() -> Self {
        Self {
            sum: 0,
            heap: BinaryHeap::new(),
            size: 0,
            count: HashMap::new(),
        }
    }

    fn sum(&self) -> i64 {
        self.sum
    }

    fn len(&self) -> usize {
        self.size
    }

    fn remove(&mut self, x: i64) {
        // x 的需删除次数加1，但不执行真正的删除
        *self.count.entry(x).or_insert(0) += 1;
        self.size -= 1;
        self.sum -= x;
    }

    fn remove_apply(&mut self) {
        // 执行真正的删除动作，查看堆顶元素的删除次数，如果
        // 删除次数大于 0 则删除堆顶元素，删除次数减1。如此
        // 循环，直到堆顶元素不再需要删除为止
        while let Some(top) = self.heap.peek() {
            if let Some(cnt) = self.count.get_mut(top) && *cnt > 0 {
                *cnt -= 1;
                self.heap.pop();
            } else {
                break;
            }
        }
    }

    fn push(&mut self, x: i64) {
        let cnt = self.count.entry(x).or_insert(0);
        if *cnt > 0 {
            *cnt -= 1;
        } else {
            self.heap.push(x);
        }

        self.size += 1;
        self.sum += x;
    }

    fn pop(&mut self) -> i64 {
        self.remove_apply();
        let &x = self.heap.peek().unwrap();
        self.heap.pop();
        self.size -= 1;
        self.sum -= x;
        x
    }

    fn peek(&mut self) -> i64 {
        self.remove_apply();
        *self.heap.peek().unwrap()
    }

    fn push_pop(&mut self, x: i64) -> i64 {
        self.remove_apply();
        self.heap.push(x);
        self.sum += x;

        let &top = self.heap.peek().unwrap();
        self.heap.pop();
        self.sum -= top;
        top
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        
        fn median_sliding_window(nums: &Vec<i32>, k: usize) -> Vec<i64> {
            let n = nums.len();
            let mut ans = vec![0; n - k + 1];
            let mut left = LazyRemoveHeap::new();   // 最大堆
            let mut right = LazyRemoveHeap::new();  // 最小堆（元素取反）

            for (i, &x) in nums.iter().enumerate() {
                let x = x as i64;
                // 进入窗口
                if left.len() == right.len() {
                    // left 和 right 大小相等时，将 num 放到 right 中
                    // 并将 right 中的最小值取出添加到 left 中
                    left.push(-right.push_pop(-x));
                } else {
                    // left 比 right 多一个元素时，将 num 放到 left 中
                    // 并将 left 中的最大值取出添加到 right 中
                    right.push(-left.push_pop(x));
                }

                if i + 1 < k { continue; }

                // 计算答案
                let l = i + 1 - k;
                let v = left.peek();
                let s1 = v * left.len() as i64 - left.sum();
                let s2 = (-right.sum()) - v * right.len() as i64;
                ans[l] = s1 + s2;

                // 离开窗口
                let x = nums[l] as i64;
                if x <= left.peek() {
                    left.remove(x);
                    if left.len() < right.len() {
                        let rmin = right.pop();
                        left.push(-rmin);
                    }
                } else {
                    right.remove(-x);
                    if left.len() > right.len() + 1 {
                        let lmax = left.pop();
                        right.push(-lmax);
                    }
                }
            }

            ans
        }

        let n = nums.len();
        let x = x as usize;
        let k = k as usize;
        let dis = median_sliding_window(&nums, x);
        let mut dp = vec![vec![0; n + 1]; k + 1];

        for i in 1..=k {
            dp[i][i * x - 1] = i64::MAX;
            for j in i * x..=n - (k - i) * x {
                dp[i][j] = std::cmp::min(
                    dp[i][j - 1],
                    dp[i - 1][j - x] + dis[j - x]
                );
            }
        }

        dp[k][n]
    }
}
