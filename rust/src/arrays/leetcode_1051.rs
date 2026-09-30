struct Solution;

impl Solution {
    // 法一：快速排序，O(NlogN)，0ms，击败100%
    // pub fn height_checker(heights: Vec<i32>) -> i32 {
    //     let mut expected = heights.clone();
    //     expected.sort_unstable();
    //     // println!("heights : {:?}", heights);
    //     // println!("expected: {:?}", expected);

    //     let mut ans = 0;
    //     for i in 0..expected.len() {
    //         if heights[i] != expected[i] {
    //             ans += 1;
    //         }
    //     }

    //     ans
    // }

    // 法二：计数排序，O(N)，0ms，击败100%
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut count = [0_u8; 128];
        for &h in heights.iter() {
            count[h as usize] += 1;
        }

        let mut ans = 0;
        let mut idx = 0;
        for &h in heights.iter() {
            while count[idx] == 0 {
                idx += 1;
            }

            if h != idx as i32 {
                ans += 1;
            }
            count[idx] -= 1;
        }

        ans
    }
}