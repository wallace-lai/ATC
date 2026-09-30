struct Solution;

impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut v = vec![0; n];
        let mut ans = 0;

        pub fn do_check(slice: &[i32]) -> bool {
            for i in 1..slice.len() {
                if slice[i] <= slice[i - 1] {
                    return false;
                }
            }
            true
        }

        let mut checker = |beg, end| -> bool {
            if beg == 0 && end == n - 1 {
                return true;
            }
            if beg != 0 && end != n - 1 {
                v.clear();
                v.extend_from_slice(&nums[0..beg]);
                v.extend_from_slice(&nums[end + 1..]);
                return do_check(&v);
            }

            let slice = if beg == 0 {
                &nums[end + 1..]
            } else {
                &nums[0..beg]
            };
            return do_check(slice);
        };

        for i in 0..n {
            for d in 0..n {
                if i + d >= n { break; }
                if checker(i, i + d) {
                    let slice = &nums[i..(i + d)];
                    println!("slice is {:?}", slice);
                    ans += 1;
                }
            }
        }

        ans
    }
}
