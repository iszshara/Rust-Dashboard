//! This module fetches memory resource information

use sysinfo::System;
use super::converter::byte_to_gib;

/// Returns the total and used memory as well as the total and used swap memory formatted as string

pub fn format_ram_info(sys: &System) -> (String, String, String, String){
    let ram_total_memory = format!("Total memory: {:.2} GB", byte_to_gib(sys.total_memory()));
    let ram_used_memory = format!("Used Memory: {:.2} GB", byte_to_gib(sys.used_memory()));
    let ram_total_swap = format!("Total Swap: {:.2} GB", byte_to_gib(sys.total_swap()));
    let ram_used_swap = format!("Used Swap: {:.2} GB", byte_to_gib(sys.used_swap()));

    println!("{}, {}, {}, {}", ram_total_memory, ram_used_memory, ram_total_swap, ram_used_swap);

    (ram_total_memory, ram_used_memory, ram_total_swap, ram_used_swap)
}