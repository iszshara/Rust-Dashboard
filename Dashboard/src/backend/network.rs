use ratatui::style::Color;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::{
    style::{Style, Stylize},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
};
use std::collections::HashMap;
use sysinfo::{Networks, System};

type DataPoint = (f64, f64); // Tuple for time and value
type DataHistory = Vec<DataPoint>; // History of data points for download and upload
type NetworkHistory = (DataHistory, DataHistory); // Tuple for download and upload history
type NetworkHistoryMap = HashMap<String, NetworkHistory>; // Map for each interface to its download and upload history
pub struct NetworkManager {
    networks: Networks,

    previous_received: HashMap<String, u64>,
    previous_transmitted: HashMap<String, u64>,
    network_history: NetworkHistoryMap,
    // Vec<()> because the data changes over time
    // f64 for time and the value for upload/download
    scaled_download: Vec<(f64, f64)>,
    scaled_upload: Vec<(f64, f64)>,
    time_counter: f64,
    selected_interface: String,
}

impl Default for NetworkManager {
    fn default() -> Self {
        let networks = Networks::new_with_refreshed_list();
        let mut network_history = HashMap::new();

        // initializes every found interface with an empty history
        for (interface_name, _) in networks.iter() {
            network_history.insert(interface_name.to_string(), (Vec::new(), Vec::new()));
        }

        let selected_interface = networks
            .iter()
            .next()
            .map(|(name, _)| name.to_string())
            .unwrap_or_default();

        // create a new instance of NetworkManager with the initialized values
        Self {
            networks: Networks::new_with_refreshed_list(),
            previous_received: HashMap::new(),
            previous_transmitted: HashMap::new(),
            network_history,
            scaled_download: Vec::new(),
            scaled_upload: Vec::new(),
            time_counter: 0.0,
            selected_interface,
        }
    }
}

impl NetworkManager {
    // This setter-method is used to change the currently selected interface
    // and update the history for this interface.
    pub fn set_selected_interface(&mut self, interface: String) {
        self.selected_interface = interface;
    }

    // This getter-method returns the keys of the network history map,
    pub fn network_history_keys(&self) -> Vec<String> {
        self.network_history.keys().cloned().collect()
    }

    // This getter-method returns the currently selected interface.
    pub fn get_selected_interface(&self) -> &String {
        &self.selected_interface
    }

    // The update_network_data method updates the network history for the currently selected interface.
    // It takes the received and transmitted data differences and updates the history.
    // The time_counter is incremented to keep track of the time for the chart.
    // It also ensures that the history does not exceed 50 data points.
    // This method is called whenever new network data is available.
    pub fn update_network_data(
        &mut self,
        received_diff: u64,
        transmitted_diff: u64,
        interface: &str,
    ) {
        self.time_counter += 1.0;
        let download = received_diff as f64;
        let upload = transmitted_diff as f64;

        if let Some((download_history, upload_history)) = self.network_history.get_mut(interface) {
            download_history.push((self.time_counter, download));
            // Upload als negative Werte speichern, damit er im Diagramm nach unten verläuft
            upload_history.push((self.time_counter, -upload));

            // Behalte nur die letzten 50 Datenpunkte
            if download_history.len() > 50 {
                download_history.remove(0);
                upload_history.remove(0);
            }
        }
    }

    // Creates a chart widget for the network traffic of the selected interface.
    // It uses the network history to plot the download and upload data.
    // The chart is scaled based on the maximum value found in the data.
    // The unit is determined based on the maximum value to display it in a human-readable format.
    // The chart is styled with colors for download (green) and upload (red).
    // The x-axis represents time, and the y-axis represents the data rate in the appropriate unit.
    pub fn get_network_widget(&mut self) -> Chart<'_> {
        // Da selected_interface immer gesetzt ist, können wir unwrap() verwenden.
        let (download_data, upload_data) =
            self.network_history.get(&self.selected_interface).unwrap();

        // Determine the maximum value to scale the chart.
        // The max value is determined by iterating over both download and upload data.
        // The "_" placeholder is used to ignore the timestamp, as it is not relevant for the maximum value.
        // The body of the closure is called with value.abs() to ensure we always
        // have a positive value, as upload values are stored as negative.
        let max_value = download_data
            .iter()
            .chain(upload_data.iter())
            .map(|(_, value)| value.abs())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        // unit is a &'static str, as the values are string literals and the lifetime is static,
        // meaning the values should be displayed as long as the program itself runs.
        let (unit, scale_factor) = if max_value > 1024.0 * 1024.0 * 1024.0 {
            ("GB/s", 1024.0 * 1024.0 * 1024.0)
        } else if max_value > 1024.0 * 1024.0 {
            ("MB/s", 1024.0 * 1024.0)
        } else if max_value > 1024.0 {
            ("KB/s", 1024.0)
        } else {
            ("B/s", 1.0)
        };

