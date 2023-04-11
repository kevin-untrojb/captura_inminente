use std::process::Command;

#[test]
fn test_captura_inminente_ganan_blancas_ok() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("tests/test_cases/caso_gana_blanca.txt")
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "B\n");
}
