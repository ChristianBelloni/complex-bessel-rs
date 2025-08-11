#![allow(unused)]
use std::{path::PathBuf, process::Command};

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

#[cfg(not(doc))]
pub fn main() {
    let res = Command::new("gfortran")
        .arg("-print-file-name=libgfortran.a")
        .output()
        .unwrap()
        .stdout;

    if res.is_empty() {
        p!("gfortran -print-file-name=libgfortran.a returned an empty string!\n make sure you have gfortran13 installed");
        println!("cargo:rustc-link-search=libgfortran.a");
    } else {
        let path = String::from_utf8(res).unwrap();
        let path = PathBuf::from(path);
        let path = path.parent().unwrap();
        if !path.to_string_lossy().is_empty() {
            println!("cargo:rustc-link-search={}", path.display());
        }
    }

    println!("cargo:rustc-link-lib=gfortran");

    cc::Build::new()
        .files([
            "amos/amos_iso_c_fortran_wrapper.f90",
            "amos/machine.for",
            "amos/zbesh.for",
        ])
        .compiler("gfortran")
        // .flag("-std=legacy")
        // .flag("-fdefault-real-8") // use 8 bytes for all floats
        .flag("-Wno-maybe-uninitialized") // suppress the maybe-unitialized warnings
        .flag("-O3") // opitmize level 3
        .flag("-Wno-compare-reals")
        .flag("-Wno-intrinsic-shadow")
        .flag("-Wno-do-subscript")
        .flag("-Wno-unused-dummy-argument")
        .cpp_link_stdlib(None)
        // .flag("-lgfortran")
        .static_flag(true)
        .compile("amos");
}
