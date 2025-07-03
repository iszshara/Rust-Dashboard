//! Tests für das Cpu Modul
#[cfg(test)]
/// importiert die notwendigen Module für die Tests, wobei mockall::*
/// alles aus der mockall Bibliothek importiert und die anderen Imports
/// aus dem linux_dashboard crate kommen, da diese gestestet werden sollen.
/// mock! { pub System {} }: weist mockall an, eine Mock_Struktur namens Mock System zu genrieren.
/// impl SystemInfo for System: hier sagt man mockall: generiere eine Implementierung des SystemInfo-Traits für die MockSystem-Struktur und
/// für dieses SystemInfo-Traint sind die Methoden , die gemockt werden sollen, get_cpus und global_cpu_usage mit den angegebenen Signaturen (Paramater und Rückgabetypen).
/// Warum ist das so?
/// mockall, muss wissen, welche Methoden des Traits gemockt werden sollen und welche Signaturen diese Methoden haben.
/// 1) Die Infos werden verwendet, um expect_*-Methoden zu genrieren. Für jede Methode die aufgelistet wir, generiert mockall eine entsprechene expect_*-Methode (z.B. get_cpus => expect_get_cpus).
/// Diese expect_*-Methoden sind es, mit denen man das Verhalten des Mocks definiert (z.B. was er dann im Endeffekt zurückgeben soll).
/// 2) Es wird sichergestellt, dass der Mock das Trait korrekt implementiert. Indem man die Methodensignaturen hier angibt,
/// kann mockall überprüfen, ob das Mock tatsächlich das SystemInfo-Trait korrelt implementiert hat.
/// Das ist wichtig, damit man den Mock an Funtkionen übergeben kann, die ein &impl SystemInfo erwarten.
/// 3) mockall benötigt die vollständigen Methodensignaturen, um den korrekten COde für die Implementierung zu genrieren.
/// mock! ist ein Makro.
///
/// Im #[test]:
/// Zuerst wird eine MockSystem-Instanz erstellt des hier generierten Mocks.
/// Dann wird ein Vector mit 128 simulierten CPU-Objekten erstellt
/// Im nächsten Schritt kommt der entscheidende Teil:
/// mock_system.expect_get_cpus().returning(move || cpus.clone());
/// expect_get_cpus(): sagt mockall, dass erwartet wird, dass die Methode get_cpus auf dem Mock aufgerufen werden soll.
/// .returning(move || cpus.clone()): definiert das Verhalten des Mocks, wenn get_cpus aufgerufen wird.
/// In diesem Fall soll es eine Kopie der vorbereiteten cpus-Liste zurückgeben.
/// Das move ist notwendig, da der Closure (|| cpus.clone()) den Besitz von cpus übernimmt.
/// let result = cpu::format_cpu_usage(&mock_system);
/// hier wird die eigentliche Funktion format_cpu_usage mit unserem Mock aufgerufen.
/// Die Funktion weiß nicht, dass sie mit einem Mock arbeitet, da der Mock das SystemInfo-traint implementiert.
/// Das assert! prüft, ob das Ergebnis der format_cpu_usage-Funktion die erwarteten Werte enthält.
mod tests {
    use linux_dashboard::backend::cpu;
    use linux_dashboard::backend::cpu_info;
    use linux_dashboard::backend::system_info::SystemInfo;
    use mockall::*;

    mock! {
        pub System {
            fn get_cpus(&self) -> Vec<cpu_info::Cpu>;
            fn global_cpu_usage(&self) -> f32;
        }
    }

    impl SystemInfo for MockSystem {
        fn get_cpus(&self) -> Vec<cpu_info::Cpu> {
            self.get_cpus()
        }

        fn global_cpu_usage(&self) -> f32 {
            self.global_cpu_usage()
        }
    }

    #[test]
    fn test_format_cpu_usage_with_128_cores() {
        let mut mock_system = MockSystem::new();
        let mut cpus: Vec<cpu_info::Cpu> = Vec::new();
        for i in 0..128 {
            cpus.push(cpu_info::Cpu {
                usage: i as f32,
                brand: format!("CPU Brand {}", i),
            });
        }

        mock_system
            .expect_get_cpus()
            .returning(move || cpus.clone());

        let result = cpu::format_cpu_usage(&mock_system);

        for i in 0..128 {
            assert!(result.contains(&format!("CPU {:02}: {:>5.2}%", i, i as f32)));
        }
    }

    #[test]
    fn test_format_cpu_name() {
        let mut mock_system = MockSystem::new();
        let mut cpus: Vec<cpu_info::Cpu> = Vec::new();
        cpus.push(cpu_info::Cpu {
            usage: 10.0,
            brand: "Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz".to_string(),
        });

        mock_system
            .expect_get_cpus()
            .returning(move || cpus.clone());

        let result = cpu::format_cpu_name(&mock_system);
        assert_eq!(result, "Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz");
    }

    #[test]
    fn test_format_total_cpu_usage() {
        let mut mock_system = MockSystem::new();
        mock_system.expect_global_cpu_usage().returning(|| 50.5);

        let result = cpu::format_total_cpu_usage(&mock_system);
        assert_eq!(result, "Total Usage: 50.50% ");
    }
}
