#![allow(dead_code)]
fn fechada_trapezio(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    (b - a) * (f(a) + f(b)) / 2.0
}

fn fechada_simpson(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let m = (a + b) / 2.0;
    (b - a) * (f(a) + 4.0 * f(m) + f(b)) / 6.0
}

fn fechada_38(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let h = (b - a) / 3.0;
    (3.0 * h / 8.0) * (f(a) + 3.0 * f(a + h) + 3.0 * f(a + 2.0 * h) + f(b))
}

fn fechada_composta_trapezio(a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = (f(a) + f(b)) / 2.0;
    for i in 1..n {
        sum += f(a + i as f64 * h);
    }
    h * sum
}

fn fechada_composta_simpson(a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    if n % 2 != 0 {
        panic!("n deve ser par para Simpson composta.");
    }
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n {
        let x = a + i as f64 * h;
        sum += if i % 2 == 0 { 2.0 * f(x) } else { 4.0 * f(x) };
    }
    h * sum / 3.0
}

fn fechada_composta_38(a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    if n % 3 != 0 {
        panic!("n deve ser múltiplo de 3 para 3/8 composta.");
    }
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n {
        let x = a + i as f64 * h;
        sum += if i % 3 == 0 { 2.0 * f(x) } else { 3.0 * f(x) };
    }
    3.0 * h * sum / 8.0
}

#[derive(Debug, Clone, Copy)]
pub enum NewtonCotesFechada {
    Trapezio, // f nos extremos
    Simpson,  // f nos extremos e meio
    Regra38   // 3 subintervalos, 4 pontos
}

impl NewtonCotesFechada {
    pub fn aplicar(&self,   a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
        match self {
            Self::Trapezio => fechada_trapezio(a, b, f),
            Self::Simpson => fechada_simpson(a, b, f),
            Self::Regra38 => fechada_38(a, b, f)
        }
    }

    pub fn aplicar_composta(&self, a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
        match self {
            Self::Trapezio => fechada_composta_trapezio(a, b, n, f),
            Self::Simpson => fechada_composta_simpson(a, b, n, f),
            Self::Regra38 => fechada_composta_38(a, b, n, f)
        }
    }
}