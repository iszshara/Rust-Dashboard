//! This module fetches process information

use sysinfo::System;
use super::converter::byte_to_gib;

/// Returns the Linux processes id, name, status, cpu usage and memory usage formatted as a string

pub fn format_processes_id(sys: &System) -> String {
    let header = format!(
        "{:<8} {:<15} {:<5} {:>10} {:>15}",
        "PID", "Name", "Status", "CPU (%)", "Memory (GB)"
    );

    let processes = sys.processes()
        .iter()
        .map(|(pid, process)| {
            format!(
                "{:<8} {:<15} {:<5} {:>10.2} {:>15.2}",
                pid, // PID mit fester Breite von 8 Zeichen
                truncate_string(&process.name().to_string_lossy(), 30), // Name mit fester Breite von 30 Zeichen
                format!("{:?}", process.status()), // Status mit fester Breite von 15 Zeichen
                process.cpu_usage(), // CPU-Auslastung mit 2 Dezimalstellen
                byte_to_gib(process.memory()) // Speicher in GB mit 2 Dezimalstellen
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n{}", header, processes)
}

/// Truncates a string to a specified length, adding "..." if it exceeds the limit
fn truncate_string(input: &str, max_len: usize) -> String {
    if input.len() > max_len {
        format!("{}...", &input[..max_len - 3])
    } else {
        input.to_string()
    }
}