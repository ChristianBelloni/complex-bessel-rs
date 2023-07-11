use num::complex::Complex64;

pub fn diff_bessel<F: Fn(f64, Complex64) -> Result<Complex64, i32>>(
    f: F,
    order: f64,
    z: Complex64,
    n: f64,
    phase: f64,
) -> Result<Complex64, i32> {
    let mut p = 1.0;

    let mut s = f(order - n, z)?;

    for i in 1..(n as _) {
        p = phase * (p * (n - (i as f64) + 1.0));

        s += p * f(order - n + 2.0 * (i as f64), z)?;
    }

    let answer = s / 2.0_f64.powf(n);
    Ok(answer)
}
