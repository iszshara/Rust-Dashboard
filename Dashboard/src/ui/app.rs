//! This module contains the main application logic for the Rust Dashboard UI.
/// It is responsible for running the terminal UI, managing user interactions, and updating the display based on system information.
/// It uses the `ratatui` crate for rendering the UI and `sysinfo` for fetching system data.
/// System data is fetched asynchronously in a background tokio task.
use crate::backend::host::host_info_table;
use crate::backend::processes::kill_process;
use crate::backend::processes::{SortOrder, create_process_rows};
use crate::backend::system_info::SystemInfo;
use crate::{
    backend::{
        cpu::{format_cpu_name, format_cpu_usage},
        host::get_current_user,
        memory::ram_info_table,
        network::NetworkManager,
    },
    ui::layout::{self},
};
use chrono::Local;
use color_eyre::Result;
use crossterm::event::KeyEventKind;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode};
use ratatui::layout::Constraint;
use ratatui::style::Color;
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, Clear, Gauge, Paragraph, Scrollbar, Table, Wrap,
};
use ratatui::widgets::{ScrollbarOrientation, ScrollbarState};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::Alignment,
    prelude::*,
    style::Style,
};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use sysinfo::{Signal, System};

const MIN_WIDTH: u16 = 110;
const MIN_HEIGHT: u16 = 24;

#[derive(PartialEq, Eq)]
enum ActiveBlock {
    Cpu,
    Processes,
}

#[derive(PartialEq)]
enum Mode {
    Normal,
    Input,
}

struct App {
    running: bool,
    active_block: ActiveBlock,
    cpu_scroll_state: ScrollbarState,
    process_scroll_state: ScrollbarState,
    cpu_scroll: usize,
    process_scroll: usize,
    pub current_fetch_interval: u64,
    pub minus_button_rect: Rect,
    pub plus_button_rect: Rect,
    mode: Mode,
    input: String,
    show_popup: bool,
    show_manual: bool,
    sort_order: SortOrder,
    network_manager: NetworkManager,
    process_status: Option<(String, Color)>,
    kill_message: Option<(String, Instant)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            active_block: ActiveBlock::Cpu,
            cpu_scroll_state: ScrollbarState::default(),
            process_scroll_state: ScrollbarState::default(),
            cpu_scroll: 0,
            process_scroll: 0,
            current_fetch_interval: 1000,
            minus_button_rect: Rect::default(),
            plus_button_rect: Rect::default(),
            mode: Mode::Normal,
            input: String::new(),
            show_popup: true,
            show_manual: false,
            sort_order: SortOrder::default(),
            network_manager: NetworkManager::default(),
            process_status: None,
            kill_message: None,
        }
    }
}

