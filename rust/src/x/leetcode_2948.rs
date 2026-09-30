struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let len = nums.len();
        let mut v: Vec<(i32, usize)> = nums.iter()
            .enumerate()
            .map(|(i, num)|(*num, i))
            .collect();
        v.sort_unstable_by_key(|item| item.0);
        let indexs: Vec<usize> = v.iter().map(|item| item.1).collect();
        let values: Vec<i32> = v.iter().map(|item| item.0).collect();
        
        // println!("v is {:?}", v);

        let mut ans = nums.clone();
        let mut idx = 0;
        while idx < len {
            let start = idx;
            let mut sub_indexs = vec![];
            let mut sub_values = vec![];
            while idx < len && (idx == start ||
                values[idx] - values[idx - 1] <= limit) {
                sub_indexs.push(indexs[idx]);
                sub_values.push(values[idx]);
                idx += 1;
            }

            sub_indexs.sort_unstable();
            for (index, value) in sub_indexs.into_iter().zip(sub_values.into_iter()) {
                ans[index] = value;
            }
        }

        ans
    }
}