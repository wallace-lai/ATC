struct Solution;

impl Solution {
    #[cfg(any())]
    // WA
    // 法一：二分搜索
    pub fn search(arr: Vec<i32>, target: i32) -> i32 {
        let mut l = 0i32;
        let mut r = arr.len() as i32 - 1;
        let mut ans = -1;

        while l <= r {
            let m = l + (r - l) / 2;
            let (ul, um) = (l as usize, m as usize);

            if arr[ul] == target {
                // l 是当前区间的最左端，一定是最小索引，直接返回
                return l;
            } else if arr[ul] < target {
                // arr[ul] < target
                if arr[um] == target {
                    ans = m;
                    r = m - 1;
                } else if arr[ul] <= arr[um] {
                    // 左半区间 [l, m] 有序
                    if arr[um] < target {
                        l = m + 1;
                    } else {
                        r = m - 1;
                    }
                } else {
                    // 右半区间 [m, r] 有序
                    l = m + 1;
                }
            } else {
                // arr[ul] > target
                if arr[um] == target {
                    ans = m;
                    l = m + 1;
                } else if arr[ul] >= arr[um] {
                    // 右半区间 [m, r] 有序
                    if arr[um] > target {
                        r = m - 1;
                    } else {
                        l = m + 1;
                    }
                } else {
                    // 左半区间 [l, m] 有序
                    r = m - 1;
                }
            }
        }

        ans
    }

    #[cfg(any())]
    // WA，数组中有重复数字，这个做法不行
    // 法二：找旋转点 + 两段各做 lower_bound
    pub fn search(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let mut l = 0;
        let mut r = n - 1;

        // 找最小值位置（旋转点），重复时收缩
        while l < r {
            let m = (l + r) / 2;
            if arr[m] > arr[r] {
                // 左半区间 [l..=m] 有序
                l = m + 1;
            } else if arr[m] < arr[r] {
                // 右半区间 [m..=r] 有序
                r = m;
            } else {
                // 相等时收缩
                r -= 1;
            }
        }

        let pivot = l;

        // 两边分别做 lower_bound，找到第一个 == target 的位置
        fn lower_bound(a: &[i32], l: usize, r: usize, t: i32) -> Option<usize> {
            let mut ans = None;
            let (mut l, mut r) = (l as i32, r as i32);
            while l <= r {
                let m = (l + r) / 2;
                let v = a[m as usize];
                if v == t {
                    ans = Some(m as usize);
                    r = m - 1;
                } else if v < t {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
            ans
        }

        let a = lower_bound(&arr, 0, if pivot == 0 { 0 } else { pivot - 1 }, target)
            .filter(|_| pivot > 0);
        let b = lower_bound(&arr, pivot, n - 1, target);
        match (a, b) {
            (Some(x), Some(y)) => x.min(y) as i32,
            (Some(x), None) => x as i32,
            (None, Some(y)) => y as i32,
            _ => -1,
        }
    }

    // 法三：二分搜索
    // 和 leetcode 81的区别：81题要求只要存在target就返回true
    // 这里要求返回多个重复target中最靠前的下标值
    pub fn search(arr: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = arr.len() as i32 - 1;

        while l <= r {
            // 重点：当 l 符合时直接返回，因为找到是最小索引
            if arr[l as usize] == target { return l; }

            let m = l + (r - l) / 2;
            let (ul, um, ur) = (l as usize, m as usize, r as usize);
            // 重点：当中间元素等于target时，右边界移到中间，因为左边可能还有相等的值
            if arr[um] == target {
                r = m;
            } else if arr[ul] < arr[um] {
                if arr[ul] <= target && target < arr[um] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else if arr[ul] > arr[um] {
                if arr[um] < target && target <= arr[ur] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            } else {
                // 重点：当中间数字与左边数字相等时，左边界右移
                l += 1;
            }
        }

        -1
    }
}
