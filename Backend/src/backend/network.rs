//! This module fetches network information

use sysinfo::Networks;

// Erstellt einen neuen Netzwerkmanager und lädt die Schnittstellen
pub struct NetworkManager {
    networks: Networks,
}

impl NetworkManager {
    /// Erstellt einen neuen NetworkManager und lädt die Netzwerkschnittstellen
    pub fn new() -> Self {
        let networks = Networks::new();
        Self { networks }
    }

    /// Aktualisiert nur die Daten der vorhandenen Netzwerkschnittstellen
    pub fn format_network(&mut self) -> String {
        // Aktualisiere die Daten der Netzwerkschnittstellen
        self.networks.refresh(true);

        let mut data_transfer = String::new();

        for (interface_name, data) in self.networks.iter() {
            let network_info = format!(
                "{}: {} B (down), {} B (up)\n",
                interface_name,
                data.total_received(),
                data.total_transmitted(),
            );
            data_transfer.push_str(&network_info);
        }

        data_transfer
    }
}