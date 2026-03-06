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
// Symmetric Difference Tests
// ============================================================================

#[test]
fn test_symmetric_difference_basic() {
    let test_cases: Vec<((Vec<i32>, bool), (Vec<i32>, bool), (Vec<i32>, bool))> = vec![
        // (mask1, mask2, expected)
        // Partially overlapping
        (
            (vec![0, 10], false),
            (vec![5, 15], false),
            (vec![0, 5, 10, 15], false),
        ),
        // Non-overlapping
        (
            (vec![0, 5], false),
            (vec![10, 15], false),
            (vec![0, 5, 10, 15], false),
        ),
        // mask2 contained in mask1
        (
            (vec![0, 20], false),
            (vec![5, 10], false),
            (vec![0, 5, 10, 20], false),
        ),
        // Same range
        (
            (vec![0, 10], false),
            (vec![0, 10], false),
            (vec![], false),
        ),
        // mask1 contained in mask2
        (
            (vec![5, 10], false),
            (vec![0, 20], false),
            (vec![0, 5, 10, 20], false),
        ),
    ];

    for ((kp1, rev1), (kp2, rev2), (expected_kp, expected_rev)) in test_cases {
        let mask1: OrdMask<i32> = unsafe { OrdMask::with_checked(kp1, rev1) };
        let mask2: OrdMask<i32> = unsafe { OrdMask::with_checked(kp2, rev2) };
        let expected: OrdMask<i32> = unsafe { OrdMask::with_checked(expected_kp, expected_rev) };
        let result = mask1.symmetric_difference(&mask2);
        assert_mask_eq!(result, expected, "symmetric_difference basic test failed");
    }
}

#[test]
fn test_symmetric_difference_with_empty() {
    // Empty set: symmetric difference is just the other set
    let empty = ordmask![];
    let mask = ordmask![0, 10];

    assert_eq!(empty.symmetric_difference(&mask), mask);
    assert_eq!(mask.symmetric_difference(&empty), mask);
    assert_eq!(empty.symmetric_difference(&empty), empty);
}

#[test]
fn test_symmetric_difference_with_universal() {
    // Universal set: symmetric difference with mask is complement
    let universal = ordmask![..];
    let mask = ordmask![0, 10];
    let empty = ordmask![];

    assert_eq!(universal.symmetric_difference(&mask), ordmask![.., 0, 10]);
    assert_eq!(mask.symmetric_difference(&universal), ordmask![.., 0, 10]);
    assert_eq!(universal.symmetric_difference(&universal), empty);
    assert_eq!(universal.symmetric_difference(&empty), universal);
}

#[test]
fn test_symmetric_difference_with_reversed() {
    // Reversed (complement) sets
    let reversed1 = ordmask![.., 0, 10]; // (-∞, 0) ∪ [10, ∞)
    let reversed2 = ordmask![.., 5, 15]; // (-∞, 5) ∪ [15, ∞)
    let normal = ordmask![0, 10];

    // Symmetric difference of two reversed
    let result = reversed1.symmetric_difference(&reversed2);
    assert_eq!(result, ordmask![0, 5, 10, 15]);

    // Symmetric difference of reversed with normal
    let result = reversed1.symmetric_difference(&normal);
    assert_eq!(result, ordmask![..]);

    // Symmetric difference of reversed2 with normal
    // reversed2 = (-∞, 5) ∪ [15, ∞), normal = [0, 10)
    // Sym diff = (-∞, 0) ∪ [5, 10) ∪ [15, ∞)
    let result = reversed2.symmetric_difference(&normal);
    assert_eq!(result, ordmask![.., 0, 5, 10, 15]);
}

