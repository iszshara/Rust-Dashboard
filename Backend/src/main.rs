use std::{fmt::format, thread, time::Duration};

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

        print_system_info();
        //print_total_cpu_usage(&sys);
        format_total_cpu_usage(&sys);
        print_cpu_usage(&sys);
        // print_ram_info(&sys);
        format_ram_info(&sys);
        print_number_of_cpu(&sys);
        format_disk_information();
        // print_processes_id(&sys);
        // print_network();

        thread::sleep(Duration::from_millis(1000));
        
    }

    fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn print_cpu_usage(sys: &System) {
        println!("Core Usage: ");
        for (i, cpu) in sys.cpus().iter().enumerate() {
            print!("Core {}: {:.2}% \n",
                i,
                cpu.cpu_usage()
            );
        }
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

    fn print_system_info() {
        println!("System name:              {:?}", System::name());
        println!("System kernel version:    {:?}", System::kernel_version());
        println!("System OS version:        {:?}", System::os_version());
        println!("System host name:         {:?}", System::host_name());
    }
    
    fn print_number_of_cpu(sys: &System) {
        println!("Number of CPUs: {}", sys.cpus().len());
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
