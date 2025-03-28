use sysinfo::System;

pub fn format_system_info() -> (String, String, String, String) {
    let system_name = format!("System name: {:?}\n", System::name());
    let kernel_version = format!("System kernel version: {:?}\n", System::kernel_version());
    let os_version = format!("System OS version: {:?}\n", System::os_version());
    let host_name = format!("System host name: {:?}\n", System::host_name());

    println!("{}{}{}{}", system_name, kernel_version, os_version, host_name);

    (system_name, kernel_version, os_version, host_name)
} 