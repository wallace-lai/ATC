struct Solution;

impl Solution {
    #[cfg(any())]
    // WA
    // 法一：字符串，比较直观
    pub fn find_closed_numbers(num: i32) -> Vec<i32> {
        let s: Vec<char> = format!("{:b}", num).chars().collect();
        let n = s.len();
        println!("s is {:?}", s);

        let ones = s.iter().filter(|&&c| c == '1').count();
        let mut bigger = -1;
        let mut smaller = -1;

        // 稍大：从右往左找 01 -> 10，右边 1 全挤最右边
        'OUTER1: for i in (0..n - 1).rev() {
            if s[i] == '0' && s[i + 1] == '1' {
                let mut t = s.clone();
                // 01 -> 10
                t[i] = '1';
                t[i + 1] = '0';
                // 右边的 1 全挤最右边
                // i+1 右边剩余 1 的个数
                let mut rest = ones as i32 - (
                    t[..=i].iter()
                    .filter(|&&c| c == '1')
                    .count() as i32
                );
                // 剩余的 1 全挤在最右边
                for k in (i + 1)..n {
                    if rest > 0 {
                        t[k] = '1';
                        rest -= 1;
                    } else {
                        t[k] = '0';
                    }
                }
                let v = i64::from_str_radix(&t.iter().collect::<String>(), 2).unwrap();
                if v <= i32::MAX as i64 { bigger = v as i32; break 'OUTER1; }
            }
        }

        // 稍小：从右往左找 10 -> 01，右边 1 全挤在最左
        'OUTER2: for i in (0..n - 1).rev() {
            if s[i] == '1' && s[i + 1] == '0' {
                let mut t = s.clone();
                t[i] = '0';
                t[i + 1] = '1';
                
                let mut rest = ones as i32 - (
                    t[..=i].iter()
                    .filter(|&&c| c == '1')
                    .count() as i32
                ) - 1;

                for k in (i + 1)..n {
                    if rest > 0 {
                        t[k] = '1';
                        rest -= 1;
                    } else {
                        t[k] = '0';
                    }
                }

                smaller = i32::from_str_radix(&t.iter().collect::<String>(), 2).unwrap();
                break 'OUTER2;
            }
        }

        vec![bigger, smaller]
    }

    // 法二：位运算
    // 思路：要让 1 的个数不变但值变大或者变小，本质上是把某个 1 往左移动（变大）
    // 或者往右移动（变小）
    // 稍大数：从右往左找最右边的 01
    pub fn find_closed_numbers(num: i32) -> Vec<i32> {

        fn next_bigger(num: i32) -> i32 {
            let mut c = num;
            let mut c0: i32 = 0;
            let mut c1: i32 = 0;

            // 尾部 0 的个数
            while (c & 1) == 0 && c != 0 {
                c0 += 1;
                c >>= 1;
            }
            // 紧接的 1 的个数
            while (c & 1) == 1 {
                c1 += 1;
                c >>= 1;
            }
            // 全 1（无 0 可翻）或 p 到达符号位（结果溢出）
            if c0 + c1 >= 31 || c0 + c1 == 0 {
                return -1;
            }

            let p = c0 + c1;
            let mut n = num;
            n |= 1 << p;                    // p 位置 1
            n &= !((1 << p) - 1);           // p 右边清零
            n |= (1 << (c1 - 1)) - 1;       // 最低 c1-1 个位置 1
            n
        }

        fn next_smaller(num: i32) -> i32 {
            let mut c = num;
            let mut c0: i32 = 0;
            let mut c1: i32 = 0;

            // 尾部 1 的个数
            while (c & 1) == 1 {
                c1 += 1;
                c >>= 1;
            }
            // 全 1，无法再小
            if c == 0 {
                return -1;
            }
            // 紧跟的 0 的个数
            while (c & 1) == 0 && c != 0 {
                c0 += 1;
                c >>= 1;
            }

            let p = c0 + c1;
            let mut n = num;
            n &= !(1 << p);                          // p 位置 0
            n &= !((1 << p) - 1);                    // p 右边清零
            n |= ((1 << (c1 + 1)) - 1) << (c0 - 1);  // 把 c1+1 个 1 靠左放
            n
        }

        vec![next_bigger(num), next_smaller(num)]
    }
}