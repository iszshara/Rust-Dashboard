use ratatui::layout::{Constraint, Direction, Layout, Rect};

//use crate::main;

pub fn terminal_layout(area: Rect) -> Vec<Rect> {
    // Hautplayout was den Bereich in zwei Bereiche teilt
    let main_chunks = Layout::default()
        .direction(Direction::Vertical) // Zuerst vertical teilen
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),       // Gauge Bar oben
                Constraint::Percentage(100), // Rest darunter
            ]
            .as_ref(),
        )
        .split(area);

    // Horizontale Aufteilung des unteren Bereichs
    let lower_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50), // Linker Bereich
                Constraint::Percentage(50), // Rechter Bereich
            ]
            .as_ref(),
        )
        .split(main_chunks[1]);

    // Linke Seite aufteilen
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(30), // CPU Bereich
                Constraint::Percentage(70), // Network
            ]
            .as_ref(),
        )
        .split(lower_chunks[0]);

    // Rechte Seite aufteilen
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(lower_chunks[1]);

    vec![
        main_chunks[0],  // Gauge Bar
        left_chunks[0],  // CPU Bereich
        left_chunks[1],  // Network Bereich
        right_chunks[0], // Memory Bereich
        right_chunks[1], // Prozesse Bereich
    ]
}
