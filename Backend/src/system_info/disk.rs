use sysinfo::Disks;
use super::converter::kib_to_gib;

pub fn format_disk_information() -> String {
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