/// Entry point for the terminal UI.
/// Spawns a background tokio task that refreshes system data at the configured interval.
/// The UI thread reads from the shared state and handles user input without blocking on data fetching.
pub async fn run_ui(mut terminal: DefaultTerminal) -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;

    terminal.clear()?;

    let sys = Arc::new(Mutex::new(System::new_all()));
    {
        let mut s = sys.lock().unwrap();
        s.refresh_all();
    }

    // Channel to communicate the current fetch interval to the background task
    let (interval_tx, interval_rx) = tokio::sync::watch::channel(1000u64);

    // Background task: refreshes system data at the configured interval
    let sys_bg = Arc::clone(&sys);
    let bg_handle = tokio::spawn(async move {
        let mut rx = interval_rx;
        loop {
            let interval = *rx.borrow();
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_millis(interval)) => {
                    let mut s = sys_bg.lock().unwrap();
                    s.refresh_all();
                }
                result = rx.changed() => {
                    if result.is_err() {
                        // Sender dropped, exit
                        break;
                    }
                }
            }
        }
    });

    let mut app = App::default();
    let app_result = app.run(&mut terminal, &sys, &interval_tx);

    // Signal the background task to stop by dropping the sender
    drop(interval_tx);
    let _ = bg_handle.await;

    // Cleanup
    crossterm::execute!(
        io::stdout(),
        LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    disable_raw_mode()?;

    app_result
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        sys: &Arc<Mutex<System>>,
        interval_tx: &tokio::sync::watch::Sender<u64>,
    ) -> Result<()> {
        let mut last_tick = Instant::now();
        let mut needs_redraw = true;

        loop {
            if !self.running {
                return Ok(());
            }

            let tick_rate = Duration::from_millis(self.current_fetch_interval);
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(Duration::ZERO);

            if crossterm::event::poll(timeout)? {
                let evt = event::read()?;
                // Only handle and redraw for key events, ignore mouse events
                if matches!(&evt, Event::Key(_)) {
                    let mut s = sys.lock().unwrap();
                    self.handle_event(evt, &mut s)?;
                    let _ = interval_tx.send(self.current_fetch_interval);
                    needs_redraw = true;
                }
            }

            // Tick abgelaufen -> neue Daten verfuegbar, neu zeichnen
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
                needs_redraw = true;
            }

            if needs_redraw {
                needs_redraw = false;
                let mut s = sys.lock().unwrap();
                terminal.draw(|frame| {
                    let size = frame.area();
                    if size.width < MIN_WIDTH || size.height < MIN_HEIGHT {
                        self.render_size_error(frame, size);
                    } else {
                        self.render(frame, &mut s);
                    }
                })?;
            }
        }
    }

    pub fn handle_event(&mut self, evt: Event, sys: &mut System) -> Result<()> {
        if let Event::Key(KeyEvent { code, kind, .. }) = evt {
            if kind != KeyEventKind::Press {
                return Ok(());
            }

            // Input-Mode has priority
            if self.mode == Mode::Input {
                match code {
                    KeyCode::Char(c) if c.is_ascii_digit() => self.input.push(c),
                    KeyCode::Backspace => {
                        self.input.pop();
                    }
                    KeyCode::Enter => {
                        if let Ok(pid) = self.input.parse::<usize>() {
                            if let Some(msg) = kill_process(sys, pid, Signal::Kill) {
                                self.kill_message = Some((msg, Instant::now()));
                            }
                        }
                        self.mode = Mode::Normal;
                    }
                    KeyCode::Esc => self.mode = Mode::Normal,
                    _ => {}
                }
                return Ok(());
            }

            // Normal Mode
            match code {
                KeyCode::Char('q') => self.running = false,
                KeyCode::Enter => {
                    if self.show_popup {
                        self.show_popup = false;
                    }
                }
                KeyCode::Esc => {
                    if !self.show_popup {
                        self.show_manual = !self.show_manual;
                    }
                }
                KeyCode::Char('i') => {
                    let interfaces: Vec<_> = self.network_manager.network_history_keys();
                    if !interfaces.is_empty() {
                        let current_index = interfaces
                            .iter()
                            .position(|x| x == self.network_manager.get_selected_interface())
                            .unwrap_or(0);
                        let next_index = (current_index + 1) % interfaces.len();
                        self.network_manager
                            .set_selected_interface(interfaces[next_index].clone());
                    }
                }
                KeyCode::Char('c') => {
                    self.sort_order = match self.sort_order {
                        SortOrder::CpuAsc => SortOrder::CpuDesc,
                        _ => SortOrder::CpuAsc,
                    }
                }
                KeyCode::Char('m') => {
                    self.sort_order = match self.sort_order {
                        SortOrder::MemoryAsc => SortOrder::MemoryDesc,
                        _ => SortOrder::MemoryAsc,
                    }
                }
                KeyCode::Char('p') => {
                    self.sort_order = match self.sort_order {
                        SortOrder::PidAsc => SortOrder::PidDesc,
                        _ => SortOrder::PidAsc,
                    }
                }
                KeyCode::Char('n') => {
                    self.sort_order = match self.sort_order {
                        SortOrder::NameAsc => SortOrder::NameDesc,
                        _ => SortOrder::NameAsc,
                    }
                }
                KeyCode::Tab => {
                    self.active_block = match self.active_block {
                        ActiveBlock::Cpu => ActiveBlock::Processes,
                        ActiveBlock::Processes => ActiveBlock::Cpu,
                    };
                }
                KeyCode::Up => match self.active_block {
                    ActiveBlock::Cpu => {
                        self.cpu_scroll = self.cpu_scroll.saturating_sub(1);
                        self.cpu_scroll_state = self.cpu_scroll_state.position(self.cpu_scroll);
                    }
                    ActiveBlock::Processes => {
                        self.process_scroll = self.process_scroll.saturating_sub(1);
                        self.process_scroll_state =
                            self.process_scroll_state.position(self.process_scroll);
                    }
                },
                KeyCode::Down => match self.active_block {
                    ActiveBlock::Cpu => {
                        self.cpu_scroll = self.cpu_scroll.saturating_add(1);
                        self.cpu_scroll_state = self.cpu_scroll_state.position(self.cpu_scroll);
                    }
                    ActiveBlock::Processes => {
                        self.process_scroll = self.process_scroll.saturating_add(1);
                        self.process_scroll_state =
                            self.process_scroll_state.position(self.process_scroll);
                    }
                },
                KeyCode::Left => {
                    self.current_fetch_interval =
                        self.current_fetch_interval.saturating_sub(100).max(100);
                }
                KeyCode::Right => {
                    self.current_fetch_interval =
                        self.current_fetch_interval.saturating_add(100).min(60000);
                }
                KeyCode::Char('M') => {
                    self.mode = Mode::Input;
                    self.input.clear();
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn render_size_error(&self, frame: &mut Frame, size: Rect) {
        let current_width_style = if size.width >= MIN_WIDTH {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };
        let current_height_style = if size.height >= MIN_HEIGHT {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };

        let message_text = Text::from(vec![
            Line::from(Span::styled(
                "Terminal window is too small!",
                Style::default().fg(Color::Red),
            )),
            Line::from(""),
            Line::from(vec![
                Span::raw("Current size: "),
                Span::styled(format!("{}", size.width), current_width_style),
                Span::raw("x"),
                Span::styled(format!("{}", size.height), current_height_style),
            ]),
            Line::from(vec![
                Span::raw("Required: "),
                Span::styled(
                    format!("{MIN_WIDTH}x{MIN_HEIGHT}"),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(""),
            Line::from(Span::raw("Please adjust terminal size.")),
        ])
        .alignment(Alignment::Center);

        let paragraph = Paragraph::new(message_text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Error")
                    .title_alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Red)),
            );
        frame.render_widget(Clear, size);
        frame.render_widget(paragraph, size);
    }

    fn render(&mut self, frame: &mut Frame, sys: &mut System) {
        let area = frame.area();

        self.render_outer_frame(frame, area);
        self.render_top_bar(frame, area);

        let inner_area = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });
        let chunks = layout::terminal_layout(inner_area);

        self.render_cpu_gauge(frame, sys, chunks[0]);
        self.render_cpu_cores(frame, sys, chunks[1]);
        self.render_network_info(frame, chunks[2]);
        self.render_memory(frame, sys, chunks[3]);
        self.render_processes(frame, sys, chunks[4]);
        self.render_network_chart(frame, chunks[5]);
        self.render_host_info(frame, chunks[6]);

        if self.show_popup {
            self.render_welcome_popup(frame, area);
        }
        if self.show_manual {
            self.render_manual(frame, area);
        }
    }

    fn render_outer_frame(&self, frame: &mut Frame, area: Rect) {
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(Span::styled("System Monitor", Style::default()))
            .title_alignment(Alignment::Left)
            .title_bottom(
                Line::from(vec![Span::styled(
                    "Press 'Esc' for options",
                    Style::default(),
                )])
                .left_aligned(),
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
                    system_uptime().to_string(),
                    Style::default(),
                )])
                .right_aligned(),
            );
        frame.render_widget(outer_block, area);
    }

    fn render_top_bar(&mut self, frame: &mut Frame, area: Rect) {
        let top_bar_area = Rect::new(area.x + 1, area.y, area.width.saturating_sub(2), 1);

        let current_time_str = Local::now().format("%H:%M:%S").to_string();
        let time_paragraph = Paragraph::new(current_time_str).alignment(Alignment::Center);
        frame.render_widget(time_paragraph, top_bar_area);

        let interval_display = format!("Fetch Interval: {}ms", self.current_fetch_interval);
        let minus_btn_text: &str = "[ ◄";
        let plus_btn_text: &str = "► ]";

        let total_right_content_width = minus_btn_text.len() as u16
            + 1
            + interval_display.len() as u16
            + 1
            + plus_btn_text.len() as u16;

        let right_content_start_x =
            top_bar_area.x + top_bar_area.width.saturating_sub(total_right_content_width);
        let y_pos = top_bar_area.y;

        self.minus_button_rect =
            Rect::new(right_content_start_x, y_pos, minus_btn_text.len() as u16, 1);
        self.plus_button_rect = Rect::new(
            right_content_start_x
                + minus_btn_text.len() as u16
                + 1
                + interval_display.len() as u16
                + 1,
            y_pos,
            plus_btn_text.len() as u16,
            1,
        );

        let fetch_interval_spans = Line::from(vec![
            Span::styled(minus_btn_text, Style::default().fg(Color::Yellow)),
            Span::raw(" "),
            Span::styled(interval_display, Style::default()),
            Span::raw(" "),
            Span::styled(plus_btn_text, Style::default().fg(Color::Yellow)),
        ]);

        let fetch_interval_paragraph =
            Paragraph::new(fetch_interval_spans).alignment(Alignment::Right);
        frame.render_widget(fetch_interval_paragraph, top_bar_area);
    }

    fn render_cpu_gauge(&self, frame: &mut Frame, sys: &System, area: Rect) {
        let cpu_gauge = Gauge::default()
            .block(
                Block::default()
                    .title(format_cpu_name(sys))
                    .borders(Borders::ALL),
            )
            .gauge_style(Style::default().fg(Color::LightBlue).bg(Color::Gray))
            .percent(sys.global_cpu_usage() as u16);
        frame.render_widget(cpu_gauge, area);
    }

    fn render_cpu_cores(&mut self, frame: &mut Frame, sys: &System, area: Rect) {
        self.cpu_scroll_state = self.cpu_scroll_state.content_length(sys.get_cpus().len());

        let cpu_block = Block::default()
            .title("CPU Core Usage ")
            .borders(Borders::ALL)
            .border_style(if self.active_block == ActiveBlock::Cpu {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            });
        let cpu_widget = Paragraph::new(format_cpu_usage(sys))
            .block(cpu_block)
            .wrap(Wrap { trim: true })
            .scroll((self.cpu_scroll as u16, 0));
        frame.render_widget(cpu_widget, area);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(style::Color::LightBlue)
                .begin_symbol(Some("^"))
                .end_symbol(Some("v"))
                .thumb_symbol("░"),
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.cpu_scroll_state,
        );
    }

    fn render_memory(&self, frame: &mut Frame, sys: &System, area: Rect) {
        let memory_block = Block::default()
            .title("Memory Usage ")
            .borders(Borders::ALL);
        let memory_table = ram_info_table(sys).block(memory_block);
        frame.render_widget(memory_table, area);
    }

    fn render_network_info(&mut self, frame: &mut Frame, area: Rect) {
        let network_block = Block::default().title("Network").borders(Borders::ALL);
        let network_widget = Paragraph::new(self.network_manager.format_network())
            .block(network_block)
            .wrap(Wrap { trim: true });
        frame.render_widget(network_widget, area);
    }

    fn render_network_chart(&mut self, frame: &mut Frame, area: Rect) {
        let network_diagram = self.network_manager.get_network_widget();
        frame.render_widget(network_diagram, area);
    }

    fn render_processes(&mut self, frame: &mut Frame, sys: &System, area: Rect) {
        let process_rows = create_process_rows(sys, self.sort_order);
        let num_processes = process_rows.len();
        self.process_scroll_state = self.process_scroll_state.content_length(num_processes);

        if let Some((msg, timestamp)) = &self.kill_message {
            if timestamp.elapsed().as_secs() >= 5 {
                self.kill_message = None;
            } else {
                let msg_color = if msg.starts_with('✅') {
                    Color::Green
                } else {
                    Color::Red
                };
                let msg_area = Rect::new(area.x, area.y.saturating_sub(2), area.width, 1);
                let paragraph = Paragraph::new(msg.clone()).style(Style::default().fg(msg_color));
                frame.render_widget(paragraph, msg_area);
            }
        }

        let block_title = if self.mode == Mode::Input {
            format!("Enter PID to kill: {}", self.input)
        } else if let Some((msg, color)) = &self.process_status {
            Span::styled(msg.clone(), Style::default().fg(*color)).to_string()
        } else {
            "Processes".to_string()
        };

        let processes_block = Block::default()
            .title(block_title)
            .title_bottom(
                Line::from(vec![
                    Span::styled("C", Style::default().fg(Color::Yellow)),
                    Span::raw("PU───"),
                    Span::styled("M", Style::default().fg(Color::Yellow)),
                    Span::raw("emory───"),
                    Span::styled("P", Style::default().fg(Color::Yellow)),
                    Span::raw("ID───"),
                    Span::styled("N", Style::default().fg(Color::Yellow)),
                    Span::raw("ame"),
                ])
                .left_aligned(),
            )
            .borders(Borders::ALL)
            .border_style(if self.active_block == ActiveBlock::Processes {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            });

        let table_height = area.height as usize - 2;
        let visible_rows = process_rows
            .into_iter()
            .skip(self.process_scroll)
            .take(table_height);

        let widths = [
            Constraint::Length(8),
            Constraint::Length(30),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(12),
        ];

        let processes_table = Table::new(visible_rows, widths)
            .column_spacing(1)
            .style(Style::default().fg(Color::White))
            .block(processes_block);

        frame.render_widget(processes_table, area);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(style::Color::LightBlue)
                .begin_symbol(Some("^"))
                .end_symbol(Some("v"))
                .thumb_symbol("░"),
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.process_scroll_state,
        );
    }

    fn render_host_info(&self, frame: &mut Frame, area: Rect) {
        let host_info_block = Block::default()
            .title("Host System Information ")
            .borders(Borders::ALL);
        let host_info_table = host_info_table().block(host_info_block);
        frame.render_widget(host_info_table, area);
    }

    fn render_welcome_popup(&self, frame: &mut Frame, area: Rect) {
        const POPUP_WIDTH: u16 = 35;
        const POPUP_HEIGHT: u16 = 5;

        let popup_area = Rect::new(
            (area.width.saturating_sub(POPUP_WIDTH)) / 2,
            (area.height.saturating_sub(POPUP_HEIGHT)) / 2,
            POPUP_WIDTH,
            POPUP_HEIGHT,
        );

        let username = format!("Current User: {}", get_current_user());
        let mut content = String::new();
        content.push_str(&format!(
            "{:-^width$}\n",
            username,
            width = POPUP_WIDTH as usize - 2
        ));

        let empty_lines = POPUP_HEIGHT as usize - 4;
        for _ in 0..empty_lines {
            content.push('\n');
        }

        let popup_block = Block::default()
            .title_top(Line::from("Welcome to the Dashboard ").centered())
            .title_bottom(Line::from("Press Enter to close ").centered())
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

    fn render_manual(&self, frame: &mut Frame, area: Rect) {
        let manual_area = Rect::new(
            (area.width.saturating_sub(60)) / 2,
            (area.height.saturating_sub(20)) / 2,
            60,
            20,
        );

        let manual_description: [&str; 9] = [
            "Press 'i' to switch network interface\n",
            "Press 'c' to sort by CPU usage\n",
            "Press 'm' to sort by Memory usage\n",
            "Press 'p' to sort by PID\n",
            "Press 'n' to sort by Name\n",
            "Press 'Tab' to switch between CPU and Processes view\n",
            "Use Up/Down arrows to scroll through CPU or Processes\n",
            "Use Left/Right arrows to adjust fetch interval\n",
            "Press 'q' to quit the application\n",
        ];

        let manual_content = manual_description
            .iter()
            .map(|s| Line::from(Span::raw(*s)))
            .collect::<Vec<_>>();

        let manual_block = Block::default()
            .title("Options")
            .title_alignment(Alignment::Center)
            .title_bottom("Press 'Esc' to close")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::LightBlue));

        let manual_paragraph = Paragraph::new(manual_content)
            .block(manual_block)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);

        frame.render_widget(Clear, manual_area);
        frame.render_widget(manual_paragraph, manual_area);
    }
}

fn system_uptime() -> String {
    let uptime = System::uptime();
    if uptime < 60 {
        format!("Uptime: {uptime} seconds ")
    } else if uptime < 3600 {
        let minutes = uptime / 60;
        format!("Uptime: {minutes} minutes ")
    } else if uptime < 86400 {
        let hours = uptime / 3600;
        format!("Uptime: {hours} hours ")
    } else {
        let days = uptime / 86400;
        format!("Uptime: {days} days ")
    }
}
