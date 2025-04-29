use rand::Rng;

pub struct Nomes {
    pub nomes: Vec<String>,
}

impl Nomes {
    fn comprimento(&self) -> usize {
        self.nomes.len()
    }

    fn pega_nome(&self, posicao: usize) -> Option<&str>{
        //let nome_escolhido = self.nomes.get(posicao).unwrap();
        //return Some(nome_escolhido);
        self.nomes.get(posicao).map(|s| s.as_str())
    } 
}

pub fn escolhe_nome(nomes: &Nomes) -> Option<&str> {
    let indice_aleatorio = rand::thread_rng().gen_range(0..=nomes.comprimento() - 1);
    let nome = nomes.pega_nome(indice_aleatorio);
    return nome;
}