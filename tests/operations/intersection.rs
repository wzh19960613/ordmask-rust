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
// Intersection Tests
// ============================================================================

#[test]
fn test_intersection_basic() {
    let test_cases: Vec<((Vec<i32>, bool), (Vec<i32>, bool), (Vec<i32>, bool))> = vec![
        // (mask1, mask2, expected)
        // Overlapping ranges
        (
            (vec![0, 10], false),
            (vec![5, 15], false),
            (vec![5, 10], false),
        ),
        // Non-overlapping ranges - empty intersection
        (
            (vec![0, 5], false),
            (vec![10, 15], false),
            (vec![], false),
        ),
        // One contained in another
        (
            (vec![0, 20], false),
            (vec![5, 10], false),
            (vec![5, 10], false),
        ),
        // Same range
        (
            (vec![0, 10], false),
            (vec![0, 10], false),
            (vec![0, 10], false),
        ),
        // Edge touching
        (
            (vec![0, 5], false),
            (vec![5, 10], false),
            (vec![], false), // [0,5) ∩ [5,10) = empty
        ),
    ];

    for ((kp1, rev1), (kp2, rev2), (expected_kp, expected_rev)) in test_cases {
        let mask1: OrdMask<i32> = unsafe { OrdMask::with_checked(kp1, rev1) };
        let mask2: OrdMask<i32> = unsafe { OrdMask::with_checked(kp2, rev2) };
        let expected: OrdMask<i32> = unsafe { OrdMask::with_checked(expected_kp, expected_rev) };
        let result = OrdMask::intersection(&[&mask1, &mask2]);
        assert_mask_eq!(result, expected, "intersection basic test failed");
    }
}

#[test]
fn test_intersection_with_empty() {
    // Empty set annihilates intersection
    let empty = ordmask![];
    let mask = ordmask![0, 10];
    let universal = ordmask![..];

    assert_eq!(OrdMask::intersection(&[&empty, &mask]), empty);
    assert_eq!(OrdMask::intersection(&[&mask, &empty]), empty);
    assert_eq!(OrdMask::intersection(&[&empty, &empty]), empty);
    assert_eq!(OrdMask::intersection(&[&empty, &universal]), empty);
}

#[test]
fn test_intersection_with_universal() {
    // Universal set is identity for intersection
    let universal = ordmask![..];
    let mask = ordmask![0, 10];
    let empty = ordmask![];

    assert_eq!(OrdMask::intersection(&[&universal, &mask]), mask);
    assert_eq!(OrdMask::intersection(&[&mask, &universal]), mask);
    assert_eq!(OrdMask::intersection(&[&universal, &universal]), universal);
    assert_eq!(OrdMask::intersection(&[&universal, &empty]), empty);
}

#[test]
fn test_intersection_with_reversed() {
    // Reversed (complement) sets
    let reversed1 = ordmask![.., 0, 10]; // (-∞, 0) ∪ [10, ∞)
    let reversed2 = ordmask![.., 5, 15]; // (-∞, 5) ∪ [15, ∞)
    let normal = ordmask![0, 10];

    // Intersection of two reversed
    let result = OrdMask::intersection(&[&reversed1, &reversed2]);
    assert_eq!(result, ordmask![.., 0, 15]);

    // Intersection of reversed with normal
    let result = OrdMask::intersection(&[&reversed1, &normal]);
    assert_eq!(result, ordmask![]);

    // Intersection of reversed2 with normal
    // reversed2 = (-∞, 5) ∪ [15, ∞), normal = [0, 10)
    // Intersection = [0, 5)
    let result = OrdMask::intersection(&[&reversed2, &normal]);
    assert_eq!(result, ordmask![0, 5]);
}

