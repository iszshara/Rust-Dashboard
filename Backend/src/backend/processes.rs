use crate::backend::converter::byte_to_gib;
use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Cell, Row, Table},
};
use sysinfo::System;

fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}

pub fn create_process_table(sys: &System) -> Table<'static> {
    // Header-Zeile erstellen
    let header = Row::new(vec![
        Cell::from("PID"),
        Cell::from("Name"),
        Cell::from("Status"),
        Cell::from("CPU (%)"),
        Cell::from("Memory (GB)"),
    ])
    .style(Style::default().fg(Color::Yellow));

    // Prozess-Daten in Tabellenzeilen umwandeln
    let mut rows = vec![header]; // Füge Header als erste Zeile ein
    rows.extend(sys.processes().iter().map(|(pid, process)| {
        Row::new(vec![
            Cell::from(pid.to_string()),
            Cell::from(truncate_string(&process.name().to_string_lossy(), 30)),
            Cell::from(format!("{:?}", process.status())),
            Cell::from(format!("{:.2}", process.cpu_usage())),
            Cell::from(format!("{:.2}", byte_to_gib(process.memory()))),
        ])
    }));
    let widths = [
        Constraint::Length(8),  // PID
        Constraint::Length(30), // Name
        Constraint::Length(10), // Status
        Constraint::Length(10), // CPU
        Constraint::Length(12), // Memory
    ];
    Table::new(rows, widths)
        .widths(&[
            Constraint::Length(8),  // PID
            Constraint::Length(30), // Name
            Constraint::Length(10), // Status
            Constraint::Length(10), // CPU
            Constraint::Length(12), // Memory
        ])
        .column_spacing(1)
        .style(Style::default().fg(Color::White))
}
