struct Solution;

impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut even: Vec<i32> = (0..nums.len())
            .step_by(2)
            .map(|i| nums[i])
            .collect();
        let mut odd: Vec<i32> = (1..nums.len())
            .step_by(2)
            .map(|i| nums[i])
            .collect();

        // println!("even is {:?}", even);
        // println!("odd is {:?}", odd);

        even.sort_unstable();
        odd.sort_unstable_by_key(|&n| -n);

        let mut i = 0;
        let mut j = 0;
        let mut ans = Vec::with_capacity(nums.len());
        while i < even.len() && j < odd.len() {
            ans.push(even[i]);
            ans.push(odd[j]);
            i += 1;
            j += 1;
        }
        if i < even.len() { ans.extend_from_slice(&even[i..]); }
        if j < odd.len() { ans.extend_from_slice(&odd[j..]); }

        ans
    }
}