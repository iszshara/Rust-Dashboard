//! This module fetches Host Information

use sysinfo::System;

/// Returns the system name of the Linux Distribution, kernel version of the Linux Distribution, the OS Version the Linux is currently on and the hostname of the system
/// formatted as a string 

pub fn format_system_info() -> (String, String, String, String) {
    let system_name = format!("System name: {:?}\n", System::name());
    let kernel_version = format!("System kernel version: {:?}\n", System::kernel_version());
    let os_version = format!("System OS version: {:?}\n", System::os_version());
    let host_name = format!("System host name: {:?}\n", System::host_name());

    println!("{}{}{}{}", system_name, kernel_version, os_version, host_name);

    (system_name, kernel_version, os_version, host_name)
} 