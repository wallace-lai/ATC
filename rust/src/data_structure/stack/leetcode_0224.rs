struct Solution;

impl Solution {
    fn eval(s: String) -> i32 {
        // 字符转字符串引用
        fn c2str(op: char) -> &'static str {
            match op {
                '+' => "+",
                '-' => "-",
                '*' => "*",
                '/' => "/",
                _ => panic!("Illegal operator {op} !"),
            }
        }

        // 运算符优先级
        fn pri(op:&str) -> i32 {
            match op {
                "+" => 1,
                "-" => 1,
                "*" => 2,
                "/" => 2,
                "u-" => 3,
                _ => panic!("Unknown operator {:?} !", op),
            }
        }

        // 执行运算符
        fn apply_op(ops: &mut Vec<&str>, nums: &mut Vec<i32>) {
            if let Some(op) = ops.pop() {
                if op == "u-" {
                    // 遇到负号
                    let a = nums.pop().unwrap();
                    nums.push(-a);
                    return;
                }

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
                    _ => { panic!("Unknown operator {:?} !", op); }
                }
            }
        }

        let b = s.as_bytes();
        let n = b.len();
        let mut nums: Vec<i32> = Vec::new();
        let mut ops: Vec<&str> = Vec::new();

        let mut i = 0;
        let mut prev = None;

        while i < n {
            let c = b[i] as char;
            if c.is_whitespace() { i += 1; continue; }

            if c.is_ascii_digit() {
                // 遇到数字，解析后将其压入nums栈中
                let mut p = i;
                let mut num = 0;
                while p < n && b[p].is_ascii_digit() {
                    num = num * 10 + (b[p] - b'0') as i32;
                    p += 1;
                }
                nums.push(num);
                prev = Some(b[p - 1] as char);
                i = p;
            } else if c == '(' {
                // 遇到左括号，将其压入ops栈中
                ops.push("(");
                prev = Some(c);
                i += 1;
            } else if c == ')' {
                // 遇到右括号，不断弹出ops栈顶运算符，直到遇到左括号'('
                while let Some(&op) = ops.last() && op != "(" {
                    apply_op(&mut ops, &mut nums);
                }
                if ops.is_empty() {
                    // 有未配对的括号，表达式非法
                    panic!("Illegal expression, parentheses mismatch !");
                }
                ops.pop();  // 弹出左括号'('
                prev = Some(c);
                i += 1;
            } else if "+-*/".contains(c) {
                // 遇到运算符，先检查其是否为负号
                if c == '-' && (prev.is_none() || "+-*/(".contains(prev.unwrap())) {
                    ops.push("u-");
                    prev = Some(c);
                    i += 1;
                    continue;
                }
                // 当运算符优先级不高于栈顶时，先计算栈顶
                while let Some(&op) = ops.last() &&
                    op != "(" && pri(op) >= pri(c2str(c)) {
                    apply_op(&mut ops, &mut nums);
                }
                ops.push(c2str(c));
                prev = Some(c);
                i += 1;
            } else {
                panic!("Illegal character {c} !");
            }
        }

        while let Some(&op) = ops.last() {
            if op == "(" {
                panic!("Illegal expression, parentheses mismatch !");
            }
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