# OrdMask

[> English Version](README.md)

`ordmask` 是一个用于高效基于范围的集合运算和成员检查的库。
它将值的集合表示为区间的集合，并支持各种集合运算。

## 功能特性

- 高效的区间成员检查
- 支持 `union`（并集）、`intersection`（交集）、`minus`（差集）、`complement`（补集）和 `symmetric_difference`（对称差集）运算
- 适用于任何实现了 `Ord`、`Clone` 和 `WithMin` trait 的类型
- 尽可能实现零分配操作
- 可选的 feature `serde` 支持，用于序列化/反序列化

## 构造

```rust
use ordmask::{OrdMask, ordmask};

// [0, 10), [20, 30) 和 [40, MAX]
let mask = ordmask![0, 10, 20, 30, 40];
assert!(mask.included(&5));
assert!(mask.excluded(&10));
assert!(mask.included(&50));

// 从 `Vec<T>` 创建
assert_eq!(mask, OrdMask::from(vec![0, 10, 20, 30, 40]));

// 从可疑点和谓词创建
use std::collections::BTreeSet;
assert_eq!(mask, OrdMask::from_suspicious_points_set(
    BTreeSet::from([0, 10, 20, 30, 40]),
    |x| matches!(x, 0..10 | 20..30 | 40..),
    false
));

// 从可疑点映射创建
use std::collections::BTreeMap;
let map = BTreeMap::from([(0, true), (10, false), (20, true), (30, false), (40, true)]);
assert_eq!(mask, OrdMask::from_suspicious_points_map(map, false));

// [MIN, 10)
let mask = ordmask![.., 10];
assert_eq!(mask, OrdMask::less_than(10));
assert!(mask.included(&9));
assert!(mask.excluded(&10));

// [10, MAX]
let mask = ordmask![10];
assert_eq!(mask, OrdMask::not_less_than(10));
assert!(mask.excluded(&9));
assert!(mask.included(&10));

// [10, 20)
let mask = ordmask![10, 20];
assert_eq!(mask, OrdMask::in_range(10, 20));
assert!(mask.included(&10));
assert!(mask.included(&15));
assert!(mask.excluded(&20));
assert!(mask.excluded(&25));

// 全集
let mask = ordmask![..];
assert_eq!(mask, OrdMask::universal());
assert!(mask.is_universal());
assert!(mask.included(&0));

// 空集
let mask = ordmask![];
assert_eq!(mask, OrdMask::empty());
assert!(mask.is_empty());
assert!(mask.excluded(&0));
```

### 类型标注

可以使用 `<T>` 语法在宏中显式指定类型：

```rust
use ordmask::{OrdMask, ordmask};

// 使用 <T> 显式指定类型
let mask = ordmask![<i64>];        // 空集
let mask = ordmask![<u8>..];       // 全集
let mask = ordmask![<u64> 10];     // [10, MAX]
let mask = ordmask![<i32> 10, 20]; // [10, 20)
let mask = ordmask![<u32> .., 10]; // [MIN, 10)
```

## 并集 (Union)

```rust
use ordmask::{OrdMask, ordmask};

let a = ordmask![0, 15];
let b = ordmask![5, 20];
let c = ordmask![10, 30];
// &a | &b | &c：引用运算符不会 move（消耗）值
assert_eq!(&a | &b | &c, OrdMask::union(&[&a, &b, &c]));
// a | b | c：非引用运算符会 move（消耗）值
assert_eq!(a | b | c, ordmask![0, 30]);
```

## 交集 (Intersection)

```rust
use ordmask::{OrdMask, ordmask};

let a = ordmask![0, 15];
let b = ordmask![5, 20];
let c = ordmask![10, 30];
// &a & &b & &c：引用运算符不会 move（消耗）值
assert_eq!(&a & &b & &c, OrdMask::intersection(&[&a, &b, &c]));
// a & b & c：非引用运算符会 move（消耗）值
assert_eq!(a & b & c, ordmask![10, 15]);
```

## 差集与补集 (Minus and Complement)

