//! This module contains helper functions for unit conversion

/// Converter which converts Bytes to GiB
/// # Example
/// ```
/// use linux_dashboard::backend::converter::byte_to_gib;
/// let gib = byte_to_gib(1073741824);
/// assert_eq!(gib, 1.0);
/// ```
///
pub fn byte_to_gib(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}

/// Formats a byte value into a human-readable string with the appropriate unit (B, KB, MB, GB).
/// # Example
/// ```
/// use linux_dashboard::backend::converter::format_bytes;
/// assert_eq!(format_bytes(512), "512 B");
/// assert_eq!(format_bytes(1048576), "1.00 MB");
/// ```
///
pub fn format_bytes(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;

    let bytes_f = bytes as f64;
    if bytes_f >= GIB {
        format!("{:.2} GB", bytes_f / GIB)
    } else if bytes_f >= MIB {
        format!("{:.1} MB", bytes_f / MIB)
    } else if bytes_f >= KIB {
        format!("{:.0} KB", bytes_f / KIB)
    } else {
        format!("{bytes} B")
    }
}
