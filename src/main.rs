pub mod config;
pub mod file_reader;
pub mod lcs;

use std::process;

use crate::config::Config;
use crate::lcs::LCS;

/// # Diff utilizando el algoritmo LCS
///
/// Programa básico para calcular la subsecuencia más larga entre dos archivos con el algoritmo LCS
///
/// ## Modo de uso:
/// ```bash
/// $ cargo run [archivo1] [archivo2]
/// ```
fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("\x1b[31mError: {}\x1b[0m", err);
        process::exit(1);
    });
    let longest_common_subsequence = LCS::new(config).unwrap_or_else(|err| {
        eprintln!("\x1b[31mError leyendo archivos: {}\x1b[0m", err);
        process::exit(1);
    });
    longest_common_subsequence.print_diff()
}
