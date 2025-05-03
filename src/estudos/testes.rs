/*
Testes garantem que o código funcione como o esperado, em backend verificam APIs por exemplo, em IA verificam algoritmos

Testes unitario -> testam funções ou modulos isolados, são definidos com #[test] em um modulo tests

Testes de integração -> também no diretorio tests/ testam as interações do modulo

*/

pub fn somar(a : i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {
    use super::somar;

    #[test]
    fn test_somar_positivos() {
        assert_eq!(somar(1,2), 3)
    }
    
    #[test]
    fn test_somar_negativos() {
        assert_eq!(somar(-1, -1), -2);
    }
}