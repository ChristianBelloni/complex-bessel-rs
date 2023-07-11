use crate::bindings::zbesk_wrap;
use num::complex::Complex64;

unsafe fn _bessel_k(order: f64, z: Complex64) -> Result<Complex64, i32> {
    let zr = z.re;
    let zi = z.im;
    let nu = order.abs();
    let kode = 1;
    let n = 1;

    let mut cyr = 0.0;
    let mut cyi = 0.0;
    let mut nz = 0;
    let mut ierr = 0;

    zbesk_wrap(zr, zi, nu, kode, n, &mut cyr, &mut cyi, &mut nz, &mut ierr);

    if zi == 0.0 && zr >= 0.0 {
        cyi = 0.0;
    }

    if ierr != 0 {
        Err(ierr)?;
    }

    Ok(Complex64::new(cyr, cyi))
}

/// Computes the value of the modified Bessel function of the second kind at z
///
/// # Examples
/// ```rust
///
/// # use complex_bessel::bessel_k::bessel_k;
/// # use num::complex::Complex64;
///
/// let res = bessel_k(2.1, Complex64::new(-2.0, 43.1));
///
/// assert_eq!(res.unwrap(), Complex64::new(1.4070411886914238, 0.03590109130313471));
/// # res.unwrap();
///
/// ```
pub fn bessel_k(order: f64, z: Complex64) -> Result<Complex64, i32> {
    unsafe { _bessel_k(order, z) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex64;

    #[test]
    fn test_bessel_k() {
        let res = bessel_k(2.1, Complex64::new(-2.0, 43.1)).unwrap();

        println!("{res}");
    }
}
