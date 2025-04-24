use color_eyre::{owo_colors::OwoColorize, Result};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap, Clear},
    style::{Style, Stylize},
    DefaultTerminal, Frame,
    backend::CrosstermBackend,
    buffer::Buffer,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    }, 
};
use sysinfo::System;
use std::{time::Instant, time::Duration};
use crate::{
    app::event::KeyEvent, backend::{
        cpu::{format_cpu_usage, format_total_cpu_usage}, host::{format_username, get_current_user}, memory::format_ram_info, network::{self, format_network}, processes::format_processes_id
    }, ui::layout::{self, terminal_layout}
};
//use derive_setters::Setters; // Ensure the derive macro is in scope
use lipsum::lipsum;
use std::env;


/// run_ui() ist der Einstiegspunkt für die UI des Terminals.
/// Sie initialisiert die notwendigen Komponenten und startet die Hauptschleife,
/// die das Terminal-UI rendert und aktualisiert
/// 
/// color_eyre wird aufgerufen, um die Fehlerberichtserstattung zu konfigurieren
/// 
/// ratatui::init() wird verwendet, um das Terminal für die Anzeige vorzubereiten. 
/// 
/// let mut sys = System::new_all();
/// sys.refresh_all();
/// erstellt ein neues Systemobjekt, welches Daten über das System sammelt
/// 
/// let result = run(...) startet die Hauptschleife.
/// Diese rendert die UI und aktualisiert die Systeminformationen in regelmäßigen Abständen
/// Sie nimmt das Terminal und die Systeminformationen als Parameter entgegen
    
pub fn run_ui() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let mut sys = System::new_all();
    sys.refresh_all();
    
    let result = run(terminal, &mut sys);
    ratatui::restore();
    result
}

/// Runtime function to render the Terminal and to refresh it 
/// setzt eine Anfangs- und Endzeitpunkts Tick Rate fest
/// das User Interface wird in einer Schleife dann in dem jeweiligen Intervall immer neu gerendert mit der Information zu zB CPU Auslastung
/// Das wird unterbrochen wenn es innerhalb von einer Zeitspanne von 50 ms ein "KeyEvent" gibt, bei dem definiert wurde das 'q' für das Beende der Schleife steht
fn run(mut terminal: DefaultTerminal, sys: &mut System) -> Result<()> {
    let tick_rate = Duration::from_millis(1000);
    let mut last_tick = Instant::now();
    let mut show_popup = true;

    loop {

        if last_tick.elapsed() >= tick_rate {
            sys.refresh_all();
            last_tick = Instant::now();
        }
        
        terminal.draw(|frame| render(frame, sys, &mut show_popup))?;
        
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = event::read()? {
                break Ok(());
            } else if let Event::Key(KeyEvent {code: KeyCode::Enter, ..}) = event::read()? {
                show_popup = false;
            }
        }
    }
}

