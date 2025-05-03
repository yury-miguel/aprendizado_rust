use rust_aprendizado::exercicios::calculadora_testes::dividir;

#[test]
fn teste_1() {
    assert_eq!(dividir(10, 2), Ok(5));
}

#[test]
fn teste_2() {
    assert_eq!(dividir(2, 0), Err("Divisao por Zero".to_string()));
}