//! Warum cpu_info.rs existiert:
/// Was es ist: Dies ist eine einfache struct Cpu. Sie repräsentiert die grundlegenden
/// Informationen über eine CPu, die unsere Anwendung benötigt:
/// die Auslastung und den Markennamen der CPU.
/// Warum man es braucht: Man kann nicht auf sysinfo::Cpu zugreifen, da es eine
/// komplexe Struktur ist, die nicht einfach für Tests erstellt oder manipuliert werden kann.
/// Durch die Definition einer eigenen Cpu-Struktur entkoppelt man die Anwendung von internen Details des sysinfo-Crates.
/// Das macht den Code flexibler und vorallem testbar.

#[derive(Clone)]
pub struct Cpu {
    pub usage: f32,
    pub brand: String,
}
