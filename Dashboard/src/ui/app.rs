use crate::backend::processes;
use crate::backend::processes::SortOrder;
use crate::{
    app::event::KeyEvent,
    backend::{
        cpu::{format_cpu_name, format_cpu_usage},
        host::get_current_user,
        memory::ram_info_table,
        network::NetworkManager,
        //processes::format_processes_id
    },
    ui::layout::{self},
};
use chrono::Local;
use color_eyre::Result;
use ratatui::text::{Line, Span};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode},
    layout::Alignment,
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Gauge, Paragraph, Wrap},
};
use std::{time::Duration, time::Instant};
use sysinfo::System;

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
    let mut sort_order = SortOrder::default();

    loop {
        if last_tick.elapsed() >= tick_rate {
            sys.refresh_all();
            last_tick = Instant::now();
        }

        terminal.draw(|frame| {
            render(
                frame,
                sys,
                &mut show_popup,
                &mut network_manager,
                &mut sort_order,
            )
        })?;

        // Ein Event lesen und dann abarbeiten
        if event::poll(Duration::from_millis(500))? {
            let evt = event::read()?;
            match evt {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => break Ok(()),
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    show_popup = false;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('i'),
                    ..
                }) => {
                    let interfaces: Vec<_> = network_manager.network_history_keys();
                    if !interfaces.is_empty() {
                        let current_index = interfaces
                            .iter()
                            .position(|x| x == network_manager.get_selected_interface())
                            .unwrap_or(0);
                        let next_index = (current_index + 1) % interfaces.len();
                        network_manager.set_selected_interface(interfaces[next_index].clone());
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'), // 'c' for CPU
                    ..
                }) => {
                    sort_order = match sort_order {
                        SortOrder::CpuAsc => SortOrder::CpuDesc,
                        _ => SortOrder::CpuAsc, // Default to ascending if not currently CPU sorted
                    };
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('m'), // 'm' for Memory
                    ..
                }) => {
                    sort_order = match sort_order {
                        SortOrder::MemoryAsc => SortOrder::MemoryDesc,
                        _ => SortOrder::MemoryAsc,
                    };
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('p'), // 'p' for PID
                    ..
                }) => {
                    sort_order = match sort_order {
                        SortOrder::PidAsc => SortOrder::PidDesc,
                        _ => SortOrder::PidAsc,
                    };
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('n'), // 's' for Name
                    ..
                }) => {
                    sort_order = match sort_order {
                        SortOrder::NameAsc => SortOrder::NameDesc,
                        _ => SortOrder::NameAsc,
                    };
                }
                _ => {}
            }
        }
    }
}

