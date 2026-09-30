struct Solution;

impl Solution {
    // 法一：两次遍历
    // pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    //     let mut m = image;
    //     // 就地反转矩阵的每一行
    //     m.iter_mut().for_each(|row| row.reverse());
    //     // 就地翻转矩阵的每个元素
    //     m.iter_mut().for_each(|row| {
    //         row.iter_mut().for_each(|elem| *elem ^= 1);
    //     });
    //     m
    // }

    // 法一：一次遍历
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut m = image;
        for row in m.iter_mut() {
            let (mut left, mut right) = (0, row.len().checked_sub(1).unwrap_or(0));
            while left < right {
                // 交换并同时取反
                let tmp = row[left] ^ 1;
                row[left] = row[right] ^ 1;
                row[right] = tmp;

                left += 1;
                right -= 1;
            }

            // 奇数长度时，中间元素单独取反
            if left == right {
                row[left] ^= 1;
            }
        }

        m
    }
}