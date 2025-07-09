//! This module fetches Host Information

use std::env;

use ratatui::prelude::Constraint;
use ratatui::widgets::{Cell, Row, Table};

/// Returns the system name of the Linux Distribution,
/// kernel version of the Linux Distribution, the OS Version the Linux is currently on and
/// the hostname of the system formatted as a string

pub fn host_info_table() -> Table<'static> {
    // let mut sys = System::new_all();
    // sys.refresh_all();

    let rows = vec![
        Row::new(vec![
            Cell::from("System Name"),
            Cell::from(sysinfo::System::name().unwrap_or_else(|| "Unknown System".to_string())),
        ]),
        Row::new(vec![
            Cell::from("Kernel Version"),
            Cell::from(
                sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown Kernel".to_string()),
            ),
        ]),
        Row::new(vec![
            Cell::from("OS Version"),
            Cell::from(sysinfo::System::os_version().unwrap_or_else(|| "Unknown OS".to_string())),
        ]),
        Row::new(vec![
            Cell::from("Host Name"),
            Cell::from(sysinfo::System::host_name().unwrap_or_else(|| "Unknown Host".to_string())),
        ]),
    ];
    let widths = [Constraint::Length(20), Constraint::Length(30)];
    Table::new(rows, widths)
        .column_spacing(1)
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
}

pub fn get_current_user() -> String {
    env::var("USER").unwrap_or_else(|_| "Unknown User".to_string())
}
