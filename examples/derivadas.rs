use trabalho_final::unidade1::backward::Backward;
use trabalho_final::unidade1::central::Central;
use trabalho_final::unidade1::forward::Forward;
#[allow(unused_imports)]
use trabalho_final::unidade1::*;

fn main() {
    // e^x * sin(x)
    let f1 = |x: f64| (x.exp()) * x.sin();
    let f1_1 = |x: f64| (x.exp()) * (x.sin() + x.cos());
    let f1_2 = |x: f64| (x.exp()) * (2.0 * x.cos());
    let f1_3 = |x: f64| (x.exp()) * (2.0 * x.cos() - 2.0 * x.sin());

    // ln(x^2 + 1)
    let f2 = |x: f64| (x * x + 1.0).ln();
    let f2_1 = |x: f64| 2.0 * x / (x * x + 1.0);
    let f2_2 = |x: f64| 2.0 * (1.0 - x * x) / ((x * x + 1.0).powi(2));
    let f2_3 = |x: f64| 4.0 * x * (x * x - 3.0) / ((x * x + 1.0).powi(3));

    let x1 = 0.5;
    let x2 = 1.0;
    let delta = 1e-3;

    println!(
        "Valores exatos para e^x * sin(x) com x = 0.5: \nderivada primeira: {}\nderivada segunda: {}\n derivada terceira: {}",
        f1_1(x1), f1_2(x1), f1_3(x1)
    );

    println!(
        "Valores exatos para ln(x^2 + 1) com x = 1.0: \nderivada primeira: {}\nderivada segunda: {}\n derivada terceira: {}",
        f2_1(x2), f2_2(x2), f2_3(x2)
    );

    let ordens = [
        OrdemErro::Linear,
        OrdemErro::Quadratico,
        OrdemErro::Cubico,
        OrdemErro::Quarta,
    ];

    let filosofias: [(&dyn Filosofia, &str); 3] = [
        (&Forward {}, "FORWARD"),
        (&Backward {}, "BACKWARD"),
        (&Central {}, "CENTRAL"),
    ];

    for (filosofia, filosofia_nome) in filosofias {
        println!("========== {filosofia_nome} ==========");
        for ordem in ordens {
            if filosofia_nome == "CENTRAL" {
                if ordem == OrdemErro::Linear || ordem == OrdemErro::Cubico {
                    continue;
                }
            }
            println!(
                "Derivada primeira de e^x * sin(x) com x = 0.5 (erro {ordem:?}): {:.12}\nDerivada primeira de ln(x^2 + 1) com x = 1.0 (erro {ordem:?}): {:.12}",
                filosofia.derivada_primeira(&f1, x1, delta, ordem),
                filosofia.derivada_primeira(&f2, x2, delta, ordem)
            );
            println!(
                "Derivada segunda de e^x * sin(x) com x = 0.5 (erro {ordem:?}): {:.12}\nDerivada segunda de ln(x^2 + 1) com x = 1.0 (erro {ordem:?}): {:.12}",
                filosofia.derivada_segunda(&f1, x1, delta, ordem),
                filosofia.derivada_segunda(&f2, x2, delta, ordem)
            );
            println!(
                "Derivada terceira de e^x * sin(x) com x = 0.5 (erro {ordem:?}): {:.12}\nDerivada terceira de ln(x^2 + 1) com x = 1.0 (erro {ordem:?}): {:.12}",
                filosofia.derivada_terceira(&f1, x1, delta, ordem),
                filosofia.derivada_terceira(&f2, x2, delta, ordem)
            );
            println!();
        }
    }
}
