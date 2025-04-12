/*
Pattern Matching é uma técnica poderosa paa comparar valores contra padões e extrair dados.

1 - MATCH EXPRESSION: a expessão match compaa um valo com uma série de padões e executa o código associado ao padrão correspondente
2 - IF LET: é uma forma concisa de lidar com um único padrão, ignorando os outros
3 - WHILE LET: segue em um loop enquanto o padrão corresponde
4 - PADRÕES @ (binding): o operador @ permite criar uma vaiável que contém um valo ao mesmo tempo que o testa
5 - GUARDS MATCH: pode adicionar condições extras a um padrão usando guards
*/

use std::pin;

pub struct Ponto {
    x: i32,
    y: i32
}

pub enum Mensagem {
    Sair, 
    Mover {x: i32, y: i32},
    Escrever(String),
    MudaCor(i32, i32, i32)
}

pub fn match_numero(numero: i32) {
    match numero {
        1 => print!("Um"),
        2|3|5|7|11|13 => print!("Numero primo"),
        15..=20 => print!("Entre 15 e 20"),
        _ => print!("Outro Numero")
    }
}

pub fn deseestrutura_tuplas() {
    let ponto = (5, 15);
    match ponto {
        (0, 0) => print!("Na Origem"),
        (0, y) => print!("No eixo Y, y={}", y),
        (x, 0) => print!("No eixo X, x={}", x),
        (x, y) => print!("No Ponto ({}, {})", x, y),
    }
}

pub fn deseestrutura_struct() {
    let ponto = Ponto{x: 0, y: 7};

    match ponto {
        Ponto {x: 0, y: 0} => print!("Na Origem"),
        Ponto {x: 0, y} => print!("No eixo Y, y={}", y),
        Ponto {x, y: 0} => print!("No eixo X, x={}",x),
        Ponto {x, y} => print!("No Ponto ({}, {})", x, y),
    }
}

pub fn deseestrutura_enum() {
    let msg = Mensagem::MudaCor(255, 255, 0);

    match msg {
        Mensagem::Sair => print!("Sair"),
        Mensagem::Escrever(texto) => print!("Escrevendo o texto: {}", texto),
        Mensagem::Mover { x, y } => print!("Movendo para ({}, {})", x, y),
        Mensagem::MudaCor(r, g, b ) => print!("Mudando cor para ({}, {}, {})", r, g, b)
    }
}

pub fn padrao_unico() {
    // If Let é uma forma conscisa de lidar com um unico padrão ignorando os outros
    let valor_opcional = Some(5);

    match valor_opcional {
        Some(x) => print!("O valor é {}",x),
        None => ()
    }

    if let Some(x) = valor_opcional {
        print!("O valor é {}", x)
    }
}

pub fn padrao_correspondente() {
    // While Let Continua no loop enquanto o padrao for correspondido
    let mut pilha = Vec::new();

    pilha.push(1);
    pilha.push(2);
    pilha.push(3);

    while let Some(topo) = pilha.pop() {
        print!("Elemento removido: {}", topo)
    }
}

pub fn opeador() {
    // O operador @ permite criar uma variavel nova que contém um valor ao mesmo tempo que testa dentro do match
    let x = 4;

    match x {
        n @ 1..=5 => print!("{} está entre 1 e 5", n),
        n @ 6..=10 => print!("{} está entre 6 e 10", n),
        _ => print!("Outro Valor"),
    }
}

pub fn guards_mathc() {
    // Permite adicionar condições extras a um padrão usando guards
    let num = 5;

    match num {
        x if x < 0 => print!("Negativo"),
        x if x % 2 == 0 => print!("Par: {}", x), 
        x => print!("Impar positivo: {}", x)
    }
}

fn main() {
    let numero = 11;

    match_numero(numero);
    deseestrutura_enum();
    deseestrutura_struct();
    deseestrutura_tuplas();
    padrao_unico();
    padrao_correspondente();
    opeador();
    guards_mathc();
}