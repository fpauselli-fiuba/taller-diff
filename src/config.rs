use std::env;
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
