use std::{fmt::format, os, thread, time::Duration};

use sysinfo::{
    Components, Disks, Networks, System,
};

fn kib_to_gib(kib: u64) -> f64 {
    kib as f64 / 1_073_741_824.0
}

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
        // print_processes_id(&sys);
        // print_network();

        thread::sleep(Duration::from_millis(1000));
        
    }

    fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn format_cpu_usage(sys: &System) -> String{
        println!("Core Usage: ");
        sys.cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| format!("CPU {:02}: {:>5.2}%\n", i, cpu.cpu_usage()))
            .collect::<String>()
    }

    fn format_total_cpu_usage(sys: &System) -> String {
        let total_cpu_usage = format!("Total CPU Usage: {:.2}% ", sys.global_cpu_usage());
        println!("{}", total_cpu_usage);
        total_cpu_usage
    }

    fn format_ram_info(sys: &System) -> (String, String, String, String){

        let ram_total_memory = format!("Total memory: {:.2} GB", kib_to_gib(sys.total_memory()));
        let ram_used_memory = format!("Used Memory: {:.2} GB", kib_to_gib(sys.used_memory()));
        let ram_total_swap = format!("Total Swap: {:.2} GB", kib_to_gib(sys.total_swap()));
        let ram_used_swap = format!("Used Swap: {:.2} GB", kib_to_gib(sys.used_swap()));

        println!("{}, {}, {}, {}", ram_total_memory, ram_used_memory, ram_total_swap, ram_used_swap);

        (ram_total_memory, ram_used_memory, ram_total_swap, ram_used_swap)
        
    }

    fn format_system_info() -> (String, String, String, String) {
        let system_name = format!("System name: {:?}\n", System::name());
        let kernel_version = format!("System kernel version: {:?}\n", System::kernel_version());
        let os_version = format!("System OS version: {:?}\n", System::os_version());
        let host_name = format!("System host name: {:?}\n", System::host_name());

        println!("{}{}{}{}", system_name, kernel_version, os_version, host_name);

        (system_name, kernel_version, os_version, host_name)
    } 
    
    fn format_number_of_cpu(sys: &System) -> String{
        let number_of_cpus = format!("Number of CPUs: {}", sys.cpus().len().to_string());
        println!("{}", number_of_cpus);
        number_of_cpus
    }

    fn format_disk_information() -> String {
        let mut result = String::new(); 
        let disks = Disks::new_with_refreshed_list();
        
        for disk in disks.list() {
            let disk_info = format!(
                "[{:?}] Total Space: {:.2} GB | Available Space: {:.2} GB\n",
                disk.name(),
                kib_to_gib(disk.total_space()),
                kib_to_gib(disk.available_space())
            );
            result.push_str(&disk_info);
            println!("{}", disk_info);
        }
        

        result 
    }
    
    // fn print_processes_id(sys: &System) {
    //     sys.process(pid)
    // }
    // fn print_processes_id(sys: &System) {
    //     for (pid, process) in sys.processes() {
    //         println!("[{pid}] Name: {:?} | Status: {:?} | CPU Usage: {:.2?}% | Memory Usage: {:.2?} GB", 
    //             process.name(),
    //             process.status(),
    //             process.cpu_usage(),
    //             kib_to_gib(process.memory())
    //         );
    //     }
    // }

    // fn print_network() {
    //     let networks = Networks::new_with_refreshed_list();
    //     println!("=> networks:");
    //     for (interface_name, data) in &networks {
    //         println!(
    //             "{interface_name}: {} B (down) / {} B (up)",
    //             data.total_received(),
    //             data.total_transmitted(),
    //         );
    //     }
    // }

}