```rust
use ordmask::{OrdMask, ordmask};

let a = ordmask![0, 15];
let b = ordmask![5, 8];
let c = ordmask![10, 20];
// &a - &b - &c：引用运算符不会 move（消耗）值
assert_eq!(&a - &b - &c, OrdMask::minus(&a, &[&b, &c]));
// a - b - c：非引用运算符会 move（消耗）值
assert_eq!(a - b - c, ordmask![0, 5, 8, 10]);

let a = ordmask![0, 15];
// !&a：引用运算符和 `a.complement()` 不会 move（消耗）值
assert_eq!(!&a, a.complement());
// !a：非引用运算符和 `a.to_complement()` 会 move（消耗）值
assert_eq!(!a, ordmask![.., 0, 15]);
```

## 对称差集 (Symmetric Difference)

```rust
use ordmask::{OrdMask, ordmask};

let a = ordmask![0, 15];
let b = ordmask![5, 20];
// &a ^ &b：引用运算符不会 move（消耗）值
assert_eq!(&a ^ &b, OrdMask::symmetric_difference(&a, &b));
// a ^ b：非引用运算符会 move（消耗）值
assert_eq!(a ^ b, ordmask![0, 5, 15, 20]);
```

## 区间 (Spans)

`OrdMask` 提供了迭代包含区间的方法。每个区间以元组 `(start, end)` 的形式返回，表示一个左闭右开区间 `[start, end)`。

> **注意**：
> - 使用 spans 功能需要类型 `T` 实现 `WithMax` trait（库已为所有标准整数类型提供实现）。
> - 由于区间是左闭右开的 `[start, end)`，`MAX` 是否被包含可能会产生疑惑。可以使用 `.is_max_value_included()` 来检查最大值是否在 mask 中。

### 基本迭代

使用 `.spans()` 迭代所有区间：

```rust
use ordmask::ordmask;

// 空集没有区间
assert_eq!(ordmask![<i32>].spans().collect::<Vec<_>>(), vec![]);

// 全集有一个区间 [MIN, MAX]
assert_eq!(
    ordmask![..].spans().collect::<Vec<_>>(),
    vec![(i32::MIN, i32::MAX)]
);

// 单个区间 [1, 2)
assert_eq!(ordmask![1, 2].spans().collect::<Vec<_>>(), vec![(1, 2)]);

// 多个区间：[MIN, 1) 和 [2, MAX]
assert_eq!(
    ordmask![.., 1, 2].spans().collect::<Vec<_>>(),
    vec![(i32::MIN, 1), (2, i32::MAX)]
);
```

### 消费型迭代

使用 `.into_spans()` 会消费 mask 并返回一个拥有所有权的迭代器：

```rust
use ordmask::ordmask;

assert_eq!(
    ordmask![.., 1, 2_i32].into_spans().collect::<Vec<_>>(),
    vec![(i32::MIN, 1), (2, i32::MAX)]
);
```

### 区间数量和值计数

使用 `.spans_count()` 可在 **O(1)** 时间内获取区间数量，且不用消耗迭代器。
其结果等同于 `.spans().count()`，但更加高效。

使用 `.values_count()` 获取包含值的总数。
它支持惰性比较，无需计算完整计数。

```rust
use ordmask::ordmask;

// 区间数量
assert_eq!(ordmask![.., 10].spans_count(), 1);      // [MIN, 10)
assert_eq!(ordmask![.., 10, 20].spans_count(), 2);  // [MIN, 10), [20, MAX]
assert_eq!(ordmask![<u32>].spans_count(), 0);
assert_eq!(ordmask![<u32>..].spans_count(), 1);

// 值计数（惰性比较）
// [0, 10)
assert!(ordmask![<u32> .., 10].values_count() == 10);
// [0, 10), [20, MAX]
assert!(ordmask![<u32> .., 10, 20].values_count() == u32::MAX - 10 + 1);
// 空集计数为 0
assert!(ordmask![<u32>].values_count() == 0);
```

