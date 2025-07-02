//! This module fecthes CPU resource informations

use sysinfo::System;

/// Returns the full CPU-utilization of all cores formatted as string
///
/// #Example
/// ```
/// let output = format_cpu_usage(&sys);
/// println!("{}", output);
/// ```

pub fn format_cpu_usage(sys: &System) -> String {
    sys.cpus()
        .iter()
        .enumerate()
        .map(|(i, cpu)| format!("CPU {:02}: {:>5.2}%\n", i, cpu.cpu_usage()))
        .collect::<String>()
}

/// Returns the global CPU usage
///
/// #Example
///
/// ```
/// let output = format_total_cpu_usage(&sys);
/// println!("{}", output);
/// ```

/// This function returns the total CPU usage of the system as a formatted string
pub fn format_total_cpu_usage(sys: &System) -> String {
    let total_cpu_usage = format!("Total Usage: {:.2}% ", sys.global_cpu_usage());
    total_cpu_usage
}

// Notiz: Es gibt noch Vendor ID
/// nimmt nur den ersten CPU
/// holt danach den Namen des ersten CPUs
/// + Fallback, falls keine CPUs vorhanden sind
pub fn format_cpu_name(sys: &System) -> String {
    let cpu_name = sys
        .cpus()
        .get(0)
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());
    cpu_name
}
