use ordmask::{OrdMask, OrderedSub, WithMax, WithMin, ordmask};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct MyStruct(i32);

impl WithMin for MyStruct {
    const MIN: Self = MyStruct(i32::MIN);
}

impl WithMax for MyStruct {
    const MAX: Self = MyStruct(i32::MAX);
}

impl OrderedSub for MyStruct {
    type Target = u32;

    fn ordered_sub(&self, other: &Self) -> Self::Target {
        self.0.ordered_sub(&other.0) // Same as the library does for i32
    }
}

#[test]
fn test_custom_type_empty_mask() {
    let mask: OrdMask<MyStruct> = ordmask![];
    assert_eq!(mask.spans_count(), 0);
    assert!(mask.values_count() == 0u32);
}

#[test]
fn test_custom_type_single_span() {
    // [MyStruct(0), MyStruct(10)) - size should be 10
    let mask = ordmask![<MyStruct> MyStruct(0), MyStruct(10)];
    assert_eq!(mask.spans_count(), 1);
    assert!(mask.values_count() == 10u32);
}

#[test]
fn test_custom_type_from_universal() {
    // [MIN, MyStruct(5)) - size should be 5 - i32::MIN = 5 + 2147483648
    let mask = ordmask![<MyStruct> .., MyStruct(5)];
    assert_eq!(mask.spans_count(), 1);
    // 5 - i32::MIN = 5 - (-2147483648) = 2147483653
    assert!(mask.values_count() == 2147483653u32);
}

#[test]
fn test_custom_type_from_empty_to_max() {
    let mask = ordmask![<MyStruct> MyStruct(0)];
    assert_eq!(mask.spans_count(), 1);
    // values_count = (MAX - 0) + 1 = i32::MAX + 1 = 2147483648
    // because MAX value is included in the mask
    assert!(mask.values_count() == 2147483648u32);
    assert!(mask.is_max_value_included());
}

#[test]
fn test_custom_type_multiple_spans() {
    // [MIN, MyStruct(5)) and [MyStruct(10), MAX]
    let mask = ordmask![<MyStruct> .., MyStruct(5), MyStruct(10)];
    assert_eq!(mask.spans_count(), 2);

    // First span: 5 - MIN = 2147483653
    // Second span: MAX - 10 = 2147483637, plus 1 for MAX being included
    let expected = 2147483653u32 + 2147483638u32;
    assert!(mask.values_count() == expected);
}

#[test]
fn test_custom_type_included() {
    let mask = ordmask![<MyStruct> MyStruct(0), MyStruct(10)];
    assert!(mask.included(&MyStruct(0)));
    assert!(mask.included(&MyStruct(5)));
    assert!(!mask.included(&MyStruct(10)));
    assert!(!mask.included(&MyStruct(-1)));
}

#[test]
fn test_custom_type_universal() {
    let mask = ordmask![<MyStruct> ..];
    assert!(mask.is_universal());
    assert_eq!(mask.spans_count(), 1);
}

#[test]
#[should_panic]
fn test_custom_type_values_count_overflow() {
    let mask = ordmask![<MyStruct> ..];
    // values_count().get() = (MAX - MIN) + 1 = u32::MAX + 1 = 4294967296 > u32::MAX
    mask.values_count().get();
}
