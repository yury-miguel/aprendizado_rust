/*
1 - STRUCTS: são dados compostos que agrupam valores relacionados em um unico tipo personalizado, é como uma classe em python,
mas sem herança, Rust usa composição
2 - ENUMS: permitem definir um tipo enumerando suas possíveis variantes
3 - OPTION: Rust não tem null, mas tem Option<T> para representar ausência ou presença de um valor
4 - RESULT: Result<T> ou Result<E> é usado para operações que podem falhar
5 - MÉTODOS: São funções associadas a structs, definidas com impl, Podem acessar os dados de uma struct usando self
*/

use::std::fs::File;

// Struct com campos nomeados
pub struct Usuario {
    nome: String,
    email: String,
    idade: i32,
    ativo: bool
}
pub struct Retangulo {
    largura: u32,
    altura: u32
}

// Struct de tupla
pub struct Ponto(i32, i32, i32);

// Struct de unidade (sem campos)
pub struct Marca;

// Criando instancias das structs
pub fn instancias_struct() {
    let usuario_1 = Usuario {
        nome: String::from("Yury"),
        email: String::from("yury@gmail.com"),
        idade: 30,
        ativo: true,
    };
    println!("Usuario {}", usuario_1.nome);

    // Struct mutável, permite modificar um campo posteriormente
    let mut usuario_2 = Usuario {
        nome: String::from("teste"),
        email: String::from("teste@gmail.com"),
        idade: 15,
        ativo: true,
    };
    usuario_2.email = String::from("teste2@gmail.com.br");
    print!("\nUsuario 2 {}", usuario_2.email);

    // Cria uma nova instância usando valoes de usuario_1
    let usuario_3 = Usuario{
        email: String::from("user3@gmail.com"),
        ..usuario_1
    };
    print!("\nUsuario 3: {}, {}", usuario_3.email, usuario_3.nome);

    let origem = Ponto(5,0,0);
    print!("\nPonto 0 {}", origem.1);

    let m = Marca;

}

// Implementando métodos para o retângulo
impl Retangulo {

    // Recebe &self
    pub fn area(&self) -> u32{
        self.largura * self.altura
    }

    // Modifica dados da instância
    pub fn redimensionar(&mut self, largura: u32, altura: u32){
        self.largura = largura;
        self.altura = altura;
    }

    // Função associada (não recebe self)
    pub fn quadrado(tamanho: u32) -> Retangulo{
        Retangulo {
            largura: tamanho,
            altura: tamanho
        }
    }

}

#[derive(Debug)]
pub enum EstadoPedido {
    Novo, 
    Processado,
    Enviado,
    Entregue,
    Cancelado
}

#[derive(Debug)]
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

pub enum Mensagem {
    Sair, // sem dados
    Mover {x: i32, y: i32}, // struct anonima
    Escrever (String), // Sting
    MudaCor(i32, i32, i32) // tupla de tres valoes
}

impl Mensagem {
    pub fn processar(&self) {
        match self {
            Mensagem::Sair => print!("\nSaindo do programa"),
            Mensagem::Mover {x, y} => print!("\nMovendo para {}, {}", x, y),
            Mensagem::Escrever(texto) => print!("\nEscrevendo {} no programa", texto),
            Mensagem::MudaCor(r,g,b) => print!("\nAlterando as cores para ({},{},{})", r, g, b)
        }
    }
}

fn main() {

    /* ======= STRUCTS ====== */
    instancias_struct();

    let mut retangulo = Retangulo {
        largura: 30,
        altura: 50
    };
    print!("\nÁrea = {}", retangulo.area());

    retangulo.redimensionar(60, 40);
    print!("\nNova Área = {}", retangulo.area());

    let quadrado = Retangulo::quadrado(20);
    print!("\nÁrea do quadrado = {}", quadrado.area());


    
    /* ======= ENUMS ====== */
    let pedido_status = EstadoPedido::Processado;
    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("\n{:?}, {:?}, {:?}", pedido_status, localhost, loopback);

    let m1 = Mensagem::Sair;
    let m2 = Mensagem::Mover {x: 10, y: 20};
    let m3 = Mensagem::Escrever(String::from("Olá"));
    let m4 = Mensagem::MudaCor(255, 0, 0);

    m1.processar();
    m2.processar();
    m3.processar();
    m4.processar();


    
    /* ======= OPTION E RESULT ====== */
    let numero = Some(5);
    let texto = Some("Hello");
    let ausente: Option<i32> = None;

    match ausente {
        Some(n) => print!("\nTemos um numero: {}", n),
        None => print!("\nSem número"),
    }

    let resultado_arq = File::open("arquivo_teste.txt");
    match resultado_arq {
        Ok(arquivo) => print!("\nArquivo aberto com sucesso {:?}", arquivo),
        Err(erro) => print!("\nErro ao abrir arquivo: {}", erro)
        
    }

    
}