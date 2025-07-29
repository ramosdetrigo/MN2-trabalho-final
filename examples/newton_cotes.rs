use trabalho_final::unidade2::newton_cotes::*;

fn main() {
    let a = 0.0;
    let b = 1.0;
    let n = 12;
    let f = |x: f64| x.powi(3);

    let regras = [
        NewtonCotes::Fechada(NewtonCotesFechada::Trapezio),
        NewtonCotes::Fechada(NewtonCotesFechada::Simpson),
        NewtonCotes::Fechada(NewtonCotesFechada::Regra38),
        
        NewtonCotes::Aberta(NewtonCotesAberta::Meio),
        NewtonCotes::Aberta(NewtonCotesAberta::Grau2),
        NewtonCotes::Aberta(NewtonCotesAberta::Grau3),
        NewtonCotes::Aberta(NewtonCotesAberta::Grau4),
    ];

    for regra in &regras {
        let resultado = regra.aplicar_composta(a, b, n, &f);
        println!("{:?} composta (n = {}): {:.10}", regra, n, resultado);
    }

    println!("Valor exato: {:.10}", 0.25);
}
