// Tenta converter um numero string para um i32
pub fn ler_numero(texto: &str) -> Result<i32, String> {
    match texto.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(erro) => Err(String::from(format!("Erro ao converter {} para int32 {}", texto, erro))),
    }
}