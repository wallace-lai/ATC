## 二分搜索

```rust
fn binary_search(nums: &Vec<i32>, t: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid] == t {
            return mid;
        } else if nums[mid] < t {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    -1
}
```

## 二分边界

对于左闭右开区间`[left, right)`：

### 找到第一个满足条件的位置

```rust
// 左闭右开区间
let mut left = 0;
let mut right = nums.len();

while left < right {
    let mid = left + (right - left) / 2;
    if condition() {
        right = mid;
    } else {
        left = mid + 1;
    }
}

return left;
```

### 找到第一个`>= t`的位置 - `lower_bound()`

```rust
fn lower_bound(nums: &Vec<i32>, t: i32) -> i32 {
    // 左闭右开区间
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] >= t {
            right = mid;        // mid可能是答案，但还需往前找
        } else {
            left = mid + 1;     // mid和左边都不满足 >= t
        }
    }

    left    // left == nums.len()时，表示不存在
}
```

### 找到第一个`> t`的位置 - `upper_bound()`

```rust
fn lower_bound(nums: &Vec<i32>, t: i32) -> i32 {
    // 左开右闭区间
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] > t {
            right = mid;    // mid可能是答案
        } else {
            left = mid + 1; // mid和左边均 <= t，不满足 > t
        }
    }

    left
}
```


|想要找的位置|实现方式|
|---|---|
|第一个 `>= target`|	`lower_bound(nums, target)`|
|第一个 `> target`|	`upper_bound(nums, target)`|
|第一个 `== target`|	`pos = lower_bound(nums, target)`，再检查 `nums[pos] == target`|
|最后一个 `== target`|	`pos = upper_bound(nums, target) - 1`，再检查 `nums[pos] == target`|
|最后一个 `<= target`|	`upper_bound(nums, target) - 1`|
|最后一个 `< target`|	`lower_bound(nums, target) - 1`|

注意：得到的位置可能越界，比如 `-1` 或 `len(nums)`，使用前要判断。
