use std::env;
use std::fs::File;
use std::io::{BufRead};
use std::io::BufReader;

use std::vec::Vec;
use chess::Piece;
use chess_error::ChessError;
mod posicion;
mod chess;
mod chess_error;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

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

    let (pieza1, pieza2) = match piezas.get(0..2) {
        Some([a,b]) => (a, b),
        _ => {
            println!("Error: deben haber 2 piezas, la cantidad de piezas es: {}", piezas.len());
            return Err(Box::new(ChessError::new(String::from("Error: deben haber 2 piezas"))));
        },
    };

    match (pieza1.can_kill(pieza2), pieza2.can_kill(pieza1)) {
        (true, false) => {
            if pieza1.get_color() == chess::PieceColor::White{
                println!("B");
            }else {
                println!("N");
            }
        } ,
        (false, true) => {
            if pieza2.get_color() == chess::PieceColor::White{
                println!("B");
            }else {
                println!("N");
            }
        },
        (true, true) => println!("E"),
        (false, false) => println!("P"),
    }
    Ok(())
}
