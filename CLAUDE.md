# Rust System Dashboard

Terminal-basiertes System-Monitoring-Dashboard. Erstes Rust-Projekt, inspiriert durch @orhundev's TUI-Videos.

## Architektur

```
Dashboard/src/
  main.rs              # Entry point, tokio async runtime
  lib.rs               # Re-exports backend + ui Module
  backend/
    cpu.rs             # CPU-Auslastung formatieren (nutzt SystemInfo Trait)
    cpu_info.rs        # Eigene Cpu-Struct fuer Testbarkeit (entkoppelt von sysinfo)
    system_info.rs     # SystemInfo Trait + impl fuer sysinfo::System (Dependency Injection)
    memory.rs          # RAM/Swap Info als ratatui Table
    network.rs         # NetworkManager: Traffic-Tracking, History, Live-Chart
    processes.rs       # Prozess-Liste mit Sortierung, Kill-Funktion, Suchfilter
    host.rs            # Host-Info (OS, Kernel, Hostname) mit Caching (HostInfo struct)
    disk.rs            # Disk-Info (existiert, wird aber noch nicht in der UI angezeigt)
    converter.rs       # byte_to_gib() und format_bytes() Helper
  ui/
    app.rs             # Haupt-App-Logik: Event-Loop, Rendering, State-Management
    layout.rs          # Terminal-Layout (Gauge oben, Links/Rechts Split unten)
  tests/
    cpu_test.rs        # CPU-Tests mit mockall (MockSystem)
```

## Kernkonzepte

- **Async Data Fetching**: Background-tokio-Task refresht System-Daten, UI liest ueber `Arc<Mutex<System>>`
- **Watch-Channel**: `tokio::sync::watch` kommuniziert das Fetch-Interval zum Background-Task
- **Testbarkeit**: `SystemInfo`-Trait ermoeglicht Mocking der sysinfo-Abhaengigkeit
- **NetworkManager**: Eigene Netzwerk-Verwaltung mit History (50 Datenpunkte), automatischer Skalierung (B/KB/MB/GB), Braille-Chart

## Modi / Keybindings

- **Normal Mode**: Standard-Navigation (q, Tab, Pfeiltasten, c/m/p/n fuer Sortierung, i fuer Interface)
- **Input Mode** (Shift+M): PID eingeben zum Killen eines Prozesses
- **Search Mode** (/): Prozesse nach Name filtern

## Was bereits umgesetzt ist

### Vor dem Refactoring (bis ~August 2025)
- CPU-Gauge (Gesamtauslastung) + CPU-Core-Liste mit Scrollbar
- Memory-Table (RAM + Swap)
- Netzwerk-Traffic-Anzeige + Live-Chart mit Download/Upload
- Prozess-Liste mit Sortierung (CPU, Memory, PID, Name - jeweils Asc/Desc)
- Host-Info-Block (OS, Kernel, Hostname)
- Welcome-Popup beim Start
- Manual/Help-Overlay (Esc)
- Einstellbares Fetch-Interval (Links/Rechts Pfeiltasten, 100ms-60000ms)
- Minimum-Window-Size-Check mit farbiger Fehlermeldung
- Kill-Prozess-Funktion (Shift+M)
- Scrollbars fuer CPU und Prozesse
- System-Uptime-Anzeige
- Unit-Tests fuer CPU-Modul mit mockall

### Refactoring-Session (April 2026)
- render()-Methode von ~380 Zeilen in 10 Einzelmethoden aufgeteilt
- Tick-Rate-Bug gefixt (aktualisiert sich jetzt zur Laufzeit)
- println! aus disk.rs entfernt (haette TUI korrumpiert)
- Netzwerk-Einheiten von 1000er auf 1024er vereinheitlicht
- kill_process() nutzt jetzt existierendes System statt neues zu erstellen
- Input-Mode Bug gefixt (Enter schloss gleichzeitig Popup UND killte Prozess)
- unwrap() in get_network_widget() durch unwrap_or() ersetzt
- Async Data Fetching mit tokio implementiert (Background-Task + Arc<Mutex> + watch-Channel)
- sys.refresh_all() aus format_network() entfernt (verursachte CPU-Spikes bei jedem Render)
- Mouse-Events werden ignoriert (EnableMouseCapture entfernt)
- Netzwerk-Text wird gecached (nur bei Tick aktualisiert, nicht bei jedem Render)
- truncate_string() UTF-8 safe gemacht (char-basiert statt byte-basiert)
- lipsum Dependency entfernt (wurde nicht genutzt)
- format_bytes() Helper fuer dynamische Memory-Einheiten (KB/MB/GB statt immer GB)
- Scroll-Begrenzung fuer CPU und Prozesse (kann nicht mehr ueber das Ende scrollen)
- host_info_table() wird gecached (HostInfo struct, einmal erstellt statt bei jedem Frame)
- Prozess-Suche/Filter implementiert (/ zum Suchen, filtert nach Name)
- Kill-Schutz: PID 0/1 und eigene PID werden abgelehnt
- Kill nutzt SIGTERM statt SIGKILL (graceful shutdown)
- Mutex-Poisoning wird graceful gehandelt (unwrap_or_else statt unwrap)
- Panic-Hook fuer Terminal-Cleanup (Raw-Mode wird bei Panic restored)
- PID-Eingabe auf 10 Ziffern begrenzt
- .gitignore gefixt (.idea/ statt idea/, Cargo.lock nicht mehr ignoriert, .DS_Store/.env hinzugefuegt)

## Offene Punkte / Ideen

- disk.rs existiert aber wird nicht in der UI angezeigt
- CPU-History-Graph (wie beim Netzwerk, Chart ueber Zeit)
- Memory-Gauge (Pendant zum CPU-Gauge)
- Prozess-Auswahl mit Cursor statt PID-Eingabe
- Mehr Tests (Memory, Network, Processes, converter)
- Doc-Tests sind teilweise kaputt (pre-existing, nicht durch Refactoring verursacht)
- cross Dependency in Cargo.toml pruefen (wird die gebraucht?)
