use ordmask::{OrdMask, ordmask};

#[test]
fn simplify() {
    let test_cases = [
        (vec![0, 0], vec![]),
        (vec![0, 0, 1, 1], vec![]),
        (vec![0, 0, 1, 1, 2], vec![2]),
        (vec![0, 2, 2, 4], vec![0, 4]),
        (vec![0, 2, 2, 2, 4, 4, 4, 6, 6, 8, 8], vec![0, 2, 4]),
        (vec![1, 1, 1, 1, 2, 2, 3, 3, 3, 3], vec![]),
        (
            vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 6],
            vec![1, 2, 3],
        ),
        (vec![1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 4], vec![2]),
        (vec![1, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 5], vec![1, 3]),
        (vec![1, 1, 1, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5], vec![1, 3, 5]),
        (vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![]),
        (
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9, 10],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        ),
    ];

    for (input, expected) in test_cases {
        let mut mask = unsafe { OrdMask::with_checked(input.clone(), false) };
        assert!(
            mask.simplify(),
            "Test failed for input: {input:?}, not modified when `simplify`"
        );
        let mut should_be = unsafe { OrdMask::with_checked(expected.clone(), false) };
        assert!(
            !should_be.simplify(),
            "Test failed for input: {expected:?}, modified when `simplify`"
        );
        assert_eq!(
            mask, should_be,
            "Test failed for input: {input:?}, expected: {expected:?}, got: {mask:?}"
        );
    }
}

#[test]
fn simplify_empty() {
    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![], false) };
    assert!(!mask.simplify());
    assert_eq!(mask, ordmask![]);
}

#[test]
fn simplify_at_min_value() {
    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0], true) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![]);

    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0, 0], true) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![0]);

    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0, 0, 0], true) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![]);

    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0], false) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![..]);

    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0, 0], false) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![]);

    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0, 0, 0], false) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![..]);

    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0, 1], true) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![1]);

    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0, 0, 1], true) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![0, 1]);

    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0, 0, 0, 1], true) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![1]);

    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0, 1], false) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![.., 1]);

    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0, 0, 1], false) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![1]);

    let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0, 0, 0, 1], false) };
    assert!(mask.simplify());
    assert_eq!(mask, ordmask![.., 1]);
}

#[test]
fn no_need_to_simplify() {
    let test_cases = [vec![], vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 2, 3, 4]];
    for case in test_cases {
        let mut mask = unsafe { OrdMask::<u32>::with_checked(case.clone(), false) };
        assert!(!mask.simplify());
        let mut mask = unsafe { OrdMask::<u32>::with_checked(case, true) };
        assert!(!mask.simplify());
    }
}
