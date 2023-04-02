#[derive(Debug)]
pub struct ChessError {
    pub message: String,
}
impl ChessError {
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