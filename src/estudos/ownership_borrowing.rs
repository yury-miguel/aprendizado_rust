/*
1. REGRAS DE OWNERSHIP

* Cada valor tem seu próprio dono
* So pode haver um dono por vez
* Quando o dono sai do escopo o valor é descartado
* Tipos como int, float, booleanos, caracteres ou tuplas possuem o COPY com trait, assim o valor é copiado automaticamente
* Quando passa um valor para uma função, a propriedade é transferida para o parametro da função, e depois descartada


2. REGRAS BORROWING

* Permite usar valores sem tomar posse (& e mut)
* & é um empréstimo imutável
* &mut é um emprestimo mutável
* Pode ter qualquer numero de referências imutaveis (&T), mas só pode ter uma referencia mutável (&mut T) de cada vez

** RUST POSSUI LIFETIME, RASTREIA O TEMPO EM QUE UMA REFERENCIA DE VARIÁVEL É VÁLIDA
** UMA REFERÊNCIA É COMO UM PONTEIRO, SEGUIMOS ELE PARA ACESSAR DADOS DE UMA OUTRA VARIÁVEL, PORÉM UMA REFERÊNCIA É GARANTIDA PELO COMPILADOR PARA APONTAR UM VALOR VÁLIDO DE UM TIPO ESPECIFICO

*/


pub fn exemplo_1() {

    let s1 = String::from("Ola, exemplo 1");
    let s2 = s1;
    println!("S2 {}", s2);
    //println!("S1 {}", s1) Causaria um erro, pois o valor foi movido para s2


    let x = 5;
    let y = x;
    println!("X {}, Y {}", x, y) // Como é do tipo inteiro, o valor não é transferido, apenas copiado

    /*
        s1 contém: um ptr que aponta para o conteúdo da str na heap, 
        um len (14) que é o compimento da string 
        e um capacity que é o espaço alocado (14)

        Internamente os valores de ptr, len e capacity foram copiados para o s2,
        e o valor de s1 é liberado para evitar double free
    */

}

pub fn exemplo_2(){
    let s1 = String::from("EXEMPLO 2");
    let s2 = s1.clone(); // Cria uma cópia profunda, duplicando os dados do heap
    println!("S1 {}, S2 {}", s1, s2)
}

pub fn exemplo_3(alguma_string: String) {
    println!("{}", alguma_string)
} // alguma_string é descartada da memória

pub fn exemplo_4(algum_inteiro: i32) {
    println!("{}", algum_inteiro)
} // algum_inteiro sai do escopo, mas não é efeito nenhum

pub fn exemplo_5(s: &String) -> usize {
    s.len()
} // s sai do escopo, mas como é apenas uma referencia, o valor ainda permanece na memóra, não é descartado

pub fn exemplo_6(s: &mut String) {
    s.push_str(", Exemplo 6"); // Modifica o valor da variavel pela referência
}

pub fn exercicio_1() {
    let s1 = String::from("Hello exercicio 1");
    let s2 = s1;
    
    // println!("{} {}", s1, s2); Falhará, pois o valor de s1 foi transferido para s2
    // o correto seria usar o clone() ou &s1
}

pub fn exercicio_2(string: &mut String, sufixo: &str) {
    string.push_str(sufixo);
}

pub fn exercicio_3() {
    let mut s1 = String::from("Exercicio 3, aprendendo Rust");
    
    // referencias imutaveis
    let s2 = &s1;
    let s3 = &s1;
    let s4 = &s1;
    println!("S1 inicio {}\n{}\n{}\n{}\n", s1, s2, s3, s4);

    // referencia mutavel
    let s5 = &mut s1;
    s5.push_str("gosto de rust");
    print!("S1 final {}", s1)
}

pub fn tarefa_1<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1 > string2 {
        string1
    } else {
        string2
    }
}

pub fn tarefa_2(v1: &Vec<i32>, v2: &mut Vec<i32>){
    let resultado1: i32 = v1.iter().sum();
    v2.sort();

    print!("\nResultado soma {}\nResultado Ordenado {:?}", resultado1, v2)
}

fn main() {
    exemplo_1();
    exemplo_2(); 
    
    let teste_exemplo3 = String::from("Exemp 3");
    let teste_exemplo4 = 5;
    let teste_exemplo5 = String::from("Exemp 5");
    let mut teste_exemplo6 = String::from("Exemp 6");

    exemplo_3(teste_exemplo3);
    exemplo_4(teste_exemplo4);
    exemplo_5(&teste_exemplo5);
    exemplo_6(&mut teste_exemplo6);
    print!("{}", teste_exemplo6);

    //exercicio_1();
    let mut exercicio2 = String::from("Teste exerc2");
    let sufixo2 = String::from("aprendendo rust");
    exercicio_2(&mut exercicio2, &sufixo2);
    println!("\nExercicio 2, {}", exercicio2);
    
    exercicio_3();
    
    let s1 = String::from("tarefa 1");
    let s2 = "task 1";
    tarefa_1(s1.as_str(), s2);

    let vec_tarefa2 = vec![4,3,5,2];
    let mut vec2_tarefa = vec![4,3,5,2];
    tarefa_2(&vec_tarefa2, &mut vec2_tarefa);
}