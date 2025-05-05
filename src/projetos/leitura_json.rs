use std::fs::File;
use thiserror::Error;

#[derive(Error, Debug)]
enum Erros {
    #[error("Arquivo não encontrado: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Json Inválido: {0}")]
    Error(#[from] std::serde_json::Error),

    #[error("Dados invalidos: {0}")]
    InvalidValue(String),
}
//pub fn le_dados():