> **警告**：对全集调用 `.values_count().get()` 可能会因溢出而 panic（因为 `MAX - MIN + 1` 会溢出）。建议在调用 `.get()` 前使用 `.is_universal()` 进行检查。不过，惰性比较（如 `values_count() < value`）是安全的，因为它们可以提前终止而无需计算完整计数。

# 值迭代 (Value Iteration)

使用 `.values()` 和 `.into_values()` 迭代单个包含的值（而非区间）。

> **注意**：值迭代需要类型 `T` 实现 `std::ops::Add<Output = T>` 和 `WithOne`（除了 `WithMin` 和 `WithMax`）。库已为所有标准整数类型提供实现。

```rust
use ordmask::ordmask;

// [1, 4) 包含值 1, 2, 3
assert_eq!(ordmask![1, 4].values().collect::<Vec<_>>(), vec![1, 2, 3]);

// 多个区间：[1, 3) 和 [5, 7)
assert_eq!(
    ordmask![1, 3, 5, 7].values().collect::<Vec<_>>(),
    vec![1, 2, 5, 6]
);

// 使用 into_values() 会消费 mask
assert_eq!(ordmask![1, 4].into_values().collect::<Vec<_>>(), vec![1, 2, 3]);

// 支持 for 循环
let mask = ordmask![1, 4];
let mut sum = 0;
for v in mask.values() {
    sum += v;
}
assert_eq!(sum, 6);
```

> **警告**：对于大型 mask（如全集），值迭代可能会产生大量值。请谨慎使用。

## 类型要求

`OrdMask<T>` 要求 `T` 实现 `WithMin` trait，这是一个为具有最小值的类型定义的 trait。库为所有标准整数类型提供了实现：

```rust
use ordmask::WithMin;

// 内置实现支持：
// u8, u16, u32, u64, u128, usize
// i8, i16, i32, i64, i128, isize

assert_eq!(i32::MIN, <i32 as WithMin>::MIN);
assert_eq!(u64::MIN, <u64 as WithMin>::MIN);
```

如需使用自定义类型，至少实现 `WithMin` 以使用 `OrdMask`。然后：
- `.spans_count()` 已实现
- 如需使用 `.spans()` 或 `.into_spans()`，还需实现 `WithMax`
- 如需使用 `.values()` 或 `.into_values()`，还需实现：
    - `WithMax`
    - `WithOne`
    - `std::ops::Add`
- 如需使用 `.values_count()`，还需实现：
    - `WithMax`
    - `OrderedSub<Target = COUNT>`，且 `COUNT` 应实现：
        - `WithZero`
        - `WithOne`
        - `std::ops::Add`
        - \[可选\] `PartialOrd` 以使用 `COUNT` 的比较运算符

```rust
use ordmask::prelude::*;

#[derive(Clone, Ord, Debug, PartialOrd, Eq, PartialEq)]
struct MyType(i32);

// 必要实现。启用 `.spans_count()`。
impl WithMin for MyType {
    const MIN: Self = MyType(i32::MIN);
}

assert!(ordmask![..].included(&MyType(1)));

// 启用 `.spans()` 和 `.into_spans()`。
impl WithMax for MyType {
    const MAX: Self = MyType(i32::MAX);
}

assert_eq!(
    ordmask![MyType(0), MyType(10)].spans().collect::<Vec<_>>(),
    vec![(MyType(0), MyType(10))]
);

// 启用 `.values_count()` 和 `COUNT` 的比较运算符。
impl OrderedSub for MyType {
    type Target = u32; // WithZero, WithOne, std::ops::Add, PartialOrd

    fn ordered_sub(&self, other: &Self) -> Self::Target {
        self.0.ordered_sub(&other.0) // 与库对 i32 的实现相同
    }
}

assert!(ordmask![MyType(0), MyType(10)].values_count() == 10);

// 启用 `.values()` 和 `.into_values()`。
impl WithOne for MyType {
    const ONE: Self = MyType(1);
}

impl std::ops::Add for MyType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

assert_eq!(
    ordmask![MyType(0), MyType(3)].values().collect::<Vec<_>>(),
    vec![MyType(0), MyType(1), MyType(2)]
);
```
