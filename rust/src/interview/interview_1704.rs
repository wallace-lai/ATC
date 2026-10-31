struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：数学求和法
    // 时间复杂度：O(n)
    // 空间复杂度：O(1)
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let expected = n * (n + 1) / 2;
        let actual: usize = nums.iter().map(|&i| i as usize).sum();
        (expected - actual) as i32
    }

    #[cfg(any())]
    // 法二：异或法
    // 思路：把 0..=n 全部异或，再和 nums 全部异或，剩下的就是缺失的数
    // 时间复杂度：O(n)
    // 空间复杂度：O(1)
    // 无溢出风险，对任意 n 都安全
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut x = n;  // 0..=n 共 n+1 个数，先把 n 放进去
        for (i, &v) in nums.iter().enumerate() {
            x ^= i as i32;  // 异或上 0..n-1
            x ^= v;         // 异或上数组元素
        }
        x
    }

    // 法三：排序后线性扫描
    // 思路：排序后，缺失的位置满足 nums[i] != i
    // 时间复杂度：O(n)
    // 空间复杂度：O(1)

    // 法四：循环排序
    // 思路：利用下标本身，把每个数放到它对应的下标上，然后扫描哪个位置对不上
    // 此为原地哈希的经典模板，可以复用到 找重复数/找缺失数 等一系列问题上
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        for i in 0..n {
            // 把 nums[i] 换到它该在的位置 nums[nums[i]]
            while nums[i] != i as i32 && nums[i] < n as i32 {
                let j = nums[i] as usize;
                nums.swap(i, j);
            }
        }

        for i in 0..n {
            if nums[i] != i as i32 {
                return i as i32;
            }
        }

        n as i32
    }
}