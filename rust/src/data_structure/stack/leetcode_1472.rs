struct BrowserHistory {
    curr: usize,
    history: Vec<String>,
}

impl BrowserHistory {

    fn new(homepage: String) -> Self {
        let mut ans = Self {
            curr: 0,
            history: Vec::new(), 
        };
        ans.curr = 1;
        ans.history.push(homepage);
        ans
    }
    
    fn visit(&mut self, url: String) {
        self.history.truncate(self.curr);
        self.history.push(url);
        self.curr += 1;
    }
    
    fn back(&mut self, steps: i32) -> String {
        let x = 0.max(self.curr as i32 - 1 - steps);
        let ret = self.history[x as usize].clone();
        self.curr = x as usize + 1;
        ret
    }
    
    fn forward(&mut self, steps: i32) -> String {
        let last = self.history.len() as i32 - 1;
        let x = last.min(self.curr as i32 - 1 + steps);
        let ret = self.history[x as usize].clone();
        self.curr = x as usize + 1;
        ret
    }
}
