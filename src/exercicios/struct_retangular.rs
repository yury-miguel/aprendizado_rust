pub struct Retangulo {
    pub width: f64,
    pub height: f64,
}

impl Retangulo {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimetro(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }

    pub fn escalar(&mut self, fator: f64) {
        self.width *= fator;
        self.height *= fator;

    }  
}