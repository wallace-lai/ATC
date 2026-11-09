struct Solution;

impl Solution {
    pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        
        // 借助柱子 b，把 a 顶部的 n 个盘子搬到 c 顶部
        fn move_disk(n: usize, a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
            if n == 0 { return; }

            // 借助柱子 c，把 a 顶部的 n-1 个盘子搬到 b 顶部
            move_disk(n - 1, a, c, b);
            // 把 a 剩余的 1 个盘子搬到 c 顶部，随后 a 上无盘子
            c.push(a.pop().unwrap());
            // 借助柱子 a，把 b 顶部的 n-1 个盘子搬到 c 顶部
            move_disk(n - 1, b, a, c)
        }

        let n = a.len();
        move_disk(n, a, b, c);
    }
}