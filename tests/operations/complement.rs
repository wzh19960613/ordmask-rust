use ordmask::{OrdMask, ordmask};

// ============================================================================
// Helper macro for testing with descriptive error messages
// ============================================================================

macro_rules! assert_mask_eq {
    ($left:expr, $right:expr, $msg:expr) => {
        assert_eq!(
            $left, $right,
            "{}: left={:?}, right={:?}",
            $msg, $left, $right
        );
    };
}

// ============================================================================
// Complement Tests
// ============================================================================

#[test]
fn test_complement_basic() {
    let test_cases: Vec<(Vec<i32>, bool, Vec<i32>, bool)> = vec![
        // (input_kp, input_rev, expected_kp, expected_rev)
        // Normal ranges
        (vec![0, 10], false, vec![0, 10], true),
        (vec![5, 15, 20, 30], false, vec![5, 15, 20, 30], true),
        // Reversed ranges
        (vec![0, 10], true, vec![0, 10], false),
        (vec![5, 15, 20, 30], true, vec![5, 15, 20, 30], false),
        // Empty
        (vec![], false, vec![], true),
        (vec![], true, vec![], false),
    ];

    for (input_kp, input_rev, expected_kp, expected_rev) in test_cases {
        let mask: OrdMask<i32> = unsafe { OrdMask::with_checked(input_kp, input_rev) };
        let expected: OrdMask<i32> = unsafe { OrdMask::with_checked(expected_kp, expected_rev) };
        let result = mask.complement();
        assert_mask_eq!(result, expected, "complement basic test failed");
    }
}

#[test]
fn test_complement_empty() {
    // Complement of empty set is universal set
    let empty: OrdMask<i32> = ordmask![];

    assert_eq!(empty.complement(), ordmask![..]);

    // Double complement returns to original
    assert_eq!(empty.complement().complement(), empty);
}

#[test]
fn test_complement_universal() {
    // Complement of universal set is empty set
    let universal: OrdMask<i32> = ordmask![..];

    assert_eq!(universal.complement(), ordmask![]);

    // Double complement returns to original
    assert_eq!(universal.complement().complement(), universal);
}

#[test]
fn test_complement_with_reversed() {
    // Test complement of reversed (complement) sets
    let reversed1 = ordmask![.., 0, 10]; // (-∞, 0) ∪ [10, ∞)

    // Complement of reversed is normal
    assert_eq!(reversed1.complement(), ordmask![0, 10]);

    // Double complement
    assert_eq!(reversed1.complement().complement(), reversed1);
}

#[test]
fn test_complement_with_infinite() {
    // Infinite sets (unbounded on one side)
    let inf_right = ordmask![0]; // [0, ∞)
    let inf_left = ordmask![.., 10]; // (-∞, 10)

    // Complement of [0, ∞) is (-∞, 0)
    assert_eq!(inf_right.complement(), ordmask![.., 0]);

    // Complement of (-∞, 10) is [10, ∞)
    assert_eq!(inf_left.complement(), ordmask![10]);
}

#[test]
fn test_complement_at_edge() {
    // Edge cases with u32
    let a: OrdMask<u32> = ordmask![.., 0]; // empty
    let b = ordmask![u32::MAX]; // max only
    let c = ordmask![0, 10]; // from min
    let d = ordmask![100, u32::MAX]; // to max - 1

    // Complement of a (empty) is universal
    assert_eq!(a.complement(), ordmask![..]);

    // Complement of b (max only) = [0, u32::MAX)
    assert_eq!(b.complement(), ordmask![.., u32::MAX]);

    // Complement of c [0,10) = (-∞, 0) ∪ [10, ∞) represented as reversed
    assert_eq!(c.complement(), ordmask![.., 0, 10]);

    // Complement of d [100, u32::MAX) = [0, 100) ∪ [u32::MAX, ∞) represented as reversed
    assert_eq!(d.complement(), ordmask![.., 100, u32::MAX]);
}

#[test]
fn test_complement_operator() {
    let mask = ordmask![0, 10];
    let expected = ordmask![.., 0, 10];

    // Test operator
    assert_eq!(!&mask, expected);
    assert_eq!(!mask.clone(), expected);

    // Double negation
    assert_eq!(!(!&mask), mask);
    assert_eq!(!(!mask.clone()), mask);
}

#[test]
fn test_to_complement() {
    // Test consuming complement
    let mask = ordmask![0, 10];
    let expected = ordmask![.., 0, 10];

    let complement = mask.to_complement();
    assert_eq!(complement, expected);
}

#[test]
fn test_reverse() {
    // Test in-place reverse
    let mut mask = ordmask![0, 10];
    let expected = ordmask![.., 0, 10];

    mask.reverse();
    assert_eq!(mask, expected);

    // Double reverse returns to original
    mask.reverse();
    assert_eq!(mask, ordmask![0, 10]);
}

#[test]
fn test_complement_is_involution() {
    // Complement of complement should be the original (involution property)
    let test_masks: Vec<OrdMask<i32>> = vec![
        ordmask![],
        ordmask![..],
        ordmask![0, 10],
        ordmask![5, 15, 20, 30],
        ordmask![.., 0, 10],
        ordmask![.., 5, 15, 20, 30],
        ordmask![0],
        ordmask![.., 10],
    ];

    for mask in test_masks {
        assert_eq!(
            mask.complement().complement(),
            mask,
            "Double complement failed for {:?}",
            mask
        );
    }
}

#[test]
fn test_complement_with_union() {
    // De Morgan's law: !(A ∪ B) = !A ∩ !B
    let a = ordmask![0, 10];
    let b = ordmask![5, 15];

    let left = OrdMask::union(&[&a, &b]).complement();
    let right = OrdMask::intersection(&[&a.complement(), &b.complement()]);

    assert_eq!(left, right);
}

#[test]
fn test_complement_with_intersection() {
    // De Morgan's law: !(A ∩ B) = !A ∪ !B
    let a = ordmask![0, 10];
    let b = ordmask![5, 15];

    let left = OrdMask::intersection(&[&a, &b]).complement();
    let right = OrdMask::union(&[&a.complement(), &b.complement()]);

    assert_eq!(left, right);
}

#[test]
fn test_complement_complex_ranges() {
    // Multiple ranges
    let mask = ordmask![0, 5, 10, 15, 20, 25];
    let complement = mask.complement();

    // Verify complement includes points not in original
    assert!(!mask.included(&-1));
    assert!(complement.included(&-1));

    assert!(mask.included(&2));
    assert!(!complement.included(&2));

    assert!(!mask.included(&7));
    assert!(complement.included(&7));

    assert!(mask.included(&12));
    assert!(!complement.included(&12));

    assert!(!mask.included(&30));
    assert!(complement.included(&30));

    // Double complement
    assert_eq!(complement.complement(), mask);
}
