use std::env;
use std::fs::File;
use std::io::{BufRead};
use std::io::BufReader;

use std::vec::Vec;
use chess::Piece;
use chess_error::ChessError;

mod position;
mod chess;
mod chess_error;
mod piece_color;
mod piece_type;

/// # Captura inminente
///
/// Este es un programa de ejemplo para determinar si dos piezas en un tablero de ajedrez pueden atacarse mutuamente.
/// Solo se admiten 2 piezas y el tablero debe ser de 8x8
///
/// ## Uso
///
/// ```
/// cargo run -- archivo_de_tablero.txt
/// ```
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let piezas = match get_pieces_from_file(file_path) {
        Ok(pieces) => pieces,
        Err(err) => {
            return Err(err);
        },
    };

    let (pieza1, pieza2) = match piezas.get(0..2) {
        Some([a,b]) => (a, b),
        _ => {
            println!("Error: deben haber 2 piezas, la cantidad de piezas es: {}", piezas.len());
            return Err(Box::new(ChessError::new(String::from("Error: deben haber 2 piezas"))));
        },
    };

    print_result_of_the_battle(pieza1,pieza2);

    Ok(())
}

/// Imprime el resultado de la batalla entre dos piezas de ajedrez.
///
/// # Argumentos
///
/// * `pieza1` - La primera pieza a comparar.
/// * `pieza2` - La segunda pieza a comparar.
///
fn print_result_of_the_battle(pieza1:&Piece, pieza2:&Piece){
    match (pieza1.can_kill(pieza2), pieza2.can_kill(pieza1)) {
        (true, false) => {
            if pieza1.get_color() == piece_color::PieceColor::White{
                println!("B");
            }else {
                println!("N");
            }
        } ,
        (false, true) => {
            if pieza2.get_color() == piece_color::PieceColor::White{
                println!("B");
            }else {
                println!("N");
            }
        },
        (true, true) => println!("E"),
        (false, false) => println!("P"),
    }
}

/// Obtiene dos piezas de ajedrez a partir de un tablero con formato especifico
/// dentro de un archivo.
///
/// # Argumentos
///
/// * `file_path` - La ruta del archivo que contiene las piezas.
///
fn get_pieces_from_file(file_path:&String) -> Result<Vec<Piece>, Box<dyn std::error::Error>>{
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
                continue
            }
            if c != '_' && c != ' '{
                x += 1;
                match Piece::new(c,x,y) {
                    Ok(pieza) => {piezas.push(pieza);}
                    Err(error) => {
                        println!("Error: no se pudo crear la pieza: {}",error);
                        return Err(Box::new(ChessError::new(error)));
                    }
                }
            }
        }
        if x > 9 {
            println!("Error: el tablero es mayor a 8x8");
            return Err(Box::new(ChessError::new(String::from("Error: el tablero es mayor a 8x8"))));
        }
        y += 1;
        x = 1;
    }

    if y > 9 {
        println!("Error: el tablero es mayor a 8x8");
        return Err(Box::new(ChessError::new(String::from("Error: el tablero es mayor a 8x8"))));
    }

    if piezas.len() != 2 {
        println!("Error: deben haber 2 piezas, la cantidad de piezas es: {}",piezas.len());
        return Err(Box::new(ChessError::new(String::from("Error: deben haber 2 piezas"))));
    }

    Ok(piezas)
}
