use std::{alloc::Layout, f64::consts::PI};

pub enum Forma {
    Circulo(f64),
    Retangulo(f64, f64)
}

pub fn area_forma(forma: Forma) -> Option<f64> {
    match forma{
        Forma::Circulo(raio) => {
            if raio < 0.0 {
                None
            } else{
                Some(PI * raio * raio)
            }
        }
        Forma::Retangulo(altura, largura) => {
            if largura < 0.0 || altura < 0.0 {
                None
            } else {
                Some(altura * largura)
            }
        }
    }
}