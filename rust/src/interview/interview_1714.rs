struct Solution;

// 简单 xorshift 伪随机数生成器
struct XorShift(u64);

impl XorShift {
    fn new(seed: u64) -> Self {
        // 种子为 0 时退化，用黄金分隔常数兜底
        XorShift(if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed })
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

#[cfg(any())]
impl Solution {
    // Lomuto 划分，随机选择 pivot
    fn partition(
        arr: &mut [i32],
        left: usize,
        right: usize,
        rng: &mut XorShift
    ) -> usize {
        let pivot_idx = left + (rng.next() as usize) % (right - left + 1);
        arr.swap(pivot_idx, right);

        let pivot = arr[right];
        let mut i = left;
        for j in left..right {
            if arr[j] < pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, right);
        i
    }

    // 让 arr[0..k] 恰好为最小的 K 个元素（内部无序）
    fn quick_select(
        arr: &mut [i32],
        mut left: usize,
        mut right: usize,
        k: usize,
        rng: &mut XorShift
    ) {
        while left < right {
            let pivot_idx = Self::partition(arr, left, right, rng);
            // pivot_idx 是当前划分后 pivot 的最终位置
            // 如果 pivot_idx + 1 == k，说明前 k 个正好就是最小的 k 个
            if pivot_idx + 1 == k {
                return;
            } else if pivot_idx + 1 < k {
                left = pivot_idx + 1;
            } else {
                // pivot_idx + 1 > k  ⇒ pivot_idx >= k >= 1，pivot_idx - 1 不会下溢
                right = pivot_idx - 1;
            }
        }
    }

    // 法一：快速选择
    pub fn smallest_k(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if k == 0 { return vec![]; }

        let n = arr.len();
        let mut rng = XorShift::new(0xdead_beef);
        Self::quick_select(&mut arr, 0, n - 1, k, &mut rng);
        arr.truncate(k);
        arr
    }
}

use std::collections::BinaryHeap;

impl Solution {
    // 法二：维护一个大小为 K 的最大堆
    pub fn smallest_k(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if k == 0 { return vec![]; }

        let mut heap = BinaryHeap::with_capacity(k);
        for x in arr {
            if heap.len() < k {
                heap.push(x);
            } else if x < *heap.peek().unwrap() {
                heap.pop();
                heap.push(x);
            }
        }

        heap.into_vec()
    }
}