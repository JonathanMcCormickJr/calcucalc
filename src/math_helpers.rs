
/// Checks if two f64 values are equal within a specified tolerance.
/// 
/// ```rust
/// use calcucalc::math_helpers::is_equal_within_tolerance_to;
/// 
/// let a = 0.1 + 0.2;
/// let b = 0.3;
/// assert!(is_equal_within_tolerance_to(&a, &b));
/// ```
#[must_use]
pub fn is_equal_within_tolerance_to(a: &f64, b: &f64) -> bool {
    let tolerance = 1e-10;
    (a - b).abs() < tolerance
}