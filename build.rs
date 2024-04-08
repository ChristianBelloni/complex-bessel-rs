#![allow(unused)]
use std::{path::PathBuf, process::Command};

#[cfg(target_os = "windows")]
static PATH_FINDER_COMMAND: &str = "where";
#[cfg(target_os = "linux")]
static PATH_FINDER_COMMAND: &str = "which";

#[cfg(target_os = "windows")]
static GFORTRAN_NAME: &str = "gfortran";
#[cfg(target_os = "linux")]
static GFORTRAN_NAME: &str = "gfortran-13";

#[cfg(target_os = "macos")]
static GFORTRAN_NAME: &str = "gfortran-13";
#[cfg(target_os = "macos")]
static PATH_FINDER_COMMAND: &str = "which";

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

#[cfg(not(doc))]
pub fn main() {
    if std::env::var("DOCS_RS").is_ok() {
    } else {
        _ = get_fortran_compiler().expect(&format!("{} not installed!", GFORTRAN_NAME));
        let out_path: PathBuf = std::env::var("OUT_DIR").unwrap().into();
        let lib_name = "amos";
        let mut lib_path = out_path.clone();
        lib_path.push(format!("lib{lib_name}.a"));

        Command::new(GFORTRAN_NAME)
            .arg("-shared")
            .arg("-fPIC")
            .arg("amos/amos_iso_c_fortran_wrapper.f90")
            .arg("amos/machine.for")
            .arg("amos/zbesh.for")
            .arg("-o")
            .arg(lib_path.to_string_lossy().as_ref())
            .output()
            .expect(
                "failed to compile fortran library,\nare you sure you have gfortran-13 installed?",
            );
        println!("cargo:rustc-link-search={}", out_path.to_string_lossy());
        println!("cargo:rustc-link-lib=amos");
    }
}

#[cfg(doc)]
fn main() {}

fn get_fortran_compiler() -> Option<String> {
    Some(
        String::from_utf8_lossy(
            &Command::new(&PATH_FINDER_COMMAND)
                .arg(&GFORTRAN_NAME)
                .output()
                .ok()?
                .stdout,
        )
        .to_string(),
    )
}
