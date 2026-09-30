struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let n = s.len();
        let mut pre = vec![usize::MAX; n];
        let mut suf = vec![usize::MAX; n];

        let mut left_nearest = usize::MAX;
        for i in 0..n {
            if s.as_bytes()[i] as char == c {
                left_nearest = i;                
            }
            pre[i] = left_nearest;
        }

        let mut right_nearest = usize::MAX;
        for i in (0..n).rev() {
            if s.as_bytes()[i] as char == c {
                right_nearest = i;
            }
            suf[i] = right_nearest;
        }

        // println!("pre is {:?}", pre);
        // println!("suf is {:?}", suf);

        let mut ans = vec![i32::MAX; n];
        for i in 0..n {
            if pre[i] != usize::MAX && suf[i] != usize::MAX {
                ans[i] = (i - pre[i]).min(suf[i] - i) as i32;
            } else {
                ans[i] = i.abs_diff(pre[i].min(suf[i])) as i32;
            }
        }
        ans
    }
}