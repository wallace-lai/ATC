struct Solution;

impl Solution {
    // CV大法，没看懂
    pub fn valid_sequence(s: String, t: String) -> Vec<i32> {
        let n = s.len();
        let m = t.len();
        let ss = s.as_bytes();
        let ts = t.as_bytes();

        let mut suf = vec![0; n + 1];
        suf[n] = m as i32;

        let mut i = n as i32 - 1;
        let mut j = m as i32 - 1;
        while i >= 0 {
            if j >= 0 && ss[i as usize] == ts[j as usize] {
                j -= 1;
            }

            suf[i as usize] = j + 1;
            i -= 1;
        }

        let mut ans = vec![0; m];
        let mut is_changed = false;
        i = 0;
        j = 0;
        while i < n as i32 {
            if ss[i as usize] == ts[j as usize] ||
                !is_changed && suf[(i + 1) as usize] <= j + 1 {
                if ss[i as usize] != ts[j as usize] {
                    is_changed = true;
                }

                ans[j as usize] = i;
                j += 1;
                if j == m as i32 {
                    return ans;
                }
            }

            i += 1;
        }

        vec![]
    }
}