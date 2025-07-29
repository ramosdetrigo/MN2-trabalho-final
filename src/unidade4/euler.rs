pub fn passo_euler_explicito(
    s: &Vec<f64>,
    t: f64,
    delta: f64,
    f: impl Fn(&Vec<f64>, f64) -> Vec<f64>,
) -> Vec<f64> {
    let ds = f(&s, t); // F(S(ti), ti)
    s.iter() // S1 = S0 + delta*F(S0, t0)
        .zip(ds.iter())
        .map(|(x, xs)| x + delta * xs)
        .collect()
}

// Retorna uma lista de estados (t, estado)
pub fn euler_explicito(
    s0: &Vec<f64>,
    t0: f64,
    tf: f64,
    delta: f64,
    f: impl Fn(&Vec<f64>, f64) -> Vec<f64>,
) -> Vec<(f64, Vec<f64>)> {
    let mut estados: Vec<(f64, Vec<f64>)> = Vec::new(); // sequência de estados
    let mut t = t0; // t acumulado
    let mut s = s0.clone(); // estado acumulado

    // iteramos todos os pontos entre t0 e tf com o delta
    while t <= tf + 1e-10 {
        estados.push((t, s.clone()));

        s = passo_euler_explicito(&s, t, delta, &f);

        t += delta;
    }

    estados
}

pub fn passo_euler_implicito(
    s_anterior: &Vec<f64>,
    t: f64,
    delta: f64,
    f: impl Fn(&Vec<f64>, f64) -> Vec<f64>,
) -> Vec<f64> {
    let mut s_next = s_anterior.clone(); // chute inicial: o estado anterior

    for _ in 0..20 {
        let f_sn = f(&s_next, t);

        let mut guess = vec![0.0; s_anterior.len()];
        for i in 0..s_anterior.len() {
            guess[i] = s_anterior[i] + delta * f_sn[i];
        }

        let convergiu = guess
            .iter()
            .zip(&s_next)
            .all(|(new, old)| (new - old).abs() <= 1e-10);

        if convergiu {
            break;
        }

        s_next = guess;
    }

    s_next
}

pub fn euler_implicito(
    s0: &Vec<f64>,
    t0: f64,
    tf: f64,
    delta: f64,
    f: impl Fn(&Vec<f64>, f64) -> Vec<f64>,
) -> Vec<(f64, Vec<f64>)> {
    let mut estados: Vec<(f64, Vec<f64>)> = Vec::new(); // sequência de estados
    let mut t = t0; // t acumulado
    let mut s = s0.clone(); // estado acumulado

    while t <= tf + 1e-10 {
        estados.push((t, s.clone())); // adiciona o estado atual à lista
        t += delta; // incrementa o tempo

        s = passo_euler_implicito(&s, t, delta, &f);
    }

    estados // retorna a sequência de estados
}
