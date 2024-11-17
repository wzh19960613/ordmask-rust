use ordmask::OrdMask;

#[test]
fn simplify() {
    let test_cases = vec![
        (vec![], vec![]),
        (vec![0, 0, 1, 1], vec![]),
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
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        ),
    ];

    for (input, expected) in test_cases {
        let mask = OrdMask::try_from(input.clone()).unwrap();
        assert_eq!(
            mask,
            unsafe { OrdMask::with_unchecked(expected.clone(), false) },
            "Test failed for input: {:?}, expected: {:?}, got: {:?}",
            input,
            expected,
            mask
        );
    }
}
