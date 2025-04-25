use color_eyre::{owo_colors::OwoColorize, Result};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap, Clear, Gauge},
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
        cpu::{format_cpu_name, format_cpu_usage, format_total_cpu_usage}, host::{format_username, get_current_user}, memory::format_ram_info, network::{self, NetworkManager}, processes::format_processes_id
    }, ui::layout::{self, terminal_layout}
};

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
    let mut network_manager = NetworkManager::new();

    loop {

        if last_tick.elapsed() >= tick_rate {
            sys.refresh_all();
            network_manager.format_network();
            last_tick = Instant::now();
        }
        
        terminal.draw(|frame| render(frame, sys, &mut show_popup, &mut network_manager))?;
        
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
fn render(frame: &mut Frame, sys: &System, show_popup: &mut bool, network_manager: &mut NetworkManager) {
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

    // CPU Usage
    let cpu_usage = format_total_cpu_usage(sys);
    let cpu_core_usage = format_cpu_usage(sys);
    let combined_cpu_information = format!("{}\n{}", cpu_usage, cpu_core_usage);
    // let cpu_widget = Paragraph::new(format!("{}", cpu_usage))
    //     .style(Style::default().fg(Color::Green))
    //     .alignment(Alignment::Left);
    

    let cpu_block = Block::default()
        .title("CPU Usage")
        .borders(Borders::ALL);
    let cpu_widget = Paragraph::new(combined_cpu_information)
        .block(cpu_block)
        .wrap(Wrap { trim: true });
    frame.render_widget(cpu_widget, chunks[1]);
    
    // CPU Gauge
    let cpu_usage_value = cpu_usage.trim_end_matches('%').parse().unwrap_or(0.0);
    let cpu_gauge = Gauge::default()
        .block(Block::default().title(format_cpu_name(sys)).borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::LightBlue).bg(Color::Gray))
        .percent(cpu_usage_value as u16);

    frame.render_widget(cpu_gauge, chunks[0]);

    // Memory Block
    let memory_block = Block::default()
        .title("Memory Usage")
        .borders(Borders::ALL);
    let memory_widget = Paragraph::new(format_ram_info(&sys))
        .block(memory_block)
        .wrap(Wrap { trim: true });
    frame.render_widget(memory_widget, chunks[3]);

    // Network Block
    let network_block = Block::default()
        .title("Network")
        .borders(Borders::ALL);
    let network_info = network_manager.format_network();
    let network_widget = Paragraph::new(network_info)
            .block(network_block)
            .wrap(Wrap { trim: true });
    frame.render_widget(network_widget.clone(), chunks[2]);

    let processes_block = Block::default()
        .title("Processes")
        .borders(Borders::ALL);
    let processes_widget = Paragraph::new(format_processes_id(&sys));
    frame.render_widget(processes_widget, chunks[4]);

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
        let popup_width = 35;
        let popup_height = 5;

        // Zentriertes Popup-Bereich berechnen
        let popup_area = Rect::new(
            (area.width.saturating_sub(popup_width)) / 2,
            (area.height.saturating_sub(popup_height)) / 2,
            popup_width,
            popup_height,
        );

        // let ascii_art = load_ascii_art(
        //     "/home/luis/Rust-Dashboard/Backend/src/ui/ascii_art.txt"
        // );
        
        // fn load_ascii_art(file_path: &str) -> String {
        //     std::fs::read_to_string(file_path).unwrap_or_else(|_| "ASCII art not found".to_string())
        // }

        // for line in ascii_art.lines() {
        //     content.push_str(&format!("{:^width$}\n", line, width = popup_width as usize - 2));
        // }

        let username = format!("Guten Moin {}", get_current_user());
        let mut content = String::new();

        // content.push_str(&ascii_art);
        // content.push('\n');

        // content.push_str(&username);

        content.push_str(&format!("{:^width$}\n", username, width = popup_width as usize - 2));

        let empty_lines = popup_height as usize - 4;
        for _ in 0..empty_lines{
            content.push('\n');
        }

        //content.push_str(&format!("{:>width$}", "Press Double Enter", width = popup_width as usize - 2));
        

        let popup_block = Block::default()
            .title("Welcome to the Rust Dashboard")
            //.title_alignment(Alignment::Left)
            .title_bottom("Press Enter to close")
            .title_alignment(Alignment::Right)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::LightBlue));

        let popup_paragraph = Paragraph::new(content)
            .block(popup_block)
            .wrap(Wrap {trim: true })
            .style(Style::default().fg(Color::White));

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_paragraph, popup_area);
    }
        
}