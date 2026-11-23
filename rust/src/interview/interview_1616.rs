struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：排序后比较 O(nlogn)
    pub fn sub_sort(array: Vec<i32>) -> Vec<i32> {
        let mut copy = array.clone();
        copy.sort_unstable();

        let mut m = -1;
        let mut n = -1;
        for (i, (&a, &b)) in array.iter()
            .zip(copy.iter()).enumerate() {
            if a != b { m = i as i32; break; }
        }
        for (i, (&a, &b)) in array.iter()
            .zip(copy.iter()).enumerate().rev() {
            if a != b { n = i as i32; break; }
        }

        if m != -1 && n != -1 {
            vec![m, n]
        } else {
            vec![-1, -1]
        }
    }

    // 法二：双指针 (O(n))
    // 右边界 n：最右边那个【比它左边某个元素还小】的位置
    // 左边界 m：最左边那个【比它右边某个元素还大】的位置
    pub fn sub_sort(array: Vec<i32>) -> Vec<i32> {
        let n = array.len();
        if n < 2 { return vec![-1, -1]; }

        let mut left = -1;
        let mut right = -1;

        // 从左往右找右边界
        let mut max_seen = array[0];
        for i in 1..n {
            if array[i] < max_seen {
                // i 这个位置【比它左边某个元素还小】
                right = i as i32;
            } else {
                max_seen = array[i];
            }
        }

        // 从右往左找左边界
        let mut min_seen = array[n - 1];
        for i in (0..n -1).rev() {
            if array[i] > min_seen {
                // i 这个位置【比它右边某个元素还大】
                left = i as i32;
            } else {
                min_seen = array[i];
            }
        }

        vec![left, right]
    }
}