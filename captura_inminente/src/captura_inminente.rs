use chess::Piece;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec::Vec;

use crate::chess;
use crate::chess_error::ChessError;
use crate::piece_color;

pub fn run(file_path: &String) -> Result<String, Box<dyn std::error::Error>> {
    let piezas = match get_pieces_from_file(file_path) {
        Ok(pieces) => pieces,
        Err(err) => {
            return Err(err);
        }
    };

    let (pieza1, pieza2) = match piezas.get(0..2) {
        Some([a, b]) => (a, b),
        _ => {
            println!(
                "Error: deben haber 2 piezas, la cantidad de piezas es: {}",
                piezas.len()
            );
            return Err(Box::new(ChessError::new(String::from(
                "Error: deben haber 2 piezas",
            ))));
        }
    };

    Ok(get_result_of_the_battle(pieza1, pieza2))
}

/// Imprime el resultado de la batalla entre dos piezas de ajedrez.
///
/// # Argumentos
///
/// * `pieza1` - La primera pieza a comparar.
/// * `pieza2` - La segunda pieza a comparar.
///
fn get_result_of_the_battle(pieza1: &Piece, pieza2: &Piece) -> String {
    match (pieza1.can_kill(pieza2), pieza2.can_kill(pieza1)) {
        (true, false) => {
            if pieza1.get_color() == piece_color::PieceColor::White {
                "B".to_string()
            } else {
                "N".to_string()
            }
        }
        (false, true) => {
            if pieza2.get_color() == piece_color::PieceColor::White {
                "B".to_string()
            } else {
                "N".to_string()
            }
        }
        (true, true) => "E".to_string(),
        (false, false) => "P".to_string(),
    }
}

/// Obtiene dos piezas de ajedrez a partir de un tablero con formato especifico
/// dentro de un archivo.
///
/// # Argumentos
///
/// * `file_path` - La ruta del archivo que contiene las piezas.
///
fn get_pieces_from_file(file_path: &String) -> Result<Vec<Piece>, Box<dyn std::error::Error>> {
    let file = File::open(file_path).expect("Error: No se pudo abrir el archivo");
    let reader = BufReader::new(file);

    let mut x = 1;
    let mut y = 1;

    let mut piezas = vec![];

    for line in reader.lines() {
        let line = line.expect("Error: es posible leer la linea");
        for c in line.chars() {
            if c == '_' {
                x += 1;
                continue;
            }
            if c != '_' && c != ' ' {
                x += 1;
                match Piece::new(c, x, y) {
                    Ok(pieza) => {
                        piezas.push(pieza);
                    }
                    Err(error) => {
                        println!("Error: no se pudo crear la pieza: {}", error);
                        return Err(Box::new(ChessError::new(error)));
                    }
                }
            }
        }
        if x > 9 {
            println!("Error: el tablero es mayor a 8x8");
            return Err(Box::new(ChessError::new(String::from(
                "Error: el tablero es mayor a 8x8",
            ))));
        }
        y += 1;
        x = 1;
    }

    if y > 9 {
        println!("Error: el tablero es mayor a 8x8");
        return Err(Box::new(ChessError::new(String::from(
            "Error: el tablero es mayor a 8x8",
        ))));
    }

    if piezas.len() != 2 {
        println!(
            "Error: deben haber 2 piezas, la cantidad de piezas es: {}",
            piezas.len()
        );
        return Err(Box::new(ChessError::new(String::from(
            "Error: deben haber 2 piezas",
        ))));
    }

    Ok(piezas)
}

mod tests {
    use crate::captura_inminente;

    #[test]
    fn test_captura_inminente_empate_ok() {
        let result =
            captura_inminente::run(&String::from("tests/test_cases/caso-de-prueba-empate.txt"));
        assert_eq!(result.unwrap(), "E".to_string());
    }

    #[test]
    fn test_captura_inminente_ganan_blancas_ok() {
        let result = captura_inminente::run(&String::from("tests/test_cases/caso_gana_blanca.txt"));
        assert_eq!(result.unwrap(), "B".to_string());
    }

    #[test]
    fn test_captura_inminente_ganan_negras_ok() {
        let result = captura_inminente::run(&String::from("tests/test_cases/caso_gana_negra.txt"));
        assert_eq!(result.unwrap(), "N".to_string());
    }

    #[test]
    fn test_captura_inminente_no_gana_ninguna_ok() {
        let result = captura_inminente::run(&String::from(
            "tests/test_cases/caso-de-prueba-ninguna-gana.txt",
        ));
        assert_eq!(result.unwrap(), "P".to_string());
    }
    #[test]
    fn test_capture_imminent_tablero_9x9_debe_retornar_error() {
        let result = captura_inminente::run(&String::from("tests/test_cases/caso_tablero_9x9.txt"));
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_peon_blanco_captura_a_caballo_negro() {
        let result = captura_inminente::run(&String::from(
            "tests/test_cases/peon_blanco_captura_a_caballo_negro.txt",
        ));
        assert_eq!(result.unwrap(), "B".to_string());
    }

    #[test]
    fn test_peon_blanco_no_puede_capturar_hacia_abajo() {
        let result = captura_inminente::run(&String::from(
            "tests/test_cases/peon_blanco_no_puede_capturar_hacia_abajo.txt",
        ));
        assert_eq!(result.unwrap(), "P".to_string());
    }

    #[test]
    fn test_peon_negro_captura_a_torre_blanca() {
        let result = captura_inminente::run(&String::from(
            "tests/test_cases/peon_negro_captura_a_torre_blanca.txt",
        ));
        assert_eq!(result.unwrap(), "N".to_string());
    }

    #[test]
    fn test_peon_negro_no_puede_capturar_hacia_arriba() {
        let result = captura_inminente::run(&String::from(
            "tests/test_cases/peon_negro_no_puede_capturar_hacia_arriba.txt",
        ));
        assert_eq!(result.unwrap(), "P".to_string());
    }

    #[test]
    fn test_captura_inminentedebe_mas_de_dos_piezas_retornar_error() {
        let result = captura_inminente::run(&String::from("tests/test_cases/caso_3_piezas.txt"));
        assert_eq!(result.is_err(), true);
    }
}
