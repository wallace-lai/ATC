struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stk = Vec::new();

        for t in tokens.iter() {
            match t.as_str() {
                "+" => {
                    let a = stk.pop().unwrap();
                    let b = stk.pop().unwrap();
                    stk.push(a + b);
                },
                "-" => {
                    let a = stk.pop().unwrap();
                    let b = stk.pop().unwrap();
                    stk.push(b - a);
                },
                "*" => {
                    let a = stk.pop().unwrap();
                    let b = stk.pop().unwrap();
                    stk.push(a * b);
                },
                "/" => {
                    let a = stk.pop().unwrap();
                    let b = stk.pop().unwrap();
                    stk.push(b / a);
                },
                _ => {
                    let n = t.parse::<i32>().unwrap();
                    stk.push(n);
                }
            }
        }

        stk[0]
    }
}