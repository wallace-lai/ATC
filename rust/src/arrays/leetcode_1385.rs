struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut ans = 0;
        for &i in arr1.iter() {
            let mut ok = true;
            for &k in arr2.iter() {
                ok = ok && ((i - k).abs() > d);
            }
            if ok { ans += 1; }
        }

        ans
    }
}