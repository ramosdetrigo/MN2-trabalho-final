pub fn rungekutta_ordem3(
    s0: &Vec<f64>,
    t0: f64,
    tf: f64,
    dt: f64,
    f: impl Fn(&Vec<f64>, f64) -> Vec<f64>,
) -> Vec<(f64, Vec<f64>)>
{
    let mut estados = Vec::new();
    let mut s = s0.to_vec();
    let mut t = t0;

    while t <= tf + 1e-10 {
        estados.push((t, s.clone()));

        let k1 = f(&s, t);

        let sk2: Vec<f64> = s
            .iter()
            .zip(&k1)
            .map(|(&si, &k1i)| si + 0.5 * dt * k1i)
            .collect();
        let k2 = f(&sk2, t + 0.5 * dt);

        let sk3: Vec<f64> = s
            .iter()
            .zip(&k1)
            .zip(&k2)
            .map(|((&si, &k1i), &k2i)| si - dt * k1i + 2.0 * dt * k2i)
            .collect();
        let k3 = f(&sk3, t + dt);

        for i in 0..s.len() {
            s[i] += dt * (k1[i] + 4.0 * k2[i] + k3[i]) / 6.0;
        }

        t += dt;
    }

    estados
}


pub fn rungekutta_ordem4(
    s0: &Vec<f64>,
    t0: f64,
    tf: f64,
    delta: f64,
    f: impl Fn(&Vec<f64>, f64) -> Vec<f64>,
) -> Vec<(f64, Vec<f64>)> {
    let mut estados: Vec<(f64, Vec<f64>)> = Vec::new();
    let mut t = t0;
    let mut s = s0.clone();

    while t <= tf + 1e-10 {
        estados.push((t, s.clone()));

        let k1 = f(&s, t);

        let s_k2: Vec<f64> = s.iter()
            .zip(k1.iter())
            .map(|(si, k1i)| si + delta * 0.5 * k1i)
            .collect();
        let k2 = f(&s_k2, t + delta * 0.5);

        let s_k3: Vec<f64> = s.iter()
            .zip(k2.iter())
            .map(|(si, k2i)| si + delta * 0.5 * k2i)
            .collect();
        let k3 = f(&s_k3, t + delta * 0.5);

        let s_k4: Vec<f64> = s.iter()
            .zip(k3.iter())
            .map(|(si, k3i)| si + delta * k3i)
            .collect();
        let k4 = f(&s_k4, t + delta);

        for i in 0..s.len() {
            s[i] += delta / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }

        t += delta;
    }

    estados
}
