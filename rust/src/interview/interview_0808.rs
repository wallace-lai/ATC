struct Solution;

impl Solution {
    // 法一：next_permutation算法
    pub fn permutation(s: String) -> Vec<String> {
    
        fn next_permutation(a: &mut [u8]) -> bool {
            let n = a.len();
            if n < 2 { return false; }

            // 从右往左找第一个升序对 a[i - 1] < a[i]
            // 用 >= 而不是 >：相等字符不算升序对，必须左移，去重关键之一
            let mut i = n - 1;
            while i > 0 && a[i - 1] >= a[i] { i -= 1; }
            if i == 0 { return false; } // 整个数组已经降序，是最后一个排列

            // 从右找第一个严格大于 a[i - 1] 的字符
            // 用 <= 而不是 <：跳过相等字符，交换才有意义，去重关键之二
            let mut j = n - 1;
            while a[j] <= a[i - 1] { j -= 1; }
            
            a.swap(i - 1, j);
            a[i..].reverse();
            true
        }

        let mut bytes = s.into_bytes();
        bytes.sort_unstable();

        let mut ans = Vec::new();
        loop {
            // ASCII字节怎么重排都是合法的 UTF-8，unwrap()是安全的
            ans.push(String::from_utf8(bytes.clone()).unwrap());
            if !next_permutation(&mut bytes) {
                break;
            }
        }

        ans
    }
}