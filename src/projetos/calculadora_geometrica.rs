pub fn calcular_area(texto: &str) -> Result<f64, String> {
    let separacao: Vec<&str> = texto.split(':').collect();

    if separacao.len() < 2 {
        return Err(String::from("Entrada inválida: faltando tipo ou valores"));
    }

    match separacao[0].to_lowercase().as_str() { // Case-insensitive
        "circulo" => {
            if let Some(raio_str) = separacao.get(1) {
                match raio_str.parse::<f64>() {
                    Ok(raio) => {
                        if raio < 0.0 {
                            Err(String::from("Raio não pode ser negativo"))
                        } else {
                            Ok(std::f64::consts::PI * raio * raio)
                        }
                    }
                    Err(_) => Err(String::from("Raio inválido: não é um número")),
                }
            } else {
                Err(String::from("Faltando raio para o círculo"))
            }
        }
        "retangulo" => {
            if let Some(valores) = separacao.get(1) {
                let dimensoes: Vec<&str> = valores.split(',').collect();
                if dimensoes.len() != 2 {
                    return Err(String::from("Retângulo precisa de largura e altura separadas por vírgula"));
                }
                match (dimensoes[0].parse::<f64>(), dimensoes[1].parse::<f64>()) {
                    (Ok(largura), Ok(altura)) => {
                        if largura < 0.0 || altura < 0.0 {
                            Err(String::from("Largura e altura não podem ser negativas"))
                        } else {
                            Ok(largura * altura)
                        }
                    }
                    _ => Err(String::from("Valores inválidos para largura ou altura")),
                }
            } else {
                Err(String::from("Faltando dimensões para o retângulo"))
            }
        }
        _ => Err(format!("Forma desconhecida: '{}'", separacao[0])),
    }
}