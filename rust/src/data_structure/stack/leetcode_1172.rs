use std::cmp::Reverse;
use std::collections::{
    BinaryHeap,
    HashMap,
};

///
/// 0 ----------    not_empty
/// 1 ----------    not_empty
/// 2 -------       not_empty, not_full
/// 3 ----------    not_empty     
/// 4               not_full
/// 5 ----------    not_emtpy
/// 6 --            not_empty, not_full
/// 7               not_full
/// 
/// not_full（最小堆）  - 栈中元素数量 < capacity - 用于push
/// not_empty（最大堆） - 栈中元素数量 > 0 - 用于pop
/// 
struct DinnerPlates {
    // 用最小堆维护所有未满栈的索引 - 快速找到最左未满栈
    // 用最大堆维护所有非空栈的索引 - 快速找到最右非空栈
    // 用哈希表存储每个栈的盘子列表
    capacity: usize,    // 每个栈的最大容量
    stacks: HashMap<usize, Vec<i32>>,       // 哈希表，存储栈id到栈内容的映射
    not_full: BinaryHeap<Reverse<usize>>,   // 最小堆，存储未满栈的索引
    not_empty: BinaryHeap<usize>,           // 最大堆，存储非空栈的索引
    next_index: usize,  // 下一个新栈的索引
}

impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            stacks: HashMap::new(),
            not_full: BinaryHeap::new(),
            not_empty: BinaryHeap::new(),
            next_index: 0   // 首次push从0号栈开始
        }
    }

    // push操作可能：
    // 1. 栈从未满变成满的，此时要将栈索引从not_full中删除
    // 2. 栈从空变成非空的，此时要将栈索引加入到not_empty中
    fn push(&mut self, val: i32) {
        // 从最小堆中找到最左未满栈索引
        let i = match self.not_full.pop() {
            Some(Reverse(i)) => { i },
            None => {
                // 所有栈都是满的，创建新栈
                let i = self.next_index;
                self.next_index += 1;
                self.stacks.insert(i, Vec::new());
                i
            }
        };

        // 往最左未满栈中插入val
        let stack = self.stacks.get_mut(&i).unwrap();
        let was_empty = stack.is_empty();
        stack.push(val);

        // 两个判断：
        if stack.len() < self.capacity {
            self.not_full.push(Reverse(i));
        }
        if was_empty {
            self.not_empty.push(i);
        }
    }
    
    // pop操作可能：
    // 1. 栈从满变成未满，此时要将栈索引加入到not_full中
    // 2. 栈从非空变成空，此时要将栈索引从not_empty中删除
    fn pop(&mut self) -> i32 {
        loop {
            // 从最大堆中找到最右非空栈索引
            let i = match self.not_empty.pop() {
                Some(i) => { i },
                None => { return -1; },
            };

            // 由于栈可能早已通过pop_at_stack已经变成空了，所以这里
            // 需要判断一下当前的栈是否真的非空，如果已经成空则需要跳过
            // 接着处理not_empty的下一个堆首栈

            // 通过哈希表查找该栈是否真的非空
            let stack = match self.stacks.get_mut(&i) {
                Some(s) if !s.is_empty() => s,
                _ => continue,  // 过期条目，跳过
            };

            let was_full = stack.len() == self.capacity;
            let val = stack.pop().unwrap();

            // 两个判断：
            // 1. 栈在删除后是否仍然非空，若是则放回非空堆中
            // 2. 栈是否由满变成未满，若是则插入到未满堆中
            if !stack.is_empty() {
                self.not_empty.push(i);
            }
            if was_full {
                self.not_full.push(Reverse(i));
            }

            return val;
        }
    }
    
    // pop_at_stack可能：
    // 1. 栈从满变成未满，此时要将栈索引加入到not_full中
    // 2. 栈从非空变成空，此时要将栈索引从not_empty中删除，但是
    // 由于当前的栈不一定是not_empty最大堆的堆首元素，无法高效地
    // 将其移除，因此这里不将栈索引从not_empty中移除
    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let i = index as usize;
        let stack = match self.stacks.get_mut(&i) {
            Some(s) if !s.is_empty() => s,
            _ => return -1,
        };

        let was_full = stack.len() == self.capacity;
        let val = stack.pop().unwrap();

        if was_full {
            self.not_full.push(Reverse(i));
        }

        // 如果栈变空，not_empty中的旧索引会在 pop() 时被懒删除跳过
        val
    }
}
