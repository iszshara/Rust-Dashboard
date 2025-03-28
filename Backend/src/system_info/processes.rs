use sysinfo::System;
use super::converter::byte_to_gib;

/// Returns the Linux processes id, name, status, cpu usage and memory usage formatted as a string

pub fn format_processes_id(sys: &System) -> String{
    sys.processes()
        .iter()
        .map(|(pid, process)| {
            format!(
                "[{pid}] Name: {:<20} | Status: {:<10} | CPU Usage: {:>5.2}% | Memory Usage: {:>6.2} GB",
                process.name().to_string_lossy(),
                format!("{:?}",process.status()),
                process.cpu_usage(),
                byte_to_gib(process.memory())
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}