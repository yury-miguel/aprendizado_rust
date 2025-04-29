/*
1 - Generics: permitem criar definições para itens como funções ou estruturas que podem fucionar
com diferentes tipos de dados. Eles são a base para criar código flexivel e reutilizavel

2 - Traits: são uma forma de definir comportamento compartilhado. Você pode pensar nisso como "interfaces"
em outras linguagens, mas com recursos mais avançados.

3 - Trait Bounds: permitem restringir tipos genéricos para que implementem comportamentos especificos.

*/


/* =========================================== GENÉRICO ===========================================  */

use std::{collections::btree_set::Intersection, fmt::format};

// Função generica que funciona com qualquer tipo <T>
fn print_value<T>(value: T)
where 
    T: std::fmt::Debug
{
    println!("Valor {:?}", value)
}

// Estrutura generica que pode armazenar qualquer tipo
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    fn get_value(&self) -> &T {
        &self.value
    }
}

// Multiplos parametros de tipos
struct Par<T, U> {
    primeiro: T,
    segundo: U
}

impl<T, U> Par<T, U> {
    fn new(primeiro: T, segundo: U) -> Self {
        Par { primeiro, segundo}
    }
}

/* =========================================== TRAITS ===========================================  */

// Trait simples
trait Descricao {
    fn descricao(&self) -> String {
        String::from("Objeto desconhecido")
    }

    // metodo que deve ser implementados
    fn nome(&self) -> String;
}

// Implementando o trait para um struct
struct Pessoa {
    nome: String,
    idade: i32,
}

impl Descricao for Pessoa {
    fn nome(&self) -> String {
        self.nome.clone()
    }

    fn descricao(&self) -> String {
        format!("{} tem {} anos", self.nome, self.idade)
    }
}


// Implementando para um tipo primitivo
impl Descricao for i32 {
    fn nome(&self) -> String {
        format!("Numero {}", self)    
    }
}

// Uma funcao que aceita qualquer tipo que implemente Descricao
fn print_descricao(item: &impl Descricao) {
    println!("Descricao: {}", item.descricao());
    println!("Nome: {}", item.nome());
}


// objects
fn trait_objetos() {
    let objetos: Vec<Box<dyn Descricao>> = vec![
        Box::new(Pessoa {
            nome: String::from("Yury"),
            idade: 40
        }),
        Box::new(40_i32)
    ];

    for objeto in objetos {
        println!("Objeto: {}", objeto.descricao());
    }
}


/* =========================================== TRAITS BOUNDS ===========================================  */

// Permitem restringir tipos genericos para que implementem comportamentos especificos

// Usando sintaxe onde
fn largest<T>(list: &[T]) -> &T 
    where 
        T: std::cmp::PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest

}


// Sintaxe alternativa
fn largest_all<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}


// Um tipo que deve implementar multiplos traits
fn print_info<T>(value: T) 
    where
        T: std::fmt::Display + std::fmt::Debug + Clone,
{
    let clonado = value.clone();

    println!("Display {}", value);
    println!("Debug {:?}", value);
    println!("Clonado {}", clonado)
}


// Traits bounds condicionais
struct Wrapper<T> {
    value: T,
}

// Implementacao disponivel somenta para os tipos que implementem Display
impl<T: std::fmt::Display> Wrapper<T> {
    fn print(&self) {
        println!("Valor: {}", self.value);
    }
}

// Implementacao disponivel somenta para os tipos que implementem PartialOrd
impl<T: std::cmp::PartialOrd> Wrapper<T> {
    fn is_greather_than(&self, other: &T) -> bool {
        self.value > *other
    }
}


/* =========================================== TRAITS AVANÇADOS ===========================================  */

trait Iterator {
    type Item; // tipo associado

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32
}

impl Iterator for Counter {
    type Item = u32; // definindo o tipo associado
    
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}


/* =========================================== SUPER TRAITS ===========================================  */

trait Printable {
    fn format(&self) -> String;
}

trait Animal : Printable {
    fn especie(&self) -> String;
    
    fn make_sound(&self) -> String;
}

struct Cat {
    name: String,
}

impl Printable for Cat {
    fn format(&self) -> String {
        format!("Um gato chamado {}", self.name)
    }
}

impl Animal for Cat {
    fn especie(&self) -> String {
        String::from("Felis catus")
    }

    fn make_sound(&self) -> String {
        String::from("Miau!")
    }
}