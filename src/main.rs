// src/main.rs

mod exercicios;
mod estudos;
mod projetos;


// ======================================== ESTUDOS ================================================

fn enum_options() {
    let op1 = estudos::enums_options::Operacao::Soma(5, 3);
    let op2 = estudos::enums_options::Operacao::Subtracao(20, 10);
    let op3 = estudos::enums_options::Operacao::Subtracao(3, 7);

    match estudos::enums_options::executar(op1) {
        Some(resultado) => println!("resultado {}", resultado),
        None => println!("Sem resultado!")
    }
    match estudos::enums_options::executar(op2) {
        Some(resultado) => println!("resultado {}", resultado),
        None => println!("Sem resultado!")
    }   
    match estudos::enums_options::executar(op3) {
        Some(resultado) => println!("resultado {}", resultado),
        None => println!("Sem resultado!")
    }
}

fn result_erro() {
    match estudos::result_erro::dividir(10, 2) {
        Ok(resultado) => println!("Resultado {}", resultado),
        Err(erro) => println!("Erro {}", erro),
    }

    match estudos::result_erro::dividir(5, 0) {
        Ok(resultado) => println!("Resultado {}", resultado),
        Err(erro) => println!("Erro {}", erro),
    }

}


fn modulo_calculadora() {

    let soma1 = estudos::calculadora::calculadora::somar(5, 5);
    let subtracao1 = estudos::calculadora::calculadora::subtrair(10, 2);
    //let multiplica1 = estudos::calculadora::calculadora::multiplicar(2, 10);

    println!("Soma {}", soma1);
    println!("Subtracao {}", subtracao1)

    /* 
        function `multiplicar` is private
        private functionrustcClick for full compiler diagnostic
        calculadora.rs(19, 5): the function `multiplicar` is defined here    
     */

}


fn hash_vec() {
    estudos::estrutura_dados::estrutura_dados();
}

// ======================================== EXERCICIOS ==============================================


fn exercicio_struc_retangular() {
    let mut retangulo =  exercicios::struct_retangular::Retangulo{
        width: 50.0,
        height: 25.0
    };

    let area = retangulo.area();
    println!("{}", area);

    let perimetro = retangulo.perimetro();
    println!("{}", perimetro);

    retangulo.escalar(2.0);
    println!("{}, {}", retangulo.width, retangulo.height)
}


fn exercicio_gerenciador_biblioteca() {
    let mut biblioteca = exercicios::gerenciador_biblioteca::Biblioteca::new();
    
    let livro = exercicios::gerenciador_biblioteca::Item::Livros("O senhor dos anéis".to_string());
    let dvd = exercicios::gerenciador_biblioteca::Item::DvDs("Eu sou a Lenda".to_string());
    let revista = exercicios::gerenciador_biblioteca::Item::Revistas("National Geographic".to_string());

    biblioteca.catalogar(livro.clone());
    biblioteca.catalogar(dvd.clone());
    biblioteca.catalogar(revista.clone());

    biblioteca.emprestar(&livro);
    biblioteca.emprestar(&dvd);
    biblioteca.devolver(&dvd);
    biblioteca.devolver(&revista);

    biblioteca.estatisticas();

}

fn enum_forma() {
    let circulo1 = exercicios::enum_forma::Forma::Circulo(5.0);
    let circulo2 = exercicios::enum_forma::Forma::Circulo(-2.0);
    let retangulo1 = exercicios::enum_forma::Forma::Retangulo(2.0, 3.0);

    match exercicios::enum_forma::area_forma(circulo1) {
        Some(area) => println!("Area do circulo (raio 5.0): {:.2}", area),
        None => println!("Raio invalida para o circulo"),
    }
    match exercicios::enum_forma::area_forma(circulo2) {
        Some(area) => println!("Area do circulo (raio -2.0): {:.2}", area),
        None => println!("Raio invalido para o circulo"),
    }
    match exercicios::enum_forma::area_forma(retangulo1) {
        Some(area) => println!("Area do retangulo: {:.2}", area),
        None => println!("Raio invalida para o retangulo"),
    }
}

fn erro_result() {
    match exercicios::erro_result::ler_numero("seis") {
        Ok(retorno) => println!("Retorno {}", retorno),
        Err(erro) => println!("Retorno erro {}", erro),
    }
}


// ======================================== EXERCICIOS ==============================================


fn calcula_formas() {
    let testes = [
        "circulo:5.0",
        "retangulo:4.0,3.0",
        "circulo:-1.0",
        "invalido",
    ];

    for teste in testes {
        match projetos::calculadora_geometrica::calcular_area(teste){
            Ok(area) => println!("Area do {}: {}", teste, area),
            Err(erro) => println!("Erro ao calcular area de {}: {}", teste, erro)
        }
    }
}



// ======================================== EXECUCAO ==============================================

fn main() {
    println!("Bem-vindo ao projeto de aprendizado Rust!");

    // Chamando estudos do módulos
    //enum_options();
    //modulo_calculadora();
    //hash_vec();


    // Chamando exercicios do módulos
    
    //exercicio_struc_retangular();
    //exercicio_gerenciador_biblioteca();
    //enum_forma();
    //result_erro();
    //erro_result();

    
    // Chamando projetos do módulos
    //calcula_formas();
}
