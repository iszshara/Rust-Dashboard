//! This module simulates an CPU for testing purposes
/// Why cpu_info.rs exists:  
/// What it is:  
/// This is a simple struct called Cpu. It represents the basic information about a CPU that our application needs:  
/// the usage and brand name of the CPU.  
/// Why it's needed:  
/// You can't directly access sysinfo::Cpu because it's a
/// complex structure that is not easy to create or manipulate for testing.  
/// By defining our own Cpu structure, we decouple the application from the internal details of the sysinfo crate.  
/// This makes the code more flexible and, above all, testable.  

#[derive(Clone)]
pub struct Cpu {
    pub usage: f32,
    pub brand: String,
}
