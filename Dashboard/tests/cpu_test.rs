use std::alloc::System;

use super::backend::*;
use sysinfo::{Cpu, CpuExt, System, SystemExt};

#[cfg(test)]

struct MockCpu {
    usage: f32,
}

impl CpuExt for MockCpu {
    fn cpu_usage(&self) -> f32 {
        self.usage
    }
}

impl SystemExt for MockCpu {
    fn cpus(&self) -> &[sysinfo::Cpu] {
        // This is a mock implementation, returning a slice of one CPU
        &[sysinfo::Cpu::new(self.usage)]
    }
}
fn test_cpu_core_usage() {
    let num_cores = 128;
    let mut mock_cpus = Vec::new();
    for i in 0..num_cores {
        mock_cpus.push(MockCpu {
            usage: i as f32 / num_cores as f32 * 100.0,
        });
    }
}
