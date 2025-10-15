use std::ffi::{c_double, c_int};
unsafe extern "C" {
    pub fn zbesy_wrap(
        zr: c_double,
        zi: c_double,
        nu: c_double,
        kode: c_int,
        N: c_int,
        cyr: *mut c_double,
        cyi: *mut c_double,
        nz: *mut c_int,
        cwrkr: *mut c_double,
        cwrki: *mut c_double,
        ierr: *mut c_int,
    );

    pub fn zbesj_wrap(
        zr: c_double,
        zi: c_double,
        nu: c_double,
        kode: c_int,
        n: c_int,
        cyr: *mut c_double,
        cyi: *mut c_double,
        nz: *mut c_int,
        ierr: *mut c_int,
    );

    pub fn zbesk_wrap(
        zr: c_double,
        zi: c_double,
        nu: c_double,
        kode: c_int,
        n: c_int,
        cyr: *mut c_double,
        cyi: *mut c_double,
        nz: *mut c_int,
        ierr: *mut c_int,
    );

    pub fn zbesi_wrap(
        zr: c_double,
        zi: c_double,
        nu: c_double,
        kode: c_int,
        n: c_int,
        cyr: *mut c_double,
        cyi: *mut c_double,
        nz: *mut c_int,
        ierr: *mut c_int,
    );
}
