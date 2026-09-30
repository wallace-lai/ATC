struct Solution;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut ans = Vec::with_capacity(len1 + len2);

        let mut i = 0;
        let mut j = 0;
        while i < len1 && j < len2 {
            let a = &nums1[i];
            let b = &nums2[j];
            let mut t = vec![0, 0];
            if a[0] == b[0] {
                t[0] = a[0];
                t[1] = a[1] + b[1];
                ans.push(t);

                i += 1;
                j += 1;
            } else if a[0] < b[0] {
                t[0] = a[0];
                t[1] = a[1];
                ans.push(t);

                i += 1;
            } else {
                t[0] = b[0];
                t[1] = b[1];
                ans.push(t);

                j += 1;
            }
        }

        while i < len1 {
            let a = &nums1[i];
            let t = a.clone();
            ans.push(t);
            i += 1;
        }
        while j < len2 {
            let b = &nums2[j];
            let t = b.clone();
            ans.push(t);
            j += 1;
        }

        ans
    }
}