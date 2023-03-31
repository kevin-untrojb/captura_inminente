use std::env;
use std::fs::File;
use std::io::{BufRead};
use std::io::BufReader;

use std::vec::Vec;
use chess::Piece;

mod posicion;
mod chess;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = File::open(file_path).expect("Error: No se pudo abrir el archivo");

    let reader = BufReader::new(file);

    let mut x = 1;
    let mut y = 1;

    let mut piezas = vec![];



    for line in reader.lines() {
        x += 1;
        let line = line.expect("Error: no se pudo leer la linea");
        for c in line.chars() {
            if c == '_' {
                y += 1;
            }
            if c != '_' && c != ' '{
                x += 1;
                match Piece::new(c,x,y) {
                    Ok(pieza) => {piezas.push(pieza);}
                    Err(error) => {
                        println!("Error: no se pudo crear la pieza: {}",error);
                        return;
                    }
                }
            }
        }
    }

    if piezas.len() != 2 {
        println!("Error: deben haber 2 piezas, la cantidad de piezas es: {}",piezas.len());
        return;
    }

    let (pieza1, pieza2) = match piezas.get(0..2) {
        Some(&[ref a, ref b]) => (a, b),
        _ => {
            println!("Error: deben haber 2 piezas, la cantidad de piezas es: {}", piezas.len());
            return;
        },
    };

    match (pieza1.can_kill(&pieza2), pieza2.can_kill(&pieza1)) {
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

}
