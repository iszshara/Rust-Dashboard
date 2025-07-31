//! This module fetches memory resource information

use super::converter::byte_to_gib;
use ratatui::prelude::Constraint;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Cell;
use ratatui::widgets::{Row, Table};
use sysinfo::System;

/// Returns:  
/// -> Total Memory...  
/// -> Used Memory...  
/// -> Total Swap Memory...  
/// -> Used Swap Memory...  
/// information in a table format.  
/// # Example
/// ```
/// use linux_dashboard::backend::memory::ram_info_table;
/// use sysinfo::System;
/// let sys = System::new_all();
/// let output = ram_info_table(&sys);
/// assert!(output.contains("Total Memory"));
/// ```
///
pub fn ram_info_table(sys: &System) -> Table<'static> {
    let rows = vec![
        Row::new(vec![
            Cell::from("Total"),
            Cell::from("Memory"),
            Cell::from(format!("{:.2} GB", byte_to_gib(sys.total_memory()))),
        ]),
        Row::new(vec![
            Cell::from("Used"),
            Cell::from("Memory"),
            Cell::from(format!("{:.2} GB", byte_to_gib(sys.used_memory()))),
        ]),
        Row::new(vec![
            Cell::from("Total"),
            Cell::from("Swap"),
            Cell::from(format!("{:.2} GB", byte_to_gib(sys.total_swap()))),
        ]),
        Row::new(vec![
            Cell::from("Used"),
            Cell::from("Swap"),
            Cell::from(format!("{:.2} GB", byte_to_gib(sys.used_swap()))),
        ]),
    ];
    let widths = [
        Constraint::Length(5),
        Constraint::Length(6),
        Constraint::Length(10),
        Constraint::Length(10),
    ];
    Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::default().fg(Color::White))
}
