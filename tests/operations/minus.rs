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
// Minus/Difference Tests
// ============================================================================

#[test]
fn test_minus_basic() {
    let test_cases: Vec<((Vec<i32>, bool), (Vec<i32>, bool), (Vec<i32>, bool))> = vec![
        // (mask1, mask2, expected)
        // mask1 - mask2
        // Partially overlapping
        (
            (vec![0, 10], false),
            (vec![5, 15], false),
            (vec![0, 5], false),
        ),
        // Non-overlapping
        (
            (vec![0, 5], false),
            (vec![10, 15], false),
            (vec![0, 5], false),
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
            (vec![], false),
        ),
    ];

    for ((kp1, rev1), (kp2, rev2), (expected_kp, expected_rev)) in test_cases {
        let mask1: OrdMask<i32> = unsafe { OrdMask::with_checked(kp1, rev1) };
        let mask2: OrdMask<i32> = unsafe { OrdMask::with_checked(kp2, rev2) };
        let expected: OrdMask<i32> = unsafe { OrdMask::with_checked(expected_kp, expected_rev) };
        let result = mask1.minus(&[&mask2]);
        assert_mask_eq!(result, expected, "minus basic test failed");
    }
}

#[test]
fn test_minus_with_empty() {
    // Empty set as subtrahend: nothing to remove
    let empty = ordmask![];
    let mask = ordmask![0, 10];

    assert_eq!(mask.minus(&[&empty]), mask);
    assert_eq!(empty.minus(&[&mask]), empty);
    assert_eq!(empty.minus(&[&empty]), empty);
}

#[test]
fn test_minus_with_universal() {
    // Universal set as subtrahend: everything is removed
    let universal = ordmask![..];
    let mask = ordmask![0, 10];
    let empty = ordmask![];

    assert_eq!(mask.minus(&[&universal]), empty);
    assert_eq!(universal.minus(&[&mask]), ordmask![.., 0, 10]);
    assert_eq!(universal.minus(&[&empty]), universal);
}

#[test]
fn test_minus_with_reversed() {
    // Reversed (complement) sets
    let reversed1 = ordmask![.., 0, 10]; // (-∞, 0) ∪ [10, ∞)
    let normal = ordmask![0, 10];

    // normal - reversed1: remove (-∞,0) ∪ [10,∞) from [0,10] = [0,10]
    let result = normal.minus(&[&reversed1]);
    assert_eq!(result, normal);

    // reversed1 - normal: remove [0,10] from (-∞,0) ∪ [10,∞) = same
    let result = reversed1.minus(&[&normal]);
    assert_eq!(result, reversed1);

    // reversed1 - reversed2
    let reversed2 = ordmask![.., 5, 15]; // (-∞, 5) ∪ [15, ∞)
    let result = reversed1.minus(&[&reversed2]);
    // (-∞,0)∪[10,∞) - ((-∞,5)∪[15,∞)) = [5,∞)∩((-∞,0)∪[10,∞)) - [15,∞)
    // = [10,15]
    assert_eq!(result, ordmask![10, 15]);
}

#[test]
fn test_minus_with_infinite() {
    // Infinite sets (unbounded on one side)
    let inf_right = ordmask![0]; // [0, ∞)
    let inf_left = ordmask![.., 10]; // (-∞, 10)
    let finite = ordmask![5, 15];

    // inf_right - finite: [0,∞) - [5,15) = [0,5) ∪ [15,∞)
    let result = inf_right.minus(&[&finite]);
    assert_eq!(result, ordmask![0, 5, 15]);

    // inf_left - finite: (-∞,10) - [5,15) = (-∞,5)
    let result = inf_left.minus(&[&finite]);
    assert_eq!(result, ordmask![.., 5]);

    // finite - inf_right: [5,15) - [0,∞) = empty
    let result = finite.minus(&[&inf_right]);
    assert_eq!(result, ordmask![]);

    // finite - inf_left: [5,15) - (-∞,10) = [10,15)
    let result = finite.minus(&[&inf_left]);
    assert_eq!(result, ordmask![10, 15]);
}

#[test]
fn test_minus_at_edge() {
    let a = ordmask![.., 0]; // empty for u32
    let b = ordmask![u32::MAX]; // max only
    let c = ordmask![0, 10]; // from min
    let d = ordmask![100, u32::MAX]; // to max - 1
    let e = ordmask![5, 15]; // normal

    assert_eq!(a.minus(&[&b]), a);
    assert_eq!(b.minus(&[&a]), b);

    assert_eq!(a.minus(&[&c]), a);
    assert_eq!(c.minus(&[&a]), c);

    assert_eq!(a.minus(&[&d]), a);
    assert_eq!(d.minus(&[&a]), d);

    assert_eq!(a.minus(&[&e]), a);
    assert_eq!(e.minus(&[&a]), e);

    assert_eq!(c.minus(&[&b]), c);
    assert_eq!(b.minus(&[&c]), b);

    assert_eq!(d.minus(&[&b]), ordmask![100, u32::MAX]);
    assert_eq!(b.minus(&[&d]), b);

    assert_eq!(e.minus(&[&b]), e);
    assert_eq!(b.minus(&[&e]), b);

    assert_eq!(c.minus(&[&d]), c);
    assert_eq!(d.minus(&[&c]), d);

    assert_eq!(c.minus(&[&e]), ordmask![0, 5]);
    assert_eq!(e.minus(&[&c]), ordmask![10, 15]);

    assert_eq!(d.minus(&[&e]), d);
    assert_eq!(e.minus(&[&d]), e);
}

#[test]
fn test_minus_operator() {
    let mask1 = ordmask![0, 10];
    let mask2 = ordmask![5, 15];
    let expected = ordmask![0, 5];

    // Test all operator combinations
    assert_eq!(&mask1 - &mask2, expected);
    assert_eq!(mask1.clone() - mask2.clone(), expected);
    assert_eq!(&mask1 - mask2.clone(), expected);
    assert_eq!(mask1.clone() - &mask2, expected);
    assert_eq!(mask1 - mask2, expected);
}

#[test]
fn test_minus_multiple() {
    // Minus with multiple subtrahends
    let mask1 = ordmask![0, 30];
    let mask2 = ordmask![5, 10];
    let mask3 = ordmask![15, 20];
    let result = mask1.minus(&[&mask2, &mask3]);
    assert_eq!(result, ordmask![0, 5, 10, 15, 20, 30]);

    // Overlapping subtrahends
    let mask1 = ordmask![0, 20];
    let mask2 = ordmask![5, 12];
    let mask3 = ordmask![10, 18];
    let result = mask1.minus(&[&mask2, &mask3]);
    assert_eq!(result, ordmask![0, 5, 18, 20]);
}

#[test]
fn test_minus_complex_ranges() {
    // Multiple ranges in each mask
    // mask1 = [0, 5) ∪ [10, 15) ∪ [20, 25)
    // mask2 = [2, 3) ∪ [12, 18) ∪ [22, 24)
    let mask1 = ordmask![0, 5, 10, 15, 20, 25];
    let mask2 = ordmask![2, 3, 12, 18, 22, 24];
    let result = mask1.minus(&[&mask2]);
    // Result = [0, 2) ∪ [3, 5) ∪ [10, 12) ∪ [20, 22) ∪ [24, 25)
    assert_eq!(result, ordmask![0, 2, 3, 5, 10, 12, 20, 22, 24, 25]);
}
