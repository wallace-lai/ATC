struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut v: Vec<(usize, usize)> = mat.iter()
            .enumerate()
            .map(|(i, r)| (
                i, r.iter().filter(|&k| *k == 1).count()
            ))
            .collect();

        v.sort_by_key(|(_, r)| *r);
        // println!("v is {:?}", v);

        v.iter().map(|(i, _)| *i as i32).take(k).collect()
    }
}