//! This module fetches memory resource information

use super::converter::byte_to_gib;
use ratatui::prelude::Constraint;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Cell;
use ratatui::widgets::{Row, Table};
use sysinfo::System;

/// Returns total / used memory and swap / used swap information in a table format
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
    ];
    Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::default().fg(Color::White))
}