#[test]
fn test_intersection_with_infinite() {
    // Infinite sets (unbounded on one side)
    let inf_right = ordmask![0]; // [0, ∞)
    let inf_left = ordmask![.., 10]; // (-∞, 10)
    let finite = ordmask![5, 15];

    // Intersection of inf_right and inf_left
    let result = OrdMask::intersection(&[&inf_right, &inf_left]);
    assert_eq!(result, ordmask![0, 10]);

    // Intersection of inf_right and finite
    let result = OrdMask::intersection(&[&inf_right, &finite]);
    assert_eq!(result, ordmask![5, 15]);

    // Intersection of inf_left and finite
    let result = OrdMask::intersection(&[&inf_left, &finite]);
    assert_eq!(result, ordmask![5, 10]);
}

#[test]
fn test_intersection_at_edge() {
    let a = ordmask![.., 0]; // empty
    let b = ordmask![u32::MAX]; // max only
    let c = ordmask![0, 10]; // from min
    let d = ordmask![100, u32::MAX]; // to max - 1
    let e = ordmask![5, 15]; // normal

    assert_eq!(OrdMask::intersection(&[&a, &b]), a);
    assert_eq!(OrdMask::intersection(&[&b, &a]), a);

    assert_eq!(OrdMask::intersection(&[&a, &c]), a);
    assert_eq!(OrdMask::intersection(&[&c, &a]), a);

    assert_eq!(OrdMask::intersection(&[&a, &d]), a);
    assert_eq!(OrdMask::intersection(&[&d, &a]), a);

    assert_eq!(OrdMask::intersection(&[&a, &e]), a);
    assert_eq!(OrdMask::intersection(&[&e, &a]), a);

    assert_eq!(OrdMask::intersection(&[&b, &c]), ordmask![]);
    assert_eq!(OrdMask::intersection(&[&c, &b]), ordmask![]);

    assert_eq!(OrdMask::intersection(&[&b, &d]), ordmask![]);
    assert_eq!(OrdMask::intersection(&[&d, &b]), ordmask![]);

    assert_eq!(OrdMask::intersection(&[&b, &e]), ordmask![]);
    assert_eq!(OrdMask::intersection(&[&e, &b]), ordmask![]);

    assert_eq!(OrdMask::intersection(&[&c, &d]), ordmask![]);
    assert_eq!(OrdMask::intersection(&[&d, &c]), ordmask![]);

    assert_eq!(OrdMask::intersection(&[&c, &e]), ordmask![5, 10]);
    assert_eq!(OrdMask::intersection(&[&e, &c]), ordmask![5, 10]);

    assert_eq!(OrdMask::intersection(&[&d, &e]), ordmask![]);
    assert_eq!(OrdMask::intersection(&[&e, &d]), ordmask![]);
}

#[test]
fn test_intersection_operator() {
    let mask1 = ordmask![0, 10];
    let mask2 = ordmask![5, 15];
    let expected = ordmask![5, 10];

    // Test all operator combinations
    assert_eq!(&mask1 & &mask2, expected);
    assert_eq!(mask1.clone() & mask2.clone(), expected);
    assert_eq!(&mask1 & mask2.clone(), expected);
    assert_eq!(mask1.clone() & &mask2, expected);
    assert_eq!(mask1 & mask2, expected);
}

#[test]
fn test_intersection_multiple() {
    // Intersection of multiple masks
    let mask1 = ordmask![0, 20];
    let mask2 = ordmask![5, 15];
    let mask3 = ordmask![10, 25];
    let result = OrdMask::intersection(&[&mask1, &mask2, &mask3]);
    assert_eq!(result, ordmask![10, 15]);

    // Non-overlapping intersection with multiple
    let mask1 = ordmask![0, 5];
    let mask2 = ordmask![10, 15];
    let mask3 = ordmask![20, 25];
    let result = OrdMask::intersection(&[&mask1, &mask2, &mask3]);
    assert_eq!(result, ordmask![]);
}

#[test]
fn test_intersection_complex_ranges() {
    // Multiple ranges in each mask
    let mask1 = ordmask![0, 5, 10, 15, 20, 25];
    let mask2 = ordmask![3, 7, 12, 18, 22, 30];
    let result = OrdMask::intersection(&[&mask1, &mask2]);
    assert_eq!(result, ordmask![3, 5, 12, 15, 22, 25]);
}
