use std::cmp;
use std::error::Error;
use std::fs;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let archivo1 = &args[1];
    let archivo2 = &args[2];

    println!("Comparando {archivo1} con {archivo2}");

    if let Ok(lines1) = read_file_lines(archivo1) {
        if let Ok(lines2) = read_file_lines(archivo2) {
            let c = longest_common_subsequence(&lines1, &lines2);
            print_diff(&c, &lines1, &lines2, lines1.len(), lines2.len())
        }
    } else {
        println!("Error leyendo archivo");
    }
}

fn read_file_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let lines = content.lines().map(|l| l.to_string()).collect();
    Ok(lines)
}

fn longest_common_subsequence(x: &Vec<String>, y: &Vec<String>) -> Vec<Vec<i32>> {
    let m = x.len();
    let n = y.len();
    let mut c: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];

    for (i, _) in x.iter().enumerate().take(m) {
        for (j, _) in y.iter().enumerate().take(n) {
            if x[i] == y[j] {
                c[i + 1][j + 1] = c[i][j] + 1;
            } else {
                c[i + 1][j + 1] = cmp::max(c[i + 1][j], c[i][j + 1]);
            }
        }
    }
    c
}

fn print_diff(c: &Vec<Vec<i32>>, x: &Vec<String>, y: &Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
        print_diff(c, x, y, i - 1, j - 1);
        println!(" {}", x[i - 1]);
    } else if j > 0 && (i == 0 || c[i][j - 1] > c[i - 1][j]) {
        print_diff(c, x, y, i, j - 1);
        println!("> {}", y[j - 1]);
    } else if i > 0 && (j == 0 || c[i][j - 1] < c[i - 1][j]) {
        print_diff(c, x, y, i - 1, j);
        println!("< {}", x[i - 1]);
    } else {
        println!("")
    }
}
