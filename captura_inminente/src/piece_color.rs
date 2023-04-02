/// Representa el color de una pieza de ajedrez.
#[derive(Copy, Clone)]
pub enum PieceColor {
    White,
    Black,
}
impl PartialEq for PieceColor {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (PieceColor::White, PieceColor::White) | (PieceColor::Black, PieceColor::Black))
    }
}