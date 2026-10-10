use std::collections::HashMap;

struct RangeFreqQuery {
    index: HashMap<i32, Vec<usize>>,    // val --> val在数组中出现的位置
}

impl RangeFreqQuery {
    // 找到第一个 >= t 的位置
    fn lower_bound(nums: &Vec<usize>, t: i32) -> i32 {
        // 左闭右开区间
        let mut left = 0;
        let mut right = nums.len() as i32;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] >= t as usize {
                right = mid;    // mid可能是答案，需往前找
            } else {
                left = mid + 1;
            }
        }

        left
    }

    // 找到第一个 >t 的位置
    fn upper_bound(nums: &Vec<usize>, t: i32) -> i32 {
        // 左闭右开区间
        let mut left = 0;
        let mut right = nums.len() as i32;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] > t as usize {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn new(arr: Vec<i32>) -> Self {
        let mut ans = Self {
            index: HashMap::new(),
        };

        for (i, &v) in arr.iter().enumerate() {
            ans.index.entry(v).or_insert(vec![]).push(i);
        }

        ans
    }
    
    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        let mut ans = 0;
        if let Some(pos) = self.index.get(&value) {
            // 从pos中找到第一个 >= left 的值和最后一个 <= right 的值
            let beg = Self::lower_bound(pos, left);
            let end = Self::upper_bound(pos, right) - 1;
            if end >= beg && beg < pos.len() as i32 && end >= 0 {
                ans += end - beg + 1;
            }
        }

        ans
    }
}
