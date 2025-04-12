use std::fs::File;
use std::io::{self, Read};

pub fn ler_arquivo(caminho: &str) -> Result<String, io::Error>{
    let mut arquivo = File::open(caminho)?; // Propaga o erro automaticamente
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo)?;
    Ok(conteudo)
}

pub fn processar_arquivo(caminho: &str) {
    match ler_arquivo(caminho) {
        Ok(conteudo) => {
            println!("Arquivo lido com sucesso!");
            println!("Conteudo do arquivo:\n {}", conteudo);

            let num_linhas = conteudo.lines().count();
            println!("O arquivo possui {} linhas", num_linhas)
        },
        Err(erro) => {
            println!("Erro ao ler arquivo: {:?}", erro);

            match  erro.kind() {
                io::ErrorKind::NotFound => println!("Arquivo não encontrado"),
                io::ErrorKind::PermissionDenied => println!("Permissão negada"),
                _ => println!("Outro erro encontrado!"),
            }

        }
    }
}

pub fn main() {
    processar_arquivo("arq_teste.txt");
}