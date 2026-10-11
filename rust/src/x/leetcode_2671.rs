use std::collections::HashMap;

struct FrequencyTracker {
    // add 会使得数字的频率增加
    // delete 使得数字的频率减少
    // hasFreq 直接查询对应的栈是否为空即可
    freq: HashMap<i32, i32>,        // val --> 频率
    group: HashMap<i32, i32>        // 频率 --> 该频率下的元素个数
}

// 43ms，击败100%
impl FrequencyTracker {
    fn new() -> Self {
        Self {
            freq: HashMap::new(),
            group: HashMap::new(),
        }
    }
    
    // 频率改变时需要同时更新
    fn update(&mut self, val: i32, d: i32) {
        let f = self.freq.entry(val).or_insert(0);
        *self.group.entry(*f).or_insert(0) -= 1;
        *f += d;
        *self.group.entry(*f).or_insert(0) += 1;
    }

    fn add(&mut self, val: i32) {
        self.update(val, 1);
    }
    
    fn delete_one(&mut self, val: i32) {
        if *self.freq.get(&val).unwrap_or(&0) > 0 {
            self.update(val, -1);
        }
    }
    
    fn has_frequency(&mut self, frequency: i32) -> bool {
        *self.group.get(&frequency).unwrap_or(&0) > 0
    }
}
