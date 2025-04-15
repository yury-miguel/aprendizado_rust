/* 
Em rust, módulos organizam o código em namespaces, ajudando a manter projetos grandes gerenciaveis.
Um módulo pode estar em um arquivo separado ou em um mod.rs dentro de uma pasta

- mod nome {...} -> define um módulo
- use nome::item -> Importa items de um módulo
- pub: torna o modulo publico 
*/

pub mod calculadora {
    pub fn somar(n1: i32, n2: i32) -> i32 {
        n1 + n2
    }

    pub fn subtrair(n1: i32, n2: i32) -> i32 {
        n1 - n2
    }

    fn multiplicar(n1: i32, n2: i32) -> i32 {
        n1 * n2
    }  
}