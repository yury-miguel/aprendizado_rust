/*
Enums: define um tipo que pode ser de varios variantes. Cada variante pode carregar dados diferentes.
Option: Um enum embutido em Rust para representar a presenca ou ausencia de um valor

enum Option<T> {
    Some(T), // Contem o valor do tipo T
    None, //Não contem o valor do tipo T
}

*/

pub enum Operacao {
    Soma(i32, i32),
    Subtracao(i32, i32),
}

pub fn executar(op: Operacao) -> Option<i32> {
    match op {
        Operacao::Soma(a, b) => Some(a + b),
        Operacao::Subtracao(a, b ) => {
            if a >= b {
                Some(a - b)
            } else {
                None // Retorna None se o resultado seria negativo
            }
        }
    }
}