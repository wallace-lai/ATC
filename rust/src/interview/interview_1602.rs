use std::collections::HashMap;

struct WordsFrequency {
    m: HashMap<String, i32>,
}

impl WordsFrequency {

    fn new(book: Vec<String>) -> Self {
        let mut ans = Self {
            m: HashMap::new()
        };

        for word in book {
            *ans.m.entry(word).or_insert(0) += 1;
        }

        ans
    }
    
    fn get(&self, word: String) -> i32 {
        *self.m.get(&word).unwrap_or(&0)
    }
}