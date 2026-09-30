struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn half_questions(questions: Vec<i32>) -> i32 {
        assert!(questions.len() & 1 == 0);
        let mut n = questions.len() as i32 / 2;

        let mut count = HashMap::with_capacity(questions.len());
        for &q in questions.iter() {
            *count.entry(q).or_insert(0) += 1;
        }

        let mut v: Vec<i32> = count.into_iter().map(|(_, v)| v).collect();
        v.sort_unstable_by(|a, b| { b.cmp(&a) });
        // println!("v is {:?}", v);

        let mut idx = 0;
        while n > 0 && idx < v.len() {
            n -= v[idx];
            idx += 1;
        }

        idx as i32
    }
}