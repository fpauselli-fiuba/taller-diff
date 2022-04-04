use std::cmp;
use std::env;
use std::error::Error;
use std::fs;

/// Struct de Configuración
///
/// En su interior, se guardan los parámetros ingresados por línea de comandos
/// - file1: path al primer archivo
/// - file2: path al segundo archivo
pub struct Config {
    pub file1: String,
    pub file2: String,
}

impl Config {
    /// Genera una configuración válida, o devuelve un error de argumentos insufificentes
    ///
    /// # Ejemplo de uso:
    /// ```rust
    /// let maybeConfig = Config::new();
    /// if(Ok(config)) {
    ///   // --snip--
    /// } else {
    ///  // --handle error--
    /// }
    /// ```
    pub fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() != 3 {
            return Err("Se requieren dos argumentos");
        }
        let file1 = args[1].clone();
        let file2 = args[2].clone();
        Ok(Config { file1, file2 })
    }
}

/// Struct con la información y lógica para calcular el algoritmo LCS
///
/// En su interior, se guardan los siguientes atributos:
///
/// - lines_file1: Vector con las líneas del primer archivo,
/// - len_file1: Cantidad de líneas del primer archivo,
/// - lines_file2: Vector con las líneas del segundo archivo,
/// - len_file2: Cantidad de líneas del segundo archivo,
/// - lcs_grid: Grilla computada del algoritmo LCS,
pub struct LCS {
    lines_file1: Vec<String>,
    len_file1: usize,
    lines_file2: Vec<String>,
    len_file2: usize,
    lcs_grid: Vec<Vec<i32>>,
}

impl LCS {
    /// Genera un LCS válido. En caso de no encontrar alguno de los archivos, devuelve error de lectura.
    ///
    /// Parámetros:
    /// - `config: Config` Struct de configuración válido
    ///
    /// La función se encarga de buscar y leer los archivos pasados en config, y calcular la grilla de LCS
    pub fn new(config: Config) -> Result<LCS, Box<dyn Error>> {
        let lines_file1 = FileReader::read_file_lines(&config.file1)?;
        let lines_file2 = FileReader::read_file_lines(&config.file2)?;
        let lcs_grid = Self::longest_common_subsequence(&lines_file1, &lines_file2);
        Ok(LCS {
            len_file1: lines_file1.len(),
            len_file2: lines_file2.len(),
            lines_file1,
            lines_file2,
            lcs_grid,
        })
    }

    /// Función encargada de calcular el LCS.
    ///
    /// Parámetros:
    /// - `lines_file1: &[String]` Vector con las líneas del primer archivo
    /// - `lines_file2: &[String]` Vector con las líneas del segundo archivo
    ///
    /// Retorno:
    /// - `Vec<Vec<i32>>` Grilla computada del algoritmo LCS
    fn longest_common_subsequence(lines_file1: &[String], lines_file2: &[String]) -> Vec<Vec<i32>> {
        let len_file1 = lines_file1.len();
        let len_file2 = lines_file2.len();
        let mut lcs_grid: Vec<Vec<i32>> = vec![vec![0; len_file2 + 1]; len_file1 + 1];

        for (i, _) in lines_file1.iter().enumerate().take(len_file1) {
            for (j, _) in lines_file2.iter().enumerate().take(len_file2) {
                if lines_file1[i] == lines_file2[j] {
                    lcs_grid[i + 1][j + 1] = lcs_grid[i][j] + 1;
                } else {
                    lcs_grid[i + 1][j + 1] = cmp::max(lcs_grid[i + 1][j], lcs_grid[i][j + 1]);
                }
            }
        }
        lcs_grid
    }

    /// Función encargada de imprimir por salida estándar el diff entre los dos archivos
    pub fn print_diff(&self) {
        self.print_diff_place(self.len_file1, self.len_file2)
    }

    /// Función auxiliar recursiva que imprime por salida estándar el diff entre los dos archivos
    fn print_diff_place(&self, i: usize, j: usize) {
        if i > 0 && j > 0 && self.lines_file1[i - 1] == self.lines_file2[j - 1] {
            self.print_diff_place(i - 1, j - 1);
            println!(" {}", self.lines_file1[i - 1]);
        } else if j > 0 && (i == 0 || self.lcs_grid[i][j - 1] > self.lcs_grid[i - 1][j]) {
            self.print_diff_place(i, j - 1);
            println!("> {}", &self.lines_file2[j - 1]);
        } else if i > 0 && (j == 0 || self.lcs_grid[i][j - 1] < self.lcs_grid[i - 1][j]) {
            self.print_diff_place(i - 1, j);
            println!("< {}", self.lines_file1[i - 1]);
        } else {
            println!();
        }
    }
}

struct FileReader {}

impl FileReader {
    /// Función de lectura de archivos. En caso de no existir el archivo, devuelve un error de lectura.
    ///
    /// Parámetros:
    /// - `path: &str` Ruta o path al archivo
    ///
    /// Retorno correcto:
    /// - `Vec<String>` Vector cuyo cada elemento es una línea del archivo leído
    ///
    /// Error:
    /// - `Box<dyn Error>`
    pub fn read_file_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let lines = content.lines().map(|l| l.to_string()).collect();
        Ok(lines)
    }
}
