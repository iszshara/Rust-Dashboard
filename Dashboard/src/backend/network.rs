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

        // Erstellen einer neuen Instanz von NetworkManager mit den initialisierten Werten
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
    // dient zur Änderung der aktuell ausgewählten Schnittstelle
    // und aktualisiert die Historie für diese Schnittstelle.
    pub fn set_selected_interface(&mut self, interface: String) {
        self.selected_interface = interface;
    }

    // Gibt eine Liste aller verfügbaren Netzwerk-Interfaces zurück.
    pub fn network_history_keys(&self) -> Vec<String> {
        self.network_history.keys().cloned().collect()
    }

    // Gibt das aktuell ausgewählte Interface zurück.
    pub fn get_selected_interface(&self) -> &String {
        &self.selected_interface
    }

    // Aktualisiert die Netzwerkdaten für ein bestimmtes Interface.
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

    // Erzeugt das Chart-Widget für das aktuell ausgewählte Interface.
    pub fn get_network_widget(&mut self) -> Chart<'_> {
        // Da selected_interface immer gesetzt ist, können wir unwrap() verwenden.
        let (download_data, upload_data) =
            self.network_history.get(&self.selected_interface).unwrap();

        // Bestimme den maximalen Wert, um die Skalierung zu ermitteln.
        // in map wird mittels des "_" Platzhalters der Wert für den Zeitstempel
        // ignoriert, da der Zeitstempel nicht relevant ist für den Maximalwert.
        // Weiterhin wird dann der Körper des Closure, value.abs() aufgerufen, da wir immer einen absoluten Wert brauchen
        // aufgrund der negativen Upload-Werte.
        let max_value = download_data
            .iter()
            .chain(upload_data.iter())
            .map(|(_, value)| value.abs())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        // unit ist ein &'static str, da die Werte String-Literale sind und die Lifetime ist static, da die Werte so lange wie das Programm an sich lebt die Werte anzeigen soll.
        let (unit, scale_factor) = if max_value > 1024.0 * 1024.0 * 1024.0 {
            ("GB/s", 1024.0 * 1024.0 * 1024.0)
        } else if max_value > 1024.0 * 1024.0 {
            ("MB/s", 1024.0 * 1024.0)
        } else if max_value > 1024.0 {
            ("KB/s", 1024.0)
        } else {
            ("B/s", 1.0)
        };

        // Aktualisiere die skalieren Datenfelder – diese Felder leben in der Instanz.
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

    // Formatiert die Netzwerkdaten als String für die Anzeige.
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

            // Speichern von Updates inklusive Interface-Namen, um sie später zu aktualisieren.
            network_updates.push((interface_name.to_string(), received_diff, transmitted_diff));

            // Erzeuge den anzuzeigenden String für das Interface.
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

        // Aktualisiere die Historie für jedes Interface
        for (interface, received_diff, transmitted_diff) in network_updates {
            self.update_network_data(received_diff, transmitted_diff, &interface);
        }

        data_transfer
    }
}
