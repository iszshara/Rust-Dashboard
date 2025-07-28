//! This module fetches Disk Resource Information

use super::converter::byte_to_gib;
use sysinfo::Disks;

/// Returns disk names, the total space of the disk and the available space of the disk formatted as a string.
/// # Example
/// ```
/// use linux_dashboard::backend::disk::format_disk_information;
/// let output = format_disk_information();
/// assert!(output.contains("Total Space"));
/// ```
///
pub fn format_disk_information() -> String {
    let mut result = String::new();
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() {
        let disk_info = format!(
            "[{:?}] Total Space: {:.2} GB | Available Space: {:.2} GB\n",
            disk.name(),
            byte_to_gib(disk.total_space()),
            byte_to_gib(disk.available_space())
        );
        result.push_str(&disk_info);
        println!("{disk_info}");
    }

    result
}
