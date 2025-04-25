use ratatui::layout::{Constraint, Direction, Layout, Rect};

//use crate::main;


pub fn terminal_layout(area: Rect) -> Vec<Rect> {
    // Hautplayout was den Bereich in zwei Bereiche teilt
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50), //Linker Bereich
                Constraint::Percentage(50), //Rechter Bereich
            ]
            .as_ref(),
        )
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50), //Oberer Bereich
                Constraint::Percentage(50), //Unterer Bereich
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);

    let cpu_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                // Constraint::Length(3),
                // Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Percentage(50),
                //Constraint::Percentage(50),
            ]
            .as_ref()
        )
        .split(left_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(30), // Oberer Bereich: Fester Platz für zB Überschrift
                Constraint::Percentage(70), // Unterer Bereich: für den Rest wie zB Prozesse
            ]
            .as_ref(),
        )
        .split(main_chunks[1]);

    vec![
        cpu_chunks[0], // CPU Gauge Bar
        cpu_chunks[1], // CPU Bereich
        left_chunks[1], // Memory Bereich
        right_chunks[0], // 
        right_chunks[1],
        
    ]


}