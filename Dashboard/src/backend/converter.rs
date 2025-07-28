//! This module contains a helper function

/// Converter which converts Bytes to GiB
/// # Example
/// ```
/// use linux_dashboard::backend::converter::byte_to_gib;
/// let gib = byte_to_gib(1073741824);
/// assert_eq!(gib, 1.0);
/// ```
///
pub fn byte_to_gib(kib: u64) -> f64 {
    kib as f64 / 1_073_741_824.0
}
