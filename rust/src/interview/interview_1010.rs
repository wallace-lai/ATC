struct StreamRank {
    n: usize,
    b: Vec<i64>,
}


impl StreamRank {

    fn new() -> Self {
        // x <= 50000，偏移后最大下标50001，开50002即可
        let n = 50002;
        Self {
            n: n,
            b: vec![0; n],
        }
    }
    
    #[inline]
    fn lowbit(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    // 单点更新：count[i] += 1
    fn update(&mut self, mut i: usize) {
        while i < self.n {
            self.b[i] += 1;
            i += Self::lowbit(i);
        }
    }

    // 前缀和查询：[1..=i] 的和
    fn query(&self, mut i: usize) -> i32 {
        let mut ans = 0;
        while i > 0 {
            ans += self.b[i];
            i -= Self::lowbit(i);
        }
        ans as _
    }

    // 题目接口
    fn track(&mut self, x: i32) {
        self.update(x as usize + 1);
    }
    
    fn get_rank_of_number(&self, x: i32) -> i32 {
        self.query(x as usize + 1)
    }
}