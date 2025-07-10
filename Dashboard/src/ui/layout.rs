//! This module defines the layout for the terminal UI of the Dashboard.
/// The function 'terminal_layout', divides the terminal area into several sections called chunks.
use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn terminal_layout(area: Rect) -> Vec<Rect> {
    // Hautplayout was den Bereich in zwei Bereiche teilt
    let main_chunks = Layout::default()
        .direction(Direction::Vertical) // Zuerst vertical teilen
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),       // Gauge Bar top
                Constraint::Percentage(100), // everything else below
            ]
            .as_ref(),
        )
        .split(area);

    // Horizontale Aufteilung des unteren Bereichs
    let lower_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50), // left area
                Constraint::Percentage(50), // right area
            ]
            .as_ref(),
        )
        .split(main_chunks[1]);

    // Linke Seite aufteilen
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(30), // CPU area
                Constraint::Percentage(20), // Network area
                Constraint::Percentage(50), // Network diagram
            ]
            .as_ref(),
        )
        .split(lower_chunks[0]);

    // Rechte Seite aufteilen
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(lower_chunks[1]);

    let right_divided_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50), // Memory area
                Constraint::Percentage(50), // Process area
            ]
            .as_ref(),
        )
        .split(right_chunks[0]);

    vec![
        main_chunks[0],          // Gauge Bar
        left_chunks[0],          // CPU area
        left_chunks[1],          // Network area
        right_divided_chunks[0], // Memory area
        right_chunks[1],         // Prozesse area
        left_chunks[2],          // Network Diagram
        right_divided_chunks[1], // System Info area
    ]
}
