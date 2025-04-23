//! This module fetches memory resource information

use sysinfo::System;
use super::converter::byte_to_gib;

/// Returns memory and swap information formatted as a single string
pub fn format_ram_info(sys: &System) -> String {
    let ram_total_memory = format!("Total memory: {:.2} GB", byte_to_gib(sys.total_memory()));
    let ram_used_memory = format!("Used memory: {:.2} GB", byte_to_gib(sys.used_memory()));
    let ram_total_swap = format!("Total swap: {:.2} GB", byte_to_gib(sys.total_swap()));
    let ram_used_swap = format!("Used swap: {:.2} GB", byte_to_gib(sys.used_swap()));

    let combined_info = format!(
        "RAM: {:.2} GB / {:.2} GB\nSwap: {:.2} GB / {:.2} GB",
        ram_total_memory, ram_used_memory, ram_total_swap, ram_used_swap
    );

    let max_lines = 2;
    let truncated_info = combined_info
        .lines()
        .take(max_lines)
        .collect::<Vec<&str>>()
        .join("\n");

    truncated_info
}