//! This module fetches network information

use sysinfo::Networks;

// Erstellt einen neuen Netzwerkmanager und lädt die Schnittstellen
pub struct NetworkManager {
    networks: Networks,
}

impl NetworkManager {
    /// Erstellt einen neuen NetworkManager und lädt die Netzwerkschnittstellen
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();
        Self { networks }
    }
    
    /// Aktualisiert die Netzwerkschnittstellen
    pub fn format_network(&mut self) -> String {
        self.networks.refresh(true);
        
        let mut data_transfer = String::new();
    
        for (interface_name, data) in self.networks.iter() {
            let received = data.total_received();
            let transmitted = data.total_transmitted();
    
            if received == 0 && transmitted == 0 {
                continue; // Überspringe Schnittstellen ohne Daten
            }
    
            let network_info = if received < 1000 && transmitted < 1000 {
                // Werte kleiner als 1000 -> Bytes
                format!("{}: {} B (down), {} B (up)\n", interface_name, received, transmitted)
            } else if received < 1000 * 1000 && transmitted < 1000 * 1000 {
                // Werte kleiner als 1.000.000 -> KB
                let down_kb = received / 1000;
                let up_kb = transmitted / 1000;
                format!("{}: {} KB (down), {} KB (up)\n", interface_name, down_kb, up_kb)
            } else if received < 1000 * 1000 * 1000 && transmitted < 1000 * 1000 * 1000 {
                // Werte kleiner als 1.000.000.000 -> MB
                let down_mb = received / (1000 * 1000);
                let up_mb = transmitted / (1000 * 1000);
                format!("{}: {} MB (down), {} MB (up)\n", interface_name, down_mb, up_mb)
            } else {
                // Werte größer als 1.000.000.000 -> GB
                let down_gb = received / (1000 * 1000 * 1000);
                let up_gb = transmitted / (1000 * 1000 * 1000);
                format!("{}: {} GB (down), {} GB (up)\n", interface_name, down_gb, up_gb)
            };
            
            data_transfer.push_str(&network_info);
            
        }
    
        data_transfer
    }
}