use crate::backend::host::host_info_table;
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
use ratatui::layout::Constraint;
use ratatui::style::Color;
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, Clear, Gauge, Paragraph, Scrollbar, Table, Wrap,
};
use ratatui::widgets::{ScrollbarOrientation, ScrollbarState};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::Alignment,
    prelude::*,
    style::Style,
};
use std::time::{Duration, Instant};
use sysinfo::System;

const MIN_WIDTH: u16 = 110;
const MIN_HEIGHT: u16 = 24;

#[derive(PartialEq, Eq)]
enum ActiveBlock {
    Cpu,
    Processes,
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_block: ActiveBlock::Cpu,
            cpu_scroll_state: ScrollbarState::default(),
            process_scroll_state: ScrollbarState::default(),
            cpu_scroll: 0,
            process_scroll: 0,
            current_fetch_interval: 1000, // Initial 1000ms
            minus_button_rect: Rect::default(),
            plus_button_rect: Rect::default(),
        }
    }
}

#[allow(dead_code)] // used to get rid of annoying warnings
struct App {
    active_block: ActiveBlock,
    cpu_scroll_state: ScrollbarState,
    process_scroll_state: ScrollbarState,
    cpu_scroll: usize,
    process_scroll: usize,
    pub current_fetch_interval: u64,
    pub minus_button_rect: Rect,
    pub plus_button_rect: Rect,
}

/// run_ui() is the entry point for the terminal UI.
/// It initializes the necessary components and starts the main loop,
/// which renders and updates the terminal UI.
///
/// color_eyre is called to configure error reporting.
///
/// ratatui::init() is used to prepare the terminal for display.
///
/// let mut sys = System::new_all();
/// sys.refresh_all();
/// creates a new System object that collects data about the system.
///
/// let result = run(...) starts the main loop.
/// It renders the UI and updates the system information at regular intervals.
/// It takes the terminal and system information as parameters.

pub fn run_ui() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let mut sys = System::new_all();
    sys.refresh_all();

    let app = App::default();
    let app_result = app.run(terminal, &mut sys);
    ratatui::restore();
    app_result
}

