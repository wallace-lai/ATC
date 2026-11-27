struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：数学
    // 时间复杂度：O(n)
    // 空间复杂度：O(1)
    pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i64 + 2;

        // 1..=n 的和与平方和
        let sum_all = n * (n + 1) / 2;
        let sq_sum_all = n * (n + 1) * (2 * n + 1) / 6;

        let mut sum_nums = 0;
        let mut sq_sum_nums = 0;
        for &x in nums.iter() {
            let x = x as i64;
            sum_nums += x;
            sq_sum_nums += x * x;
        }

        let s1 = sum_all - sum_nums;    // a + b
        let s2 = sq_sum_all - sq_sum_nums;  // a^2 + b^2

        // (a - b)^2 = 2(a^2 + b^2) - (a + b)^2 = 2 * s2 - s1^2
        let diff = ((2 * s2 - s1 * s1) as f64).sqrt() as i64;

        let a = (s1 - diff) / 2;
        let b = (s1 + diff) / 2;
        vec![a as _, b as _]
    }

    // 法二：异或分组法
    // 时间复杂度：O(n)
    // 空间复杂度：O(1)
    pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32 + 2;

        // 1. 整体异或，得到 a ^ b
        let mut xor_all = 0;
        for i in 1..=n {
            xor_all ^= i;
        }
        for &x in nums.iter() {
            xor_all ^= x;
        }

        // 2. 取最低位的 1 作为分组依据
        // 由于 a != b，所以 a ^ b != 0
        let mask = xor_all & (!xor_all + 1);

        // 3. 分组异或
        let (mut a, mut b) = (0, 0);
        for i in 1..=n {
            if i & mask != 0 {
                a ^= i;
            } else {
                b ^= i;
            }
        }
        for &x in nums.iter() {
            if x & mask != 0 {
                a ^= x;
            } else {
                b ^= x;
            }
        }

        vec![a, b]
    }
}