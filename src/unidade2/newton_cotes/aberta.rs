// Abertas
fn aberta_m(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    (b - a) * f((a + b) / 2.0)
}

fn aberta_2(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let h = (b - a) / 3.0;
    (3.0 / 2.0) * h * (f(a + h) + f(a + 2.0 * h))
}

fn aberta_3(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let h = (b - a) / 4.0;
    (4.0 / 3.0) * h * (2.0 * f(a + h) - f(a + 2.0 * h) + 2.0 * f(a + 3.0 * h))
}

fn aberta_4(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let h = (b - a) / 5.0;
    (5.0 / 24.0) * h * (
        11.0 * f(a + h) +
        f(a + 2.0 * h) +
        f(a + 3.0 * h) +
        11.0 * f(a + 4.0 * h)
    )
}

// Abertas compostas
fn integrar_aberta_composta(
    a: f64,
    b: f64,
    n: usize,
    regra: &dyn Fn(f64, f64, &dyn Fn(f64) -> f64) -> f64,
    f: &dyn Fn(f64) -> f64,
) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.0;
    for i in 0..n {
        let x0 = a + i as f64 * h;
        let x1 = x0 + h;
        sum += regra(x0, x1, f);
    }
    sum
}


fn aberta_composta_meio(a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    integrar_aberta_composta(a, b, n, &aberta_m, f)
}

fn aberta_composta_2(a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    integrar_aberta_composta(a, b, n, &aberta_2, f)
}

fn aberta_composta_3(a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    integrar_aberta_composta(a, b, n, &aberta_3, f)
}

fn aberta_composta_4(a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    integrar_aberta_composta(a, b, n, &aberta_4, f)
}

#[derive(Debug, Clone, Copy)]
pub enum NewtonCotesAberta {
    Meio,     // Grau 1 – ponto médio
    Grau2,    // Grau 2 – dois pontos (a + h, a + 2h)
    Grau3,    // Grau 3 – três pontos (a + h, a + 2h, a + 3h)
    Grau4,    // Grau 4 – quatro pontos (a + h, ..., a + 4h)
}

impl NewtonCotesAberta {
    /// Aplica a fórmula simples no intervalo `[a, b]`
    pub fn aplicar(&self, a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
        match self {
            Self::Meio => aberta_m(a, b, f),
            Self::Grau2 => aberta_2(a, b, f),
            Self::Grau3 => aberta_3(a, b, f),
            Self::Grau4 => aberta_4(a, b, f),
        }
    }

    /// Aplica a fórmula composta com `n` subintervalos
    pub fn aplicar_composta(&self, a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
        match self {
            Self::Meio => aberta_composta_meio(a, b, n, f),
            Self::Grau2 => aberta_composta_2(a, b, n, f),
            Self::Grau3 => aberta_composta_3(a, b, n, f),
            Self::Grau4 => aberta_composta_4(a, b, n, f),
        }
    }
}