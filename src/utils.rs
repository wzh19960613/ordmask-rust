/// Check if a slice is strictly increasing or non-decreasing.
///
/// - `STRICTLY = true`: check for strictly increasing (no duplicates allowed)
/// - `STRICTLY = false`: check for non-decreasing (duplicates allowed)
///
/// Returns `(is_valid, first_invalid_pos)` where `is_valid` indicates if the slice
/// meets the requirement, and `first_invalid_pos` is the index of the first violation.
pub fn is_increasing<const STRICTLY: bool, T: PartialOrd>(arr: &[T]) -> (bool, usize) {
    let len = arr.len();
    for i in 1..len {
        if match STRICTLY {
            true => arr[i] <= arr[i - 1],
            false => arr[i] < arr[i - 1],
        } {
            return (false, i);
        }
    }
    (true, len)
}
