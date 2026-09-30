struct Solution;

impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, k: i32) -> i32 {
        // 计算数组中负数的个数
        let negative = nums.iter().filter(|&num| *num < 0).count();
        let k = k as usize;

        // negative和k之间的所有情况
        // 1. negative为0
        // 2. negative不为0，且小于k
        // 3. negative不为0，且大于等于k
        if negative == 0 {
            // 此时数组中不存在负数
            let mut sum: i32 = nums.iter().sum();
            if k % 2 == 1 {
                let &min = nums.iter().min().unwrap();
                sum -= 2 * min;
            }
            return sum;
        } else if negative < k {
            // 负数个数小于反转次数，可以先将所有负数反转成正数
            nums.sort_unstable();
            nums.iter_mut().for_each(|x| if *x < 0 { *x = -*x; });
            let mut sum: i32 = nums.iter().sum();

            // 此时，数组全为正数且剩余反转次数为 k-negative
            if (k - negative) % 2 == 1 {
                // 如果必须反转一次，则反转数组最小的数即可
                let &min = &nums.iter().min().unwrap();
                sum -= 2 * min;
            }
            return sum;
        } else {
            // negative >= k
            // 将前k个最小的负数反转成正数即可
            nums.sort_unstable();
            return nums.iter().enumerate()
                .map(|(i, num)| { if i < k { -*num } else { *num } })
                .sum()
        }
    }
}