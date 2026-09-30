struct Solution;

impl Solution {
    pub fn is_triangle(a: i32, b: i32, c: i32) -> bool {
        if a + b > c && b + c > a && c + a > b {
            return true;
        }

        false
    }

    // 0ms - 击败100%
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];

        if !Self::is_triangle(a, b, c) {
            return "none".to_string();
        }

        let ans;
        if a == b && b == c {
            ans = "equilateral".to_string();
        } else if a != b && b != c && c != a {
            ans = "scalene".to_string();
        } else {
            ans = "isosceles".to_string();
        }

        ans
    }
}