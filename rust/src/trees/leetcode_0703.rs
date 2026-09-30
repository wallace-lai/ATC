use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    // 使用Reverse包裹的最小堆
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(kth: i32, nums: Vec<i32>) -> Self {
        let mut ans = Self {
            k: kth as usize,
            heap: BinaryHeap::with_capacity((kth + 1) as usize),
        };

        for num in nums.into_iter() {
            ans.add(num);
        }

        ans
    }
    
    fn add(&mut self, val: i32) -> i32 {
        // 关键优化：堆已满且val <= 第K大，直接丢弃
        if self.heap.len() == self.k {
            if let Some(&Reverse(top)) = self.heap.peek() {
                if val <= top {
                    return top;
                }
            }
        }

        // 插入新元素
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }

        self.heap.peek().map(|&Reverse(v)| v).unwrap()
    }
}