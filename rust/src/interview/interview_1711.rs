struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：单次遍历
    // 存在以下几个可优化的点：
    // 1. for i in 0..ws.len() {...} 存在边界检查，可优化
    // 2. 距离是整数，最小值就是1，一旦 ans == 1就可以提前退出了
    // 3. ws[i] == w1 每次比较都要经过 String 的 Deref，可以在循环
    // 外就将 w1 转成 &str，这样循环内就是纯 str 比较了
    // 4. 开头建议判断 w1 和 w2 是否相等，防御性编程
    pub fn find_closest(ws: Vec<String>, w1: String, w2: String) -> i32 {
        let mut ans = i32::MAX;
        let mut p1 = -1;
        let mut p2 = -1;

        for i in 0..ws.len() {
            if ws[i] == w1 {
                if p2 >= 0 {
                    ans = ans.min(i as i32 - p2);
                }
                p1 = i as i32;
            } else if ws[i] == w2 {
                if p1 >= 0 {
                    ans = ans.min(i as i32 - p1);
                }
                p2 = i as i32;
            }
        }

        ans
    }

    // 法二：法一的性能优化版本
    pub fn find_closest(ws: Vec<String>, w1: String, w2: String) -> i32 {
        let (s1, s2) = (w1.as_str(), w2.as_str());
        // 防御性编程，理论上应该找相邻两次出现的最小间距
        // 但在这里不做处理，仅提醒一下
        // if s1 == s2 { }

        let mut ans = i32::MAX;
        let mut p1 = -1;
        let mut p2 = -1;

        for (i, w) in ws.iter().enumerate() {
            let i = i as i32;
            let s = w.as_str();

            if s == s1 {
                if p2 >= 0 {
                    ans = ans.min(i - p2);
                }
                p1 = i;
            } else if s == s2 {
                if p1 >= 0 {
                    ans = ans.min(i - p1);
                }
                p2 = i;
            }

            // 理论下界，提前退出
            if ans == 1 { return 1; }
        }

        ans
    }
}