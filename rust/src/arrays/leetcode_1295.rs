struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for n in nums {
            let u = n.unsigned_abs();
            let w = if u == 0 { 1 } else { u.ilog10() + 1 };
            if w & 1 == 0 { ans += 1; }
        }
        ans
    }
}