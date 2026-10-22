use std::collections::VecDeque;

struct AnimalShelf {
    // (编号，种类)
    dog: VecDeque<Vec<i32>>,
    cat: VecDeque<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AnimalShelf {

    fn new() -> Self {
        Self {
            dog: VecDeque::new(),
            cat: VecDeque::new(),
        }
    }
    
    fn enqueue(&mut self, animal: Vec<i32>) {
        let t = animal[1];
        match t {
            0 => { self.cat.push_back(animal); },    // 猫
            1 => { self.dog.push_back(animal); },    // 狗
            _ => {}
        }
    }
    
    fn dequeue_any(&mut self) -> Vec<i32> {
        match (self.cat.len(), self.dog.len()) {
            (0, 0) => { vec![-1, -1] },
            (0, _) => { self.dog.pop_front().unwrap() },
            (_, 0) => { self.cat.pop_front().unwrap() },
            (_, _) => {
                let first_cat = self.cat.front().unwrap();
                let first_dog = self.dog.front().unwrap();
                if first_cat[0] < first_dog[0] {
                    // 猫最老
                    self.cat.pop_front().unwrap()
                } else {
                    // 狗最老
                    self.dog.pop_front().unwrap()
                }
            }
        }
    }
    
    fn dequeue_dog(&mut self) -> Vec<i32> {
        if let Some(front) = self.dog.pop_front() {
            return front;
        }
        vec![-1, -1]
    }
    
    fn dequeue_cat(&mut self) -> Vec<i32> {
        if let Some(front) = self.cat.pop_front() {
            return front;
        }
        vec![-1, -1]
    }
}