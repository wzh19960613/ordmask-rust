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
// Union Tests
// ============================================================================

#[test]
fn test_union_basic() {
    let test_cases: Vec<((Vec<i32>, bool), (Vec<i32>, bool), (Vec<i32>, bool))> = vec![
        // (mask1, mask2, expected)
        // Normal + Normal
        (
            (vec![0, 10], false),
            (vec![5, 15], false),
            (vec![0, 15], false),
        ),
        (
            (vec![0, 5], false),
            (vec![10, 15], false),
            (vec![0, 5, 10, 15], false),
        ),
        (
            (vec![0, 10], false),
            (vec![20, 30], false),
            (vec![0, 10, 20, 30], false),
        ),
        // Overlapping ranges
        (
            (vec![0, 20], false),
            (vec![5, 10], false),
            (vec![0, 20], false),
        ),
        // Same range
        (
            (vec![0, 10], false),
            (vec![0, 10], false),
            (vec![0, 10], false),
        ),
    ];

    for ((kp1, rev1), (kp2, rev2), (expected_kp, expected_rev)) in test_cases {
        let mask1: OrdMask<i32> = unsafe { OrdMask::with_checked(kp1, rev1) };
        let mask2: OrdMask<i32> = unsafe { OrdMask::with_checked(kp2, rev2) };
        let expected: OrdMask<i32> = unsafe { OrdMask::with_checked(expected_kp, expected_rev) };
        let result = OrdMask::union(&[&mask1, &mask2]);
        assert_mask_eq!(result, expected, "union basic test failed");
    }
}

#[test]
fn test_union_with_empty() {
    // Empty set is identity for union
    let empty = ordmask![];
    let mask = ordmask![0, 10];
    let universal = ordmask![..];

    assert_eq!(OrdMask::union(&[&empty, &mask]), mask);
    assert_eq!(OrdMask::union(&[&mask, &empty]), mask);
    assert_eq!(OrdMask::union(&[&empty, &empty]), empty);
    assert_eq!(OrdMask::union(&[&empty, &universal]), universal);
}

#[test]
fn test_union_with_universal() {
    // Universal set absorbs everything in union
    let universal = ordmask![..];
    let mask = ordmask![0, 10];
    let empty = ordmask![];

    assert_eq!(OrdMask::union(&[&universal, &mask]), universal);
    assert_eq!(OrdMask::union(&[&mask, &universal]), universal);
    assert_eq!(OrdMask::union(&[&universal, &universal]), universal);
    assert_eq!(OrdMask::union(&[&universal, &empty]), universal);
}

#[test]
fn test_union_with_reversed() {
    // Reversed (complement) sets
    // ordmask![.., 0, 10] = (-∞, 0) ∪ [10, ∞)
    let reversed1 = ordmask![.., 0, 10]; // (-∞, 0) ∪ [10, ∞)
    let reversed2 = ordmask![.., 5, 15, 20]; // (-∞, 5) ∪ [15, 20)
    let normal = ordmask![0, 10];

    // Union of two reversed
    let result = OrdMask::union(&[&reversed1, &reversed2]);
    assert_eq!(result, ordmask![.., 5, 10]);

    let result = OrdMask::union(&[&reversed1, &normal]);
    assert_eq!(result, ordmask![..]);

    let result = OrdMask::union(&[&reversed2, &normal]);
    assert_eq!(result, ordmask![.., 10, 15, 20]);
}

#[test]
fn test_union_with_infinite() {
    // Infinite sets (unbounded on one side)
    let inf_right = ordmask![0]; // [0, ∞)
    let inf_left = ordmask![.., 10]; // (-∞, 10)
    let finite = ordmask![5, 15];

    // Union of inf_right and inf_left should be universal
    assert_eq!(OrdMask::union(&[&inf_right, &inf_left]), ordmask![..]);

    // Union of inf_right and finite
    let result = OrdMask::union(&[&inf_right, &finite]);
    assert_eq!(result, ordmask![0]); // [0, ∞) since finite is contained

    let result = OrdMask::union(&[&inf_left, &finite]);
    assert_eq!(result, ordmask![.., 15]);
}

#[test]
fn test_union_at_edge() {
    let a = ordmask![.., 0]; // empty
    let b = ordmask![u32::MAX]; // max only
    let c = ordmask![0, 10]; // from min
    let d = ordmask![100, u32::MAX]; // to max - 1
    let e = ordmask![5, 15]; // normal

    assert_eq!(OrdMask::union(&[&a, &b]), b);
    assert_eq!(OrdMask::union(&[&b, &a]), b);

    assert_eq!(OrdMask::union(&[&a, &c]), c);
    assert_eq!(OrdMask::union(&[&c, &a]), c);

    assert_eq!(OrdMask::union(&[&a, &d]), d);
    assert_eq!(OrdMask::union(&[&d, &a]), d);

    assert_eq!(OrdMask::union(&[&a, &e]), e);
    assert_eq!(OrdMask::union(&[&e, &a]), e);

    assert_eq!(OrdMask::union(&[&b, &c]), ordmask![0, 10, u32::MAX]);
    assert_eq!(OrdMask::union(&[&c, &b]), ordmask![0, 10, u32::MAX]);

    assert_eq!(OrdMask::union(&[&b, &d]), ordmask![100]);
    assert_eq!(OrdMask::union(&[&d, &b]), ordmask![100]);

    assert_eq!(OrdMask::union(&[&b, &e]), ordmask![5, 15, u32::MAX]);
    assert_eq!(OrdMask::union(&[&e, &b]), ordmask![5, 15, u32::MAX]);

    assert_eq!(OrdMask::union(&[&c, &d]), ordmask![0, 10, 100, u32::MAX]);
    assert_eq!(OrdMask::union(&[&d, &c]), ordmask![0, 10, 100, u32::MAX]);

    assert_eq!(OrdMask::union(&[&c, &e]), ordmask![0, 15]);
    assert_eq!(OrdMask::union(&[&e, &c]), ordmask![0, 15]);

    assert_eq!(OrdMask::union(&[&d, &e]), ordmask![5, 15, 100, u32::MAX]);
    assert_eq!(OrdMask::union(&[&e, &d]), ordmask![5, 15, 100, u32::MAX]);
}

#[test]
fn test_union_operator() {
    let mask1 = ordmask![0, 10];
    let mask2 = ordmask![5, 15];
    let expected = ordmask![0, 15];

    // Test all operator combinations
    assert_eq!(&mask1 | &mask2, expected);
    assert_eq!(mask1.clone() | mask2.clone(), expected);
    assert_eq!(&mask1 | mask2.clone(), expected);
    assert_eq!(mask1.clone() | &mask2, expected);
    assert_eq!(mask1 | mask2, expected);
}

#[test]
fn test_union_multiple() {
    // Union of multiple masks
    let mask1 = ordmask![0, 5];
    let mask2 = ordmask![10, 15];
    let mask3 = ordmask![20, 25];
    let result = OrdMask::union(&[&mask1, &mask2, &mask3]);
    assert_eq!(result, ordmask![0, 5, 10, 15, 20, 25]);

    // Overlapping union
    let mask1 = ordmask![0, 10];
    let mask2 = ordmask![5, 15];
    let mask3 = ordmask![12, 20];
    let result = OrdMask::union(&[&mask1, &mask2, &mask3]);
    assert_eq!(result, ordmask![0, 20]);
}
