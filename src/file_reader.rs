use std::error::Error;
use std::fs;
pub struct FileReader {}

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
