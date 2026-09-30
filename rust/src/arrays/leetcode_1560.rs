struct Solution;

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; n as usize];
        for i in 1..rounds.len() {
            let mut start = rounds[i - 1] - 1;
            let end = rounds[i] - 1;
            if i == 1 {
                count[start as usize] += 1;
            };

            while start != end {
                start = (start + 1 + n) % n;
                count[start as usize] += 1;
            }
        }

        let mut ans = Vec::new();
        let &max = count.iter().max().unwrap();
        for i in 0..count.len() {
            if count[i] == max { ans.push(i as i32 + 1); }
        }

        ans
    }
}



