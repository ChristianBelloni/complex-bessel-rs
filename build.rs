#![allow(unused)]
use std::{path::PathBuf, process::Command};

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub fn main() {
    _ = get_fortran_compiler().expect("gfortran-13 not installed!");
    let out_path: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    let lib_name = "amos";
    let mut lib_path = out_path.clone();
    lib_path.push(format!("lib{lib_name}.a"));

    Command::new("gfortran-13")
        .arg("-shared")
        .arg("amos/amos_iso_c_fortran_wrapper.f90")
        .arg("amos/machine.for")
        .arg("amos/zbesh.for")
        .arg("-o")
        .arg(lib_path.to_string_lossy().as_ref())
        .output()
        .expect("failed to compile fortran library,\nare you sure you have gfortran-13 installed?");
    println!("cargo:rustc-link-search={}", out_path.to_string_lossy());
    println!("cargo:rustc-link-lib=amos");
}

fn get_fortran_compiler() -> Option<String> {
    Some(
        String::from_utf8_lossy(
            &Command::new("which")
                .arg("gfortran-13")
                .output()
                .ok()?
                .stdout,
        )
        .to_string(),
    )
}
