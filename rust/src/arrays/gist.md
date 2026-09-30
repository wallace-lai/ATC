
# 数组

## 遍历

```rust
    // 二维数组遍历（C++风格，不可变）
    pub fn traverse1(mat: Vec<Vec<i32>>) {
        let rows = mat.len();
        let cols = mat[0].len();

        for i in 0..rows {
            for j in 0..cols {
                println!("{}", mat[i][j]);
            }
        }

        println!("{:?}", mat);
    }
```

```rust
    // 二维数组遍历（C++风格，可变）
    pub fn traverse2(mut mat: Vec<Vec<i32>>) {
        let rows = mat.len();
        let cols = mat[0].len();

        for i in 0..rows {
            for j in 0..cols {
                println!("{}", mat[i][j]);
                mat[i][j] += 1;
            }
        }

        println!("{:?}", mat);
    }
```

```rust
    // 二维数组遍历（迭代器，不可变）
    pub fn traverse3(mat: Vec<Vec<i32>>) {
        for row in mat.iter() {
            for item in row.iter() {
                println!("{}", *item);
            }
        }

        println!("{:?}", mat);
    }
```

```rust
    // 二维数组遍历（迭代器，可变）
    pub fn traverse4(mut mat: Vec<Vec<i32>>) {
        for row in mat.iter_mut() {
            for item in row.iter_mut() {
                println!("{}", *item);
                *item += 1;
            }
        }

        println!("{:?}", mat);
    }
```

```rust
    // 二维数组遍历（带索引遍历，不可变）
    pub fn traverse5(mat: Vec<Vec<i32>>) {
        for (i, row) in mat.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                println!("v[{}][{}] = {}", i, j, *item);
            }
        }
    }
```

```rust
    // 二维数组遍历（扁平化，不可变）
    pub fn traverse6(mat: Vec<Vec<i32>>) {
        for item in mat.iter().flatten() {
            println!("{}", *item);
        }
    }
```

```rust
    // 二维数组转一维数组
    pub fn traverse7(mat: Vec<Vec<i32>>) -> Vec<i32> {
        // 消耗，无额外拷贝
        mat.into_iter()
            .flatten()
            .collect()
        
        // 不消耗，需拷贝
        // mat.iter()
        //     .flatten()
        //     .cloned()
        //     .collect()
    }
```

## 2. 构造

```rust
    // 二维数组构造
    pub fn from_vec(mut data: Vec<i32>, rows: usize, cols: usize) -> Vec<Vec<i32>> {
        assert_eq!(data.len(), rows * cols);

        data.chunks_exact(cols)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
```

## 3. 反转

### 行内逆序

```rust
    // 就地反转矩阵的每一行
    let mut m: Vec<Vec<i32>> = image;
    // 函数式风格
    m.iter_mut().for_each(|row| row.reverse());

    // for循环风格
    // for row in m.iter_mut() {
    //     row.reverse();
    // }
```

### 矩阵转置
```rust
    // 法一：基础实现
    pub fn tp1(mat: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();
        let mut ret = vec![vec![0; n]; m];

        for i in 0..n {
            for j in 0..m {
                // 读连续但写不连续，破坏缓存局部性
                ret[j][i] = mat[i][j];
            }
        }

        ret
    }

    // 法二：按列填充
    pub fn tp2(mat: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();
        let mut ret = vec![vec![0; n]; m];

        for j in 0..m {
            let mut res_row = &mut ret[j];
            for i in 0..n {
                // 读不连续但写连续
                res_row[i] = mat[i][j];
            }
        }

        ret
    }

    // 法三：分块转置
    pub fn tp3(mat: &Vec<Vec<i32>>, block_size: usize) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();
        let mut ret = vec![vec![0; n]; m];

        for i_block in (0..n).step_by(block_size) {
            for j_block in (0..m).step_by(block_size) {
                let i_end = (i_block + block_size).min(n);
                let j_end = (j_block + block_size).min(m);
                // 处理块内元素
                for i in i_block..i_end {
                    for j in j_block..j_end {
                        ret[j][i] = mat[i][j]
                    }
                }
            }
        }

        ret
    }
```