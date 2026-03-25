use ordmask::{OrdMask, ordmask};

#[test]
fn test_empty() {
    let mask: OrdMask<i32> = ordmask![];
    assert_eq!(mask.spans().collect::<Vec<_>>(), vec![]);
}

#[test]
fn test_universal() {
    let mask = ordmask![<i32> ..];
    assert_eq!(mask.spans().collect::<Vec<_>>(), vec![(i32::MIN, i32::MAX)]);
}

#[test]
fn test_from_universal_single_point() {
    let mask = ordmask![<i32> .., 5];
    assert_eq!(mask.spans().collect::<Vec<_>>(), vec![(i32::MIN, 5)]);
}

#[test]
fn test_from_empty_single_point() {
    let mask = ordmask![<i32> 5];
    assert_eq!(mask.spans().collect::<Vec<_>>(), vec![(5, i32::MAX)]);
}

#[test]
fn test_from_universal_two_points() {
    let mask = ordmask![<i32> .., 1, 2];
    assert_eq!(
        mask.spans().collect::<Vec<_>>(),
        vec![(i32::MIN, 1), (2, i32::MAX)]
    );
}

#[test]
fn test_from_empty_two_points() {
    let mask = ordmask![<i32> 1, 2];
    assert_eq!(mask.spans().collect::<Vec<_>>(), vec![(1, 2)]);
}

#[test]
fn test_from_universal_three_points() {
    let mask = ordmask![<i32> .., 1, 2, 5];
    assert_eq!(
        mask.spans().collect::<Vec<_>>(),
        vec![(i32::MIN, 1), (2, 5)]
    );
}

#[test]
fn test_from_empty_three_points() {
    let mask = ordmask![<i32> 1, 2, 5];
    assert_eq!(
        mask.spans().collect::<Vec<_>>(),
        vec![(1, 2), (5, i32::MAX)]
    );
}

#[test]
fn test_into_spans() {
    let mask = ordmask![<i32> .., 1, 2];
    assert_eq!(
        mask.into_spans().collect::<Vec<_>>(),
        vec![(i32::MIN, 1), (2, i32::MAX)]
    );
}

#[test]
fn test_for_loop() {
    let mask = ordmask![<i32> 1, 2];
    let mut spans = Vec::new();
    for (start, end) in mask.spans() {
        spans.push((start, end));
    }
    assert_eq!(spans, vec![(1, 2)]);

    // Empty mask has no spans
    assert_eq!(ordmask![<i32>].spans().collect::<Vec<_>>(), vec![]);
    // Universal mask has one span
    assert_eq!(
        ordmask![..].spans().collect::<Vec<_>>(),
        vec![(i32::MIN, i32::MAX)]
    );
    // Mask with two points
    assert_eq!(ordmask![1, 2].spans().collect::<Vec<_>>(), vec![(1, 2)]);
    // Mask with alternating spans
    assert_eq!(
        ordmask![.., 1, 2].spans().collect::<Vec<_>>(),
        vec![(i32::MIN, 1), (2, i32::MAX)]
    );
}
