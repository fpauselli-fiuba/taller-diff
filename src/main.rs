use diff::{Config, LCS};
use std::process;

/// # Diff utilizando el algoritmo LCS
///
/// Programa básico para calcular la subsecuencia máß larga entre dos archivos con el algoritmo LCS
///
/// ## Modo de uso:
/// ```bash
/// $ cargo run [archivo1] [archivo2]
/// ```
fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
    let longest_common_subsequence = LCS::new(config).unwrap_or_else(|err| {
        eprintln!("Error leyendo archivos: {}", err);
        process::exit(1);
    });
    longest_common_subsequence.print_diff()
}
