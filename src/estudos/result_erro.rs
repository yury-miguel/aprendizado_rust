/*
Result é um enum embutido em Rust para representar o resultado de uma operação que pode falhar, diferente de <Option> que lida com ausencia de valor
o <Result> é usado quando uma operação pode falhar por um motivo especifico e você sabe por que.
Use match e "?" em funções que retornam <Result> para tratar os casos Ok e Err

Erros recuperaveis: Situcações que pode lidar e continuar com a execução
Erros irrecuperaveis: Situações graves onde é melhor parar o program

A macro panic! é para erros irrecuperáveis
O tipo Result<T, E> é para erros recuperaveis
Com ? pode se encadear operações que podem falhar

o método unwrap() extrai o valor de sucesso ou causa panic se for erro
o método expect() permite ter uma mensagem personalizada para erros
o método unwrap_or_else() permite fornecer uma função para lidar com erros

enum Result {
    Ok(T),
    Err(T),
}

Usar Result ao inves de Option para erro recuperaveis (a menos que a ausencia de valor seja esperada)
Não usar unrwap() ou expect() em libs, deixar o usuario decidir, a menos que o erro nao deva existir
O anyhow simplifica o tratamento de erros em aplicacoes

*/


use std::fs::File;
use std::io::{self, Read};
use anyhow::{Result, Context, anyhow};
use thiserror::Error;

pub fn panico() {
    println!("Antes do panic");

    panic!("Algo deu muito errado");

    print!("Depois do panic!") // Não será executado

    /* Outro Exemplo

    let vetor = vec![1, 2, 3];
    let valor = vetor[9]; // Causaria um BOOM !

    */
}


pub fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Não divisível por 0"))
    } else {
        Ok(a/b)
    }
}

pub fn abre_arquivo() {
    let arquivo = File::open("arquivo/teste.txt");

    match arquivo {
        Ok(arquivo) => {
            println!("Arquivo aberto com sucesso");
        },

        Err(erro) => {
            println!("Não foi possivel abrir o arquivo: {}", erro);
        }
    }
}

pub fn abre_arquivo2() {
    let arquivo = File::open("arquivos/teste.txt").unwrap_or_else(|erro| {
            println!("Não foi possivel abrir o arquivo: {}", erro);

            // Cria um arquivo padrão
            File::open("arquivos/padrao.txt").expect("Arquivo padrao também não existe!")
        });
}

pub fn le_arquivo() -> Result<String, io::Error> {
    let mut arquivo = File::open("arquivos/teste.txt")?;
    let mut conteudo = String::new();

    arquivo.read_to_string(&mut conteudo)?;

    Ok(conteudo)
}

pub fn ler_config() -> Result<String> {
    let conteudo = std::fs::read_to_string("config.txt")
    .context("Falha ao ler arquivo de configuração")?;

    if conteudo.is_empty() {
        return Err(anyhow!("O arquivo de configuração esta vazio"));
    }

    Ok(conteudo)
}

#[derive(Error, Debug)]
enum erros {
    #[error("erro de IO: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Valor invalido: {0}")]
    InvalidValue(String),

    #[error("operacao nao autoraizada")]
    Unauthorized,

}

pub fn meu_erro(valor: &str) -> Result<(), erros> {
    if valor.is_empty() {
        return Err(erros::InvalidValue("valor vazio".to_string()));
    }

    // Pode causar erro de IO
    std::fs::write("saida.txt", valor)?;

    Ok(())

}