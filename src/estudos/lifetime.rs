/* 
Lifetimes são um conceito avançado que lida com quanto tempo uma referencia é valida
*/

// Estrutura que armazena referencias para nome e sobrenome de uma pessoa
pub struct Person<'a> {
    nome: &'a str,
    sobrenome: &'a str
}

// Funcao que retorna a referencia mais curta entre duas referencias de string
pub fn verifica_string_curta<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str2
    } else {
        str1
    }

}

// Metodo para a estrutura Person que retornará o nome completo das referencias nome e sobrenome
impl Person<'_> {
    pub fn nome_completo() -> String {
        let pessoa = Person { nome: "yury", sobrenome: "miguel"};
        let Person {nome, sobrenome} = pessoa;
        let nome = format!("{}{}", nome, sobrenome);
        nome
    }
}