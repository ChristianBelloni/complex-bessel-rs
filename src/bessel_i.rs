use num::complex::Complex64;

use crate::bindings::{zbesi_wrap, zbesk_wrap};

pub fn bessel_i(order: f64, z: Complex64) -> Result<Complex64, i32> {
    unsafe { _bessel_i(order, z) }
}

unsafe fn _bessel_i(order: f64, z: Complex64) -> Result<Complex64, i32> {
    let zr = z.re;
    let zi = z.im;
    let nu = order.abs();
    let kode = 1;
    let n = 1;

    let mut cyr = 0.0;
    let mut cyi = 0.0;
    let mut nz = 0;
    let mut ierr = 0;

    unsafe {
        zbesi_wrap(zr, zi, nu, kode, n, &mut cyr, &mut cyi, &mut nz, &mut ierr);
    }

    if zi == 0.0 && zr >= 0.0 {
        cyi = 0.0;
    }

    let mut answer = Complex64::new(cyr, cyi);
    if order < 0.0 {
        let s = (std::f64::consts::PI * nu).sin();
        let mut nzk = 0;
        let mut ierrk = 0;
        let mut cyrk = 0.0;
        let mut cyik = 0.0;

        unsafe {
            zbesk_wrap(
                zr, zi, nu, kode, n, &mut cyrk, &mut cyik, &mut nzk, &mut ierrk,
            );
        }

        if zi == 0.0 && zr >= 0.0 {
            cyik = 0.0;
        }

        let answerk = Complex64::new(cyrk, cyik);

        answer += 2.0 / std::f64::consts::PI * s * answerk;

        if ierrk != 0 {
            return Err(ierrk);
        }
    }

    if ierr != 0 {
        return Err(ierr);
    }

    Ok(answer)
}