/// render() zeichnet den Rahmen der TUI App und erstellt verschiedene Objekte wie zB Paragraphen, Blöcke, etc.
fn render(frame: &mut Frame, sys: &System, show_popup: &mut bool) {
    // Gesamten Bereich des Terminals abrufen
    let area = frame.area();

    // Äußeren Rahmen erstellen
    let outer_block = Block::default()
        .title("System Monitor") // Titel des Rahmens
        .borders(Borders::ALL) // Rahmen um den gesamten Bereich
        .border_type(BorderType::Plain); // Stil des Rahmens

    // Äußeren Rahmen rendern
    frame.render_widget(outer_block, area);

    // Inneres Layout erstellen (innerhalb des äußeren Rahmens)
    let inner_area = area.inner(Margin {
        vertical: 1,
        horizontal: 1,
    }); // Platz für den Rahmen lassen
    let chunks = layout::terminal_layout(inner_area);

    // CPU-Bereich
    let cpu_block = Block::default()
        .title("CPU Usage")
        .borders(Borders::ALL);
    let cpu_widget = Paragraph::new(format_total_cpu_usage(&sys))
        .block(cpu_block)
        .wrap(Wrap { trim: true });
    frame.render_widget(cpu_widget, chunks[0]);

    // Speicher-Bereich
    let memory_block = Block::default()
        .title("Memory Usage")
        .borders(Borders::ALL);
    let memory_widget = Paragraph::new(format_ram_info(&sys))
        .block(memory_block)
        .wrap(Wrap { trim: true });
    frame.render_widget(memory_widget, chunks[1]);


    // let memory_block = Block::default()
    //     .title("Memory Usage")
    //     .borders(Borders::ALL);
    // let memory_widget = Paragraph::new(format_ram_info(&sys))
    //     .block(memory_block)
    //     .wrap(Wrap { trim: true });
    // frame.render_widget(memory_widget, chunks[1]);

    // let network_block = Block::default()
    //     .title("Network")
    //     .borders(Borders::ALL);
    // let network_widget = Paragraph::new(format_network()).block(network_block);
    // frame.render_widget(network_widget, chunks[2]);

    // let processes_block = Block::default()
    //     .title("Processes")
    //     .borders(Borders::ALL);
    // let processes_widget = Paragraph::new(format_processes_id(&sys));
    // frame.render_widget(processes_widget, chunks[3]);

    struct Popup<'a> {
        title: Line<'a>,
        content: Text<'a>,
        border_style: Style,
        title_style: Style,
        style: Style,
    }

    impl<'a> Default for Popup<'a> {
        fn default() -> Self {
            Popup {
                title: Line::from(""),
                content: Text::from(""),
                border_style: Style::default(),
                title_style: Style::default(),
                style: Style::default(),
            }
        }
    }

    impl Widget for Popup<'_> {
        fn render(self, area: Rect, buf: &mut Buffer) {
            // sicherstellen dass die Zellen unter dem Popup gecleared werden um zu verhindern das der Content außerhalb der Box ist
            Clear.render(area, buf);
            let block = Block::new()
                .title(self.title)
                .title_style(self.title_style)
                .borders(Borders::ALL)
                .border_style(self.border_style);
            Paragraph::new(self.content)
                .wrap(Wrap {trim: true})
                .style(self.style)
                .block(block)
                .render(area, buf);
        }
    }

    if *show_popup {
        let popup_width = 50;
        let popup_height = 10;

        // Zentriertes Popup-Bereich berechnen
        let popup_area = Rect::new(
            (area.width.saturating_sub(popup_width)) / 2,
            (area.height.saturating_sub(popup_height)) / 2,
            popup_width,
            popup_height,
        );

        let ascii_art = load_ascii_art(
            "/home/luis/Rust-Dashboard/Backend/src/ui/ascii_art.txt"
        );
        
        fn load_ascii_art(file_path: &str) -> String {
            std::fs::read_to_string(file_path).unwrap_or_else(|_| "ASCII art not found".to_string())
        }

        let username = format!("Guten Moin {}", get_current_user());
        let mut content = String::new();

        for line in ascii_art.lines() {
            content.push_str(&format!("{:^width$}\n", line, width = popup_width as usize - 2));
        }

        // content.push_str(&ascii_art);
        // content.push('\n');

        // content.push_str(&username);

        content.push_str(&format!("{:^width$}\n", username, width = popup_width as usize - 2));

        let empty_lines = popup_height as usize - 2;
        for _ in 0..empty_lines{
            content.push('\n');
        }

        content.push_str(&format!("{:>width$}", "Double Enter", width = popup_width as usize - 4));
        

        let popup_block = Block::default()
            .title("Moin")
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .style(Style::default().fg(Color::Green));

        let popup_paragraph = Paragraph::new(content)
            .block(popup_block)
            .wrap(Wrap {trim: true })
            .style(Style::default().fg(Color::Magenta));

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_paragraph, popup_area);
    }
        
}