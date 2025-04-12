use::std::collections::HashMap;

// Itens disponiveis na biblioteca
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Livros(String),
    DvDs(String),
    Revistas(String)
}

// Status dos itens
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Status {
    Disponivel,
    Emprestado,
}

// Ações a serem feitas
pub struct Biblioteca {
    itens: HashMap<Item, Status>,
    historico: Vec<String>,
}


// Implementação que armazena ações da biblioteca
impl Biblioteca {
    pub fn new() -> Self{
        Biblioteca { 
            itens: HashMap::new(), 
            historico: Vec::new() 
        }
    }

    pub fn catalogar(&mut self, item: Item) {
        self.itens.insert(item.clone(), Status::Disponivel);
        self.historico.push(format!("Catalogado: {:?}", item))
    }

    pub fn emprestar(&mut self, item: &Item) {
        if let Some(status) = self.itens.get_mut(item) {
            match status {
                Status::Disponivel => {
                    *status = Status::Emprestado;
                    self.historico.push(format!("Emprestado: {:?}", item))
                }
                _ => println!("Item não esta disponivel para empréstimo"),
            }
        } else {
            println!("Item não encontrado na biblioteca")
        }
    }

    pub fn devolver(&mut self, item: &Item) {
        if let Some(status) = self.itens.get_mut(item) {
            if *status == Status::Emprestado {
                *status = Status::Disponivel;
                self.historico.push(format!("Devolvido {:?}", item))
            } else{ 
                println!("Item não estava emprestado")
            }
        } else {
            println!("Item não encontrado na biblioteca")
        }
    }

    pub fn estatisticas(&self) {
        println!("Exibindo as estatisticas");
        for (item, status) in &self.itens{
            println!("{:?} - {:?}", item, status);
        }

        println!("Historico de Ações:");
        for acao in &self.historico{
            println!("{}", acao)
        }
    }
}