use std::error::Error;
use std::fs;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let archivo1 = &args[1];
    let archivo2 = &args[2];

    println!("Comparando {archivo1} con {archivo2}");

    if let Ok(lines) = read_file_lines(archivo1) {
        println!("{:?}", lines);
    } else {
        println!("Error leyendo archivo");
    }
}

fn read_file_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?.clone();
    let lines = content.lines().map(|l| l.to_string()).collect();
    Ok(lines)
}
