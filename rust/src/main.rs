#![allow(dead_code)]

mod arrays;
mod data_structure;
mod graph;
mod hard;
mod hash_table;
mod lcp;
mod lcr;
mod lcs;
mod linked_list;
mod strings;
mod trees;
mod two_points;
mod x;


/// VecDeque<T>
/// 创建
///     VecDeque::new()
///     VecDeque::with_capacity(n)
///     VecDeque::from(vec) - 来自向量
///     
/// 探查
///     deque.len()
///     deque.is_empty()
/// 
/// 插入
///     deque.push_front(val) - 队首插入
///     deque.push_back(val) - 队尾插入
/// 
///     deque.insert(idx, val)
/// 
///     deque.extend(iterable)
/// 
/// 
/// 删除
///     deque.pop_front() - 队首弹出
///     deque.pop_back() - 队尾弹出
/// 
///     deque.remove(idx)
/// 
/// 访问
///     deque.front() - 返回队首元素的引用
///     deque.front_mut() - 返回队首元素的可变引用
///     deque.back() - 返回队尾元素的引用
///     deque.back_mut() - 返回队尾元素的可变引用
/// 
///     deque[idx] - 索引访问
/// 
/// 遍历
///     deque.into_iter() - 按值遍历
///     deque.iter() - 共享引用
///     deque.iter_mut() - 可变引用
/// 




/// BinaryHeap<T>
/// 创建
///     BinaryHeap::from(vec) - 从向量中创建
/// 
/// 探查
///     heap.len()
///     heap.is_empty()
///     heap.capacity()
/// 
/// 插入
///     heap.push(val) - 向堆中添加一个值
///     heap.append(&mut heap2)
/// 
/// 删除
///     heap.pop() - 从堆中移除并返回最大值
///     heap.clear()
/// 
/// 访问
///     heap.peek() - 返回对堆中最大值的引用
///     heap.peek_mut() - 窥视、可变版
/// 
/// 遍历
///     heap.iter() - 按任意顺序遍历
///     while - 按优先顺序消耗堆中的值



/// HashSet<T> / BTreeSet<t>
/// 创建
///     HashSet::new()
///     iter.collect() - 收集
///     HashSet::with_capacity(n) - 自带容量创建
/// 
/// 探查
///     set.len()
///     set.is_empty()
///     set.contans(&val) - 包含
/// 
/// 插入
///     set.insert(val)
/// 
/// 删除
///     set.remove(&val)
///     set.retain(test) - 移除所有未通过测试的元素
/// 
/// 访问
///     set.get(&val) - 返回等于val的set成员的共享引用
///     set.take(&val) - 拿出值
///     set.replace(val) - 替换值
/// 
/// 迭代
///     for v in set - 按值迭代
///     for v in &set - 迭代生成对Set成员的共享引用
///     set.iter() - 迭代器
/// 
/// 运算
///     set1.intersection(&set2) - 交集
///     set1.union(&set2) - 并集
///     set1.difference(&set2) - 差集
///     set1.symmetric_difference(&set2) - 对称差集，异或
/// 
///     set1.is_disjoint(set2) - 是否有交集
///     set1.is_subset(set2) - 是否为子集
///     set1.is_superset(set2) - 是否为超集




