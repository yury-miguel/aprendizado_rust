/*
Vec: vetor dinâmico VEC<T> que armazena n elementos do mesmo tipo em uma sequência contigua
- metodos uteis: push, pop, iter, len, get, sort

HashMap: um mapa chave-valor HashMap<K, V> para associar valores a chaves
- metodos uteis: insert, get, remove, contains_key
*/

use std::collections::HashMap;

pub fn estrutura_dados() {
    let mut numeros = vec![3, 2, 1];
    numeros.push(1);
    numeros.sort();
    println!("Numeros ordenados: {:?}", numeros);

    let mut idades: HashMap<String, i32> = HashMap::new();
    idades.insert("Joao".to_string(), 15);
    idades.insert("Maria".to_string(), 18);
    
    if let Some(idade) = idades.get("Joao") {
        println!("Idade de Joao: {}", idade)
    }

}