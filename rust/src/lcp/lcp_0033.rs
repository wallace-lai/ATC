use std::i32;

struct Solution;

impl Solution {
    // 1ms，击败100%
    pub fn store_water(bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
        let maxk = vat.iter().max().unwrap();
        if *maxk == 0 { return 0; }

        let mut ans = i32::MAX;
        let mut k = 1;
        while k <= *maxk && k < ans {
            let mut t = 0;
            for i in 0..bucket.len() {
                t += 0.max((vat[i] + k - 1) / k - bucket[i]);
            }

            ans = ans.min(t + k);
            k += 1;
        }

        ans
    }
}