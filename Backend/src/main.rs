pub mod backend;
pub mod ui;

use backend::{
    cpu::format_cpu_usage, 
    cpu::format_total_cpu_usage, 
    cpu::format_number_of_cpu, 
    host::format_system_info, 
    memory::format_ram_info, 
    disk::format_disk_information,
    network::format_network,
    processes::format_processes_id
};

use crossterm::style::Color;
use sysinfo::System;
use std::{thread, time::Duration, io::stdout, io::Write};

use ui::app;
use color_eyre::Result;

/// refreshes the system every 1000ms and calculates all the called functions from the other files
fn main() -> color_eyre::Result<()> {

    ui::app::run_ui()
    // let mut sys = System::new_all();

    // loop {
    //     sys.refresh_all();

    //     clear_screen();

    //     format_system_info();
        
    //     format_total_cpu_usage(&sys);
    //     format_cpu_usage(&sys);
    //     format_ram_info(&sys);
    //     format_number_of_cpu(&sys);
    //     format_disk_information();
    //     let output = format_processes_id(&sys);
    //     println!("{}", output);
    //     format_network();

    //     thread::sleep(Duration::from_millis(1000));
        
    // }

// Clears the screen and flushes the standard input and output

    // fn clear_screen() {
    //     print!("\x1B[2J\x1B[1;1H");
    //     stdout().flush().unwrap();

    // } 
}
