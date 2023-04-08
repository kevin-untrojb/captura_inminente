use crate::piece_color::PieceColor;
use crate::piece_type::PieceType;
use crate::position::Position;

/// Representa una pieza de ajedrez, la cual tiene una posición en el tablero, un tipo y un color.
pub struct Piece {
    position: Position,
    piece_type: PieceType,
    color: PieceColor,
}

impl Piece {
    /// Crea una nueva pieza de ajedrez con el tipo de pieza especificado, en la posición especificada.
    ///
    /// # Arguments
    ///
    /// * `piece_type.rs` - El tipo de pieza de ajedrez, representado como un carácter.
    /// * `x` - La posición horizontal de la pieza en el tablero.
    /// * `y` - La posición vertical de la pieza en el tablero.
    ///
    /// # Returns
    ///
    /// Si se pudo crear la pieza, se devuelve Ok con la nueva pieza. Si se encontró un error, se devuelve Err con una descripción del error.
    pub fn new(piece_type: char, x: usize, y: usize) -> Result<Piece, String> {
        let piece_color = if piece_type.is_uppercase() {
            PieceColor::Black
        } else {
            PieceColor::White
        };

        let match_type = match piece_type.to_ascii_lowercase() {
            'r' => PieceType::Rey,
            'd' => PieceType::Dama,
            'a' => PieceType::Alfil,
            'c' => PieceType::Caballo,
            't' => PieceType::Torre,
            'p' => PieceType::Peon,
            _ => return Err("ERROR: pieza no valida".to_string()),
        };

        Ok(Piece {
            position: Position::new(x, y),
            piece_type: match_type,
            color: piece_color,
        })
    }

    /// Devuelve el color de la pieza.
    pub fn get_color(&self) -> PieceColor {
        self.color
    }

    /// Indica si esta pieza puede matar a otra pieza especificada como opponent.
    ///
    /// # Arguments
    ///
    /// * `opponent` - La pieza que se quiere comprobar si se puede matar.
    ///
    /// # Returns
    ///
    /// true si esta pieza puede matar a la pieza especificada, false si no puede.
    pub fn can_kill(&self, opponent: &Self) -> bool {
        let (dx, dy) = self.position.get_dx_dy(opponent.position);
        match self.piece_type {
            PieceType::Rey => {
                if dx <= 1 && dy <= 1 {
                    return true;
                }
            }
            PieceType::Dama => {
                if dx == dy
                    || self.position.x == opponent.position.x
                    || self.position.y == opponent.position.y
                {
                    return true;
                }
            }
            PieceType::Torre => {
                if self.position.x == opponent.position.x || self.position.y == opponent.position.y
                {
                    return true;
                }
            }
            PieceType::Alfil => {
                if dx == dy {
                    return true;
                }
            }
            PieceType::Caballo => {
                if (dx == 1 && dy == 2) || (dx == 2 && dy == 1) {
                    return true;
                }
            }
            PieceType::Peon => {
                if self.color == PieceColor::Black {
                    if dx == 1 && opponent.position.y == self.position.y + 1 {
                        return true;
                    }
                } else if dx == 1 && opponent.position.y == self.position.y - 1 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_kill_rey() {
        let piece1 = Piece {
            position: Position { x: 3, y: 3 },
            piece_type: PieceType::Rey,
            color: PieceColor::Black,
        };
        let piece2 = Piece {
            position: Position { x: 2, y: 2 },
            piece_type: PieceType::Rey,
            color: PieceColor::White,
        };
        assert_eq!(piece1.can_kill(&piece2), true);
    }

    #[test]
    fn test_can_kill_dama() {
        let piece1 = Piece {
            position: Position { x: 3, y: 3 },
            piece_type: PieceType::Dama,
            color: PieceColor::Black,
        };
        let piece2 = Piece {
            position: Position { x: 4, y: 4 },
            piece_type: PieceType::Rey,
            color: PieceColor::White,
        };
        assert_eq!(piece1.can_kill(&piece2), true);
    }

    #[test]
    fn test_can_kill_torre() {
        let piece1 = Piece {
            position: Position { x: 3, y: 3 },
            piece_type: PieceType::Torre,
            color: PieceColor::Black,
        };
        let piece2 = Piece {
            position: Position { x: 3, y: 7 },
            piece_type: PieceType::Rey,
            color: PieceColor::White,
        };
        assert_eq!(piece1.can_kill(&piece2), true);
    }

    #[test]
    fn test_can_kill_alfil() {
        let piece1 = Piece {
            position: Position { x: 3, y: 3 },
            piece_type: PieceType::Alfil,
            color: PieceColor::Black,
        };
        let piece2 = Piece {
            position: Position { x: 4, y: 4 },
            piece_type: PieceType::Rey,
            color: PieceColor::White,
        };
        assert_eq!(piece1.can_kill(&piece2), true);
    }

    #[test]
    fn test_can_kill_caballo() {
        let piece1 = Piece {
            position: Position { x: 3, y: 3 },
            piece_type: PieceType::Caballo,
            color: PieceColor::Black,
        };
        let piece2 = Piece {
            position: Position { x: 5, y: 4 },
            piece_type: PieceType::Rey,
            color: PieceColor::White,
        };
        assert_eq!(piece1.can_kill(&piece2), true);
    }

    #[test]
    fn test_can_kill_peon() {
        let piece1 = Piece {
            position: Position { x: 3, y: 3 },
            piece_type: PieceType::Peon,
            color: PieceColor::White,
        };
        let piece2 = Piece {
            position: Position { x: 4, y: 2 },
            piece_type: PieceType::Rey,
            color: PieceColor::Black,
        };
        assert_eq!(piece1.can_kill(&piece2), true);
    }
}
