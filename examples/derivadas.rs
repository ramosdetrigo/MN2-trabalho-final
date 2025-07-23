use trabalho_final::unidade1::backward::Backward;
use trabalho_final::unidade1::central::Central;
use trabalho_final::unidade1::forward::Forward;
#[allow(unused_imports)]
use trabalho_final::unidade1::*;

fn main() {
    let f = |x: f64| x.powi(4);
    let x = 1.0;
    let delta = 1e-3;

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
                "Derivada primeira (erro {ordem:?}): {:.12}",
                filosofia.derivada_primeira(&f, x, delta, ordem)
            );
            println!(
                "Derivada segunda (erro {ordem:?}):  {:.12}",
                filosofia.derivada_segunda(&f, x, delta, ordem)
            );
            println!(
                "Derivada terceira (erro {ordem:?}): {:.12}",
                filosofia.derivada_terceira(&f, x, delta, ordem)
            );
            println!();
        }
    }
}
