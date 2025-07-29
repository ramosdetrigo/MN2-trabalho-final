use trabalho_final::unidade2::exponencial::*;
fn f_teste1(x: f64) -> f64 {
    x.powf(-2.0 / 3.0)
}

fn f_teste2(x: f64) -> f64 {
    1.0 / x.sqrt()
}

fn f_teste3(x: f64) -> f64 {
    x.sqrt()
}

fn main() {
    let a = 0.0;
    let b = 1.0;
    let eps = 1e-8;
    let c = 8.0;

    let funclist: Vec<(&str, fn(f64) -> f64)> = vec![
        ("f(x) = x^{-2/3}", f_teste1),
        ("f(x) = 1/sqrt(x)", f_teste2),
        ("f(x) = sqrt(x)", f_teste3),
    ];

    println!("INTEGRAÇÃO NUMÉRICA COM MUDANÇAS EXPONENCIAIS (SIMPLES E DUPLA)");
    println!("Limite de integração: [0,1], eps = {}, C = {}\n", eps, c);

    for (nome, f) in funclist {
        println!(">> Testando: {}", nome);

        let g_simples = mudanca_exponencial_simples(a, b, f);
        let integral_simples = integral_com_erro(-c, c, eps, &g_simples);
        println!("  Resultado (exp. simples): {:.10}", integral_simples);

        let g_dupla = mudanca_exponencial_dupla(a, b, f);
        let integral_dupla = integral_com_erro(-c, c, eps, &g_dupla);
        println!("  Resultado (exp. dupla):   {:.10}", integral_dupla);

        println!();
    }
}
