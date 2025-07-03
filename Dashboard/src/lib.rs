//! Was lib.rs macht:
/// Dies ist die Hauptbibliotheksdatei für das Dashboard-Projekt.
/// Während mod.rs nur die Module für die Benutzeroberfläche bereitstellt, ermöglicht lib.rs das verwenden von der
/// Backend-Logik und der UI-Logik in der main.rs Datei.
#[allow(non_snake_case)]
pub mod backend;
pub mod ui;
