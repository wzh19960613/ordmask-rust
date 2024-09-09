# OrdMask

`OrdMask` is a Rust library for checking if values are included within specific ranges.

- Efficient range checking
- Support for `union`, `intersection`, `difference`, `symmetric_difference`, and `complement` operations
- Works with any type that implements the `Ord` and `Clone` traits

## Construct

```rust
use ordmask::OrdMask;

// [0, 10) and [20, \infty)
let mask = OrdMask::from(vec![0, 10, 20]);
assert!(mask.included(&0));
assert!(mask.included(&2));
assert!(mask.excluded(&10));
assert!(mask.excluded(&15));
assert!(mask.included(&20));
assert!(mask.included(&30));
// Create from key_points and a predicate
assert_eq!(mask, OrdMask::new(
    std::collections::BTreeSet::from([0, 10, 20]),
    |x| match x {
        0..10 => true,
        20.. => true,
        _ => false,
    }
));
// Create from key_points_map
assert_eq!(mask, OrdMask::from_key_points_map(std::collections::BTreeMap::from (
    [(0, true), (10, false), (20, true)]
)));

// (-\infty, 10)
let mask = OrdMask::less_than(10);
assert!(mask.included(&9));
assert!(mask.excluded(&10));

// [10, \infty)
let mask = OrdMask::not_less_than(10);
assert!(mask.excluded(&9));
assert!(mask.included(&10));

// [10, 20)
let mask = OrdMask::in_range(10, 20);
assert!(mask.included(&10));
assert!(mask.included(&15));
assert!(mask.excluded(&20));
assert!(mask.excluded(&25));

// Universal
let mask = OrdMask::universal();
assert!(mask.is_universal());
assert!(mask.included(&0));

// Empty
let mask = OrdMask::empty();
assert!(mask.is_empty());
assert!(mask.excluded(&0));
``` 

## Union

```rust
use ordmask::OrdMask;

let a = OrdMask::from(vec![0, 15]);
let b = OrdMask::from(vec![5, 20]);
let c = OrdMask::from(vec![10, 30]);
assert_eq!(&a | &b | &c, OrdMask::union(&[&a, &b, &c]));
assert_eq!(a | b | c, OrdMask::from(vec![0, 30]));
```

## Intersection

```rust
use ordmask::OrdMask;

let a = OrdMask::from(vec![0, 15]);
let b = OrdMask::from(vec![5, 20]);
let c = OrdMask::from(vec![10, 30]);
assert_eq!(&a & &b & &c, OrdMask::intersection(&[&a, &b, &c]));
assert_eq!(a & b & c, OrdMask::from(vec![10, 15]));
```

## Difference

```rust
use ordmask::OrdMask;

let a = OrdMask::from(vec![0, 15]);
let b = OrdMask::from(vec![5, 8]);
let c = OrdMask::from(vec![10, 20]);
assert_eq!(&a - &b - &c, OrdMask::difference(&a, &[&b, &c]));
assert_eq!(a - b - c, OrdMask::from(vec![0, 5, 8, 10]));
```

## Symmetric Difference

```rust
use ordmask::OrdMask;

let a = OrdMask::from(vec![0, 15]);
let b = OrdMask::from(vec![5, 20]);
assert_eq!(&a ^ &b, OrdMask::symmetric_difference(&a, &b));
assert_eq!(a ^ b, OrdMask::from(vec![0, 5, 15, 20]));
``` 

## Complement

```rust
use ordmask::OrdMask;

let a = OrdMask::from(vec![0, 15]);
assert_eq!(!&a, OrdMask::complement(&a));
assert_eq!(a.complement(), OrdMask::from(vec![i32::MIN, 0, 15]));
``` 
