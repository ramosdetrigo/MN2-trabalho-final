#![allow(dead_code)]
mod aberta;
mod fechada;

pub use aberta::NewtonCotesAberta;
pub use fechada::NewtonCotesFechada;

#[derive(Debug, Clone, Copy)]
pub enum NewtonCotes {
    Aberta(NewtonCotesAberta),
    Fechada(NewtonCotesFechada),
}


impl NewtonCotes {
    pub fn aplicar(&self, a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
        match self {
            Self::Aberta(tipo) => tipo.aplicar(a, b, f),
            Self::Fechada(tipo) => tipo.aplicar(a, b, f)
        }
    }

    /// Aplica a fórmula composta com `n` subintervalos
    pub fn aplicar_composta(&self, a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
        match self {
            Self::Aberta(tipo) => tipo.aplicar_composta(a, b, n, f),
            Self::Fechada(tipo) => tipo.aplicar_composta(a, b, n, f)
        }
    }
}

