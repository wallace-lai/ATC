struct Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut nums = Vec::<i32>::with_capacity(operations.len());

        for ops in operations.iter() {
            match ops.as_str() {
                "+" => {
                    let len = nums.len();
                    let num = nums[len - 1] + nums[len - 2];
                    nums.push(num);
                },
                "D" => {
                    let num = nums[nums.len() - 1] * 2;
                    nums.push(num);
                },
                "C" => {
                    nums.pop();
                },
                _ => {
                    let num = ops.parse::<i32>().unwrap();
                    nums.push(num);
                }
            }
        }

        nums.iter().sum()
    }
}