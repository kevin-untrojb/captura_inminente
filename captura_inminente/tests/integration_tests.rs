use std::process::Command;

#[test]
fn test_captura_inminente_empate_ok() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_cases//caso-de-prueba-empate.txt")
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "E\n");
}

#[test]
fn test_captura_inminente_ganan_blancas_ok() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_cases//caso_gana_blanca.txt")
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "B\n");
}

#[test]
fn test_captura_inminente_ganan_negras_ok() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_cases/caso_gana_negra.txt")
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "N\n");
}

#[test]
fn test_captura_inminente_no_gana_ninguna_ok() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_cases/caso-de-prueba-ninguna-gana.txt")
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "P\n");
}

#[test]
fn test_capture_imminent_tablero_9x9_debe_retornar_error() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_cases/caso_tablero_9x9.txt")
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), false);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Error: el tablero es mayor a 8x8\n");
}

#[test]
fn test_captura_inminentedebe_mas_de_dos_piezas_retornar_error() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_cases/caso_3_piezas.txt")
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), false);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Error: deben haber 2 piezas, la cantidad de piezas es: 3\n");
}