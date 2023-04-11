use std::env;

mod captura_inminente;
mod chess;
mod chess_error;
mod piece_color;
mod piece_type;
mod position;

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

    match captura_inminente::run(file_path) {
        Ok(result) => {
            println!("{}", result)
        }
        Err(err) => {
            return Err(err);
        }
    }
    Ok(())
}
