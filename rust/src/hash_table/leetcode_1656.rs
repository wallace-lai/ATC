struct OrderedStream {
    stream: Vec<String>,
    ptr: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            stream: vec!["".to_string(); n as usize + 1],
            ptr: 1
        }
    }
    
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.stream[id_key as usize] = value;
        let mut ans = vec![];
        while self.ptr < self.stream.len() &&
            self.stream[self.ptr] != "".to_string() {
            ans.push(self.stream[self.ptr].clone());
            self.ptr += 1;
        }

        ans
    }
}