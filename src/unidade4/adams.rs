use super::runge_kutta::rungekutta_ordem3;

pub fn adams_bashforth_moulton_4(
    s0: &Vec<f64>,
    t0: f64,
    tf: f64,
    dt: f64,
    f: impl Fn(&Vec<f64>, f64) -> Vec<f64>,
) -> Vec<(f64, Vec<f64>)>
{
    let base = rungekutta_ordem3(s0, t0, t0 + 2.0 * dt, dt, &f);
    let mut resultados = base.clone();

    let mut t = t0 + 2.0 * dt;
    let mut s2 = base[0].1.clone(); // t - 2dt
    let mut s1 = base[1].1.clone(); // t - dt
    let mut s  = base[2].1.clone(); // t

    while t <= tf - 2.0 * dt + 1e-10 {
        let f0 = f(&s2, t - 2.0 * dt);
        let f1 = f(&s1, t - dt);
        let f2 = f(&s, t);

        let mut sp = vec![0.0; s.len()];
        for i in 0..s.len() {
            sp[i] = s[i] + (dt / 12.0) * (23.0 * f2[i] - 16.0 * f1[i] + 5.0 * f0[i]);
        }

        let fc = f(&sp, t + dt);

        let mut sn = vec![0.0; s.len()];
        for i in 0..s.len() {
            sn[i] = s[i] + (dt / 12.0) * (5.0 * fc[i] + 8.0 * f2[i] - f1[i]);
        }

        resultados.push((t + dt, sn.clone()));

        s2 = s1;
        s1 = s;
        s = sn;
        t += dt;
    }

    resultados
}
