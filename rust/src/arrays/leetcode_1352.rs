struct ProductOfNumbers {
    prod: Vec<i32>
}

impl ProductOfNumbers {

    fn new() -> Self {
        Self {
            prod: vec![1]
        }
    }
    
    fn add(&mut self, num: i32) {
        if num != 0 {
            let &last = self.prod.last().unwrap();
            self.prod.push(last * num);
        } else {
            self.prod.clear();
            self.prod.push(1);
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let n = self.prod.len();
        if n <= k {
            return 0;
        } else {
            return self.prod[n - 1] / self.prod[n - 1 - k];
        }
    }
}