/// String
/// 创建
///     String::new() - 返回一个新的空字符串。这时还没有在堆上分配缓冲区，但将来会按需分配
///     String::with_capacity(n) - 返回一个新的空字符串，其中预先分配了一个足以容纳至少 n 字节的缓冲区
///     slice.to_string() - 分配一个新的 String，其内容是 str_slice 的副本
///     slice.to_owned() - 将 slice 的副本作为新分配的 String 返回 ???
///     iter.collect() - 通过串联迭代器的各个条目构造出字符串，迭代器的条目可以是 char 值、&str 值或 String 值
///     
/// 容量
///     capacity()
///     reserve()
///     shrink_to_fit()
/// 
/// 探查
///     // 由于String可以解引用成&str，因此在str上定义的
///     // 每个方法都可以在String上使用
///     slice.len() - 长度
///     slice.is_empty() - 如果slice.len() == 0，就返回true
///     slice[range] - 范围内切片
///     slice.split_at(i) - 返回从 slice 借来的两个共享切片的元组：一个是字节偏移量 i 之前的部分，另一个是字节偏移量 i 之后的部分
///     slice.is_char_boundary(i) - 如果字节偏移量 i 恰好落在字符边界之间并且适合作为slice 的偏移量，就返回 True
/// 
/// 插入
///     string.push(ch) - 将字符 ch 追加到 string 的末尾
///     string.push_str(slice) - 追加 slice 的全部内容。
///     string.extend(iter) - 将迭代器 iter 生成的条目追加到字符串中。迭代器可以生成 char 值、str 值或 String 值
///     string.insert(i, ch) - 在 string 内的字节偏移量 i 处插入单个字符 ch
///     string.insert_str(i, slice) - 这会在 string 内插入 slice
///     write! / writeln! - 这两个宏可以将格式化后的文本追加到String上
///     + / += - + 运算符会按值获取其左操作数，所以实际上它可以重用该 String 的缓冲区作为加法的结果
/// 
/// 删除
///     string.clear() - 清空
///     string.truncate(n) - 丢弃字节偏移量 n 之后的所有字符，留下长度最多为 n的 string
///     string.pop() - 从 string 中移除最后一个字符（如果有的话）​，并将其作为 Option<char> 返回
///     string.remove(i) - 从 string 中移除字节偏移量 i 处的字符并返回该字符，将后面的所有字符平移到前面
///     string.drain(range) - 返回给定字节索引范围内的迭代器，并在迭代器被丢弃后移除字符。范围之后的所有字符都会向前平移
///     string.replace_range(range, replacement) - 用给定的替代字符串切片替换 string 中的给定范围
/// 
/// 搜索
///     slice.contains(pattern) - 如果 slice 包含 pattern 的匹配项，就返回 true。
///     slice.starts_with(pattern) - 以pattern开头
///     slice.ends_with(pattern) - 以pattern结尾
/// 
///     slice.find(pattern) - 如果 slice 包含 pattern 的匹配项，就返回 Some(i)，其中的 i 是模式出现的字节偏移量
///     slice.rfind(pattern) - find 方法会返回第一个匹配项，rfind 方法则返回最后一个
///     
///     slice.replace(pattern, replacement) - 返回新的 String，它是通过用 replacement 急性替换pattern 的所有匹配项而形成的
///     slice.replace(pattern, replacement, n) - 最多替换前 n 个匹配项
/// 
/// 遍历
///     slice.bytes()
///         .chars()
///         .char_indices()
///         .split(' ')
///         .split_terminator(' ')
///         .rsplit_terminator(' ')
///         .split_whitespace()
///         .split_ascii_whitespace()
///         .splitn(3, ' ')
///         .rsplitn(3, ' ')
///         .matches("rr")
///         .split("rr")
///         .rsplit("rr")
/// 
///     slice.lines()
/// 
///     slice.matches(pattern)
///         .rmatches(pattern)
/// 
///     slice.match_indices(pattern)
///         .rmatch_indices(pattern)
/// 
/// Trim
///     slice.trim()
///     slice.trim_start()
///     slice.trim_end()
///     
///     slice.trim_matches(pattern)
///     slice.trim_start_matches(pattern)
///     slice.trim_end_matches(pattern)
/// 
///     slice.strip_prefix(pattern)
///     slice.strip_suffix(pattern)
/// 
/// 大小写转换
///     slice.to_uppercase()
///     slice.to_lowercase()
/// 
/// 字符串解析
///     usize::from_str()
///     f64::from_str()
///     bool::from_str()
///     char::from_str()
///     IpAddr::from_str()
/// 
///     slice.parse::<IpAddr>()?
/// 
/// 其他类型转字符串
///     format!("{}, wow", "doge") == "doge, wow"
///     to_string()
///     
/// 以UTF-8格式访问文本
/// 
/// 从UTF-8数据生成文本
/// 
/// 

