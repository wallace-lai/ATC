struct Solution;

use std::collections::HashMap;

// impl Solution {
//     pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
//         let mut map: HashMap<String, Vec<String>> = HashMap::new();
//         for str in strs {
//             // 将字符串中的字符收集到Vec<char>中并排序
//             let mut chars: Vec<char> = str.chars().collect();
//             chars.sort_unstable();
//             let key: String = chars.into_iter().collect();

//             // 原始字符串存入哈希表
//             map.entry(key).or_insert_with(Vec::new).push(str.clone());
//         }

//         map.into_values().collect()
//     }
// }

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();

        for word in strs {
            let mut counts = [0u8; 26];
            for &b in word.as_bytes() {
                let idx = (b - b'a') as usize;
                counts[idx] += 1;
            }

            map.entry(counts).or_insert_with(Vec::new).push(word);
        }

        let results: Vec<Vec<String>> = map.into_values().collect();
        results
    }
}