/// Runtime function to render the Terminal and to refresh it.
/// It sets an initial and final tick rate.
/// The user interface is then re-rendered in a loop at the respective interval with information such as CPU usage.
/// This is interrupted if there is a "KeyEvent" within a 50 ms time span, where 'q' is defined to exit the loop.
impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal, sys: &mut System) -> Result<()> {
        // Disable mouse capture to prevent performance issues with mouse wheel
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::event::DisableMouseCapture
        )?;

        let mut last_tick = Instant::now();
        let mut show_popup = true;
        let mut network_manager = NetworkManager::new();
        let mut sort_order = SortOrder::default();
        let mut show_manual = false;

        loop {
            let tick_rate = Duration::from_millis(self.current_fetch_interval);
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                let evt = event::read()?;
                match evt {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    }) => break Ok(()),
                    Event::Mouse(event::MouseEvent {
                        kind: event::MouseEventKind::Up(..), // Use .. to ignore the MouseButton value
                        column,
                        row,
                        ..
                    }) => {
                        let click_point = Rect::new(column, row, 1, 1);
                        if click_point.intersects(self.minus_button_rect) {
                            self.current_fetch_interval =
                                self.current_fetch_interval.saturating_sub(100).max(100);
                        } else if click_point.intersects(self.plus_button_rect) {
                            self.current_fetch_interval =
                                self.current_fetch_interval.saturating_add(100).min(60000);
                        }
                    }
                    Event::Mouse(event::MouseEvent {
                        kind: event::MouseEventKind::ScrollDown,
                        ..
                    }) => {}
                    Event::Mouse(event::MouseEvent {
                        kind: event::MouseEventKind::ScrollUp,
                        ..
                    }) => {}
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
                    Event::Key(KeyEvent {
                        code: KeyCode::Tab, ..
                    }) => {
                        self.active_block = match self.active_block {
                            ActiveBlock::Cpu => ActiveBlock::Processes,
                            ActiveBlock::Processes => ActiveBlock::Cpu,
                        };
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Up, ..
                    }) => match self.active_block {
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
                    Event::Key(KeyEvent {
                        code: KeyCode::Down,
                        ..
                    }) => match self.active_block {
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
                    Event::Key(KeyEvent {
                        code: KeyCode::Left,
                        ..
                    }) => {
                        self.current_fetch_interval =
                            self.current_fetch_interval.saturating_sub(100).max(100);
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Right,
                        ..
                    }) => {
                        self.current_fetch_interval =
                            self.current_fetch_interval.saturating_add(100).min(60000);
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Esc, ..
                    }) => {
                        show_manual = !show_manual;
                    }
                    _ => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                sys.refresh_all();
                last_tick = Instant::now();
            }

            terminal.draw(|frame| {
                let size = frame.area();
                if size.width < MIN_WIDTH || size.height < MIN_HEIGHT {
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
                        Line::from(""), // Empty line for spacing
                        Line::from(vec![
                            Span::raw("Current size: "),
                            Span::styled(format!("{}", size.width), current_width_style),
                            Span::raw("x"),
                            Span::styled(format!("{}", size.height), current_height_style),
                        ]),
                        Line::from(vec![
                            Span::raw("Required: "),
                            Span::styled(
                                format!("{}x{}", MIN_WIDTH, MIN_HEIGHT),
                                Style::default().fg(Color::Yellow),
                            ),
                        ]),
                        Line::from(""), // Empty line for spacing
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
                } else {
                    Self::render(
                        &mut self,
                        frame,
                        sys,
                        &mut show_popup,
                        &mut network_manager,
                        &mut sort_order,
                        &mut show_manual,
                    )
                }
            })?;
        }
    }

    /// render() draws the frame of the TUI app and creates various objects such as paragraphs, blocks, etc.
    fn render(
        &mut self,
        frame: &mut Frame,
        sys: &mut System,
        show_popup: &mut bool,
        network_manager: &mut NetworkManager,
        sort_order: &mut SortOrder,
        show_manual: &mut bool,
    ) {
        // Get the entire terminal area
        let area = frame.area();

        // Create outer frame
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            // Top left: System Monitor
            .title(Span::styled("System Monitor", Style::default()))
            .title_alignment(Alignment::Left)
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

        frame.render_widget(outer_block, area);

        // Render the top bar content (time and fetch interval)
        let top_bar_area = Rect::new(area.x + 1, area.y, area.width.saturating_sub(2), 1);

        // Current Time (centered)
        let current_time_str = Local::now().format("%H:%M:%S").to_string();
        let time_paragraph = Paragraph::new(current_time_str).alignment(Alignment::Center);
        frame.render_widget(time_paragraph, top_bar_area);

        // Fetch Interval and Buttons (right-aligned)
        let interval_display = format!("Fetch Interval: {}ms", self.current_fetch_interval);
        let minus_btn_text: &'static str = "[ ◄";
        let plus_btn_text: &'static str = "► ]";

        // Calculate total width needed for right-aligned content
        let total_right_content_width = minus_btn_text.len() as u16
            + 1
            + interval_display.len() as u16
            + 1
            + plus_btn_text.len() as u16; // +1 for spaces between elements

        // Calculate starting X for right-aligned content
        let right_content_start_x =
            top_bar_area.x + top_bar_area.width.saturating_sub(total_right_content_width);
        let y_pos = top_bar_area.y;

        // Calculate Rects for buttons
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

        // Create inner layout (within the outer frame)
        let inner_area = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }); // Leave space for the frame
        let chunks = layout::terminal_layout(inner_area);

        // CPU Usage
        let cpu_core_usage = format_cpu_usage(sys);
        self.cpu_scroll_state = self
            .cpu_scroll_state
            .content_length(sys.get_cpus().len() as usize);

        let cpu_block = Block::default()
            .title("CPU Core Usage ")
            .borders(Borders::ALL)
            .border_style(if self.active_block == ActiveBlock::Cpu {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            });
        let cpu_widget = Paragraph::new(cpu_core_usage)
            .block(cpu_block)
            .wrap(Wrap { trim: true })
            .scroll((self.cpu_scroll as u16, 0));
        frame.render_widget(cpu_widget, chunks[1]);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(style::Color::LightBlue)
                .begin_symbol(Some("^")) // ^
                .end_symbol(Some("v")) // v
                .thumb_symbol("░"), //
            chunks[1].inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.cpu_scroll_state,
        );

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
        let memory_block = Block::default()
            .title("Memory Usage ")
            .borders(Borders::ALL);
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
        let process_rows = create_process_rows(sys, *sort_order);
        let num_processes = process_rows.len();
        self.process_scroll_state = self.process_scroll_state.content_length(num_processes);

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

        let table_height = chunks[4].height as usize - 2; // -2 for borders
        let visible_rows = process_rows
            .into_iter()
            .skip(self.process_scroll)
            .take(table_height);

        let widths = [
            Constraint::Length(8),  // PID
            Constraint::Length(30), // Name
            Constraint::Length(10), // Status
            Constraint::Length(10), // CPU
            Constraint::Length(12), // Memory
        ];

        let processes_table = Table::new(visible_rows, widths)
            .column_spacing(1)
            .style(Style::default().fg(Color::White))
            .block(processes_block);

        frame.render_widget(processes_table, chunks[4]);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(style::Color::LightBlue)
                .begin_symbol(Some("^")) // ^
                .end_symbol(Some("v")) // v
                .thumb_symbol("░"), //
            chunks[4].inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.process_scroll_state,
        );

        // Network Diagram Block
        let network_diagram = network_manager.get_network_widget();
        frame.render_widget(network_diagram, chunks[5]);

        // Host Info Block
        let host_info_block = Block::default()
            .title("Host System Information ")
            .borders(Borders::ALL);
        let host_info_table = host_info_table().block(host_info_block);
        frame.render_widget(host_info_table, chunks[6]);

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
                // ensure that the cells under the popup are cleared to prevent the content from being outside the box
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
            const POPUP_WIDTH: u16 = 35;
            const POPUP_HEIGHT: u16 = 5;

            // Calculate centered popup area
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
                .title_top(
                    Line::from(vec![
                        Span::styled("".to_string(), Style::default()), // Empty span for left
                        Span::styled("Welcome to the Dashboard ", Style::default()), // Time in the middle
                        Span::styled("".to_string(), Style::default()), // Empty span for right
                    ])
                    .centered(),
                )
                .title_bottom(
                    Line::from(vec![
                        Span::styled("".to_string(), Style::default()), // Empty span for left
                        Span::styled("Press Enter to close ", Style::default()), // Time in the middle
                        Span::styled("".to_string(), Style::default()), // Empty span for right
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

        if *show_manual {
            let manual_area = Rect::new(
                (area.width.saturating_sub(60)) / 2,
                (area.height.saturating_sub(20)) / 2,
                60,
                20,
            );

            let manual_description = vec![
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
            // &&str is expected here, because iter iterates over the elements of the Vec and thus creates a reference to each element.
            // Now a reference to this reference must be created to get the value.
            let manual_content = manual_description
                .iter()
                .map(|s: &&str| Line::from(Span::raw(*s)))
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
}

fn system_uptime() -> String {
    let uptime = System::uptime();
    if uptime < 60 {
        return format!("Uptime: {} seconds ", uptime);
    } else if uptime < 3600 {
        let minutes = uptime / 60;
        return format!("Uptime: {} minutes ", minutes);
    } else if uptime < 86400 {
        let hours = uptime / 3600;
        return format!("Uptime: {} hours ", hours);
    } else {
        let days = uptime / 86400;
        return format!("Uptime: {} days ", days);
    }
}
