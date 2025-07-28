//! This module provides a table of processes with relevant information.  
//! This module fetches and formats process information from the system,  
//! including PID, name, status, CPU usage, and memory usage.  
//! It allows sorting of processes based on various criteria such as CPU usage, memory usage, PID, and name.  
use crate::backend::converter::byte_to_gib;
use ratatui::{
    style::{Color, Style},
    widgets::{Cell, Row},
};
use sysinfo::{Pid, Process, System};

/// Enum for the sort order of processes  
/// This enum defines different sorting criteria for the process list,  
/// such as CPU usage, memory consumption, PID, and name.  
/// Each variant represents a specific sort order.  
/// The implementation of Default allows defining a default sort order,  
/// which is used when no specific sort order is specified.  

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SortOrder {
    CpuDesc,
    CpuAsc,
    MemoryAsc,
    MemoryDesc,
    PidAsc,
    PidDesc,
    NameAsc,
    NameDesc,
}
/// Manually implements Default for SortOrder.  
/// This makes it possible to define a default sort order,  
/// which is used when no specific sort order is provided.  
///
impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::CpuDesc // Standard-Sortierung nach CPU-Auslastung absteigend
    }
}
/// s is short for string slice (the result is better performance bc, the Compiler is told to not look at the whole string, but only the relevant part).
/// It then gets converted into a String to return a new owned String.
/// Short: Truncates a string to a maximum length and appends "..." if it exceeds that length.
fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}

/// This function creates a vector of rows representing the processes in the system.
/// It sorts the processes based on the specified `SortOrder` and formats them into rows for display.
/// # Arguments
/// * `sys` - A reference to the `System` instance containing process information.
/// * `sort_order` - The `SortOrder` enum that specifies how to sort the processes.
/// # Returns
/// A vector of `Row` instances, each representing a process with its PID, name, status, CPU usage, and memory usage.
/// The rows are styled with a default style and the header row is styled with a yellow foreground color.
pub fn create_process_rows(sys: &System, sort_order: SortOrder) -> Vec<Row<'static>> {
    let mut processes: Vec<(&Pid, &Process)> = sys.processes().iter().collect();
    match sort_order {
        SortOrder::CpuAsc => {
            processes.sort_by(|a, b| {
                a.1.cpu_usage()
                    .partial_cmp(&b.1.cpu_usage())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        SortOrder::CpuDesc => {
            processes.sort_by(|a, b| {
                a.1.cpu_usage()
                    .partial_cmp(&b.1.cpu_usage())
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .reverse()
            });
        }
        SortOrder::MemoryAsc => {
            processes.sort_by_key(|a| a.1.memory());
        }
        SortOrder::MemoryDesc => {
            processes.sort_by_key(|a| std::cmp::Reverse(a.1.memory()));
        }
        // Add PidAsc, PidDesc, NameAsc, NameDesc similarly
        SortOrder::PidAsc => {
            processes.sort_by_key(|a| a.0); // a.0 is the Pid
        }
        SortOrder::PidDesc => {
            processes.sort_by_key(|a| std::cmp::Reverse(a.0));
        }
        SortOrder::NameAsc => {
            processes.sort_by_key(|a| a.1.name().to_ascii_lowercase()); // a.1.name() is the process name
        }
        SortOrder::NameDesc => {
            processes.sort_by_key(|a| std::cmp::Reverse(a.1.name().to_ascii_lowercase()));
        }
    }

    // create header row
    let header = Row::new(vec![
        Cell::from("PID"),
        Cell::from("Name"),
        Cell::from("Status"),
        Cell::from("CPU (%)"),
        Cell::from("Memory"),
    ])
    .style(Style::default().fg(Color::Yellow));

    // converts the process data into rows for the table
    // Each row contains the PID, truncated name, status, CPU usage, and memory usage.
    let mut rows = vec![header]; // adds header in the first row
    rows.extend(processes.iter().map(|(pid, process)| {
        Row::new(vec![
            Cell::from(pid.to_string()),
            Cell::from(truncate_string(&process.name().to_string_lossy(), 30)),
            Cell::from(format!("{:?}", process.status())),
            Cell::from(format!("{:.2}", process.cpu_usage())),
            Cell::from(format!("{:.2} GB", byte_to_gib(process.memory()))),
        ])
    }));
    rows
}
