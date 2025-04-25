//! This module fecthes CPU resource informations

use sysinfo::System;

/// Returns the full CPU-utilization of all cores formatted as string
/// 
/// #Example
/// ```
/// let output = format_cpu_usage(&sys);
/// println!("{}", output);
/// ```

pub fn format_cpu_usage(sys: &System) -> String{
    println!("Core Usage: ");
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

pub fn format_total_cpu_usage(sys: &System) -> String {
    let total_cpu_usage = format!("Total CPU Usage: {:.2}% ", sys.global_cpu_usage());
    //println!("{}", total_cpu_usage);
    total_cpu_usage
}

/// Returns the number of available CPUs
/// 
/// #Example
/// 
/// ```
/// let output = format_number_of_cpu(&sys);
/// println!("{}", output);
/// ```

pub fn format_number_of_cpu(sys: &System) -> String{
    let number_of_cpus = format!("Number of CPUs: {}", sys.cpus().len().to_string());
    println!("{}", number_of_cpus);
    number_of_cpus
}

// Notiz: Es gibt noch Vendor ID

pub fn format_cpu_name(sys: &System) -> String {
    let cpu_name = sys.cpus()
        .iter()
        .map(|cpu| cpu.brand().to_string())
        .collect::<Vec<_>>()
        .join(", ")
        ;
    cpu_name
}