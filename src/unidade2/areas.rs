use crate::unidade1::central::Central;
use crate::unidade1::{Filosofia, OrdemErro};
use std::f64::consts::PI;

/// Área de um triângulo via mudança de variáveis (jacobiano)
pub fn area_triangulo(base: f64, altura: f64, n: usize) -> f64 {
    let mut soma = 0.0;
    let dalpha = 1.0 / n as f64;
    let dbeta = 1.0 / n as f64;

    for i in 0..n {
        let _alpha = dalpha / 2.0 + i as f64 * dalpha;
        for j in 0..n {
            let beta = dbeta / 2.0 + j as f64 * dbeta;
            let da = base * altura * (1.0 - beta) * dalpha * dbeta;
            soma += da;
        }
    }
    soma
}

/// Área do trapézio com mudança de variáveis e jacobiano
pub fn area_trapezio(b: f64, c: f64, d: f64, h: f64, n: usize) -> f64 {
    let mut soma = 0.0;
    let dalpha = 1.0 / n as f64;
    let dbeta = 1.0 / n as f64;

    for i in 0..n {
        let _alpha = dalpha / 2.0 + i as f64 * dalpha;
        for j in 0..n {
            let beta = dbeta / 2.0 + j as f64 * dbeta;
            let jaco = b * h + h * (d - c - b) * beta;
            soma += jaco * dalpha * dbeta;
        }
    }
    soma
}

/// Área do círculo por coordenadas polares com mudança de variáveis
pub fn area_circulo(r: f64, n: usize) -> f64 {
    let mut soma = 0.0;
    let dalpha = 1.0 / n as f64;
    let dbeta = 2.0 * PI / n as f64;

    for i in 0..n {
        let alpha = dalpha / 2.0 + i as f64 * dalpha;
        for j in 0..n {
            let _beta = dbeta / 2.0 + j as f64 * dbeta;
            let jaco = r * r * alpha;
            soma += jaco * dalpha * dbeta;
        }
    }
    soma
}

/// Área da elipse por coordenadas polares com mudança de variáveis
pub fn area_elipse(a: f64, b: f64, n: usize) -> f64 {
    let mut soma = 0.0;
    let dalpha = 1.0 / n as f64;
    let dbeta = 2.0 * PI / n as f64;

    for i in 0..n {
        let alpha = dalpha / 2.0 + i as f64 * dalpha;
        for j in 0..n {
            let _beta = dbeta / 2.0 + j as f64 * dbeta;
            let jaco = a * b * alpha;
            soma += jaco * dalpha * dbeta;
        }
    }
    soma
}

pub fn integra_retangulo(a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    let dx = (b - a) / n as f64;
    let mut soma = 0.0;
    for i in 0..n {
        let xi = a + dx / 2.0 + i as f64 * dx;
        soma += dx * f(xi);
    }
    soma
}

// Pontos e pesos de Gauss-Legendre n=3
const PONTOS: [f64; 3] = [-0.774596669241483, 0.0, 0.774596669241483];
const PESOS: [f64; 3] = [0.555555555555556, 0.888888888888889, 0.555555555555556];

/// Área da superfície paraboloidal em região quadrada (Gauss 3x3)
pub fn area_superficie_cartesiana(integrando: &dyn Fn(f64, f64) -> f64) -> f64 {
    let jaco = 2500.0; // 50*50
    let mut soma = 0.0;

    for i in 0..3 {
        let alpha = PONTOS[i];
        let x = 50.0 * alpha;
        for j in 0..3 {
            let beta = PONTOS[j];
            let y = 50.0 * beta;
            let valor = integrando(x, y);
            soma += PESOS[i] * PESOS[j] * valor;
        }
    }
    soma * jaco
}

/// Área da superfície paraboloidal com mudança polar elíptica
pub fn area_superficie_polar(integrando: &dyn Fn(f64, f64) -> f64) -> f64 {
    let mut soma = 0.0;

    for i in 0..3 {
        let alpha = (PONTOS[i] + 1.0) / 2.0;
        let w_alpha = PESOS[i] / 2.0;
        for j in 0..3 {
            let beta = PI * (PONTOS[j] + 1.0);
            let w_beta = PESOS[j] * PI;

            let x = 40.0 * alpha * beta.cos();
            let y = 40.0 * alpha * beta.sin();
            let jaco = 1600.0 * alpha;

            soma += w_alpha * w_beta * integrando(x, y) * jaco;
        }
    }
    soma
}

/// Área da superfície paraboloidal por coordenadas cartesianas com restrição à elipse
pub fn area_superficie_cartesiana_com_elipse(integrando: &dyn Fn(f64, f64) -> f64) -> f64 {
    let mut soma = 0.0;

    for i in 0..3 {
        let alpha = PONTOS[i];
        let w_alpha = PESOS[i];
        for j in 0..3 {
            let beta = PONTOS[j];
            let w_beta = PESOS[j];

            let x = 40.0 * alpha;
            let y = 40.0 * beta;

            if (x * x) / 1600.0 + (y * y) / 1600.0 <= 1.0 {
                let jaco = 1600.0;
                soma += w_alpha * w_beta * integrando(x, y) * jaco;
            }
        }
    }
    soma
}

/// Calcula a área de uma superfície z = f(x, y)
/// sobre a região retangular [x_min, x_max] × [y_min, y_max]
/// usando quadratura de Gauss-Legendre 3x3.
pub fn area_superficie_gauss_legendre(
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    f: &dyn Fn(f64, f64) -> f64,
) -> f64 {
    const PONTOS: [f64; 3] = [-0.774596669241483, 0.0, 0.774596669241483];
    const PESOS: [f64; 3] = [0.555555555555556, 0.888888888888889, 0.555555555555556];

    let dx = (x_max - x_min) / 2.0;
    let dy = (y_max - y_min) / 2.0;
    let jaco = dx * dy;
    let h = 1e-5;

    let mut soma = 0.0;

    for i in 0..3 {
        let xi = dx * PONTOS[i] + (x_min + x_max) / 2.0;
        for j in 0..3 {
            let yj = dy * PONTOS[j] + (y_min + y_max) / 2.0;

            let central = Central {};

            // Para ∂f/∂x fixando y = yj
            let fx = central.derivada_primeira(&|x| f(x, yj), xi, h, OrdemErro::Quarta);
            // Para ∂f/∂y fixando x = xi
            let fy = central.derivada_primeira(&|y| f(xi, y), yj, h, OrdemErro::Quarta);

            let integrando = (fx * fx + fy * fy + 1.0).sqrt();

            soma += PESOS[i] * PESOS[j] * integrando;
        }
    }

    jaco * soma
}