/// render() zeichnet den Rahmen der TUI App und erstellt verschiedene Objekte wie zB Paragraphen, Blöcke, etc.
fn render(
    frame: &mut Frame,
    sys: &mut System,
    show_popup: &mut bool,
    network_manager: &mut NetworkManager,
    sort_order: &mut SortOrder,
) {
    // Gesamten Bereich des Terminals abrufen
    let area = frame.area();

    let current_time = Local::now().format("%H:%M:%S").to_string();
    // Äußeren Rahmen erstellen
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        // Links oben: System Monitor
        .title(Span::styled("System Monitor", Style::default()))
        .title_alignment(Alignment::Left)
        // Mitte oben: Aktuelle Zeit
        .title_top(
            Line::from(vec![
                Span::styled("".to_string(), Style::default()), // Leerer Span für Links
                Span::styled(current_time, Style::default()),   // Zeit in der Mitte
                Span::styled("".to_string(), Style::default()), // Leerer Span für Rechts
            ])
            .centered(),
        )
        .title_bottom(
            Line::from(vec![Span::styled(
                format!("User: {}", get_current_user()),
                Style::default(),
            )])
            .centered(),
        )
        .title_bottom(
            Line::from(vec![Span::styled(
                format!("{}", system_uptime()),
                Style::default(),
            )])
            .right_aligned(),
        );

    // Äußeren Rahmen rendern
    frame.render_widget(outer_block, area);

    // Inneres Layout erstellen (innerhalb des äußeren Rahmens)
    let inner_area = area.inner(Margin {
        vertical: 1,
        horizontal: 1,
    }); // Platz für den Rahmen lassen
    let chunks = layout::terminal_layout(inner_area);

    // CPU Usage
    //let cpu_usage = format_total_cpu_usage(sys);
    let cpu_core_usage = format_cpu_usage(sys);
    //let combined_cpu_information = format!("{}\n{}", cpu_usage, cpu_core_usage);

    let cpu_block = Block::default()
        .title("CPU Core Usage")
        .borders(Borders::ALL);
    let cpu_widget = Paragraph::new(cpu_core_usage).block(cpu_block);
    frame.render_widget(cpu_widget, chunks[1]);

    // CPU Gauge
    let cpu_usage = sys.global_cpu_usage();

    let cpu_gauge = Gauge::default()
        .block(
            Block::default()
                .title(format_cpu_name(sys))
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::LightBlue).bg(Color::Gray))
        .percent(cpu_usage as u16);
    frame.render_widget(cpu_gauge, chunks[0]);

    // Memory Block
    let memory_block = Block::default().title("Memory Usage").borders(Borders::ALL);
    let memory_table = ram_info_table(sys).block(memory_block);
    frame.render_widget(memory_table, chunks[3]);

    // Network Block
    let network_block = Block::default().title("Network").borders(Borders::ALL);
    let network_info = network_manager.format_network(sys);
    let network_widget = Paragraph::new(network_info)
        .block(network_block)
        .wrap(Wrap { trim: true });
    frame.render_widget(network_widget.clone(), chunks[2]);

    // Processes Block
    let processes_block = Block::default()
        .title("Processes")
        .title_bottom(
            Line::from(vec![
                Span::styled("C", Style::default().fg(Color::Yellow)),
                Span::raw("PU───"),
                Span::styled("M", Style::default().fg(Color::Yellow)),
                Span::raw("emory───"),
                Span::styled("P", Style::default().fg(Color::Yellow)),
                Span::raw("ID───"),
                Span::styled("N", Style::default().fg(Color::Yellow)),
                Span::raw("ame───"),
            ])
            .left_aligned(),
        )
        .borders(Borders::ALL);

    let processes_table = processes::create_process_table(sys, *sort_order);
    frame.render_widget(processes_table.block(processes_block), chunks[4]); // oder welcher chunk auch immer für Prozesse verwendet wird

    // Network Diagram Block
    let network_diagram = network_manager.get_network_widget();
    frame.render_widget(network_diagram, chunks[5]);

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
                .wrap(Wrap { trim: true })
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

        let username = format!("Current User: {}", get_current_user());
        let mut content = String::new();

        content.push_str(&format!(
            "{:-^width$}\n",
            username,
            width = popup_width as usize - 2
        ));

        let empty_lines = popup_height as usize - 4;
        for _ in 0..empty_lines {
            content.push('\n');
        }

        let popup_block = Block::default()
            //.title("Welcome to the Rust Dashboard")
            .title_top(
                Line::from(vec![
                    Span::styled("".to_string(), Style::default()), // Leerer Span für Links
                    Span::styled("Welcome to Luis Dashboard", Style::default()), // Zeit in der Mitte
                    Span::styled("".to_string(), Style::default()), // Leerer Span für Rechts
                ])
                .centered(),
            )
            .title_bottom(
                Line::from(vec![
                    Span::styled("".to_string(), Style::default()), // Leerer Span für Links
                    Span::styled("Press Enter to close", Style::default()), // Zeit in der Mitte
                    Span::styled("".to_string(), Style::default()), // Leerer Span für Rechts
                ])
                .centered(),
            )
            .title_alignment(Alignment::Right)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::LightBlue));

        let popup_paragraph = Paragraph::new(content)
            .block(popup_block)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White));

        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_paragraph, popup_area);
    }
}

fn system_uptime() -> String {
    let uptime = System::uptime();
    if uptime < 60 {
        return format!("Uptime: {} seconds", uptime);
    } else if uptime < 3600 {
        let minutes = uptime / 60;
        return format!("Uptime: {} minutes", minutes);
    } else if uptime < 86400 {
        let hours = uptime / 3600;
        return format!("Uptime: {} hours", hours);
    } else {
        let days = uptime / 86400;
        return format!("Uptime: {} days", days);
    }
}
