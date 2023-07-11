use crate::{bindings::*, derivative::diff_bessel};
use num::complex::Complex64;

pub fn bessel_y(order: f64, z: Complex64) -> Result<Complex64, i32> {
    unsafe { _bessel_y(order, z) }
}

unsafe fn _bessel_y(order: f64, z: Complex64) -> Result<Complex64, i32> {
    let zr = z.re;
    let zi = z.im;
    let nu = order.abs();
    let kode = 1;
    let n = 1;

    let mut cyr = 0.0;
    let mut cyi = 0.0;
    let mut cwrkr = 0.0;
    let mut cwrki = 0.0;

    let mut nz = 0;
    let mut ierr = 0;

    let mut answer;

    zbesy_wrap(
        zr, zi, nu, kode, n, &mut cyr, &mut cyi, &mut nz, &mut cwrkr, &mut cwrki, &mut ierr,
    );

    if zi == 0.0 && zr == 0.0 {
        cyi = 0.0;
    }

    answer = Complex64::new(cyr, cyi);

    if ierr != 0 {
        Err(ierr)?;
    }

    if order < 0.0 {
        let c = nu.cos();
        let s = nu.sin();

        let mut cyrj = 0.0;
        let mut cyij = 0.0;

        let mut nz_j = 0;
        let mut ierrj = 0;

        let kodej = 1;
        let nj = 1;

        zbesj_wrap(
            zr, zi, nu, kodej, nj, &mut cyrj, &mut cyij, &mut nz_j, &mut ierrj,
        );

        if ierrj != 0 {
            Err(ierrj)?;
        }

        let answer_j = Complex64::new(cyrj, cyij);

        answer = s * answer_j + c * answer;

        return Ok(answer);
    }

    Ok(answer)
}

pub fn bessel_y_p(order: f64, z: Complex64, n: u32) -> Result<Complex64, i32> {
    diff_bessel(bessel_y, order, z, n as _, -1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex64;
    #[test]
    fn test_bessel_y() {
        let res = bessel_y(3.2, Complex64::new(3.4, -1.3)).unwrap();
        println!("{}", res);
    }

    #[test]
    fn test_bessel_y_d() {
        for i in 1..4 {
            let res = bessel_y_p(3.7, Complex64::new(3.1, 2.1), i).unwrap();

            println!("{}", res);
        }
    }
}
