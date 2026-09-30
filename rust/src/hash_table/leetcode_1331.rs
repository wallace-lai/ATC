struct Solution;

impl Solution {
    // 9ms
    // pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    //     if arr.len() == 0 { return vec![]; }
    //     let mut v = arr.clone();
    //     v.sort_unstable();

    //     let mut id = 1;
    //     let mut prev = v[0];
    //     let mut map = HashMap::with_capacity(v.len());
    //     map.insert(prev, id);

    //     for i in 1..v.len() {
    //         if v[i] != prev {
    //             id += 1;
    //             prev = v[i];
    //         }

    //         map.insert(v[i], id);
    //     }

    //     // println!("map is {:?}", map);

    //     let mut ans = Vec::with_capacity(arr.len());
    //     for num in arr {
    //         if let Some(id) = map.get(&num) {
    //             ans.push(*id);
    //         }
    //     }

    //     ans
    // }

    // 7ms
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut indices: Vec<usize> = (0..arr.len()).collect();
        indices.sort_unstable_by_key(|i| arr[*i]);

        let mut seq = 0;
        let mut prev = i32::MIN;
        let mut ans = vec![0; arr.len()];
        for i in indices {
            if arr[i] != prev {
                seq += 1;
            }
            prev = arr[i];
            ans[i] = seq;
        }

        ans
    }
}