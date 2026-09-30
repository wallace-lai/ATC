struct Solution;

impl Solution {
    // 法一：O(nk)
    // pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    //     if k == 0 { return vec![0; code.len()]; }
    //     let mut v = vec![0; code.len()];

    //     let n = v.len() as i32;
    //     for i in 0..v.len() {
    //         let mut sum = 0;
    //         if k > 0 {
    //             for x in 1..=k {
    //                 let idx = (i as i32 + x) % n;
    //                 sum += code[idx as usize];
    //             }
    //         } else {
    //             for x in k..=-1 {
    //                 let idx = (i as i32 + x + n) % n;
    //                 sum += code[idx as usize];
    //             }
    //         }
    //         v[i] = sum;
    //     }

    //     v
    // }

    // 法二：O(n) 滑动窗口
    pub fn decrypt(_code: Vec<i32>, _k: i32) -> Vec<i32> {
        todo!()
    }
}
