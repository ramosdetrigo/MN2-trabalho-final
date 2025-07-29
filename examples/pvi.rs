use trabalho_final::unidade4::*;

fn main() {
    let f1 = |s: &Vec::<f64>, _t: f64| -> Vec<f64> {
        vec![(2.0/3.0) * s[0]]
    };
    let f1_s0 = vec![2.0];
    let f1_delta = 0.5;

    let f2 = |s: &Vec::<f64>, _t: f64| -> Vec<f64> {
        let g = 10.0;
        let k = 0.5;
        let m = 0.5;
        vec![-g - k/m * s[0], s[0]]
    };
    let f2_s0 = vec![3.0, 150.0];
    let f2_delta = 0.1;

    let e_ex1 = euler_explicito(&f1_s0, 0.0, 1.0, f1_delta, f1);
    println!(" ========== Euler explícito (PVI 1): ==========");
    for s in e_ex1 {
        println!("{s:?}");
    }

    let e_ex2 = euler_explicito(&f2_s0, 0.0, 0.2, f2_delta, f2);
    println!("\n ========== Euler explícito (PVI 2): ========== ");
    for s in e_ex2 {
        println!("{s:?}");
    }
    let mut s0 = f2_s0.clone();
    let mut t = 0.0;
    let mut contador = 0;
    while s0[1] >= 0.0 {
        s0 = passo_euler_explicito(&s0, t, f2_delta, f2);
        t += f2_delta;
        contador += 1;
    }
    println!("{contador} iterações até espaço = 0");
    
    let e_im1 = euler_implicito(&f1_s0, 0.0, 1.0, f1_delta, f1);
    println!("\n ========== Euler implícito (PVI 1): ========== ");
    for s in e_im1 {
        println!("{s:?}");
    }

    let e_im2 = euler_implicito(&f2_s0, 0.0, 0.2, f2_delta, f2);
    println!("\n ========== Euler implícito (PVI 2): ==========");
    for s in e_im2 {
        println!("{s:?}");
    }
    let mut s0 = f2_s0.clone();
    let mut t = 0.0;
    let mut contador = 0;
    while s0[1] >= 0.0 {
        s0 = passo_euler_implicito(&s0, t, f2_delta, f2);
        t += f2_delta;
        contador += 1;
    }
    println!("{contador} iterações até espaço = 0");

    let rk4_1 = rungekutta_ordem4(&f1_s0, 0.0, 1.0, f1_delta, f1);
    println!("\n ========== Runge Kutta ordem 4 (PVI 1): ==========");
    for s in rk4_1 {
        println!("{s:?}");
    }

    let rk4_2 = rungekutta_ordem4(&f2_s0, 0.0, 0.2, f2_delta, f2);
    println!("\n ========== Runge Kutta ordem 4 (PVI 2): ==========");
    for s in rk4_2 {
        println!("{s:?}");
    }

    let abm4_1 = adams_bashforth_moulton_4(&f1_s0, 0.0, 1.0, f1_delta, f1);
    println!("\n ========== Adams Bashfort Moulton ordem 4 (PVI 1): ==========");
    for s in abm4_1 {
        println!("{s:?}");
    }

    let abm4_2 = adams_bashforth_moulton_4(&f2_s0, 0.0, 0.2, f2_delta, f2);
    println!("\n ========== Adams Bashfort Moulton ordem 4 (PVI 2): ==========");
    for s in abm4_2 {
        println!("{s:?}");
    }
}