        // Updates the scaled_download and scaled_upload fields with the scaled data.
        // The data is scaled by dividing each value by the scale_factor.
        // This ensures that the chart displays the data in a human-readable format.
        self.scaled_download = download_data
            .iter()
            .map(|(x, y)| (*x, y / scale_factor))
            .collect();
        self.scaled_upload = upload_data
            .iter()
            .map(|(x, y)| (*x, y / scale_factor))
            .collect();

        let datasets = vec![
            Dataset::default()
                .name("▼ Download")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().green())
                .data(&self.scaled_download),
            Dataset::default()
                .name("▲ Upload")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().red())
                .data(&self.scaled_upload),
        ];

        let x_axis = Axis::default()
            .title("Time")
            .style(Style::default().white())
            .bounds([self.time_counter - 50.0, self.time_counter])
            .labels(Vec::<String>::new());

        let y_axis = Axis::default()
            .title(unit)
            .style(Style::default().white())
            .bounds([
                -(max_value / scale_factor).ceil(),
                (max_value / scale_factor).ceil(),
            ])
            .labels(Vec::<String>::new());

        Chart::new(datasets)
            .block(
                Block::default()
                    .title(format!(
                        "Network Traffic - Interface: {}",
                        self.selected_interface.clone()
                    ))
                    // .title_bottom("'n' - change network interface")
                    .title_bottom(Line::from(vec![
                        Span::styled("i", Style::default().fg(Color::Yellow)),
                        Span::raw(" - to change network interface"),
                    ]))
                    .borders(Borders::ALL),
            )
            .x_axis(x_axis)
            .y_axis(y_axis)
    }

    // formats the network data as a string for display.
    // It iterates over all network interfaces and calculates the received and transmitted data.
    // The data is formatted based on the size (B, KB, MB, GB)
    // and appended to the result string.
    pub fn format_network(&mut self, sys: &mut System) -> String {
        sys.refresh_all();
        let mut data_transfer = String::new();
        self.networks.refresh(true);
        let mut network_updates = Vec::new();

        // Iteriere über alle Interfaces und sammle deren Messwerte.
        for (interface_name, data) in self.networks.iter() {
            let received = data.total_received();
            let transmitted = data.total_transmitted();

            let received_diff = received
                - self
                    .previous_received
                    .get(&interface_name.to_string())
                    .unwrap_or(&0);
            let transmitted_diff =
                transmitted - self.previous_transmitted.get(interface_name).unwrap_or(&0);

            self.previous_received
                .insert(interface_name.to_string(), received);
            self.previous_transmitted
                .insert(interface_name.to_string(), transmitted);

            // saved to network_updates for later processing
            // This is done to avoid multiple println! calls in the loop,
            // which can be inefficient and clutter the output.
            // Instead, we collect the updates and process them after the loop.
            // This also allows us to format the output string in a more controlled manner.
            network_updates.push((interface_name.to_string(), received_diff, transmitted_diff));

            // Creates the formatted string for the current interface
            let network_info = if received_diff < 1000 && transmitted_diff < 1000 {
                format!(
                    "{interface_name}: {received_diff} B/s (down), {transmitted_diff} B/s (up)\n"
                )
            } else if received_diff < 1000 * 1000 && transmitted_diff < 1000 * 1000 {
                let down_kb = received_diff / 1000;
                let up_kb = transmitted_diff / 1000;
                format!("{interface_name}: {down_kb} KB/s (down), {up_kb} KB/s (up)\n")
            } else if received_diff < 1000 * 1000 * 1000 && transmitted_diff < 1000 * 1000 * 1000 {
                let down_mb = received_diff / (1000 * 1000);
                let up_mb = transmitted_diff / (1000 * 1000);
                format!("{interface_name}: {down_mb} MB/s (down), {up_mb} MB/s (up)\n")
            } else {
                let down_gb = received_diff / (1000 * 1000 * 1000);
                let up_gb = transmitted_diff / (1000 * 1000 * 1000);
                format!("{interface_name}: {down_gb} GB/s (down), {up_gb} GB/s (up)\n")
            };
            data_transfer.push_str(&network_info);
        }

        // Update the network history for each interface
        // This is done after collecting all updates to avoid modifying the history while iterating.
        // This ensures that the history is updated only once per interface,
        // which is more efficient and avoids potential issues with concurrent modifications.
        for (interface, received_diff, transmitted_diff) in network_updates {
            self.update_network_data(received_diff, transmitted_diff, &interface);
        }

        data_transfer
    }
}
