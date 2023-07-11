use num::complex::Complex64;

unsafe fn _bessel_j(order: f64, z: Complex64) -> Result<Complex64, i32> {
    let zr = z.re;
    let zi = z.im;
    let nu = order.abs();
    let kode = 1;
    let n = 1;

    let mut cyr = 0.0;
    let mut cyi = 0.0;
    let mut nz = 0;
    let mut ierr = 0;

    zbesj_wrap(zr, zi, nu, kode, n, &mut cyr, &mut cyi, &mut nz, &mut ierr);

    if zi == 0.0 && zr >= 0.0 {
        cyi = 0.0.into();
    }

    let mut answer = Complex64::new(cyr, cyi);

    if order < 0.0 {
        let c = nu.cos();
        let s = nu.sin();

        let mut cyr_y = 0.0;
        let mut cyi_y = 0.0;

        let mut cwrkr_y = 0.0;
        let mut cwrki_y = 0.0;

        let mut ierr_y = 0;

        let kode_y = 1;
        let n_y = 1;
        let mut nz_y = 0;

        zbesy_wrap(
            zr,
            zi,
            nu,
            kode_y,
            n_y,
            &mut cyr_y,
            &mut cyi_y,
            &mut nz_y,
            &mut cwrkr_y,
            &mut cwrki_y,
            &mut ierr_y,
        );

        if ierr_y != 0 {
            Err(ierr_y)?;
        }

        let answer_j = Complex64::new(cyr_y, cyi_y);

        answer = s * answer_j + c * answer;

        return Ok(answer);
    }

    if ierr != 0 {
        Err(ierr)?;
    }

    Ok(answer)
}

pub fn bessel_j(order: f64, z: Complex64) -> Result<Complex64, i32> {
    unsafe { _bessel_j(order, z) }
}

pub fn bessel_j_p(order: f64, z: Complex64, n: u32) -> Result<Complex64, i32> {
    diff_bessel(bessel_j, order, z, n as _, -1.0)
}

use crate::{
    bindings::{zbesj_wrap, zbesy_wrap},
    derivative::diff_bessel,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bessel_j() {
        let res = bessel_j(2.3, Complex64::new(-1.3, 7.9)).unwrap();

        println!("{}", res);
    }

    #[test]
    fn test_bessel_j_d() {
        for i in 1..4 {
            let res = bessel_j_p(3.7, Complex64::new(3.1, 2.1), i).unwrap();

            println!("{}", res);
        }
    }
}
