struct Solution;

impl Solution {
    // 0ms
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i = -1 as i32;
        let mut top = 0 as i32;
        while top < arr.len() as i32 {
            i += 1;

            if arr[i as usize] != 0 {
                top += 1;
            } else {
                top += 2;
            }
        }

        println!("i = {}, top = {}", i, top);

        let mut j = arr.len() as i32 - 1;
        if top as usize == arr.len() + 1 {
            arr[j as usize] = 0;
            j -= 1;
            i -= 1;
        }
    
        while j >= 0 {
            arr[j as usize] = arr[i as usize];
            j -= 1;
            if arr[i as usize] == 0 {
                arr[j as usize] = arr[i as usize];
                j -= 1;
            }

            i -= 1;
        }
    }
}