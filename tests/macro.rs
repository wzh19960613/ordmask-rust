use ordmask::{ordmask, OrdMask};

#[test]
fn test_basic_ordmask() {
    let mask = ordmask!(1, 2, 3);
    assert_eq!(mask, OrdMask::from(vec![1, 2, 3]));

    let mask2 = ordmask!(0, 2, 4);
    assert_eq!(mask2, OrdMask::from(vec![0, 2, 4]));
}

#[test]
fn test_empty_ordmask() {
    let mask: OrdMask<u64> = ordmask!();
    assert_eq!(mask, OrdMask::empty());
}

#[test]
fn test_reversed_ordmask() {
    let mask = ordmask!(_, 1, 2, 3);
    assert_eq!(mask, OrdMask::from_complement(vec![1, 2, 3]));
}

#[test]
#[should_panic]
fn test_ordmask_should_panic() {
    let _ = ordmask![1, 0];
}
