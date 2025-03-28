use sysinfo::System;

pub fn format_cpu_usage(sys: &System) -> String{
    println!("Core Usage: ");
    sys.cpus()
        .iter()
        .enumerate()
        .map(|(i, cpu)| format!("CPU {:02}: {:>5.2}%\n", i, cpu.cpu_usage()))
        .collect::<String>()
}

pub fn format_total_cpu_usage(sys: &System) -> String {
    let total_cpu_usage = format!("Total CPU Usage: {:.2}% ", sys.global_cpu_usage());
    println!("{}", total_cpu_usage);
    total_cpu_usage
}

pub fn format_number_of_cpu(sys: &System) -> String{
    let number_of_cpus = format!("Number of CPUs: {}", sys.cpus().len().to_string());
    println!("{}", number_of_cpus);
    number_of_cpus
}