struct Solution;

impl Solution {
    // O(n)，0ms，击败100%
    // pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    //     let len = arr.len();
    //     assert!(len >= 3);

    //     let mut ans = 0;
    //     for i in 1..(len - 1) {
    //         if arr[i - 1] < arr[i] && arr[i] > arr[i + 1] {
    //             ans = i;
    //             break;
    //         }
    //     }

    //     ans as i32
    // }

    // O(logn)，0ms，击败100%
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        assert!(len >= 3);

        let mut ans = 0;
        let mut left = 0;
        let mut right = len - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if arr[mid - 1] < arr[mid] && arr[mid] > arr[mid + 1] {
                ans = mid;
                break;
            } else if arr[mid - 1] < arr[mid] && arr[mid] < arr[mid + 1] {
                left = mid + 1;
            } else if arr[mid - 1] > arr[mid] && arr[mid] > arr[mid + 1] {
                right = mid - 1;
            }
        }

        ans as i32
    }
}