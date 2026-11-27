struct Solution;

impl Solution {
    // 法一：动态规划 + 贪心
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let budget = budget as usize;
        let mut f = vec![0; budget + 1];
        let mut min_price = usize::MAX;

        for p in items.iter() {
            let factor = p[0];
            let price = p[1] as usize;
            min_price = min_price.min(price);

            // 统计 factor 倍数的个数（包括 facotr 本身）
            let mut count = 0;
            for q in items.iter() {
                if q[0] % factor == 0 {
                    count += 1;
                }
            }

            // 将当前物品 p 视作一个体积为 price 且价值为 count 的物品
            for j in (price..=budget).rev() {
                f[j] = f[j].max(f[j - price] + count);
            }
        }

        let mut ans = 0;
        for i in 0..=budget {
            ans = ans.max(f[i] + (budget - i) / min_price);
        }
        ans as _
    }
}