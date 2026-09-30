struct Solution;

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let n = words[0].len();
        let len = words.len();
        let mut count: Vec<Vec<i8>> = vec![vec![0; n - 1]; len];

        for i in 0..len {
            let cnt = &mut count[i];
            let slice = words[i].as_bytes();
            for k in 1..n {
                cnt[k - 1] = slice[k] as i8 - slice[k - 1] as i8;
            }
        }

        let common = if count[0] == count[1] {
            &count[0]
        } else if count[0] == count[2] {
            &count[0]
        } else {
            &count[1]
        };

        for (i, cnt) in count.iter().enumerate() {
            if cnt != common {
                return words[i].clone();
            }
        }

        "123".to_string()
    }
}