use std::collections::LinkedList;

struct StackOfPlates {
    cap: usize,
    stks: LinkedList<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StackOfPlates {

    fn new(cap: i32) -> Self {
        Self {
            cap: cap as usize,
            stks: LinkedList::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        if self.cap == 0 {
            return;
        }

        // 当前无栈或者最后一个栈已满时，创建新栈
        if self.stks.len() == 0 ||
            self.stks.back().unwrap().len() == self.cap {
            self.stks.push_back(Vec::new());
        }
        if let Some(t) = self.stks.back_mut() {
            t.push(val);
        }
    }
    
    fn pop(&mut self) -> i32 {
        // cap为0或者当前无栈时，无法pop
        if self.cap == 0 || self.stks.is_empty() {
            return -1;
        }

        let last = self.stks.back_mut().unwrap();
        let ret = last.pop().unwrap();
        if last.is_empty() {
            self.stks.pop_back();
        }

        ret
    }
    
    fn pop_at(&mut self, index: i32) -> i32 {
        // cap为0或者index超出范围时，无法pop_at
        let index = index as usize;
        if self.cap == 0 || index >= self.stks.len() {
            return -1;
        }

        // 根据index找到对应的栈nth
        let nth = self.stks.iter_mut().nth(index).unwrap();
        let ret = nth.pop().unwrap();
        if nth.is_empty() {
            // 当栈因pop而变空时，需要将其从链表中摘下
            let mut left = self.stks.split_off(index);
            let mut tail = left.split_off(1);
            self.stks.append(&mut tail);
        }
        ret
    }
}
