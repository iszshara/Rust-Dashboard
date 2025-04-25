//! This module fetches process information

use sysinfo::System;
use super::converter::byte_to_gib;

/// Returns the Linux processes id, name, status, cpu usage and memory usage formatted as a string

pub fn format_processes_id(sys: &System) -> String {
    let header = format!(
        "{:<8} {:<40} {:<10} {:>10} {:>15}",
        "PID", "Name", "Status", "CPU (%)", "Memory (GB)"
    );

    let processes = sys.processes()
        .iter()
        .map(|(pid, process)| {
            format!(
                "{:<8} {:<20} {:<10} {:>10.2} {:>15.2}",
                pid, // PID mit fester Breite von 8 Zeichen
                process.name().to_string_lossy(), // Name mit fester Breite von 20 Zeichen
                format!("{:?}", process.status()), // Status mit fester Breite von 10 Zeichen
                process.cpu_usage(), // CPU-Auslastung mit 2 Dezimalstellen
                byte_to_gib(process.memory()) // Speicher in GB mit 2 Dezimalstellen
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n{}", header, processes)
}