struct Solution;


impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let n = nums1.len();
        let mut k = (k1 + k2) as i64;
        let mut sum = 0i64;
        let mut ans = 0i64;
        let mut v = Vec::with_capacity(n);

        for i in 0..n {
            v.push(nums1[i].abs_diff(nums2[i]));
            sum += v[i] as i64;
            ans += v[i] as i64 * v[i] as i64;
        }
        if sum <= k { return 0; }

        v.sort_unstable_by(|a, b| b.cmp(a));
        v.push(0);  // 哨兵
        for i in 0..n {
            let mut a = v[i] as i64;
            ans -= a * a;
            let j = i as i64 + 1;
            let c = j * (a - v[j as usize] as i64);
            if c <= k { k -= c; continue; }
            a -= k / j as i64;
            return ans + k % j * (a - 1) * (a - 1) + (j - k % j) * a * a;
        }

        ans
    }
}