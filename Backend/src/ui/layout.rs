use ratatui::layout::{Constraint, Direction, Layout, Rect};

//use crate::main;


pub fn terminal_layout(area: Rect) -> Vec<Rect> {
    // Hautplayout was den Bereich in zwei Bereiche teilt
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(70), //Linker Bereich
                Constraint::Percentage(30), //Rechter Bereich
            ]
            .as_ref(),
        )
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(60), //Oberer Bereich
                Constraint::Percentage(40), //Unterer Bereich
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Oberer Bereich: Fester Platz für zB Überschrift
                Constraint::Min(0), // Unterer Bereich: für den Rest wie zB Prozesse
            ]
            .as_ref(),
        )
        .split(main_chunks[1]);

    vec![
        left_chunks[0], // CPU Bereich
        left_chunks[1], // Memory Bereich
        right_chunks[0], // 
        right_chunks[1],
    ]


}