# OrdMask

[> English Version](README.md)

`OrdMask` 是一个用于高效基于范围的集合运算和成员检查的库。
它将值的集合表示为区间的集合，并支持各种集合运算。

## 功能特性

- 高效的区间成员检查
- 支持 `union`（并集）、`intersection`（交集）、`minus`（差集）、`complement`（补集）和 `symmetric_difference`（对称差集）运算
- 适用于任何实现了 `Ord`、`Clone` 和 `WithMin` trait 的类型
- 尽可能实现零分配操作
- 可选的 feature `serde` 支持，用于序列化/反序列化

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

如需使用自定义类型，请手动实现 `WithMin`：

```rust
use ordmask::{WithMin, ordmask};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
struct MyType(i32);

impl WithMin for MyType {
    const MIN: Self = MyType(i32::MIN);
}

assert!(ordmask![..].included(&MyType(1)));
```

## 构造

```rust
use ordmask::{OrdMask, ordmask};

// [0, 10) 和 [20, MAX)
let mask = ordmask![0, 10, 20];
assert!(mask.included(&0));
assert!(mask.included(&2));
assert!(mask.excluded(&10));
assert!(mask.excluded(&15));
assert!(mask.included(&20));
assert!(mask.included(&30));

// 从 `Vec<T>` 创建
assert_eq!(mask, OrdMask::from(vec![0, 10, 20]));

// 从可疑点和谓词创建
assert_eq!(mask, OrdMask::from_suspicious_points_set(
    std::collections::BTreeSet::from([0, 10, 20]),
    |x| match x {
        0..10 => true,
        20.. => true,
        _ => false,
    },
    false
));

// 从可疑点映射创建
let map = std::collections::BTreeMap::from([(0, true), (10, false), (20, true)]);
assert_eq!(mask, OrdMask::from_suspicious_points_map(map, false));

// (MIN, 10)
let mask = ordmask![.., 10];
assert_eq!(mask, OrdMask::less_than(10));
assert!(mask.included(&9));
assert!(mask.excluded(&10));

// [10, MAX)
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
let mask = ordmask![<u64> 10];     // [10, MAX)
let mask = ordmask![<i32> 10, 20]; // [10, 20)
let mask = ordmask![<u32> .., 10]; // (MIN, 10)
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
