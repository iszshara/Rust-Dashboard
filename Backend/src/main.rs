mod system_info;

use system_info::{
    cpu::format_cpu_usage, 
    cpu::format_total_cpu_usage, 
    cpu::format_number_of_cpu, 
    host::format_system_info, 
    memory::format_ram_info, 
    disk::format_disk_information,
    network::format_network,
    processes::format_processes_id
};

use sysinfo::System;
use std::{thread, time::Duration, io::stdout, io::Write};

fn main() {
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        clear_screen();

        format_system_info();
        
        format_total_cpu_usage(&sys);
        format_cpu_usage(&sys);
        format_ram_info(&sys);
        format_number_of_cpu(&sys);
        format_disk_information();
        let output = format_processes_id(&sys);
        println!("{}", output);
        format_network();

        thread::sleep(Duration::from_millis(1000));
        
    }

    fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush().unwrap();

    } 
}
