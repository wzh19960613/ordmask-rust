use ordmask::{OrdMask, ordmask};

#[test]
fn test_basic_ordmask() {
    let mask = ordmask![1, 2, 3];
    assert_eq!(mask, OrdMask::from(vec![1, 2, 3]));

    let mask = ordmask![0, 2, 4];
    assert_eq!(mask, OrdMask::from(vec![0, 2, 4]));
}

#[test]
fn test_empty_ordmask() {
    let mask: OrdMask<u64> = ordmask![];
    assert_eq!(mask, OrdMask::empty());
    let mask = ordmask![<i32>];
    assert_eq!(mask, OrdMask::empty());
}

#[test]
fn test_reversed_ordmask() {
    let mask = ordmask![.., 1, 2, 3];
    assert_eq!(mask, OrdMask::from_complement(vec![1, 2, 3]));
}

#[test]
fn test_universal_ordmask() {
    let mask: OrdMask<i32> = ordmask![..];
    assert_eq!(mask, OrdMask::universal());

    let mask = ordmask![<i32> ..];
    assert_eq!(mask, OrdMask::universal());
}

#[test]
fn test_ordmask_with_type() {
    let mask = ordmask![<u32>];
    assert_eq!(mask, OrdMask::empty());
    let mask = ordmask![<u32> ..];
    assert_eq!(mask, OrdMask::universal());
    let mask = ordmask![<u32> 1];
    assert_eq!(mask, OrdMask::from(vec![1]));
    let mask = ordmask![<u32> 1, 2];
    assert_eq!(mask, OrdMask::from(vec![1, 2]));
    let mask = ordmask![<u32> .., 1];
    assert_eq!(mask, OrdMask::from_complement(vec![1]));
    let mask = ordmask![<u32> .., 1, 2];
    assert_eq!(mask, OrdMask::from_complement(vec![1, 2]));
}

#[test]
#[should_panic]
fn test_ordmask_should_panic() {
    let _ = ordmask![1, 0];
}
