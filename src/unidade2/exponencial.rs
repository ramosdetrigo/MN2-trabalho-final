pub fn integra(a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    let dx = (b - a) / n as f64;
    let mut area = 0.0;
    for i in 0..n {
        let xi = a + dx / 2.0 + i as f64 * dx;
        area += dx * f(xi);
    }
    area
}

pub fn integral_com_erro(a: f64, b: f64, eps: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let mut n = 1;
    let mut erro = 1.0;
    let mut area_ant = 0.0;
    let mut area = 0.0;

    while erro > eps {
        n *= 2;
        area = integra(a, b, n, f);
        if area != 0.0 {
            erro = ((area - area_ant) / area).abs();
        }
        area_ant = area;
    }

    area
}

// ================= TRANSFORMAÇÕES ==================

pub fn mudanca_exponencial_simples(
    a: f64,
    b: f64,
    f: impl Fn(f64) -> f64 + 'static,
) -> impl Fn(f64) -> f64 {
    move |s: f64| {
        let t = s.tanh();
        let x = 0.5 * (b + a) + 0.5 * (b - a) * t;
        let dxds = 0.5 * (b - a) * (1.0 - t * t) / s.cosh().powi(2);
        f(x) * dxds
    }
}

pub fn mudanca_exponencial_dupla(
    a: f64,
    b: f64,
    f: impl Fn(f64) -> f64 + 'static,
) -> impl Fn(f64) -> f64 {
    move |s: f64| {
        let arg = 0.5 * std::f64::consts::PI * s.sinh();
        let t = arg.tanh();
        let x = 0.5 * (b + a) + 0.5 * (b - a) * t;
        let dt_ds = 0.5 * std::f64::consts::PI * s.cosh() * (1.0 - t * t);
        let dxds = 0.5 * (b - a) * dt_ds;
        f(x) * dxds
    }
}
