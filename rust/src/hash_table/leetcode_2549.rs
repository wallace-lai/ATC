struct Solution;

impl Solution {
    // 法一：模拟
    // pub fn distinct_integers(n: i32) -> i32 {
    //     use std::collections::HashSet;
    //     let mut table: HashSet<i32> = HashSet::new();
    //     table.insert(n);

    //     loop {
    //         let mut append = vec![];

    //         for x in table.iter() {
    //             for i in 1..=n {
    //                 if x % i == 1 && !table.contains(&i){
    //                     append.push(i);
    //                 }
    //             }
    //         }
    //         if append.len() == 0 { break; }

    //         for i in append {
    //             table.insert(i);
    //         }
    //     }

    //     table.len() as i32
    // }

    // 法二：数学推导
    pub fn distinct_integers(n: i32) -> i32 {
        if n < 3 { 1 } else { n - 1 }
    }
}