use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct FrequencyTracker {
    // add 会使得数字的频率增加
    // delete 使得数字的频率减少
    // hasFreq 直接查询对应的栈是否为空即可
    freq: HashMap<i32, i32>,        // val --> 频率
    group: HashMap<i32, i32>        // 频率 --> 该频率下的元素个数
}

impl FrequencyTracker {
    fn new() -> Self {
        Self {
            freq: HashMap::new(),
            group: HashMap::new(),
        }
    }
    
    fn add(&mut self, val: i32) {
        // 原频率的元素个数减1
        let f = self.freq.entry(val).or_insert(0);
        if *f > 0 {
            *self.group.get_mut(f).unwrap() -= 1;
        }

        // 频率增加，对应频率的元素个数加1
        *f += 1;
        let f = *f;
        *self.group.entry(f).or_insert(0) += 1;
    }
    
    fn delete_one(&mut self, val: i32) {
        // 结构中存在val时才处理
        if let Entry::Occupied(mut e) = self.freq.entry(val) {
            let f = e.get_mut();
            // 修改对应频率元素个数
            *self.group.get_mut(f).unwrap() -= 1;
            if *f > 1 {
                *self.group.get_mut(&(*f - 1)).unwrap() += 1;
            }
    
            *f -= 1;
            if *f == 0 { e.remove(); }
        }
    }
    
    fn has_frequency(&mut self, frequency: i32) -> bool {
        let ans = 
            match self.group.get(&frequency) {
                Some(num) => {
                    if *num > 0 { true } else { false }
                },
                None => { false }
            };
        ans
    }
}
