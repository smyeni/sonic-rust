fn sin_taylor(x: f64, terms: u32) -> f64 {
    let mut term = x;
    let mut sum = x;
    for n in 1..terms {
        term *= -x * x / ((2 * n) * (2 * n + 1)) as f64;
        sum += term;
    }
    sum
}

fn main() {
    let x = std::f64::consts::FRAC_PI_4;
    for t in 1..=9 {
        let err = (sin_taylor(x, t) - x.sin()).abs();
        println!("{t} terms: error = {err:e}");
    }
}
