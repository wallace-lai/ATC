struct Solution;

impl Solution {
    // 错误：分配内存大小超出限制
    // pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
    //     let mu = m as usize;
    //     let nu = n as usize;
    //     let mut value = vec![0; mu * nu];

    //     for op in &ops {
    //         let ai = op[0] as usize;
    //         let bi = op[1] as usize;
    //         for x in 0..ai {
    //             for y in 0..bi {
    //                 let idx = x * nu + y;
    //                 value[idx] += 1;
    //             }
    //         }
    //     }

    //     let mut max_value = 0;
    //     let mut value_cnt = 0;
    //     for &v in value.iter() {
    //         if v > max_value {
    //             max_value = v;
    //             value_cnt = 1;
    //         } else if v == max_value {
    //             value_cnt += 1;
    //         }
    //     }

    //     value_cnt     
    // }


    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.is_empty() {
            return m * n;
        }

        // ops不可能为空，因此unwrap()是安全的
        let min_x = ops.iter().map(|v| v[0]).min().unwrap();
        let min_y = ops.iter().map(|v| v[1]).min().unwrap();
        min_x * min_y
    }
}