struct Solution;

impl Solution {
    /// 对字符串s分情况讨论
    /// 1. s初始就是非降序的，不需要进行任何操作，返回0
    /// 2. s的长度为2并且没有按照非降序排列，无法通过操作改变s，返回-1
    /// 3. s的首字符为s中的最小字符，将子串s[1..n-1]排序一次即可，返回1
    /// 4. s的尾字符为s中的最大字符，等效情况3，返回1
    /// 
    /// 5. s的首字符为s中最大字符且s的尾字符为s中最小的字符
    ///     5.1 将子串s[0..n-2]排序，则最大字符来到了n-2的位置上
    ///     5.2 将子串s[n-2..n-1]排序，则最大字符到了s的尾字符位置，等效情况4
    ///     此时，总共需要3次排序，返回3
    /// 
    /// 6. 除此之外的任何情况下，只需要将s[1..n-1]排序，就转化成了情况4，此时
    ///     需要两次排序，返回2
    pub fn min_operations(s: String) -> i32 {
        
        fn check(s: &[u8], mn: &mut u8, mx: &mut u8) -> bool {
            let mut ans = true;
            (*mn, *mx) = (s[0], s[0]);
            for i in 1..s.len() {
                if s[i] < *mn { *mn = s[i]; }
                if s[i] > *mx { *mx = s[i]; }
                if s[i - 1] > s[i] { ans = false; }
            }
            ans
        }

        let b = s.as_bytes();
        let n = b.len();
        let (mut mn, mut mx) = (0, 0);
        let ret = check(s.as_bytes(), &mut mn, &mut mx);

        // 情况1
        if ret == true { return 0; }
        // 情况2
        if n == 2 { return -1; }
        // 情况3和4
        if mn == b[0] || mx == b[n - 1] { return 1; }
        // 情况6
        for i in 1..n - 1 {
            if b[i] == mx || b[i] == mn {
                return 2;
            }
        }
        
        // 情况5
        3
    }
}