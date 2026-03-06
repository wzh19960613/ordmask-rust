//! Tests for serde serialization/deserialization support.
//!
//! Run with: cargo test --features serde

#[cfg(feature = "serde")]
mod serde_tests {
    use ordmask::{ordmask, OrdMask};

    #[test]
    fn test_serde_empty() {
        let mask: OrdMask<i32> = ordmask![];
        let json = serde_json::to_string(&mask).unwrap();
        let decoded: OrdMask<i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(mask, decoded);
    }

    #[test]
    fn test_serde_universal() {
        let mask: OrdMask<i32> = ordmask![..];
        let json = serde_json::to_string(&mask).unwrap();
        let decoded: OrdMask<i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(mask, decoded);
    }

    #[test]
    fn test_serde_simple() {
        let mask = ordmask![0, 10, 20];
        let json = serde_json::to_string(&mask).unwrap();
        let decoded: OrdMask<i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(mask, decoded);
    }

    #[test]
    fn test_serde_complement() {
        let mask = ordmask![.., 0, 10, 20];
        let json = serde_json::to_string(&mask).unwrap();
        let decoded: OrdMask<i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(mask, decoded);
    }

    #[test]
    fn test_serde_json_format() {
        let mask = ordmask![1, 5, 10];
        let json = serde_json::to_string(&mask).unwrap();
        assert!(json.contains("\"key_points\":[1,5,10]"));
        assert!(json.contains("\"based_on_universal\":false"));
    }

    #[test]
    fn test_serdes_roundtrip_with_operations() {
        let mask1 = ordmask![0, 10];
        let mask2 = ordmask![5, 15];

        // Serialize both masks
        let json1 = serde_json::to_string(&mask1).unwrap();
        let json2 = serde_json::to_string(&mask2).unwrap();

        // Deserialize
        let decoded1: OrdMask<i32> = serde_json::from_str(&json1).unwrap();
        let decoded2: OrdMask<i32> = serde_json::from_str(&json2).unwrap();

        // Verify operations work correctly
        let union = decoded1 | decoded2;
        assert!(union.contains(&7));
        assert!(union.contains(&2));
    }
}
