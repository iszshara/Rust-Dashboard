//! This module fecthes CPU resource informations

use crate::backend::system_info::SystemInfo;

/// Returns the full CPU-utilization of all cores formatted as string
/// Shows CPU Core usage.
///
/// #Example
/// ```
/// use linux_dashboard::backend::cpu::format_cpu_usage;
/// use sysinfo::System;
/// let sys = System::new_all();
/// let output = format_cpu_usage(&sys);
/// assert!(output.contains("CPU"));
/// ```

pub fn format_cpu_usage(sys: &impl SystemInfo) -> String {
    sys.get_cpus()
        .iter()
        .enumerate()
        .map(|(i, cpu)| format!("CPU {:02}: {:>5.2}%\n", i, cpu.usage))
        .collect::<String>()
}

/// Returns the global CPU usage
///
/// #Example
///
/// ```
/// use linux_dashboard::backend::cpu::format_total_cpu_usage;
/// use sysinfo::System;
/// let sys = System::new_all();
/// let output = format_total_cpu_usage(&sys);
/// assert!(output.contains("Total Usage"));
/// ```

/// This function returns the total CPU usage of the system as a formatted string
pub fn format_total_cpu_usage(sys: &impl SystemInfo) -> String {
    let total_cpu_usage = format!("Total Usage: {:.2}% ", sys.global_cpu_usage());
    total_cpu_usage
}

/// nimmt nur den ersten CPU
/// holt danach den Namen des ersten CPUs
/// + Fallback, falls keine CPUs vorhanden sind
pub fn format_cpu_name(sys: &impl SystemInfo) -> String {
    let cpu_name = sys
        .get_cpus()
        .get(0)
        .map(|cpu| cpu.brand.clone())
        .unwrap_or_else(|| "Unknown CPU".to_string());
    cpu_name
}
