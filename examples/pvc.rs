use nalgebra::{DMatrix, DVector};

fn main() {
    let dx: f64 = 0.1;
    let n = 20; // total de intervalos
    let tam = n - 1; // 19 incógnitas internas

    // Coeficientes
    let a = 1.0 - 7.0 * dx / 2.0; // 0.65
    let b = -2.0 - dx.powi(2); // -2.01
    let c = 1.0 + 7.0 * dx / 2.0; // 1.35
    let d = 2.0 * dx.powi(2); // 0.02

    // Condições de contorno
    let u0 = 10.0;
    let u2 = 1.0;

    // Matriz A e vetor b
    let mut a_matrix = DMatrix::<f64>::zeros(tam, tam);
    let mut b_vector = DVector::<f64>::from_element(tam, d);

    for i in 0..tam {
        if i > 0 {
            a_matrix[(i, i - 1)] = a;
        }
        a_matrix[(i, i)] = b;
        if i < tam - 1 {
            a_matrix[(i, i + 1)] = c;
        }
    }

    // Ajustar vetor b pelas condições de contorno
    b_vector[0] -= a * u0;
    b_vector[tam - 1] -= c * u2;

    // Resolver o sistema linear
    let u_internal = a_matrix
        .lu()
        .solve(&b_vector)
        .expect("Falha ao resolver o sistema");

    // Combinar solução completa
    let mut x_vals = vec![0.0];
    let mut u_vals = vec![u0];

    for i in 1..=tam {
        x_vals.push(i as f64 * dx);
        u_vals.push(u_internal[i - 1]);
    }

    x_vals.push(2.0);
    u_vals.push(u2);

    // Raízes da equação característica
    let r1 = (-7.0 + 53.0_f64.sqrt()) / 2.0;
    let r2 = (-7.0 - 53.0_f64.sqrt()) / 2.0;

    // Constantes da solução exata
    let c1 = 2.2670974585753934;
    let c2 = 9.732902541424608;

    // Função da solução exata
    fn u_exata(x: f64, c1: f64, c2: f64, r1: f64, r2: f64) -> f64 {
        c1 * (r1 * x).exp() + c2 * (r2 * x).exp() - 2.0
    }

    println!(
        "\n{:^6} | {:^12} | {:^12} | {:^12} | {:^12}",
        "x", "Aproximada", "Exata", "Erro Abs", "Erro Rel (%)"
    );
    println!("{}", "-".repeat(64));

    for (_i, (&x, &u_aprox)) in x_vals.iter().zip(u_vals.iter()).enumerate() {
        let u_ex = u_exata(x, c1, c2, r1, r2);
        let erro_abs = (u_aprox - u_ex).abs();
        let erro_rel = if u_ex.abs() > 1e-10 {
            100.0 * erro_abs / u_ex.abs()
        } else {
            0.0
        };

        println!(
            "{:>5.1} | {:>12.6} | {:>12.6} | {:>12.6} | {:>12.6}",
            x, u_aprox, u_ex, erro_abs, erro_rel
        );
    }
}
