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
        let media = if estudante.notas.is_empty() {
            0.0 // media é 0
        } else {
            estudante.notas.iter().sum::<f64>() / estudante.notas.len() as f64
        };
        medias_por_estudante.insert(id.clone(), media);
    } 
    
    return medias_por_estudante     
}