/// Vec<T>
/// 创建
///      let mut num: Vec<i32> = vec![]; - 创建空向量
///      let mut wrd = vec!["step", "on"] - 使用给定内容创建向量
///      let mut buf = vec![0u8; 1024] - 1024个内容为0的字节
///      let my_vec = s.into_iter().collect::<Vec<String>>(); - 从迭代器创建向量
/// 
/// 访问
///      let line = &lines[0] - 获取某个元素的引用
///      let numb = numbers[4] - 获取某个元素的副本
///      let line = lines[1].clone() - 获取某个元素的副本
///      
///      let my_ref = &buffer[4..12] - 获取切片的引用
///      let my_cpy = buffer[4..12].to_vec() - 获取切片的副本
///
///      slice.first() - 获取第一个元素的引用
///      slice.first_mut() - 获取第一个元素的可变引用
///      slice.last() - 返回最后一个元素的引用
///      slice.last_mut() - 返回最后一个元素的可变引用
///      slice.get(idx) - 返回slice[idx]引用的Some值
///      slice.get_mut(idx) - 返回slice[ix]的可变引用
///
///      slice.to_vec() - 克隆整个切片，返回一个新向量
///
///  遍历
///      1. 遍历Vec<T>或者数组[T; N]
///      2. 遍历&Vec<T>、&[T; N]或者&[T]
///      3. 遍历&mut Vec<T>、&mut [T; N]或者&mut [T]
///      
///      slice.iter() - 迭代器
///      slice.iter_mut() - 可变迭代器
///
///      slice.split_at(4)
///      slice.split_first()
///      slice.split_last()
///      slice.split(|&x|x==0)
///      slice.splitn(2, |&x|x==0)
///      slice.rsplitn(2, |&x|x==0)
///      slice.chunks(2)
///      slice.windows(4)
///
///
///  容量
///      slice.len() - 长度
///      slice.is_empty() - 为空
///      Vec::with_capacity(n) - 创建一个容量为 n 的新的空向量
///      vec.capacity() - 返回 vec 的容量，类型为 usize
///      vec.reserve(n) - 预留容量
///      vec.reserve_exact(n) - 精确预留容量
///      vec.shrink_to_fit() - 缩小到刚好够
///  
///  插入
///      vec.push(val) - 将给定 value 添加到 vec 的末尾
///      vec.insert(idx, val) - 在 vec[index] 处插入给定的 value，将 vec[index..] 中的所有当前值向右平移一个位置以腾出空间
///      vec.extend(iter) - 扩展
///      vec.append(&mut vec2) - 将vec2中所有元素移动到vec尾部
///
///  删除
///      vec.pop() - 移除并返回最后一个元素
///      vec.remove(idx) - 移除并返回 vec[index]，将 vec[index+1..] 中的所有当前值向左平移一个位置以填补空白
///
///      vec.resize(new_len, val) - 将 vec 的长度设置为 new_len。如果该操作会增加 vec的长度，则以 value 的副本填补新空间  
///      vec.resize_with(new_len, closure) - 与resize类似，但调用闭包构造新元素  
///      vec.truncate(new_len) - 截断
///      vec.split_off(idx) - 与 vec.truncate(idx) 类似，但此方法会返回一个Vec<T>，其中包含从 vec 末尾移除的那些值
///      vec.drain(range) - 这将从 vec 中移除 range 范围内的切片 vec[range]
///      vec.retain(test) - 移除所有未通过给定测试的元素
///
///      vec.dedup() - 丢弃重复的元素
///      vec.dedup_by(same) - 根据same调用结果去重
///      vec.dedup_by_key(key) - 根据key属性去重
///
///      vec.clear() - 清空
///
///  联结
///      slice.concat() - 返回通过串联所有切片组装成的新向量
///      slice.join(&sep) - 与 concat 类似，只是在切片之间插入了值sep 的副本
///
///  交换
///      slice.swap(i, j) - 交换 slice[i] 和 slice[j] 这两个元素
///      slice1.swap_with_slice(slice2) - 互换内容，要求二者长度必须相同
///      slice.swap_remove(i) - 交换后移除
///
///  填充
///      slice.fill(val) - 用 value 的克隆体填充切片
///      slice.fill_with(function) - 以funtion回调填充
///
///  排序
///      slice.sort() - 升序排序
///      slice.sort_by() - 按cmp回调排序
///      slice.sort_by_key(key) - 按key回调排序
///      
///      slice.reverse() - 就地逆转切片
///
///  搜索
///      slice.contains(&val) - 是否包含
///
///      slice.binary_search(&val) - 二分搜索
///      slice.binary_search_by(&val, cmp) - 按cmp回调二分搜索
///      slice.binary_search_by_key(&val, key) - 按key闭包二分搜索
///
///  比较
///      slice.starts_with(other) - 以other开头
///      slice.ends_with(other) - 以other结尾
///
///  随机元素（rand crate）
///      slice.choose(&mut rng) - 随机选取
///      slice.shuffle(&mut rng) - 随机洗牌
///

fn main()
{
    println!("Hello world!");
}
