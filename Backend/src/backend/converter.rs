//! This module contains a helper function

/// Converter which converts Bytes to GiB
/// 
/// #Example
/// 
/// ```
/// 1 GiB * 1024^3 = 1073741824 or
/// 1 GB * 1000^3 = 1000000000
/// ```
/// 
pub fn byte_to_gib(kib: u64) -> f64 {
    
    kib as f64 / 1_073_741_824.0
}