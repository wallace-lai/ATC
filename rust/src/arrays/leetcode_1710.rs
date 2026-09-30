struct Solution;

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_by_key(|v| -v[1]);

        let mut i = 0;
        let mut ans = 0;
        while truck_size > 0 && i < box_types.len() {
            let load = std::cmp::min(truck_size, box_types[i][0]);
            ans += load * box_types[i][1];
            i += 1;
            truck_size -= load;
        }

        ans
    }
}