struct Solution;

impl Solution {
    // WA
    // pub fn minimum_switching_times(source: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> i32 {
    //     let m = source.len();
    //     let n = source[0].len();

    //     let mut s: HashMap<i32, i32> = HashMap::with_capacity(m * n);
    //     let mut t: HashMap<i32, i32> = HashMap::with_capacity(m * n);
    //     for row in source.iter() {
    //         for col in row.iter() {
    //             *s.entry(*col).or_insert(0) += 1;
    //         }
    //     }
    //     for row in target.iter() {
    //         for col in row.iter() {
    //             *t.entry(*col).or_insert(0) += 1;
    //         }
    //     }

    //     let mut ans = 0;
    //     for (ks, vs) in s.iter() {
    //         match t.get(ks) {
    //             Some(vt) => {
    //                 let min = vt.min(vs);
    //                 ans += (vs - min) + (vt - min);
    //             },
    //             None => { ans += vs; }
    //         }
    //     }

    //     ans
    // }

    // 15ms，击败100%
    pub fn minimum_switching_times(source: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> i32 {
        let m = source.len() as i32;
        let n = source[0].len() as i32;

        let mut count = [0; 10004];
        for &i in source.iter().flatten() {
            count[i as usize] += 1;
        }

        let mut num = 0;
        for &i in target.iter().flatten() {
            if count[i as usize] > 0 {
                num += 1;
                count[i as usize] -= 1;
            }
        }

        m * n - num
    }
}