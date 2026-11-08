struct Solution;

impl Solution {
    pub fn merge(a: &mut Vec<i32>, m: i32, b: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = (m + n - 1) as usize;

        while i >= 0 && j >= 0 {
            let (ui, uj) = (i as usize, j as usize);
            if a[ui] >= b[uj] {
                a[k] = a[ui];
                i -= 1;
            } else {
                a[k] = b[uj];
                j -= 1;
            }
            k -= 1;
        }

        while i >= 0 {
            a[k] = a[i as usize];
            i -= 1;
            k -= 1;
        }
        while j >= 0 {
            a[k] = a[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}