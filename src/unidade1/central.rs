use super::{Filosofia, OrdemErro};

pub struct Central {}

impl Filosofia for Central {
    fn derivada_primeira(&self, f: &dyn Fn(f64) -> f64, x: f64, delta: f64, erro: OrdemErro) -> f64 {
        match erro {
            // (f(x+dx) - f(x-dx)) / 2dx 
            OrdemErro::Quadratico => (f(x + delta) - f(x - delta)) / (2.0 * delta),

            // (-f(x+2dx) +8f(x+dx) -8f(x-dx) +f(x-2dx)) / 12dx 
            OrdemErro::Quarta => {
                (-f(x + 2.0 * delta) + 8.0 * f(x + delta) - 8.0 * f(x - delta) + f(x - 2.0 * delta))
                    / (12.0 * delta)
            }

            _ => panic!(
                "Ordem de erro {:?} não suportada para filosofia central",
                erro
            ),
        }
    }

    fn derivada_segunda(&self, f: &dyn Fn(f64) -> f64, x: f64, delta: f64, erro: OrdemErro) -> f64 {
        match erro {
            // (f(x+dx) -2f(x) +f(x-dx)) / dx² 
            OrdemErro::Quadratico => {
                (f(x + delta) - 2.0 * f(x) + f(x - delta)) / (delta * delta)
            }

            // (-f(x+2dx) +16f(x+dx) -30f(x) +16f(x-dx) -f(x-2dx)) / 12dx²  
            OrdemErro::Quarta => {
                (-f(x + 2.0 * delta) + 16.0 * f(x + delta) - 30.0 * f(x)
                    + 16.0 * f(x - delta) - f(x - 2.0 * delta))
                    / (12.0 * delta * delta)
            }

            _ => panic!(
                "Ordem de erro {:?} não suportada para filosofia central",
                erro
            ),
        }
    }

    fn derivada_terceira(&self, f: &dyn Fn(f64) -> f64, x: f64, delta: f64, erro: OrdemErro) -> f64 {
        match erro {
            // (f(x-2dx) -2f(x-dx) +2f(x+dx) -f(x+2dx)) / 2dx³ 
            OrdemErro::Quadratico => {
                (f(x - 2.0 * delta) - 2.0 * f(x - delta) + 2.0 * f(x + delta) - f(x + 2.0 * delta))
                    / (2.0 * delta * delta * delta)
            }

            // (-f(x-3dx) +8f(x-2dx) -13f(x-dx) +13f(x+dx) -8f(x+2dx) +f(x+3dx)) / 8dx³  
            OrdemErro::Quarta => {
                (-f(x - 3.0 * delta) + 8.0 * f(x - 2.0 * delta) - 13.0 * f(x - delta)
                    + 13.0 * f(x + delta) - 8.0 * f(x + 2.0 * delta) + f(x + 3.0 * delta))
                    / (8.0 * delta * delta * delta)
            }

            _ => panic!(
                "Ordem de erro {:?} não suportada para filosofia central",
                erro
            ),
        }
    }
}
