use ordmask::{OrdMask, ordmask};

#[test]
fn test_empty() {
    let mask: OrdMask<i32> = ordmask![];
    assert_eq!(mask.values().collect::<Vec<_>>(), vec![]);
    assert_eq!(mask.into_values().collect::<Vec<_>>(), vec![]);
}

#[test]
fn test_single_value() {
    // [1, 2) contains only value 1
    let mask = ordmask![<i32> 1, 2];
    assert_eq!(mask.values().collect::<Vec<_>>(), vec![1]);
}

#[test]
fn test_multiple_values() {
    // [1, 4) contains values 1, 2, 3
    let mask = ordmask![<i32> 1, 4];
    assert_eq!(mask.values().collect::<Vec<_>>(), vec![1, 2, 3]);
}

#[test]
fn test_multiple_spans() {
    // [1, 3) and [5, 7) contains values 1, 2, 5, 6
    let mask = ordmask![<i32> 1, 3, 5, 7];
    assert_eq!(mask.values().collect::<Vec<_>>(), vec![1, 2, 5, 6]);
}

#[test]
fn test_into_iter() {
    // [1, 4) contains values 1, 2, 3
    let mask = ordmask![<i32> 1, 4];
    assert_eq!(mask.into_values().collect::<Vec<_>>(), vec![1, 2, 3]);
}

#[test]
fn test_for_loop() {
    let mask = ordmask![<i32> 1, 4];
    let mut values = Vec::new();
    for v in mask.values() {
        values.push(v);
    }
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_for_loop_into_iter() {
    let mask = ordmask![<i32> 1, 4];
    let mut values = Vec::new();
    for v in mask.into_values() {
        values.push(v);
    }
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_into_iterator() {
    let mask = ordmask![<i32> 1, 4];
    // OrdMask implements IntoIterator
    let values: Vec<i32> = mask.into_values().collect();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_from_universal() {
    // [MIN, 2) for u8 contains 0, 1
    let mask = ordmask![<u8> .., 2];
    assert_eq!(mask.values().collect::<Vec<_>>(), vec![0, 1]);
}

#[test]
fn test_mixed_spans() {
    // [MIN, 2) and [4, MAX] for u8
    let mask = ordmask![<u8> .., 2, 4];
    let values: Vec<u8> = mask.values().collect();
    // [0, 1] and [4, 255]
    assert!(values.contains(&0));
    assert!(values.contains(&1));
    assert!(values.contains(&4));
    assert!(values.contains(&255));
    assert!(!values.contains(&2));
    assert!(!values.contains(&3));
}

#[test]
fn test_empty_span() {
    // Empty mask
    let mask = ordmask![<i32> 5, 5];
    assert!(mask.is_empty());
    assert_eq!(mask.values().collect::<Vec<_>>(), vec![]);
}

#[test]
fn test_u8_type() {
    let mask = ordmask![<u8> 1, 4];
    assert_eq!(mask.values().collect::<Vec<_>>(), vec![1u8, 2, 3]);
}

#[test]
fn test_u64_type() {
    let mask = ordmask![<u64> 10, 13];
    assert_eq!(mask.values().collect::<Vec<_>>(), vec![10u64, 11, 12]);
}
