# OrdMask

`OrdMask` is a Rust library for checking if values are included within specific ranges.

- Efficient range checking
- Support for `union`, `intersection`, `minus`, `complement`, and `symmetric_difference` operations
- Works with any type that implements the `Ord` and `Clone` traits

## Construct

```rust
use ordmask::{OrdMask, ordmask};

// [0, 10) and [20, \infty)
let mask = ordmask![0, 10, 20];
assert!(mask.included(&0));
assert!(mask.included(&2));
assert!(mask.excluded(&10));
assert!(mask.excluded(&15));
assert!(mask.included(&20));
assert!(mask.included(&30));

// Create from `Vec<T>`
assert_eq!(mask, OrdMask::from(vec![0, 10, 20]));

// Create from key_points and a predicate
assert_eq!(mask, OrdMask::from_key_points_set(
    std::collections::BTreeSet::from([0, 10, 20]),
    |x| match x {
        0..10 => true,
        20.. => true,
        _ => false,
    },
    false
));

// Create from key_points_map
assert_eq!(mask, OrdMask::from_key_points_map(
    std::collections::BTreeMap::from([(0, true), (10, false), (20, true)]),
    false,
));

// (-\infty, 10)
let mask = ordmask![_, 10];
assert_eq!(mask, OrdMask::less_than(10));
assert!(mask.included(&9));
assert!(mask.excluded(&10));

// [10, \infty)
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

// Universal
let mask = ordmask![_];
assert_eq!(mask, OrdMask::universal());
assert!(mask.is_universal());
assert!(mask.included(&0));

// Empty
let mask = ordmask![];
assert_eq!(mask, OrdMask::empty());
assert!(mask.is_empty());
assert!(mask.excluded(&0));
``` 

## Union

```rust
use ordmask::{OrdMask, ordmask};

let a = ordmask![0, 15];
let b = ordmask![5, 20];
let c = ordmask![10, 30];
assert_eq!(&a | &b | &c, OrdMask::union(&[&a, &b, &c]));
assert_eq!(a | b | c, ordmask![0, 30]);
```

## Intersection

```rust
use ordmask::{OrdMask, ordmask};

let a = ordmask![0, 15];
let b = ordmask![5, 20];
let c = ordmask![10, 30];
assert_eq!(&a & &b & &c, OrdMask::intersection(&[&a, &b, &c]));
assert_eq!(a & b & c, ordmask![10, 15]);
```

## Minus and Complement

```rust
use ordmask::{OrdMask, ordmask};

let a = ordmask![0, 15];
let b = ordmask![5, 8];
let c = ordmask![10, 20];
assert_eq!(&a - &b - &c, OrdMask::minus(&a, &[&b, &c]));
assert_eq!(a - b - c, ordmask![0, 5, 8, 10]);

let a = ordmask![0, 15];
assert_eq!(!&a, a.new_complement());
assert_eq!(a.complement(), ordmask![_, 0, 15]);
```

## Symmetric Difference

```rust
use ordmask::{OrdMask, ordmask};

let a = ordmask![0, 15];
let b = ordmask![5, 20];
assert_eq!(&a ^ &b, OrdMask::symmetric_difference(&a, &b));
assert_eq!(a ^ b, ordmask![0, 5, 15, 20]);
``` 
