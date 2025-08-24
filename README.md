# Rust System Dashboard

A terminal-based system monitoring dashboard written in Rust.

## About the Project

This project provides a real-time dashboard to monitor various system metrics, including CPU usage, memory consumption, network activity, running processes, and general system information. The user interface is built using the `ratatui` crate, providing an interactive and responsive experience in the terminal.

## Features

- **CPU Monitoring:** Displays overall CPU usage as a gauge and provides a detailed view of individual core usage.
- **Memory Monitoring:** Shows detailed information about RAM and swap usage.
- **Network Monitoring:** Tracks network data transmission and reception, with a graphical representation of network activity.
- **Process List:** Lists running processes with details such as PID, CPU usage, memory usage, and name. Processes can be sorted by different criteria.
- **System Information:** Displays host information, including operating system, kernel version, and uptime.
- **Interactive UI:** Allows switching between different views, scrolling through lists, and adjusting the data refresh interval.
- **Killing Processes:** You are now able to kill processes directly from the dashboard.

## Upcoming

- Improve fetching mechanism
- Add option to choose between a second layout (or only one box at the time)

## Usage

To run the dashboard, you can either build it from the source or download a pre-compiled binary from the release page.

### Building from Source

1.  Clone the repository:
    ```bash
    git clone https://github.com/your-username/Rust-Dashboard.git
    ```
2.  Navigate to the project directory:
    ```bash
    cd Rust-Dashboard/Dashboard
    ```
3.  Build and run the project:
    ```bash
    cargo run --release
    ```

### Interaction

-   **`q`**: Quit the application.
-   **`Tab`**: Switch between the CPU and Processes panels.
-   **`Up`/`Down` Arrows**: Scroll through the active panel.
-   **`Left`/`Right` Arrows**: Adjust the data refresh interval.
-   **`i`**: Switch the selected network interface.
-   **`c`**, **`m`**, **`p`**, **`n`**: Sort the process list by CPU, Memory, PID, or Name, respectively.
-   **`Esc`**: Show/hide the options menu.
-   **`M`**: Switches to Input Mode and lets you directly type into the heading of the Processes Block.

## Dependencies

This project relies on the following main dependencies:

-   `sysinfo`: To gather system information.
-   `ratatui`: For creating the terminal user interface.
-   `crossterm`: For terminal manipulation.
-   `chrono`: For time-related functionalities.
-   `color-eyre`: For better error reporting.

## Download and Run

You can download the application from the release page.

For Linux and macOS, you need to make the file executable with the following command:

```bash
chmod +x linux_dashboard
```

## Documentation

When cloning the github you can open the project in your Terminal and type:
```bash
cargo doc --open
```
This will open your Standard Browser with the code documentation.

## Credits

Thanks to @orhundev's YouTube Channel Videos on TUI applications I learned a lot on how to use ratatui and was able to write my first Rust project, as well as my first project overall. There is still a few things to implement, but this version will cut it for the time being.
