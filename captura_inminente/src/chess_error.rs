/// La estructura ChessError representa una el error
/// producido durante la ejecución de captura inminente
#[derive(Debug)]
pub struct ChessError {
    pub message: String,
}
impl ChessError {
    /// Crea un nuevo error de ajedrez con el mensaje dado.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// let error = ChessError::new(String::from("Movimiento no válido"));
    /// ```
    pub fn new(msg: String) -> ChessError {
        ChessError{message:msg}
    }
}
impl std::fmt::Display for ChessError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ChessError {}