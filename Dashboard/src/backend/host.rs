//! This module fetches Host Information

use std::env;

use ratatui::prelude::Constraint;
use ratatui::widgets::{Cell, Row, Table};

/// Returns the system name of the Linux Distribution, kernel version of the Linux Distribution,  
/// the OS Version Linux is currently on and the hostname of the system formatted as a string.  
/// This function also returns a table with:  
/// ->system name  
/// -> kernel version  
/// -> OS version  
/// -> host name  
/// of the Linux system.  
/// It uses the sysinfo crate to fetch the system information and formats it into a table.  
/// The table has two columns: one for the label and one for the value.  
///
/// # Example
/// ```
/// use linux_dashboard::backend::host::host_info_table;
/// let output = host_info_table();
/// assert!(output.contains("System Name"));
/// ```
/// Cached host information that doesn't change at runtime.
pub struct HostInfo {
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
}

impl HostInfo {
    pub fn new() -> Self {
        Self {
            system_name: sysinfo::System::name().unwrap_or_else(|| "Unknown System".to_string()),
            kernel_version: sysinfo::System::kernel_version()
                .unwrap_or_else(|| "Unknown Kernel".to_string()),
            os_version: sysinfo::System::os_version()
                .unwrap_or_else(|| "Unknown OS".to_string()),
            host_name: sysinfo::System::host_name()
                .unwrap_or_else(|| "Unknown Host".to_string()),
        }
    }

    pub fn to_table(&self) -> Table<'static> {
        let rows = vec![
            Row::new(vec![
                Cell::from("System Name"),
                Cell::from(self.system_name.clone()),
            ]),
            Row::new(vec![
                Cell::from("Kernel Version"),
                Cell::from(self.kernel_version.clone()),
            ]),
            Row::new(vec![
                Cell::from("OS Version"),
                Cell::from(self.os_version.clone()),
            ]),
            Row::new(vec![
                Cell::from("Host Name"),
                Cell::from(self.host_name.clone()),
            ]),
        ];
        let widths = [Constraint::Length(20), Constraint::Length(30)];
        Table::new(rows, widths)
            .column_spacing(1)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
    }
}

pub fn host_info_table() -> Table<'static> {
    HostInfo::new().to_table()
}

/// Returns the current user of the system
pub fn get_current_user() -> String {
    env::var("USER").unwrap_or_else(|_| "Unknown User".to_string())
}
