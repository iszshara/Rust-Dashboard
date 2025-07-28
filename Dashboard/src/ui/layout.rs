//! This module defines the layout for the terminal UI of the Dashboard.
use ratatui::layout::{Constraint, Direction, Layout, Rect};
/// This function is responsible for creating the layout of the terminal UI.  
/// It divides the terminal into a top section for the Gauge Bar and a bottom section that is  
/// further divided into left and right parts.  
/// The left part contains sections for CPU, Network, and Network Diagram,  
/// while the right part contains sections for Memory, Processes, and System Info.  
/// It returns a vector of Rects representing the layout of the terminal.
pub fn terminal_layout(area: Rect) -> Vec<Rect> {
    // Main Layout for the terminal
    // It divides the terminal into two main parts:
    // 1. A top section for the Gauge Bar
    // 2. A bottom section that is further divided into left and right parts
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

    // Horizontal layout for the lower part of the terminal
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

    // Divide the left side into three areas
    // The first area is for CPU, the second for Network, and the third for Network
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

    // Divide the right side into two areas
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(lower_chunks[1]);

    // Divide the right side further into two areas
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
