#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrdemErro {
    Linear,
    Quadratico,
    Cubico,
    Quarta
}

pub trait Filosofia {
    fn derivada_primeira(&self, f: &dyn Fn(f64) -> f64, x: f64, delta: f64, erro: OrdemErro) -> f64;

    fn derivada_segunda(&self, f: &dyn Fn(f64) -> f64, x: f64, delta: f64, erro: OrdemErro) -> f64;

    fn derivada_terceira(&self, f: &dyn Fn(f64) -> f64, x: f64, delta: f64, erro: OrdemErro) -> f64;
}


pub mod forward;
pub mod backward;
pub mod central;
pub mod imagem;