#[test]
fn test_symmetric_difference_with_infinite() {
    // Infinite sets (unbounded on one side)
    let inf_right = ordmask![0]; // [0, ∞)
    let inf_left = ordmask![.., 10]; // (-∞, 10)
    let finite = ordmask![5, 15];

    // Symmetric difference of inf_right and inf_left
    let result = inf_right.symmetric_difference(&inf_left);
    assert_eq!(result, ordmask![.., 0, 10]);

    // Symmetric difference of inf_right and finite
    let result = inf_right.symmetric_difference(&finite);
    assert_eq!(result, ordmask![0, 5, 15]);

    // Symmetric difference of inf_left and finite
    let result = inf_left.symmetric_difference(&finite);
    assert_eq!(result, ordmask![.., 5, 10, 15]);
}

#[test]
fn test_symmetric_difference_at_edge() {
    let a = ordmask![.., 0]; // empty for u32
    let b = ordmask![u32::MAX]; // max only
    let c = ordmask![0, 10]; // from min
    let d = ordmask![100, u32::MAX]; // to max - 1
    let e = ordmask![5, 15]; // normal

    assert_eq!(a.symmetric_difference(&b), b);
    assert_eq!(b.symmetric_difference(&a), b);

    assert_eq!(a.symmetric_difference(&c), c);
    assert_eq!(c.symmetric_difference(&a), c);

    assert_eq!(a.symmetric_difference(&d), d);
    assert_eq!(d.symmetric_difference(&a), d);

    assert_eq!(a.symmetric_difference(&e), e);
    assert_eq!(e.symmetric_difference(&a), e);

    assert_eq!(c.symmetric_difference(&b), ordmask![0, 10, u32::MAX]);
    assert_eq!(b.symmetric_difference(&c), ordmask![0, 10, u32::MAX]);

    assert_eq!(d.symmetric_difference(&b), ordmask![100]);
    assert_eq!(b.symmetric_difference(&d), ordmask![100]);

    assert_eq!(e.symmetric_difference(&b), ordmask![5, 15, u32::MAX]);
    assert_eq!(b.symmetric_difference(&e), ordmask![5, 15, u32::MAX]);

    assert_eq!(c.symmetric_difference(&d), ordmask![0, 10, 100, u32::MAX]);
    assert_eq!(d.symmetric_difference(&c), ordmask![0, 10, 100, u32::MAX]);

    assert_eq!(c.symmetric_difference(&e), ordmask![0, 5, 10, 15]);
    assert_eq!(e.symmetric_difference(&c), ordmask![0, 5, 10, 15]);

    assert_eq!(d.symmetric_difference(&e), ordmask![5, 15, 100, u32::MAX]);
    assert_eq!(e.symmetric_difference(&d), ordmask![5, 15, 100, u32::MAX]);
}

#[test]
fn test_symmetric_difference_operator() {
    let mask1 = ordmask![0, 10];
    let mask2 = ordmask![5, 15];
    let expected = ordmask![0, 5, 10, 15];

    // Test all operator combinations
    assert_eq!(&mask1 ^ &mask2, expected);
    assert_eq!(mask1.clone() ^ mask2.clone(), expected);
    assert_eq!(&mask1 ^ mask2.clone(), expected);
    assert_eq!(mask1.clone() ^ &mask2, expected);
    assert_eq!(mask1 ^ mask2, expected);
}

#[test]
fn test_symmetric_difference_is_commutative() {
    // Symmetric difference should be commutative: a ^ b == b ^ a
    let mask1 = ordmask![0, 10];
    let mask2 = ordmask![5, 15];
    let mask3 = ordmask![.., 3, 20];

    assert_eq!(
        mask1.symmetric_difference(&mask2),
        mask2.symmetric_difference(&mask1)
    );
    assert_eq!(
        mask1.symmetric_difference(&mask3),
        mask3.symmetric_difference(&mask1)
    );
    assert_eq!(
        mask2.symmetric_difference(&mask3),
        mask3.symmetric_difference(&mask2)
    );
}

#[test]
fn test_symmetric_difference_complex_ranges() {
    // Multiple ranges in each mask
    let mask1 = ordmask![0, 5, 10, 15];
    let mask2 = ordmask![3, 7, 12, 18];
    let result = mask1.symmetric_difference(&mask2);
    assert_eq!(result, ordmask![0, 3, 5, 7, 10, 12, 15, 18]);
}
