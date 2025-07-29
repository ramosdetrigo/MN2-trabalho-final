pub fn gauss_legendre(a: f64, b: f64, n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    let (pesos, pontos) = match n {
        2 => (vec![1.0, 1.0], vec![-0.5773502692, 0.5773502692]),
        3 => (
            vec![0.5555555556, 0.8888888889, 0.5555555556],
            vec![-0.7745966692, 0.0, 0.7745966692],
        ),
        4 => (
            vec![0.3478548451, 0.6521451549, 0.6521451549, 0.3478548451],
            vec![-0.8611363116, -0.3399810436, 0.3399810436, 0.8611363116],
        ),
        _ => panic!("Legendre só suporta n = 2, 3 ou 4"),
    };

    let mut soma = 0.0;
    for (w, xi) in pesos.iter().zip(pontos.iter()) {
        let x = 0.5 * (b - a) * xi + 0.5 * (b + a);
        soma += w * f(x);
    }
    soma * 0.5 * (b - a)
}

pub fn gauss_hermite(n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    let (pesos, pontos) = match n {
        2 => (vec![0.8862269255; 2], vec![-0.7071067812, 0.7071067812]),
        3 => (
            vec![0.2954089756, 1.1816359006, 0.2954089752],
            vec![-1.2247448714, 0.0, 1.2247448714],
        ),
        4 => (
            vec![0.0813128354, 0.8049140900, 0.8049140900, 0.0813128354],
            vec![-1.6506801239, -0.5246476233, 0.5246476233, 1.6506801239],
        ),
        _ => panic!("Hermite só suporta n = 2, 3 ou 4"),
    };

    pesos
        .iter()
        .zip(pontos.iter())
        .map(|(w, x)| w * f(*x))
        .sum()
}

pub fn gauss_chebyshev(n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    let peso = std::f64::consts::PI / n as f64;
    (1..=n)
        .map(|i| {
            let x = ((2 * i - 1) as f64 * std::f64::consts::PI) / (2.0 * n as f64);
            let xi = x.cos();
            peso * f(xi)
        })
        .sum()
}

pub fn gauss_laguerre(n: usize, f: &dyn Fn(f64) -> f64) -> f64 {
    let (pesos, pontos) = match n {
        2 => (
            vec![0.8535533906, 0.1464466094],
            vec![0.5857864376, 3.4142135624],
        ),
        3 => (
            vec![0.7110930099, 0.2785177336, 0.0103892565],
            vec![0.4157745568, 2.2942803603, 6.2899450829],
        ),
        4 => (
            vec![0.6031541043, 0.3574186924, 0.0388879085, 0.0005392947],
            vec![0.3225476896, 1.7457611012, 4.5366202969, 9.3950709123],
        ),
        _ => panic!("Laguerre só suporta n = 2, 3 ou 4"),
    };

    pesos
        .iter()
        .zip(pontos.iter())
        .map(|(w, x)| w * f(*x))
        .sum()
}
