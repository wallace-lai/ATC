struct Operations;

impl Operations {

    fn new() -> Self {
        Self
    }
    
    // 倍增法求相反数（x必须是 32 位整数）
    fn neg(x: i64) -> i64 {
        if x == 0 { return 0; }

        // 1. 通过回绕加法得到 -1
        let neg1 = i32::MAX.wrapping_add(i32::MAX).wrapping_add(1);

        // 2. 确定累加方向和要拆分的量
        let (step0, abs_x) = if x > 0 {
            (neg1, x)
        } else {
            (1, -x)
        };

        // 3. 预处理倍增步长：step = -1, -2, -4, ...
        // 或者(1, 2, 4, ...)， mag记录对应的绝对值
        let mut steps = Vec::new();
        let mut step: i64 = step0 as i64;
        let mut mag: i64 = 1;
        while mag <= abs_x {
            steps.push((step, mag));
            step = step.wrapping_add(step); // step *= 2
            mag = mag.wrapping_add(mag);    // mag *= 2
        }
        
        // 4. 从最大步长开始贪心分析（标准二进制分解）
        let mut ans: i64 = 0;
        let mut rem: i64 = abs_x;
        for &(s, m) in steps.iter().rev() {
            if rem >= m {
                ans = ans.wrapping_add(s);
                rem = rem.wrapping_sub(m);
            }
        }

        ans
    }

    fn minus(&self, a: i32, b: i32) -> i32 {
        let (a, b) = (a as i64, b as i64);
        a.wrapping_add(Self::neg(b)) as i32
    }
    
    // 倍增加法
    fn multiply(&self, a: i32, b: i32) -> i32 {
        let negative = (a < 0) != (b < 0);
        let x = if a < 0 { Self::neg(a as i64) } else { a as i64 };
        let y = if b < 0 { Self::neg(b as i64) } else { b as i64 };

        // 让 x 是较大的绝对值，y 是较小的绝对值，以减小循环次数
        let (x, y) = if x >= y { (x, y) } else { (y, x) };

        let mut ans: i64 = 0;
        let mut rem: i64 = y;

        while rem > 0 {
            let mut count: i64 = 1;
            let mut chunk = x;

            // 倍增：找到不超过 rem 的最大块
            while count.wrapping_add(count) <= rem {
                count = count.wrapping_add(count);
                chunk = chunk.wrapping_add(chunk);
            }

            ans = ans.wrapping_add(chunk);
            rem = rem.wrapping_sub(count);
        }

        if negative { Self::neg(ans) as i32 } else { ans as i32 }
    }
    
    // 倍增减法
    fn divide(&self, a: i32, b: i32) -> i32 {
        // 边界：INT_MIN / -1 会溢出，直接返 INT_MIN
        if a == i32::MIN && b == -1 { return i32::MIN; }

        let negative = (a < 0) != (b < 0);
        let x = if a < 0 { Self::neg(a as i64) as i64 } else { a as i64 };
        let y = if b < 0 { Self::neg(b as i64) as i64 } else { b as i64 };

        let mut ans: i64 = 0;
        let mut rem = x;

        while rem >= y {
            let mut value = y;
            let mut count: i64 = 1;

            // 倍增：找到不超过 rem 的最大除数倍数
            while value.wrapping_add(value) <= rem {
                value = value.wrapping_add(value);
                count = count.wrapping_add(count);
            }

            rem = rem.wrapping_sub(value);
            ans = ans.wrapping_add(count);
        }

        if negative { Self::neg(ans) as i32 } else { ans as i32 }
    }
}
