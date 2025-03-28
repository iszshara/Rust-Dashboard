use sysinfo::Networks;

pub fn format_network() -> String {
    let mut data_transfer = String::new();
    let networks = Networks::new_with_refreshed_list();

    for (interface_name, data) in &networks {
        let network_info = format!(
            "{interface_name}: {} B (down) / {} B (up)",
            data.total_received(),
            data.total_transmitted(),
        );
        data_transfer.push_str(&network_info);
        println!("{}", network_info);
    }

    data_transfer
}