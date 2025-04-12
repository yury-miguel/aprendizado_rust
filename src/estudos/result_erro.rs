/*
Result é um enum embutido em Rust para representar o resultado de uma operação que pode falhar, diferente de <Option> que lida com ausencia de valor
o <Result> é usado quando uma operação pode falhar por um motivo especifico e você sabe por que.
Use match e "?" em funções que retornam <Result> para tratar os casos Ok e Err

enum Result {
    Ok(T),
    Err(T),
}
*/

pub fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Não divisível por 0"))
    } else {
        Ok(a/b)
    }
}

