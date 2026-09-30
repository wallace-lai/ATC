struct Solution;

impl Solution {
    // 179ms
    // pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    //     let len = digits.len();
    //     let mut set = HashSet::new();

    //     for i in 0..len {
    //         if digits[i] == 0 { continue; }
    //         for j in 0..len {
    //             if j == i { continue; }
    //             for k in 0..len {
    //                 if k == j || k == i { continue; }
    //                 if digits[k] & 1 == 1 { continue; }
    //                 let num = digits[i] * 100 + digits[j] * 10 + digits[k];
    //                 set.insert(num);
    //             }
    //         }
    //     }

    //     let mut ans: Vec<i32> = set.into_iter().collect();
    //     ans.sort();

    //     ans
    // }

    // 枚举所有的三位数
    // 0ms，击败100%
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut count = [0; 10];
        for d in digits {
            count[d as usize] += 1;
        }

        let mut tmp = [0; 10];
        let mut ans = vec![];
        for i in (100..1000).step_by(2) {
            tmp.fill(0);

            let mut n = i;
            while n > 0 {
                tmp[n % 10] += 1;
                n /= 10;
            }

            let mut is_valid = true;
            for k in 0..10 {
                if tmp[k] > 0 && count[k] < tmp[k] {
                    is_valid = false;
                    break;
                }
            }

            if is_valid { ans.push(i as i32); }
        }

        ans
    }
}