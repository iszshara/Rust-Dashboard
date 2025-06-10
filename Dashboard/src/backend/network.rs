use sysinfo::{Networks, System};
use std::collections::HashMap;

pub struct NetworkManager {
    networks: Networks,
    previous_received: HashMap<String, u64>,
    previous_transmitted: HashMap<String, u64>,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            networks: Networks::new_with_refreshed_list(),
            previous_received: HashMap::new(),
            previous_transmitted: HashMap::new(),
        }
    }
    
    pub fn format_network(&mut self, sys: &mut System) -> String {
        sys.refresh_all();
        let mut data_transfer = String::new();
        
        self.networks.refresh(true);
        for (interface_name, data) in self.networks.iter() {
            let received = data.total_received();
            let transmitted = data.total_transmitted();
            
            // Berechne die Differenz seit dem letzten Update
            let received_diff = received - self.previous_received.get(&interface_name.to_string()).unwrap_or(&0);
            let transmitted_diff = transmitted - self.previous_transmitted.get(interface_name).unwrap_or(&0);
            
            // Speichere die aktuellen Werte für das nächste Update
            self.previous_received.insert(interface_name.to_string(), received);
            self.previous_transmitted.insert(interface_name.to_string(), transmitted);
            
            // Optional: Ignoriere Interfaces, die keine Daten übertragen haben
            // if received_diff == 0 && transmitted_diff == 0 {
            //     continue;
            // }

            let network_info = if received_diff < 1000 && transmitted_diff < 1000 {
                format!("{}: {} B/s (down), {} B/s (up)\n", interface_name, received_diff, transmitted_diff)
            } else if received_diff < 1000 * 1000 && transmitted_diff < 1000 * 1000 {
                let down_kb = received_diff / 1000;
                let up_kb = transmitted_diff / 1000;
                format!("{}: {} KB/s (down), {} KB/s (up)\n", interface_name, down_kb, up_kb)
            } else if received_diff < 1000 * 1000 * 1000 && transmitted_diff < 1000 * 1000 * 1000 {
                let down_mb = received_diff / (1000 * 1000);
                let up_mb = transmitted_diff / (1000 * 1000);
                format!("{}: {} MB/s (down), {} MB/s (up)\n", interface_name, down_mb, up_mb)
            } else {
                let down_gb = received_diff / (1000 * 1000 * 1000);
                let up_gb = transmitted_diff / (1000 * 1000 * 1000);
                format!("{}: {} GB/s (down), {} GB/s (up)\n", interface_name, down_gb, up_gb)
            };
            
            data_transfer.push_str(&network_info);
        }
        
        data_transfer
    }
}