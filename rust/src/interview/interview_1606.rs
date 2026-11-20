struct Solution;

impl Solution {
    // 法一：排序 + 双指针
    pub fn smallest_difference(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
        a.sort_unstable();
        b.sort_unstable();

        let mut ans = i64::MAX;
        let mut i = 0;
        let mut j = 0;
        while i < a.len() && j < b.len() {
            let mut d = a[i] as i64 - b[j] as i64;
            if d < 0 { d = -d; }
            ans = ans.min(d);

            if a[i] == b[j] { return ans as i32; }

            if a[i] < b[j] {
                i += 1;
            } else {
                j += 1;
            }
        }

        ans as i32
    }
}