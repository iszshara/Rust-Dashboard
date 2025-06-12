use ratatui::{
    style::{Style, Stylize},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
};
use std::collections::HashMap;
use sysinfo::{Networks, System};

pub struct NetworkManager {
    networks: Networks,
    previous_received: HashMap<String, u64>,
    previous_transmitted: HashMap<String, u64>,
    download_data: Vec<(f64, f64)>,
    upload_data: Vec<(f64, f64)>,
    scaled_download: Vec<(f64, f64)>,
    scaled_upload: Vec<(f64, f64)>,
    time_counter: f64,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            networks: Networks::new_with_refreshed_list(),
            previous_received: HashMap::new(),
            previous_transmitted: HashMap::new(),
            download_data: Vec::new(),
            upload_data: Vec::new(),
            scaled_download: Vec::new(),
            scaled_upload: Vec::new(),
            time_counter: 0.0,
        }
    }

    // Füge diese Methode hinzu, um die Netzwerkdaten zu aktualisieren
    pub fn update_network_data(&mut self, received_diff: u64, transmitted_diff: u64) {
        self.time_counter += 1.0;
        let download = received_diff as f64;
        let upload = transmitted_diff as f64;

        self.download_data.push((self.time_counter, download));
        self.upload_data.push((self.time_counter, -upload)); // Negativ für Darstellung nach unten

        // Behalte nur die letzten 50 Datenpunkte
        if self.download_data.len() > 50 {
            self.download_data.remove(0);
            self.upload_data.remove(0);
        }
    }

    pub fn get_network_widget(&mut self) -> Chart<'_> {
        let max_value = self
            .download_data
            .iter()
            .chain(self.upload_data.iter())
            .map(|(_, value)| value.abs())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let (unit, scale_factor) = if max_value > 1024.0 * 1024.0 * 1024.0 {
            ("GB/s", 1024.0 * 1024.0 * 1024.0)
        } else if max_value > 1024.0 * 1024.0 {
            ("MB/s", 1024.0 * 1024.0)
        } else if max_value > 1024.0 {
            ("KB/s", 1024.0)
        } else {
            ("B/s", 1.0)
        };

        self.scaled_download = self
            .download_data
            .iter()
            .map(|(x, y)| (*x, y / scale_factor))
            .collect();
        self.scaled_upload = self
            .upload_data
            .iter()
            .map(|(x, y)| (*x, y / scale_factor))
            .collect();

        let datasets = vec![
            Dataset::default()
                .name("Download")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().green())
                .data(&self.scaled_download),
            Dataset::default()
                .name("Upload")
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
                    .title("Network Traffic")
                    .borders(Borders::ALL),
            )
            .x_axis(x_axis)
            .y_axis(y_axis)
    }

    pub fn format_network(&mut self, sys: &mut System) -> String {
        sys.refresh_all();
        let mut data_transfer = String::new();

        self.networks.refresh(true);
        let mut network_updates = Vec::new();

        for (interface_name, data) in self.networks.iter() {
            let received = data.total_received();
            let transmitted = data.total_transmitted();

            // Berechne die Differenz seit dem letzten Update
            let received_diff = received
                - self
                    .previous_received
                    .get(&interface_name.to_string())
                    .unwrap_or(&0);
            let transmitted_diff =
                transmitted - self.previous_transmitted.get(interface_name).unwrap_or(&0);

            // Speichere die aktuellen Werte für das nächste Update
            self.previous_received
                .insert(interface_name.to_string(), received);
            self.previous_transmitted
                .insert(interface_name.to_string(), transmitted);

            network_updates.push((received_diff, transmitted_diff));

            // Collect network info first

            let network_info = if received_diff < 1000 && transmitted_diff < 1000 {
                format!(
                    "{}: {} B/s (down), {} B/s (up)\n",
                    interface_name, received_diff, transmitted_diff
                )
            } else if received_diff < 1000 * 1000 && transmitted_diff < 1000 * 1000 {
                let down_kb = received_diff / 1000;
                let up_kb = transmitted_diff / 1000;
                format!(
                    "{}: {} KB/s (down), {} KB/s (up)\n",
                    interface_name, down_kb, up_kb
                )
            } else if received_diff < 1000 * 1000 * 1000 && transmitted_diff < 1000 * 1000 * 1000 {
                let down_mb = received_diff / (1000 * 1000);
                let up_mb = transmitted_diff / (1000 * 1000);
                format!(
                    "{}: {} MB/s (down), {} MB/s (up)\n",
                    interface_name, down_mb, up_mb
                )
            } else {
                let down_gb = received_diff / (1000 * 1000 * 1000);
                let up_gb = transmitted_diff / (1000 * 1000 * 1000);
                format!(
                    "{}: {} GB/s (down), {} GB/s (up)\n",
                    interface_name, down_gb, up_gb
                )
            };
            data_transfer.push_str(&network_info);
        }

        // Update network data after collecting all information
        for (received_diff, transmitted_diff) in &network_updates {
            self.update_network_data(*received_diff, *transmitted_diff);
        }

        data_transfer
    }
}
