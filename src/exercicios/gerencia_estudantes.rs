use std::{collections::HashMap, f64};

// Estrutura contendo o nome e as notas de um estudante
pub struct Estudante {
    pub nome: String,
    pub notas: Vec<f64>,
}

// Função que recebe um nome e um estudante com suas notas e então retorna a média
pub fn media_notas(estudantes: &HashMap<String, Estudante>) -> HashMap<String, f64> {
    let mut medias_por_estudante: HashMap<String, f64> = HashMap::new();

    for (id, estudante) in estudantes.iter() {
        let media = estudante.notas.iter().sum::<f64>() / estudante.notas.len() as f64;
        let nome = estudante.nome.clone();
        medias_por_estudante.insert(nome.to_string(), media);
    } 
    return medias_por_estudante     
}
