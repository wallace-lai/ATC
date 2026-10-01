struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut idx = 0;
        let mut stk = Vec::with_capacity(pushed.len());

        for x in pushed {
            stk.push(x);
            while stk.len() > 0 && stk.last().unwrap() == &popped[idx] {
                stk.pop();
                idx += 1;
            }
        }

        stk.is_empty()
    }
}