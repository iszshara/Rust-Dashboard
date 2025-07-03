//! Dokumentation für den SystemInfo Trait und seine Implementierung
/// ''' pub trait SystemInfo '''
/// Was es ist:
/// Dies ist ein Trait. In Rust definieren Traits ein Set von Verhaltensweisen (Methoden), die ein Typ implementieren kann.
/// Hier definiert man, dass jeder Typ, der SystemInfo implementiert, eine Methode get_cpus
/// (die eine Liste von  cpu_info::Cpu-Objektee zurückgibt) und global_cpu_usage (die die globale Cpu-Auslastung zurückgibt) haben muss.
///
/// ''' impl SystemInfo for System '''
/// Was es ist: Dies ist die Implementierung des SystemInfo-Traits für die konkrete sysinfo::System Struktur.
/// Hier wird definiert, wie eine echte sysinfo::System-Instanz die Methoden des SystemInfo-Traits ausführt.
/// Es wandelt die sysinfo::Cpu-Objekte in cpu_info::Cpu-Objekte um.
///
/// Warum es erstellt wurde: Dies ist der Kern der Testbarkeit. Anstatt die Funktionen direkt von sysinfo::System abhängig zu machen, werden sie von jedem Typ abhängig, der das SystemInfo-Traint implementiert (sys: &impl SystemInfo).
/// Das nennt man Dependency Injection.
/// Im Produktivcode wird sysinfo::System verwendet, aber im Testcode wird die Mock-Implementierung verwendet.
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
