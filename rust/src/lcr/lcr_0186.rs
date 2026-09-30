struct Solution;

impl Solution {
    // 1ms，击败100%
    pub fn check_dynasty(mut places: Vec<i32>) -> bool {
        places.sort_unstable();
        println!("places : {:?}", places);

        let mut zero_num = 0;
        let mut miss_num = 0;
        for i in 0..places.len() {
            if places[i] == 0 {
                zero_num += 1;
            }

            if i + 1 < places.len() {
                // 存在相同的非0朝代，则最终结果必然不可能为连续的5个朝代
                if places[i] != 0 && places[i] == places[i + 1] {
                    return false;
                }

                if places[i] != 0 && places[i + 1] != 0 {
                    miss_num += places[i + 1] - places[i] - 1;
                }
            }
        }

        println!("zero_num : {zero_num}, miss_num : {miss_num}");

        zero_num >= miss_num
    }
}