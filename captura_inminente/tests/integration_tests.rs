use std::fs::File;
use std::process::Command;
use std::io::Write;

#[test]
fn test_captura_inminente_empate() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_cases//caso-de-prueba-empate.txt")
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "E\n");
}

#[test]
fn test_captura_inminente_ganan_blancas() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_cases//caso_gana_blanca.txt")
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "B\n");
}

#[test]
fn test_captura_inminente_ganan_negras() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_cases/caso_gana_negra.txt")
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "N\n");
}
#[test]
fn test_captura_inminente_no_gana_ninguna() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_cases/caso-de-prueba-ninguna-gana.txt")
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "P\n");
}