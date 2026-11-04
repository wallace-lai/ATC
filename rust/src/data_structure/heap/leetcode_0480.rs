struct Solution;

use std::collections::HashMap;
use std::collections::BinaryHeap;

struct LazyRemoveHeap {
    heap: BinaryHeap<i64>,
    size: usize,                // 堆中实际元素大小
    count: HashMap<i64, u32>,   // 记录元素需要删除的次数
}

impl LazyRemoveHeap {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            size: 0,
            count: HashMap::new(),
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn remove(&mut self, x: i64) {
        // x 的需删除次数加1，但不执行真正的删除
        *self.count.entry(x).or_insert(0) += 1;
        self.size -= 1;
    }

    fn remove_apply(&mut self) {
        // 执行真正的删除动作，查看堆顶元素的删除次数，如果
        // 删除次数大于 0 则删除堆顶元素，删除次数减1。如此
        // 循环，直到堆顶元素不再需要删除为止
        while let Some(top) = self.heap.peek() {
            if let Some(cnt) = self.count.get_mut(top) {
                if *cnt == 0 { break; }
                *cnt -= 1;
                self.heap.pop();
            } else {
                break;
            }
        }
    }

    fn push(&mut self, x: i64) {
        self.heap.push(x);
        self.size += 1;
    }

    fn pop(&mut self) -> i64 {
        self.remove_apply();
        self.size -= 1;
        self.heap.pop().unwrap()
    }

    fn peek(&mut self) -> i64 {
        self.remove_apply();
        *self.heap.peek().unwrap()
    }
}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let n = nums.len();
        let mut ans = vec![0.0; n - k + 1];
        let mut left = LazyRemoveHeap::new();   // 最大堆
        let mut right = LazyRemoveHeap::new();  // 最小堆（元素取反）

        for (i, &x) in nums.iter().enumerate() {
            let x = x as i64;
            // 进入窗口
            if left.len() == right.len() {
                // left 和 right 大小相等时，将 num 放到 right 中
                // 并将 right 中的最小值取出添加到 left 中
                right.push(-x);
                let rmin = right.pop();
                left.push(-rmin);
            } else {
                // left 比 right 多一个元素时，将 num 放到 left 中
                // 并将 left 中的最大值取出添加到 right 中
                left.push(x);
                let lmax = left.pop();
                right.push(-lmax);
            }

            if i + 1 < k { continue; }

            // 计算答案
            let l = i + 1 - k;
            ans[l] = if k % 2 == 1 {
                left.peek() as f64
            } else {
                (left.peek() - right.peek()) as f64 / 2.0
            };

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
}