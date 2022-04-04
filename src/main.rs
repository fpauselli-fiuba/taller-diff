use diff::{Config, LCS};
use std::process;

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
