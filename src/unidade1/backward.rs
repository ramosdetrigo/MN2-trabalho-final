use super::{Filosofia, OrdemErro};

pub struct Backward {}

impl Filosofia for Backward {
    fn derivada_primeira(&self, f: &dyn Fn(f64) -> f64, x: f64, delta: f64, erro: OrdemErro) -> f64 {
        match erro {
            // (f(x) - f(x-dx)) / dx
            OrdemErro::Linear => (f(x) - f(x - delta)) / delta,

            // (3f(x) -4f(x-dx) +f(x-2dx)) / 2dx
            OrdemErro::Quadratico => {
                (3.0 * f(x) - 4.0 * f(x - delta) + f(x - 2.0 * delta)) / (2.0 * delta)
            }

            // (11f(x) -18f(x-dx) +9f(x-2dx) -2f(x-3dx)) / 6dx
            OrdemErro::Cubico => {
                (11.0 * f(x) - 18.0 * f(x - delta) + 9.0 * f(x - 2.0 * delta)
                    - 2.0 * f(x - 3.0 * delta))
                    / (6.0 * delta)
            }

            // (25f(x) -48f(x-dx) +36f(x-2dx) -16f(x-3dx) +3f(x-4dx)) / 12dx
            OrdemErro::Quarta => {
                (25.0 * f(x) - 48.0 * f(x - delta) + 36.0 * f(x - 2.0 * delta)
                    - 16.0 * f(x - 3.0 * delta)
                    + 3.0 * f(x - 4.0 * delta))
                    / (12.0 * delta)
            }
        }
    }

    fn derivada_segunda(&self, f: &dyn Fn(f64) -> f64, x: f64, delta: f64, erro: OrdemErro) -> f64 {
        match erro {
            // (f(x) -2f(x-dx) +f(x-2dx)) / dx²
            OrdemErro::Linear => (f(x) - 2.0 * f(x - delta) + f(x - 2.0 * delta)) / (delta * delta),

            // (2f(x) -5f(x-dx) +4f(x-2dx) -f(x-3dx)) / dx²
            OrdemErro::Quadratico => {
                (2.0 * f(x) - 5.0 * f(x - delta) + 4.0 * f(x - 2.0 * delta) - f(x - 3.0 * delta))
                    / (delta * delta)
            }

            // (35f(x) -104f(x-dx) +114f(x-2dx) -56f(x-3dx) +11f(x-4dx)) / 12dx²
            OrdemErro::Cubico => {
                (35.0 * f(x) - 104.0 * f(x - delta) + 114.0 * f(x - 2.0 * delta)
                    - 56.0 * f(x - 3.0 * delta)
                    + 11.0 * f(x - 4.0 * delta))
                    / (12.0 * delta * delta)
            }

            // (45f(x) -154f(x-dx) +214f(x-2dx) -156f(x-3dx) +61f(x-4dx) -10f(x-5dx)) / 12dx²
            OrdemErro::Quarta => {
                (45.0 * f(x) - 154.0 * f(x - delta) + 214.0 * f(x - delta * 2.0)
                    - 156.0 * f(x - delta * 3.0)
                    + 61.0 * f(x - delta * 4.0)
                    - 10.0 * f(x - delta * 5.0))
                    / (12.0 * delta * delta)
            }
        }
    }

    fn derivada_terceira(&self, f: &dyn Fn(f64) -> f64, x: f64, delta: f64, erro: OrdemErro) -> f64 {
        match erro {
            // (f(x) -3f(x-dx) +3f(x-2dx) -f(x-3dx)) / dx³
            OrdemErro::Linear => {
                (f(x) - 3.0 * f(x - delta) + 3.0 * f(x - 2.0 * delta) - f(x - 3.0 * delta))
                    / (delta * delta * delta)
            }

            // (-5f(x) +18f(x-dx) -24f(x-2dx) +14f(x-3dx) -3f(x-4dx)) / 2dx³
            OrdemErro::Quadratico => {
                (-5.0 * f(x) + 18.0 * f(x - delta) - 24.0 * f(x - 2.0 * delta)
                    + 14.0 * f(x - 3.0 * delta)
                    - 3.0 * f(x - 4.0 * delta))
                    / (2.0 * delta * delta * delta)
            }

            // (17f(x) -71f(x-dx) +118f(x-2dx) -98f(x-3dx) +41f(x-4dx) -7f(x-5dx)) / 4dx³
            OrdemErro::Cubico => {
                (17.0 * f(x) - 71.0 * f(x - delta) + 118.0 * f(x - delta * 2.0)
                    - 98.0 * f(x - delta * 3.0)
                    + 41.0 * f(x - delta * 4.0)
                    - 7.0 * f(x - delta * 5.0))
                    / (4.0 * delta * delta * delta)
            }

            // (49f(x) -232f(x-dx) +461f(x-2dx) -496f(x-3dx) +307f(x-4dx) -104f(x-5dx) +15f(x-6dx)) / 8dx³
            OrdemErro::Quarta => {
                (49.0 * f(x) - 232.0 * f(x - delta) + 461.0 * f(x - delta * 2.0)
                    - 496.0 * f(x - delta * 3.0)
                    + 307.0 * f(x - delta * 4.0)
                    - 104.0 * f(x - delta * 5.0)
                    + 15.0 * f(x - delta * 6.0))
                    / (8.0 * delta * delta * delta)
            }
        }
    }
}
