#[derive(Debug)]
pub struct Forma_Circular {
    pub raio: f32,
}

#[derive(Debug)]
pub struct Forma_Retangular {
    pub altura: f32,
    pub largura: f32,
}


pub fn imprime_forma<T>(forma: T)
where
    T: std::fmt::Debug
{
    println!("Forma {:?}", forma)
}