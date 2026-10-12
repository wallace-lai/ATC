struct Solution;

impl Solution {
    /// 表达式仅包含
    /// 1. 非负整数
    /// 2. 空格
    /// 3. + - * / 四种运算符
    fn eval(s: String) -> i32 {
        // 字符转字符串引用
        fn c2str(op: char) -> &'static str {
            match op {
                '+' => "+",
                '-' => "-",
                '*' => "*",
                '/' => "/",
                _ => panic!("Unknown operator {op} !"),
            }
        }

        // 运算符优先级，值越大优先级越高
        fn pri(op: &str) -> i32 {
            match op {
                "+" => 1,
                "-" => 1,
                "*" => 2,
                "/" => 2,
                _ => panic!("Unknown operator {:?} !", op),
            }
        }

        // 执行运算
        fn apply_op(ops: &mut Vec<&str>, nums: &mut Vec<i32>) {
            if let Some(op) = ops.pop() {
                let b = nums.pop().unwrap();
                let a = nums.pop().unwrap();
                match op {
                    "+" => { nums.push(a + b); },
                    "-" => { nums.push(a - b); },
                    "*" => { nums.push(a * b); },
                    "/" => {
                        if b == 0 {
                            panic!("Divided by zero !");
                        } else {
                            nums.push(a / b);
                        }
                    },
                    _ => panic!("Unknown operator {:?} !", op),
                }
            }
        }

        let b = s.as_bytes();
        let n = b.len();
        let mut nums = Vec::new();
        let mut ops = Vec::new();

        let mut i = 0;
        while i < n {
            let c = b[i] as char;
            if c.is_ascii_whitespace() {
                // 遇到空格
                i += 1;
                continue;
            }

            if c.is_ascii_digit() {
                // 遇到非负整数
                let mut p = i;
                let mut num = 0;
                while p < n && b[p].is_ascii_digit() {
                    num = num * 10 + (b[p] - b'0') as i32;
                    p += 1;
                }
                nums.push(num);
                i = p;
            } else if "+-*/".contains(c) {
                // 遇到运算符，当运算符优先级不高于栈顶时，先计算栈顶
                while let Some(&op) = ops.last() &&
                    pri(op) >= pri(c2str(c)) {
                    apply_op(&mut ops, &mut nums);
                }
                ops.push(c2str(c));
                i += 1;
            } else {
                // 遇到超出范围的非法字符
                panic!("Illegal character {c} !");
            }
        }

        while let Some(&op) = ops.last() {
            apply_op(&mut ops, &mut nums);
        }
        if nums.len() != 1 {
            panic!("Illegal expression !");
        }

        nums[0]
    }

    pub fn calculate(s: String) -> i32 {
        Self::eval(s)
    }
}

