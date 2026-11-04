
/// 思路：将当前集合中的元素分成两部分 left 和 right，保证
/// left 和 right 两个子集的大小尽可能相等。规定：
/// 1. 保证 left 中的所有元素都小于等于 right 的所有元素；
/// 2. 保证 left 和 right 的大小尽量相等，有奇数个时，left 比 right 多一个数
/// 
/// 满足上述要求，就可以用 left 中的最大值和 right 中的最小值计算中位数
/// 

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    left: BinaryHeap<i32>,              // 最大堆
    right: BinaryHeap<Reverse<i32>>,    // 最小堆
}

impl MedianFinder {

    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        if self.left.len() == self.right.len() {
            // left 和 right 大小相等时，将 num 放到 right 中
            // 并将 right 中的最小值取出添加到 left 中
            self.right.push(Reverse(num));
            let rmin = self.right.pop().unwrap();
            self.left.push(rmin.0);
        } else {
            // left 比 right 多一个元素时，将 num 放到 left 中
            // 并将 left 中的最大值取出添加到 right 中
            self.left.push(num);
            let lmax = self.left.pop().unwrap();
            self.right.push(Reverse(lmax));
        }
    }
    
    fn find_median(&self) -> f64 {
        // 如果当前有奇数个元素，中位数是 left 的堆顶；
        // 如果当前有偶数个元素，中位数是 left 的堆顶和 right 堆顶的平均值
        if self.left.len() > self.right.len() {
            *self.left.peek().unwrap() as f64
        } else {
            let &lmax = self.left.peek().unwrap();
            let &rmin = self.right.peek().unwrap();
            (lmax + rmin.0) as f64 / 2.0
        }
    }
}