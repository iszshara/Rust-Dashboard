//! Documentation for the SystemInfo Trait and its Implementation
///
/// ''' pub trait SystemInfo '''
/// What it is:
/// This is a trait. In Rust, traits define a set of behaviors (methods) that a type can implement.
/// Here, it is defined that any type that implements SystemInfo must have a get_cpus method
/// (which returns a list of cpu_info::Cpu objects) and a global_cpu_usage method (which returns the global CPU usage).
///
/// ''' impl SystemInfo for System '''
/// What it is: This is the implementation of the SystemInfo trait for the concrete sysinfo::System struct.
/// It defines how a real sysinfo::System instance executes the methods of the SystemInfo trait.
/// It converts the sysinfo::Cpu objects into cpu_info::Cpu objects.
///
/// Why it was created: This is the core of testability. Instead of making the functions directly
/// dependent on sysinfo::System, they are made dependent on any type that implements the SystemInfo
/// trait (sys: &impl SystemInfo).
/// This is called Dependency Injection.
/// In production code, sysinfo::System is used, but in test code, the mock implementation is used.
use super::cpu_info;
use sysinfo::System;

pub trait SystemInfo {
    fn get_cpus(&self) -> Vec<cpu_info::Cpu>;
    fn global_cpu_usage(&self) -> f32;
}

impl SystemInfo for System {
    fn get_cpus(&self) -> Vec<cpu_info::Cpu> {
        self.cpus()
            .iter()
            .map(|cpu| cpu_info::Cpu {
                usage: cpu.cpu_usage(),
                brand: cpu.brand().to_string(),
            })
            .collect()
    }

    fn global_cpu_usage(&self) -> f32 {
        self.global_cpu_usage()
    }
}
