use std::process;

use config::Config;
use lcs::Lcs;

mod config;
mod file_reader;
mod lcs;

/// Función run que ejecuta el programa
/// 1) lee los parámetros pasados por línea de comandos, y verifica que sean exactamente dos
/// 2) Busca un archivo con el valor de cada parámetro pasado por línea de comandos, y verifica que sean archivos válidos
/// 3) Calcula el longest_common_subsequence
/// 4) Imprime el longest_common_subsequence
pub fn run() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("\x1b[31mError: {}\x1b[0m", err);
        process::exit(1)
    });
    let longest_common_subsequence = Lcs::new(config).unwrap_or_else(|err| {
        eprintln!("\x1b[31mError leyendo archivos: {}\x1b[0m", err);
        process::exit(1);
    });
    longest_common_subsequence.print_diff()
}
