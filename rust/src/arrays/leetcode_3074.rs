struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let sum: i32 = apple.iter().sum();
        let mut ans = 0;
        let mut cap = 0;

        capacity.sort_by_key(|&cap| -cap);
        for i in 0..capacity.len() {
            cap += capacity[i];
            ans += 1;
            if cap >= sum {
                break;
            }
        }

        ans
    }
}